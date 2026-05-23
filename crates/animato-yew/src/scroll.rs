//! Scroll-driven animation helpers.

use std::cell::Cell;
use std::fmt;
use std::rc::Rc;
use yew::prelude::{
    Children, Html, NodeRef, Properties, UseStateHandle, function_component, hook, html,
    use_state_eq,
};

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

    #[cfg(target_arch = "wasm32")]
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
    active: UseStateHandle<bool>,
    progress: UseStateHandle<f32>,
    once: bool,
    fired: Rc<Cell<bool>>,
}

impl fmt::Debug for ScrollTriggerHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ScrollTriggerHandle")
            .field("once", &self.once)
            .field("fired", &self.fired.get())
            .finish_non_exhaustive()
    }
}

impl ScrollTriggerHandle {
    /// Active-state handle.
    pub fn active(&self) -> UseStateHandle<bool> {
        self.active.clone()
    }

    /// Progress state handle.
    pub fn progress(&self) -> UseStateHandle<f32> {
        self.progress.clone()
    }

    /// Update active state from an intersection ratio.
    pub fn update_ratio(&self, ratio: f32, config: &ScrollTriggerConfig) {
        if self.once && self.fired.get() {
            return;
        }

        let active = ScrollProgressCalculator::triggered(ratio, config);
        if active {
            self.fired.set(true);
        }
        set_if_changed(&self.active, active);
        set_if_changed(&self.progress, ratio.clamp(0.0, 1.0));
    }
}

