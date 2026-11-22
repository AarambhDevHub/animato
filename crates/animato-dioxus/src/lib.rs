//! # animato-dioxus
//!
//! First-class Dioxus integration for Animato.
//!
//! This crate exposes Dioxus `Signal`-backed hooks, style helpers,
//! presence/list/transition components, gesture bindings, platform detection,
//! and portable native-window animation handles. The animation engines remain
//! the renderer-agnostic Animato tween, spring, timeline, keyframe, and physics
//! types; Dioxus only owns reactive state and component rendering.

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

use dioxus::prelude::{ReadableExt, WritableExt};
use std::sync::{Arc, Mutex};

#[cfg(all(target_arch = "wasm32", feature = "web"))]
use std::cell::RefCell;
#[cfg(not(target_arch = "wasm32"))]
use std::future::Future;
#[cfg(not(target_arch = "wasm32"))]
use std::pin::Pin;
#[cfg(all(target_arch = "wasm32", feature = "web"))]
use std::rc::Rc;
#[cfg(not(target_arch = "wasm32"))]
use std::task::{Context, Poll};

#[cfg(feature = "css")]
mod css;
#[cfg(feature = "gesture")]
mod gesture;
mod hooks;
#[cfg(feature = "list")]
mod list;
#[cfg(feature = "motion")]
mod motion;
#[cfg(feature = "native")]
mod native;
#[cfg(feature = "platform")]
mod platform;
#[cfg(feature = "presence")]
mod presence;
#[cfg(feature = "scroll")]
mod scroll;
#[cfg(feature = "transition")]
mod transition;

#[cfg(feature = "css")]
pub use css::{AnimatedStyle, css_spring, css_tween};
#[cfg(feature = "gesture")]
pub use gesture::{
    DragAxis, DragConfig, DragConstraints, DragHandle, Gesture, GestureConfig, PinchHandle,
    SwipeConfig, SwipeDirection, SwipeEvent, use_drag, use_gesture, use_pinch, use_swipe,
};
pub use hooks::{
    KeyframeHandle, SpringHandle, TimelineHandle, TweenHandle, use_keyframes, use_spring,
    use_timeline, use_tween,
};
#[cfg(feature = "list")]
pub use list::{AnimatedFor, stable_key};
#[cfg(feature = "motion")]
pub use motion::{MotionConfig, MotionHandle, use_motion};
#[cfg(feature = "native")]
pub use native::{
    NativeWindowState, WindowAnimationHandle, WindowSpringHandle, use_window_animation,
    use_window_spring,
};
#[cfg(feature = "platform")]
pub use platform::{AnimationBackend, PlatformAdapter};
#[cfg(feature = "presence")]
pub use presence::{AnimatePresence, PresenceAnimation};
#[cfg(feature = "scroll")]
pub use scroll::{
    ScrollAxis, ScrollConfig, ScrollProgressCalculator, ScrollTriggerConfig, ScrollTriggerHandle,
    use_scroll_progress, use_scroll_trigger, use_scroll_velocity,
};
#[cfg(feature = "transition")]
pub use transition::{PageTransition, TransitionMode, route_transition_key};

pub(crate) fn finite_or(value: f32, fallback: f32) -> f32 {
    if value.is_finite() { value } else { fallback }
}

pub(crate) fn set_signal<T: 'static>(signal: dioxus::prelude::Signal<T>, value: T) {
    let mut signal = signal;
    signal.set(value);
}

#[allow(dead_code)]
pub(crate) fn read_signal<T: Clone + 'static>(signal: dioxus::prelude::Signal<T>) -> T {
    signal.read().clone()
}

pub(crate) fn with_lock<T, R>(value: &Arc<Mutex<T>>, f: impl FnOnce(&mut T) -> R) -> R {
    let mut guard = value
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    f(&mut guard)
}

/// Target duration between animation frames (~60 fps).
#[cfg(not(target_arch = "wasm32"))]
const FRAME_DURATION: std::time::Duration = std::time::Duration::from_millis(16);

