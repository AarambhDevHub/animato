//! Scroll-driven animation helpers.

use leptos::html;
use leptos::prelude::*;
#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
use std::cell::{Cell, RefCell};
use std::fmt;
#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
use std::rc::Rc;

/// Scroll axis used by scroll progress and drag helpers.
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

    fn apply_smoothing(&mut self, target: f32) -> f32 {
        let target = target.clamp(0.0, 1.0);
        self.current = if self.config.smooth {
            let factor = self.config.smooth_factor.clamp(0.0, 1.0);
            self.current + (target - self.current) * factor
        } else {
            target
        };

        self.current
    }

    #[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
    fn calculate_target(&mut self, target: f32) -> f32 {
        self.apply_smoothing(target)
    }

    /// Return whether an intersection ratio activates a trigger.
    pub fn triggered(ratio: f32, config: &ScrollTriggerConfig) -> bool {
        ratio >= config.threshold.clamp(0.0, 1.0)
    }
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

/// Scroll trigger handle.
#[derive(Clone)]
pub struct ScrollTriggerHandle {
    active: ReadSignal<bool>,
    set_active: WriteSignal<bool>,
    progress: ReadSignal<f32>,
    set_progress: WriteSignal<f32>,
    once: bool,
    fired: std::sync::Arc<std::sync::Mutex<bool>>,
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
    pub fn active(&self) -> ReadSignal<bool> {
        self.active
    }

    /// Progress signal.
    pub fn progress(&self) -> ReadSignal<f32> {
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
        self.set_active.set(active);
        self.set_progress.set(ratio.clamp(0.0, 1.0));
    }
}

/// Return scroll progress for a target element.
pub fn use_scroll_progress(target: NodeRef<html::Div>, config: ScrollConfig) -> ReadSignal<f32> {
    let (progress, set_progress) = signal(if crate::ssr::is_hydrating() { 1.0 } else { 0.0 });

    #[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
    if !crate::ssr::is_hydrating() {
        use leptos::ev;
        use leptos::leptos_dom::helpers::window_event_listener;

        let calculator = Rc::new(RefCell::new(ScrollProgressCalculator::new(config)));
        let update = Rc::new(move || {
            if let Some(value) = scroll_progress_from_target(target, &calculator) {
                set_progress.set(value);
            }
        });

        update();

        let on_scroll = Rc::clone(&update);
        let scroll_handle = window_event_listener(ev::scroll, move |_| on_scroll());
        let on_resize = Rc::clone(&update);
        let resize_handle = window_event_listener(ev::resize, move |_| on_resize());

        on_cleanup(move || {
            scroll_handle.remove();
            resize_handle.remove();
        });
    }

    #[cfg(not(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate"))))]
    let _ = (target, config, set_progress);

    progress
}

/// Return a scroll trigger handle for a target element.
pub fn use_scroll_trigger(
    target: NodeRef<html::Div>,
    config: ScrollTriggerConfig,
) -> ScrollTriggerHandle {
    let (active, set_active) = signal(false);
    let (progress, set_progress) = signal(0.0);
    let handle = ScrollTriggerHandle {
        active,
        set_active,
        progress,
        set_progress,
        once: config.once,
        fired: std::sync::Arc::new(std::sync::Mutex::new(false)),
    };

    #[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
    if !crate::ssr::is_hydrating() {
        use leptos::ev;
        use leptos::leptos_dom::helpers::window_event_listener;

        let update_config = config.clone();
        let update_handle = handle.clone();
        let update = Rc::new(move || {
            if let Some(ratio) = intersection_ratio(target, update_config.pin) {
                update_handle.update_ratio(ratio, &update_config);
            }
        });

        update();

        let on_scroll = Rc::clone(&update);
        let scroll_handle = window_event_listener(ev::scroll, move |_| on_scroll());
        let on_resize = Rc::clone(&update);
        let resize_handle = window_event_listener(ev::resize, move |_| on_resize());

        on_cleanup(move || {
            scroll_handle.remove();
            resize_handle.remove();
        });
    }

    #[cfg(not(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate"))))]
    let _ = (target, config);

    handle
}

/// Return the current scroll velocity in pixels per second.
pub fn use_scroll_velocity() -> ReadSignal<f32> {
    let (velocity, set_velocity) = signal(0.0);

    #[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
    if !crate::ssr::is_hydrating() {
        use leptos::ev;
        use leptos::leptos_dom::helpers::window_event_listener;

        let last_position = Rc::new(Cell::new(window_scroll_position(ScrollAxis::Vertical)));
        let last_time = Rc::new(Cell::new(crate::now_ms()));

        let scroll_handle = {
            let last_position = Rc::clone(&last_position);
            let last_time = Rc::clone(&last_time);
            window_event_listener(ev::scroll, move |_| {
                let now = crate::now_ms();
                let position = window_scroll_position(ScrollAxis::Vertical);
                let dt = ((now - last_time.replace(now)) / 1000.0).max(0.0) as f32;
                let previous = last_position.replace(position);
                let value = if dt > 0.0 {
                    (position - previous) / dt
                } else {
                    0.0
                };
                set_velocity.set(crate::finite_or(value, 0.0));
            })
        };

        on_cleanup(move || scroll_handle.remove());
    }

    #[cfg(not(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate"))))]
    let _ = set_velocity;

    velocity
}

