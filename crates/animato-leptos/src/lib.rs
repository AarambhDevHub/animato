//! # animato-leptos
//!
//! First-class Leptos integration for Animato.
//!
//! The crate provides signal-backed animation hooks, scroll helpers,
//! mount/unmount-style wrappers, FLIP list scaffolding, gesture bindings, CSS
//! style interpolation, and SSR-aware guards. Browser hooks start from Leptos
//! signals and drive Animato's renderer-agnostic animation engines from an rAF
//! loop; server-side hooks return static final values and never touch browser
//! APIs.

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

#[cfg(feature = "css")]
mod css;
#[cfg(feature = "gesture")]
mod gesture;
mod hooks;
#[cfg(feature = "list")]
mod list;
#[cfg(feature = "presence")]
mod presence;
#[cfg(feature = "scroll")]
mod scroll;
mod ssr;
#[cfg(feature = "transition")]
mod transition;

#[cfg(feature = "css")]
pub use css::{AnimatedStyle, css_spring, css_tween};
#[cfg(feature = "gesture")]
pub use gesture::{
    DragConfig, DragHandle, PinchHandle, SwipeConfig, SwipeEvent, use_drag, use_gesture, use_pinch,
    use_swipe,
};
pub use hooks::{
    KeyframeHandle, SpringHandle, TimelineHandle, TweenHandle, use_keyframes, use_spring,
    use_timeline, use_tween,
};
#[cfg(feature = "list")]
pub use list::AnimatedFor;
#[cfg(feature = "presence")]
pub use presence::{AnimatePresence, PresenceAnimation};
#[cfg(feature = "scroll")]
pub use scroll::{
    ScrollAxis, ScrollConfig, ScrollProgressCalculator, ScrollTriggerConfig, ScrollTriggerHandle,
    SmoothScroll, use_scroll_progress, use_scroll_trigger, use_scroll_velocity,
};
pub use ssr::{SsrFallback, is_hydrating, use_client_only};
#[cfg(feature = "transition")]
pub use transition::{PageTransition, TransitionMode};

pub(crate) fn finite_or(value: f32, fallback: f32) -> f32 {
    if value.is_finite() { value } else { fallback }
}

#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
pub(crate) fn now_ms() -> f64 {
    leptos::prelude::window()
        .performance()
        .map(|performance| performance.now())
        .unwrap_or(0.0)
}

#[cfg(not(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate"))))]
#[allow(dead_code)]
pub(crate) fn now_ms() -> f64 {
    0.0
}

#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
pub(crate) fn spawn_raf_loop(mut tick: impl FnMut(f32) -> bool + 'static) {
    use leptos::prelude::{is_browser, on_cleanup, request_animation_frame_with_handle};
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;
    use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    };

    if !is_browser() {
        return;
    }

    let cancelled = Arc::new(AtomicBool::new(false));
    let cleanup_cancelled = Arc::clone(&cancelled);
    on_cleanup(move || cleanup_cancelled.store(true, Ordering::Relaxed));

    let last_timestamp = Rc::new(Cell::new(None::<f64>));
    let callback: Rc<RefCell<Option<Box<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let callback_ref = Rc::clone(&callback);
    let cancelled_ref = Arc::clone(&cancelled);
    let last_ref = Rc::clone(&last_timestamp);

    *callback.borrow_mut() = Some(Box::new(move || {
        if cancelled_ref.load(Ordering::Relaxed) {
            return;
        }

        let now = now_ms();
        let dt = last_ref
            .replace(Some(now))
            .map(|last| ((now - last) / 1000.0).max(0.0) as f32)
            .unwrap_or(0.0)
            .min(0.25);

        let keep_running = tick(dt);
        if keep_running && !cancelled_ref.load(Ordering::Relaxed) {
            let next = Rc::clone(&callback_ref);
            let _ = request_animation_frame_with_handle(move || {
                if let Some(callback) = next.borrow_mut().as_mut() {
                    callback();
                }
            });
        }
    }));

    let first = Rc::clone(&callback);
    let _ = request_animation_frame_with_handle(move || {
        if let Some(callback) = first.borrow_mut().as_mut() {
            callback();
        }
    });
}

#[cfg(not(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate"))))]
pub(crate) fn spawn_raf_loop(_tick: impl FnMut(f32) -> bool + 'static) {}

pub(crate) fn with_lock<T, R>(
    value: &std::sync::Arc<std::sync::Mutex<T>>,
    f: impl FnOnce(&mut T) -> R,
) -> R {
    let mut guard = value
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    f(&mut guard)
}