/// Spawn the per-hook animation loop for the current component.
///
/// `tick` receives the elapsed seconds since the previous frame and returns
/// `true` while the underlying engine is actively advancing. When `tick`
/// reports idle the loop parks instead of spinning, and [`LoopWaker::notify`]
/// resumes it as soon as a handle method reactivates the engine.
///
/// Must be called during the hooks phase (it uses [`dioxus::prelude::use_future`]
/// internally on native targets).
pub(crate) fn spawn_animation_loop(waker: LoopWaker, tick: impl FnMut(f32) -> bool + 'static) {
    #[cfg(all(target_arch = "wasm32", feature = "web"))]
    {
        spawn_raf_loop(waker, tick);
        return;
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        use dioxus::prelude::use_future;

        let mut tick = Some(tick);
        use_future(move || {
            let mut tick = tick.take();
            let waker = waker.clone();
            async move {
                let Some(mut tick) = tick.take() else {
                    return;
                };
                let mut last = std::time::Instant::now();
                loop {
                    let now = std::time::Instant::now();
                    let dt = now.duration_since(last).as_secs_f32().min(0.25);
                    last = now;
                    if tick(dt) {
                        // Yield between frames instead of blocking the executor
                        // thread; the timer works with any async runtime.
                        futures_timer::Delay::new(FRAME_DURATION).await;
                    } else {
                        // The engine is idle (paused, settled, or complete):
                        // park until a handle method wakes the loop again.
                        waker.parked().await;
                        // Restart the clock so the first frame after waking is a
                        // normal-sized step rather than the whole idle period.
                        last = std::time::Instant::now();
                    }
                }
            }
        });
    }

    #[cfg(all(target_arch = "wasm32", not(feature = "web")))]
    {
        let _ = (waker, tick);
    }
}

/// Wake signal for a parked animation loop.
///
/// Every spawned loop owns one waker, shared with the handle that drives the
/// engine. Handle methods call `notify()` whenever the engine may have become
/// active again; a parked loop resumes on the next notification.
#[derive(Clone, Default)]
pub(crate) struct LoopWaker {
    inner: Arc<LoopWakerState>,
}

#[derive(Default)]
struct LoopWakerState {
    /// Native targets park on a permit-plus-waker pair.
    #[cfg(not(target_arch = "wasm32"))]
    native: Mutex<NativeWakeState>,
    /// Web targets register a resumption thunk while the rAF loop is parked.
    #[cfg(all(target_arch = "wasm32", feature = "web"))]
    web: WebWakeState,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Default)]
struct NativeWakeState {
    pending: bool,
    waker: Option<std::task::Waker>,
}

#[cfg(all(target_arch = "wasm32", feature = "web"))]
#[derive(Default)]
struct WebWakeState {
    resume: RefCell<Option<Rc<dyn Fn()>>>,
}

impl LoopWaker {
    /// Wake the parked loop. Safe to call while the loop is already running.
    pub(crate) fn notify(&self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut native = self
                .inner
                .native
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            native.pending = true;
            if let Some(waker) = native.waker.take() {
                waker.wake();
            }
        }

        #[cfg(all(target_arch = "wasm32", feature = "web"))]
        {
            // Take the thunk so repeated notifications never stack frames; a
            // parked loop re-registers a fresh thunk whenever it parks again.
            if let Some(resume) = self.inner.web.resume.borrow_mut().take() {
                resume();
            }
        }
    }

    /// Register the thunk that resumes a parked request-animation-frame loop.
    #[cfg(all(target_arch = "wasm32", feature = "web"))]
    pub(crate) fn set_resume(&self, resume: Rc<dyn Fn()>) {
        *self.inner.web.resume.borrow_mut() = Some(resume);
    }

    /// Park the calling async loop until the next `notify()`.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn parked(&self) -> Parked {
        Parked {
            state: Arc::clone(&self.inner),
        }
    }
}

/// Future that resolves once the owning loop's waker is notified.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct Parked {
    state: Arc<LoopWakerState>,
}

#[cfg(not(target_arch = "wasm32"))]
impl Future for Parked {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let mut native = self
            .state
            .native
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if native.pending {
            native.pending = false;
            native.waker = None;
            return Poll::Ready(());
        }
        native.waker = Some(cx.waker().clone());
        Poll::Pending
    }
}

