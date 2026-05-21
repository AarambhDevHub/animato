//! Private requestAnimationFrame loop used by hooks.

use std::cell::{Cell, RefCell};
use std::fmt;
use std::rc::Rc;

/// Lazy rAF loop that runs only while active.
#[derive(Clone)]
pub(crate) struct RafLoop {
    inner: Rc<RafLoopInner>,
}

struct RafLoopInner {
    active: Cell<bool>,
    scheduled: Cell<bool>,
    last_timestamp: Cell<Option<f64>>,
    #[allow(dead_code)]
    tick: RefCell<Box<dyn FnMut(f32) -> bool>>,
}

impl fmt::Debug for RafLoop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RafLoop")
            .field("active", &self.inner.active.get())
            .field("scheduled", &self.inner.scheduled.get())
            .finish_non_exhaustive()
    }
}

impl RafLoop {
    pub(crate) fn new(tick: impl FnMut(f32) -> bool + 'static) -> Self {
        Self {
            inner: Rc::new(RafLoopInner {
                active: Cell::new(false),
                scheduled: Cell::new(false),
                last_timestamp: Cell::new(None),
                tick: RefCell::new(Box::new(tick)),
            }),
        }
    }

    pub(crate) fn kick(&self) {
        self.inner.active.set(true);
        self.schedule();
    }

    pub(crate) fn stop(&self) {
        self.inner.active.set(false);
        self.inner.last_timestamp.set(None);
    }

    #[cfg(target_arch = "wasm32")]
    fn schedule(&self) {
        use wasm_bindgen::JsCast;
        use wasm_bindgen::closure::Closure;

        if self.inner.scheduled.get() || !self.inner.active.get() {
            return;
        }

        self.inner.scheduled.set(true);
        let this = self.clone();
        let callback = Closure::once_into_js(move |timestamp: f64| {
            this.on_frame(timestamp);
        });
        if let Some(window) = web_sys::window() {
            let _ = window.request_animation_frame(callback.as_ref().unchecked_ref());
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn schedule(&self) {}

    #[cfg(target_arch = "wasm32")]
    fn on_frame(&self, timestamp: f64) {
        self.inner.scheduled.set(false);
        if !self.inner.active.get() {
            self.inner.last_timestamp.set(None);
            return;
        }

        let dt = self
            .inner
            .last_timestamp
            .replace(Some(timestamp))
            .map(|last| ((timestamp - last) / 1000.0).max(0.0) as f32)
            .unwrap_or(0.0)
            .min(0.25);
        let keep_running = (self.inner.tick.borrow_mut())(dt);

        if keep_running && self.inner.active.get() {
            self.schedule();
        } else {
            self.stop();
        }
    }
}