/// Return scroll progress for a target element.
#[hook]
pub fn use_scroll_progress(target: NodeRef, config: ScrollConfig) -> UseStateHandle<f32> {
    let progress = use_state_eq(|| 0.0);

    #[cfg(target_arch = "wasm32")]
    {
        let progress = progress.clone();
        yew::prelude::use_effect_with((), move |_| {
            use std::cell::RefCell;

            let calculator = Rc::new(RefCell::new(ScrollProgressCalculator::new(config)));
            let update = Rc::new(move || {
                if let Some(value) = scroll_progress_from_target(&target, &calculator) {
                    set_if_changed(&progress, value);
                }
            });

            update();
            let scroll = WindowListener::new("scroll", {
                let update = Rc::clone(&update);
                move |_| update()
            });
            let resize = WindowListener::new("resize", {
                let update = Rc::clone(&update);
                move |_| update()
            });

            move || drop((scroll, resize))
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    let _ = (target, config);

    progress
}

/// Return a scroll trigger handle for a target element.
#[hook]
pub fn use_scroll_trigger(target: NodeRef, config: ScrollTriggerConfig) -> ScrollTriggerHandle {
    let active = use_state_eq(|| false);
    let progress = use_state_eq(|| 0.0);
    let handle = ScrollTriggerHandle {
        active,
        progress,
        once: config.once,
        fired: Rc::new(Cell::new(false)),
    };

    #[cfg(target_arch = "wasm32")]
    {
        let handle = handle.clone();
        yew::prelude::use_effect_with((), move |_| {
            let update_config = config.clone();
            let update = Rc::new(move || {
                if let Some(ratio) = intersection_ratio(&target, update_config.pin) {
                    handle.update_ratio(ratio, &update_config);
                }
            });

            update();
            let scroll = WindowListener::new("scroll", {
                let update = Rc::clone(&update);
                move |_| update()
            });
            let resize = WindowListener::new("resize", {
                let update = Rc::clone(&update);
                move |_| update()
            });

            move || drop((scroll, resize))
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    let _ = (target, config);

    handle
}

/// Return the current scroll velocity in pixels per second.
#[hook]
pub fn use_scroll_velocity() -> UseStateHandle<f32> {
    let velocity = use_state_eq(|| 0.0);

    #[cfg(target_arch = "wasm32")]
    {
        let velocity = velocity.clone();
        yew::prelude::use_effect_with((), move |_| {
            let last_position = Rc::new(Cell::new(window_scroll_position(ScrollAxis::Vertical)));
            let last_time = Rc::new(Cell::new(crate::now_ms()));

            let listener = WindowListener::new("scroll", {
                let last_position = Rc::clone(&last_position);
                let last_time = Rc::clone(&last_time);
                move |_| {
                    let now = crate::now_ms();
                    let position = window_scroll_position(ScrollAxis::Vertical);
                    let dt = ((now - last_time.replace(now)) / 1000.0).max(0.0) as f32;
                    let previous = last_position.replace(position);
                    let value = if dt > 0.0 {
                        (position - previous) / dt
                    } else {
                        0.0
                    };
                    set_if_changed(&velocity, crate::finite_or(value, 0.0));
                }
            });

            move || drop(listener)
        });
    }

    velocity
}

/// Properties for [`SmoothScroll`].
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct SmoothScrollProps {
    /// Child content.
    pub children: Children,
}

/// Momentum scroll container.
#[function_component(SmoothScroll)]
pub fn smooth_scroll(props: &SmoothScrollProps) -> Html {
    html! {
        <div
            data-animato-smooth-scroll="true"
            style="overflow:auto; overscroll-behavior:contain; scroll-behavior:smooth;"
        >
            { for props.children.iter() }
        </div>
    }
}

fn set_if_changed<T>(state: &UseStateHandle<T>, next: T)
where
    T: PartialEq + 'static,
{
    if **state != next {
        state.set(next);
    }
}

#[cfg(target_arch = "wasm32")]
fn scroll_progress_from_target(
    target: &NodeRef,
    calculator: &Rc<std::cell::RefCell<ScrollProgressCalculator>>,
) -> Option<f32> {
    let element = target.cast::<web_sys::Element>()?;
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

#[cfg(target_arch = "wasm32")]
fn intersection_ratio(target: &NodeRef, _pin: bool) -> Option<f32> {
    let element = target.cast::<web_sys::Element>()?;
    let rect = element.get_bounding_client_rect();
    let viewport = viewport_size(ScrollAxis::Vertical);
    let element_size = (rect.height() as f32).max(f32::EPSILON);
    let visible_start = (rect.top() as f32).max(0.0);
    let visible_end = (rect.bottom() as f32).min(viewport);
    let visible = (visible_end - visible_start).max(0.0);
    Some((visible / element_size).clamp(0.0, 1.0))
}

#[cfg(target_arch = "wasm32")]
fn viewport_size(axis: ScrollAxis) -> f32 {
    let Some(window) = web_sys::window() else {
        return 1.0;
    };
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

#[cfg(target_arch = "wasm32")]
fn window_scroll_position(axis: ScrollAxis) -> f32 {
    let Some(window) = web_sys::window() else {
        return 0.0;
    };
    let value = match axis {
        ScrollAxis::Vertical | ScrollAxis::Both => window.scroll_y(),
        ScrollAxis::Horizontal => window.scroll_x(),
    };

    value.unwrap_or(0.0) as f32
}

#[cfg(target_arch = "wasm32")]
struct WindowListener {
    target: web_sys::EventTarget,
    event: &'static str,
    closure: wasm_bindgen::closure::Closure<dyn FnMut(web_sys::Event)>,
}

#[cfg(target_arch = "wasm32")]
impl WindowListener {
    fn new(event: &'static str, mut f: impl FnMut(web_sys::Event) + 'static) -> Self {
        use wasm_bindgen::JsCast;

        let target: web_sys::EventTarget = web_sys::window()
            .expect("window is available while installing a browser listener")
            .unchecked_into();
        let closure = wasm_bindgen::closure::Closure::wrap(
            Box::new(move |event| f(event)) as Box<dyn FnMut(web_sys::Event)>
        );
        let _ = target.add_event_listener_with_callback(event, closure.as_ref().unchecked_ref());
        Self {
            target,
            event,
            closure,
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl Drop for WindowListener {
    fn drop(&mut self) {
        use wasm_bindgen::JsCast;

        let _ = self
            .target
            .remove_event_listener_with_callback(self.event, self.closure.as_ref().unchecked_ref());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn calculator_respects_offsets_axis_and_smoothing_bounds() {
        let horizontal = ScrollConfig {
            axis: ScrollAxis::Horizontal,
            offset_start: 0.25,
            offset_end: 0.75,
            smooth: false,
            smooth_factor: 2.0,
        };
        let both = ScrollConfig {
            axis: ScrollAxis::Both,
            smooth: true,
            smooth_factor: -1.0,
            ..ScrollConfig::default()
        };
        let trigger = ScrollTriggerConfig {
            threshold: 2.0,
            once: true,
            start: "center bottom".to_owned(),
            end: "center top".to_owned(),
            scrub: true,
            pin: true,
        };

        let progress = scroll_progress_target(&horizontal, 200.0, 100.0, 400.0, 0.0);
        assert!((progress - (1.0 / 3.0)).abs() < 0.001);
        assert!(!ScrollProgressCalculator::triggered(0.99, &trigger));
        assert!(ScrollProgressCalculator::triggered(1.0, &trigger));

        let mut calc = ScrollProgressCalculator::new(both);
        assert_eq!(calc.calculate(100.0, 100.0, 100.0, 50.0), 0.0);
    }

    #[test]
    fn default_configs_are_stable_and_debuggable() {
        let scroll = ScrollConfig::default();
        let trigger = ScrollTriggerConfig::default();
        let calc = ScrollProgressCalculator::new(scroll.clone());

        assert_eq!(scroll.axis, ScrollAxis::Vertical);
        assert_eq!(trigger.start, "top bottom");
        assert_eq!(trigger.end, "bottom top");
        assert!(!trigger.once);
        assert!(!trigger.scrub);
        assert!(!trigger.pin);
        assert!(format!("{calc:?}").contains("ScrollProgressCalculator"));
        assert_eq!(format!("{:?}", ScrollAxis::Both), "Both");
    }
}
