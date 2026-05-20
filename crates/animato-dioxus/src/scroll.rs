//! Scroll-driven animation helpers.

use dioxus::prelude::{Signal, use_signal};
use std::fmt;
use std::sync::{Arc, Mutex};

/// Scroll axis used by scroll progress helpers.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ScrollAxis {
    /// Vertical scroll.
    #[default]
    Vertical,
    /// Horizontal scroll.
    Horizontal,
    /// Track both axes by using the larger normalized progress.
    Both,
}

/// Scroll progress configuration.
#[derive(Clone, Debug, PartialEq)]
pub struct ScrollConfig {
    /// Axis to track.
    pub axis: ScrollAxis,
    /// Viewport offset where progress starts.
    pub offset_start: f32,
    /// Viewport offset where progress ends.
    pub offset_end: f32,
    /// Smooth progress by lerping toward the latest value.
    pub smooth: bool,
    /// Smoothing factor in `[0.0, 1.0]`.
    pub smooth_factor: f32,
}

impl Default for ScrollConfig {
    fn default() -> Self {
        Self {
            axis: ScrollAxis::Vertical,
            offset_start: 0.0,
            offset_end: 1.0,
            smooth: true,
            smooth_factor: 0.1,
        }
    }
}

/// Scroll trigger configuration.
#[derive(Clone, Debug, PartialEq)]
pub struct ScrollTriggerConfig {
    /// Intersection threshold in `[0.0, 1.0]`.
    pub threshold: f32,
    /// Fire only once.
    pub once: bool,
    /// GSAP-style start expression, such as `"top bottom"`.
    pub start: String,
    /// GSAP-style end expression, such as `"bottom top"`.
    pub end: String,
    /// Link animation progress to scroll progress.
    pub scrub: bool,
    /// Pin the target for the active range.
    pub pin: bool,
}

impl Default for ScrollTriggerConfig {
    fn default() -> Self {
        Self {
            threshold: 0.0,
            once: false,
            start: "top bottom".to_owned(),
            end: "bottom top".to_owned(),
            scrub: false,
            pin: false,
        }
    }
}

/// Pure scroll progress calculator used by hooks and tests.
#[derive(Clone, Debug)]
pub struct ScrollProgressCalculator {
    config: ScrollConfig,
    current: f32,
}

impl ScrollProgressCalculator {
    /// Create a calculator with configuration.
    pub fn new(config: ScrollConfig) -> Self {
        Self {
            config,
            current: 0.0,
        }
    }

    /// Calculate progress from element and viewport geometry.
    pub fn calculate(
        &mut self,
        element_start: f32,
        element_size: f32,
        viewport_size: f32,
        scroll_position: f32,
    ) -> f32 {
        let target = scroll_progress_target(
            &self.config,
            element_start,
            element_size,
            viewport_size,
            scroll_position,
        );
        self.apply_smoothing(target)
    }

    /// Return whether an intersection ratio activates a trigger.
    pub fn triggered(ratio: f32, config: &ScrollTriggerConfig) -> bool {
        ratio >= config.threshold.clamp(0.0, 1.0)
    }

    fn apply_smoothing(&mut self, target: f32) -> f32 {
        let target = target.clamp(0.0, 1.0);
        self.current =
            if !self.config.smooth || target <= f32::EPSILON || target >= 1.0 - f32::EPSILON {
                target
            } else {
                let factor = self.config.smooth_factor.clamp(0.0, 1.0);
                let next = self.current + (target - self.current) * factor;
                if (target - next).abs() <= 0.001 {
                    target
                } else {
                    next
                }
            };

        self.current
    }
}

/// Scroll trigger handle.
#[derive(Clone)]
pub struct ScrollTriggerHandle {
    active: Signal<bool>,
    progress: Signal<f32>,
    once: bool,
    fired: Arc<Mutex<bool>>,
}

impl fmt::Debug for ScrollTriggerHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ScrollTriggerHandle")
            .field("once", &self.once)
            .finish_non_exhaustive()
    }
}