/// Momentum scroll container.
#[component]
pub fn SmoothScroll(
    /// Child content.
    children: Children,
) -> impl IntoView {
    let child = children();
    view! {
        <div
            data-animato-smooth-scroll="true"
            style="overflow:auto; overscroll-behavior:contain; scroll-behavior:smooth;"
        >
            {child}
        </div>
    }
}

#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
fn scroll_progress_from_target(
    target: NodeRef<html::Div>,
    calculator: &Rc<RefCell<ScrollProgressCalculator>>,
) -> Option<f32> {
    let element = target.get_untracked()?;
    let rect = element.get_bounding_client_rect();
    let config = calculator.borrow().config.clone();
    let target = match config.axis {
        ScrollAxis::Vertical => scroll_progress_target(
            &config,
            rect.top() as f32 + window_scroll_position(ScrollAxis::Vertical),
            rect.height() as f32,
            viewport_size(ScrollAxis::Vertical),
            window_scroll_position(ScrollAxis::Vertical),
        ),
        ScrollAxis::Horizontal => scroll_progress_target(
            &config,
            rect.left() as f32 + window_scroll_position(ScrollAxis::Horizontal),
            rect.width() as f32,
            viewport_size(ScrollAxis::Horizontal),
            window_scroll_position(ScrollAxis::Horizontal),
        ),
        ScrollAxis::Both => {
            let vertical = scroll_progress_target(
                &config,
                rect.top() as f32 + window_scroll_position(ScrollAxis::Vertical),
                rect.height() as f32,
                viewport_size(ScrollAxis::Vertical),
                window_scroll_position(ScrollAxis::Vertical),
            );
            let horizontal = scroll_progress_target(
                &config,
                rect.left() as f32 + window_scroll_position(ScrollAxis::Horizontal),
                rect.width() as f32,
                viewport_size(ScrollAxis::Horizontal),
                window_scroll_position(ScrollAxis::Horizontal),
            );
            vertical.max(horizontal)
        }
    };

    Some(calculator.borrow_mut().calculate_target(target))
}

#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
fn intersection_ratio(target: NodeRef<html::Div>, _pin: bool) -> Option<f32> {
    let element = target.get_untracked()?;
    let rect = element.get_bounding_client_rect();
    let viewport = viewport_size(ScrollAxis::Vertical);
    let element_size = (rect.height() as f32).max(f32::EPSILON);
    let visible_start = (rect.top() as f32).max(0.0);
    let visible_end = (rect.bottom() as f32).min(viewport);
    let visible = (visible_end - visible_start).max(0.0);
    Some((visible / element_size).clamp(0.0, 1.0))
}

#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
fn viewport_size(axis: ScrollAxis) -> f32 {
    let window = leptos::prelude::window();
    let value = match axis {
        ScrollAxis::Vertical | ScrollAxis::Both => window.inner_height(),
        ScrollAxis::Horizontal => window.inner_width(),
    };

    value
        .ok()
        .and_then(|value| value.as_f64())
        .map(|value| value as f32)
        .filter(|value| *value > 0.0)
        .unwrap_or(1.0)
}

#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
fn window_scroll_position(axis: ScrollAxis) -> f32 {
    let window = leptos::prelude::window();
    let value = match axis {
        ScrollAxis::Vertical | ScrollAxis::Both => window.scroll_y(),
        ScrollAxis::Horizontal => window.scroll_x(),
    };

    value.unwrap_or(0.0) as f32
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::{Get, Owner};

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
    fn trigger_threshold_activates() {
        let config = ScrollTriggerConfig {
            threshold: 0.5,
            ..ScrollTriggerConfig::default()
        };
        assert!(!ScrollProgressCalculator::triggered(0.49, &config));
        assert!(ScrollProgressCalculator::triggered(0.5, &config));
    }

    #[test]
    fn trigger_handle_respects_once() {
        Owner::new().with(|| {
            let config = ScrollTriggerConfig {
                once: true,
                threshold: 0.5,
                ..ScrollTriggerConfig::default()
            };
            let handle = use_scroll_trigger(NodeRef::new(), config.clone());
            handle.update_ratio(0.7, &config);
            assert!(handle.active().get());
            handle.update_ratio(0.0, &config);
            assert!(handle.active().get());
        });
    }
}