#[cfg(all(target_arch = "wasm32", feature = "web"))]
fn spawn_raf_loop(waker: LoopWaker, tick: impl FnMut(f32) -> bool + 'static) {
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;
    use wasm_bindgen::JsCast;
    use wasm_bindgen::closure::Closure;

    let Some(window) = web_sys::window() else {
        return;
    };

    let tick = Rc::new(RefCell::new(Box::new(tick) as Box<dyn FnMut(f32) -> bool>));
    let last_timestamp = Rc::new(Cell::new(None::<f64>));
    let callback: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let callback_ref = Rc::clone(&callback);
    let tick_ref = Rc::clone(&tick);
    let last_ref = Rc::clone(&last_timestamp);
    let window_ref = window.clone();

    let schedule: Rc<dyn Fn()> = Rc::new({
        let callback = Rc::clone(&callback);
        let window = window.clone();
        move || {
            if let Some(callback) = callback.borrow().as_ref() {
                let _ = window.request_animation_frame(callback.as_ref().unchecked_ref());
            }
        }
    });

    *callback.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let now = window_ref
            .performance()
            .map(|performance| performance.now())
            .unwrap_or(0.0);
        let dt = last_ref
            .replace(Some(now))
            .map(|last| ((now - last) / 1000.0).max(0.0) as f32)
            .unwrap_or(0.0)
            .min(0.25);

        if (tick_ref.borrow_mut())(dt) {
            schedule();
        } else {
            // Park: drop the stale timestamp so the next frame starts at
            // dt = 0, and let the waker schedule the resuming frame.
            last_ref.set(None);
            waker.set_resume(Rc::clone(&schedule));
        }
    }) as Box<dyn FnMut()>));

    if let Some(callback) = callback.borrow().as_ref() {
        let _ = window.request_animation_frame(callback.as_ref().unchecked_ref());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dioxus::prelude::*;
    use std::cell::RefCell;
    use std::sync::{Arc, Mutex};

    thread_local! {
        static SIGNAL_CAPTURE: RefCell<Option<Signal<i32>>> = const { RefCell::new(None) };
    }

    #[allow(non_snake_case)]
    fn SignalHelperApp() -> Element {
        let signal = use_signal(|| 1_i32);
        SIGNAL_CAPTURE.with(|slot| *slot.borrow_mut() = Some(signal));

        rsx! { div {} }
    }

    #[test]
    fn finite_or_replaces_non_finite_values() {
        assert_eq!(finite_or(2.0, 1.0), 2.0);
        assert_eq!(finite_or(f32::NAN, 1.0), 1.0);
        assert_eq!(finite_or(f32::INFINITY, 1.0), 1.0);
    }

    #[test]
    fn signal_helpers_read_and_write_values() {
        SIGNAL_CAPTURE.with(|slot| *slot.borrow_mut() = None);
        let mut dom = VirtualDom::new(SignalHelperApp);
        dom.rebuild_in_place();
        let signal =
            SIGNAL_CAPTURE.with(|slot| slot.borrow().as_ref().copied().expect("signal captured"));

        assert_eq!(read_signal(signal), 1);
        set_signal(signal, 4);
        assert_eq!(read_signal(signal), 4);
    }

    #[test]
    fn with_lock_updates_inner_value() {
        let value = Arc::new(Mutex::new(1_i32));
        let updated = with_lock(&value, |inner| {
            *inner += 2;
            *inner
        });

        assert_eq!(updated, 3);
        assert_eq!(*value.lock().expect("lock"), 3);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn loop_waker_permit_and_wake_semantics() {
        use std::pin::pin;
        use std::task::{Context, Poll, Waker};

        let waker = LoopWaker::default();
        let noop = Waker::noop();
        let mut cx = Context::from_waker(&noop);

        // A fresh park stays pending until notified.
        let mut parked = pin!(waker.parked());
        assert!(matches!(parked.as_mut().poll(&mut cx), Poll::Pending));
        assert!(matches!(parked.as_mut().poll(&mut cx), Poll::Pending));

        // Notification resolves the parked future.
        waker.notify();
        assert!(matches!(parked.as_mut().poll(&mut cx), Poll::Ready(())));

        // A notification before parking stores a permit consumed by the next park.
        waker.notify();
        let mut parked = pin!(waker.parked());
        assert!(matches!(parked.as_mut().poll(&mut cx), Poll::Ready(())));

        // Parking again without notification stays pending.
        let mut parked = pin!(waker.parked());
        assert!(matches!(parked.as_mut().poll(&mut cx), Poll::Pending));
    }
}