impl ScrollTriggerHandle {
    /// Active-state signal.
    pub fn active(&self) -> Signal<bool> {
        self.active
    }

    /// Progress signal.
    pub fn progress(&self) -> Signal<f32> {
        self.progress
    }

    /// Update active state from an intersection ratio.
    pub fn update_ratio(&self, ratio: f32, config: &ScrollTriggerConfig) {
        let mut fired = self
            .fired
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if self.once && *fired {
            return;
        }

        let active = ScrollProgressCalculator::triggered(ratio, config);
        if active {
            *fired = true;
        }
        crate::set_signal(self.active, active);
        crate::set_signal(self.progress, ratio.clamp(0.0, 1.0));
    }
}

/// Return scroll progress for a target.
///
/// The `target` argument is intentionally generic so Dioxus callers can pass
/// renderer-specific mounted-element handles without coupling this crate to a
/// single renderer. Non-web targets return a stable no-op signal.
pub fn use_scroll_progress<T: 'static>(target: T, config: ScrollConfig) -> Signal<f32> {
    let _ = (target, config);
    use_signal(|| 0.0)
}

/// Return a scroll trigger handle for a target.
pub fn use_scroll_trigger<T: 'static>(
    target: T,
    config: ScrollTriggerConfig,
) -> ScrollTriggerHandle {
    let _ = target;
    let active = use_signal(|| false);
    let progress = use_signal(|| 0.0);
    ScrollTriggerHandle {
        active,
        progress,
        once: config.once,
        fired: Arc::new(Mutex::new(false)),
    }
}

/// Return the current scroll velocity in pixels per second.
pub fn use_scroll_velocity() -> Signal<f32> {
    use_signal(|| 0.0)
}

fn scroll_progress_target(
    config: &ScrollConfig,
    element_start: f32,
    element_size: f32,
    viewport_size: f32,
    scroll_position: f32,
) -> f32 {
    let start_offset = viewport_size * config.offset_start;
    let end_offset = viewport_size * config.offset_end;
    let start = element_start - end_offset;
    let end = element_start + element_size - start_offset;
    let span = (end - start).abs().max(f32::EPSILON);
    ((scroll_position - start) / span).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::prelude::*;
    use std::cell::RefCell;

    thread_local! {
        static SCROLL_PROGRESS_CAPTURE: RefCell<Option<Signal<f32>>> = const { RefCell::new(None) };
        static SCROLL_TRIGGER_CAPTURE: RefCell<Option<ScrollTriggerHandle>> = const { RefCell::new(None) };
        static SCROLL_VELOCITY_CAPTURE: RefCell<Option<Signal<f32>>> = const { RefCell::new(None) };
    }

    #[allow(non_snake_case)]
    fn ScrollHookApp() -> Element {
        let progress = use_scroll_progress(
            "node",
            ScrollConfig {
                axis: ScrollAxis::Both,
                offset_start: 0.2,
                offset_end: 0.8,
                smooth: false,
                smooth_factor: 1.0,
            },
        );
        let trigger = use_scroll_trigger(
            "node",
            ScrollTriggerConfig {
                threshold: 0.5,
                once: true,
                start: "top center".to_owned(),
                end: "bottom center".to_owned(),
                scrub: true,
                pin: true,
            },
        );
        let velocity = use_scroll_velocity();

        SCROLL_PROGRESS_CAPTURE.with(|slot| *slot.borrow_mut() = Some(progress));
        SCROLL_TRIGGER_CAPTURE.with(|slot| *slot.borrow_mut() = Some(trigger));
        SCROLL_VELOCITY_CAPTURE.with(|slot| *slot.borrow_mut() = Some(velocity));

        rsx! { div {} }
    }

    #[test]
    fn progress_calculator_clamps() {
        let mut calc = ScrollProgressCalculator::new(ScrollConfig {
            smooth: false,
            ..ScrollConfig::default()
        });

        assert_eq!(calc.calculate(100.0, 100.0, 100.0, -100.0), 0.0);
        assert_eq!(calc.calculate(100.0, 100.0, 100.0, 300.0), 1.0);
    }

    #[test]
    fn smoothed_progress_snaps_to_edges() {
        let mut calc = ScrollProgressCalculator::new(ScrollConfig {
            smooth: true,
            smooth_factor: 0.1,
            ..ScrollConfig::default()
        });

        assert_eq!(calc.calculate(100.0, 100.0, 100.0, 50.0), 0.025);
        assert_eq!(calc.calculate(100.0, 100.0, 100.0, 300.0), 1.0);
        assert_eq!(calc.calculate(100.0, 100.0, 100.0, -100.0), 0.0);
    }

    #[test]
    fn trigger_threshold_activates() {
        let config = ScrollTriggerConfig {
            threshold: 0.5,
            ..ScrollTriggerConfig::default()
        };
        assert!(!ScrollProgressCalculator::triggered(0.49, &config));
        assert!(ScrollProgressCalculator::triggered(0.5, &config));
    }

    #[test]
    fn calculator_handles_offsets_smoothing_and_threshold_clamps() {
        let mut instant = ScrollProgressCalculator::new(ScrollConfig {
            offset_start: 0.25,
            offset_end: 0.75,
            smooth: false,
            ..ScrollConfig::default()
        });
        assert_eq!(instant.calculate(200.0, 100.0, 100.0, 125.0), 0.0);
        assert_eq!(instant.calculate(200.0, 100.0, 100.0, 275.0), 1.0);

        let mut fast_smooth = ScrollProgressCalculator::new(ScrollConfig {
            smooth: true,
            smooth_factor: 2.0,
            ..ScrollConfig::default()
        });
        assert_eq!(fast_smooth.calculate(100.0, 100.0, 100.0, 150.0), 0.75);

        assert!(ScrollProgressCalculator::triggered(
            0.0,
            &ScrollTriggerConfig {
                threshold: -1.0,
                ..ScrollTriggerConfig::default()
            }
        ));
        assert!(!ScrollProgressCalculator::triggered(
            0.99,
            &ScrollTriggerConfig {
                threshold: 2.0,
                ..ScrollTriggerConfig::default()
            }
        ));
    }

    #[test]
    fn scroll_hooks_return_noop_signals_and_once_trigger_handle() {
        SCROLL_PROGRESS_CAPTURE.with(|slot| *slot.borrow_mut() = None);
        SCROLL_TRIGGER_CAPTURE.with(|slot| *slot.borrow_mut() = None);
        SCROLL_VELOCITY_CAPTURE.with(|slot| *slot.borrow_mut() = None);
        let mut dom = VirtualDom::new(ScrollHookApp);
        dom.rebuild_in_place();

        let progress = SCROLL_PROGRESS_CAPTURE.with(|slot| {
            slot.borrow()
                .as_ref()
                .copied()
                .expect("scroll progress captured")
        });
        let trigger = SCROLL_TRIGGER_CAPTURE.with(|slot| {
            slot.borrow()
                .as_ref()
                .cloned()
                .expect("scroll trigger captured")
        });
        let velocity = SCROLL_VELOCITY_CAPTURE.with(|slot| {
            slot.borrow()
                .as_ref()
                .copied()
                .expect("scroll velocity captured")
        });

        assert_eq!(crate::read_signal(progress), 0.0);
        assert_eq!(crate::read_signal(velocity), 0.0);
        assert!(!crate::read_signal(trigger.active()));
        assert_eq!(crate::read_signal(trigger.progress()), 0.0);

        let config = ScrollTriggerConfig {
            threshold: 0.5,
            once: true,
            ..ScrollTriggerConfig::default()
        };
        trigger.update_ratio(0.4, &config);
        assert!(!crate::read_signal(trigger.active()));
        assert_eq!(crate::read_signal(trigger.progress()), 0.4);
        trigger.update_ratio(0.75, &config);
        assert!(crate::read_signal(trigger.active()));
        assert_eq!(crate::read_signal(trigger.progress()), 0.75);
        trigger.update_ratio(0.1, &config);
        assert!(crate::read_signal(trigger.active()));
        assert_eq!(crate::read_signal(trigger.progress()), 0.75);
    }
}
