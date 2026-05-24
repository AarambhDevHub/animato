//! Tween bindings.

use crate::easing::{easing_name, parse_easing};
use crate::error::non_negative;
use crate::types::{f32_array, loop_from_count, parse_loop_mode};
use animato_core::{Playable, Update};
use animato_tween::{Loop, Tween as CoreTween, TweenState};
use js_sys::Float32Array;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;

type Shared<T> = Arc<Mutex<CoreTween<T>>>;

pub(crate) fn lock<T>(shared: &Arc<Mutex<T>>) -> std::sync::MutexGuard<'_, T> {
    shared
        .lock()
        .expect("animato-js shared animation lock poisoned")
}

pub(crate) fn state_name(state: &TweenState) -> &'static str {
    match state {
        TweenState::Idle => "idle",
        TweenState::Running => "running",
        TweenState::Paused => "paused",
        TweenState::Completed => "completed",
    }
}

macro_rules! shared_update {
    ($name:ident, $value_ty:ty) => {
        #[derive(Clone, Debug)]
        pub(crate) struct $name {
            inner: Shared<$value_ty>,
        }

        impl $name {
            pub(crate) fn new(inner: Shared<$value_ty>) -> Self {
                Self { inner }
            }
        }

        impl Update for $name {
            fn update(&mut self, dt: f32) -> bool {
                lock(&self.inner).update(dt)
            }
        }

        impl Playable for $name {
            fn duration(&self) -> f32 {
                lock(&self.inner).duration
            }

            fn reset(&mut self) {
                lock(&self.inner).reset();
            }

            fn seek_to(&mut self, progress: f32) {
                lock(&self.inner).seek(progress);
            }

            fn is_complete(&self) -> bool {
                lock(&self.inner).is_complete()
            }

            fn as_any(&self) -> &dyn core::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
                self
            }
        }
    };
}

shared_update!(SharedTween, f32);
shared_update!(SharedTween2D, [f32; 2]);
shared_update!(SharedTween3D, [f32; 3]);
shared_update!(SharedTween4D, [f32; 4]);

/// Scalar tween from one number to another.
#[wasm_bindgen(js_name = Tween)]
#[derive(Clone, Debug)]
pub struct Tween {
    inner: Shared<f32>,
}

#[wasm_bindgen(js_class = Tween)]
impl Tween {
    /// Create a scalar tween.
    #[wasm_bindgen(constructor)]
    pub fn new(from: f32, to: f32, duration: f32) -> Self {
        Self {
            inner: Arc::new(Mutex::new(
                CoreTween::new(from, to)
                    .duration(non_negative(duration, 1.0))
                    .build(),
            )),
        }
    }

    /// Advance by `dt` seconds.
    pub fn update(&self, dt: f32) -> bool {
        lock(&self.inner).update(dt)
    }

    /// Current animated value.
    pub fn value(&self) -> f32 {
        lock(&self.inner).value()
    }

    /// Current raw progress in `[0, 1]`.
    pub fn progress(&self) -> f32 {
        lock(&self.inner).progress()
    }

    /// Current eased progress in `[0, 1]`.
    #[wasm_bindgen(js_name = easedProgress)]
    pub fn eased_progress(&self) -> f32 {
        lock(&self.inner).eased_progress()
    }

    /// Current runtime state.
    pub fn state(&self) -> String {
        state_name(lock(&self.inner).state()).to_owned()
    }

    /// Easing name for this tween.
    pub fn easing(&self) -> String {
        easing_name(&lock(&self.inner).easing).to_owned()
    }

    /// Whether playback is complete.
    #[wasm_bindgen(js_name = isComplete)]
    pub fn is_complete(&self) -> bool {
        lock(&self.inner).is_complete()
    }

    /// Pause playback.
    pub fn pause(&self) {
        lock(&self.inner).pause();
    }

    /// Resume playback.
    pub fn resume(&self) {
        lock(&self.inner).resume();
    }

    /// Reset to the beginning.
    pub fn reset(&self) {
        lock(&self.inner).reset();
    }

    /// Reverse direction while preserving progress.
    pub fn reverse(&self) {
        lock(&self.inner).reverse();
    }

    /// Seek to normalized progress.
    pub fn seek(&self, progress: f32) {
        lock(&self.inner).seek(progress);
    }

    /// Set easing by name.
    #[wasm_bindgen(js_name = setEasing)]
    pub fn set_easing(&self, name: &str) -> Result<(), JsValue> {
        lock(&self.inner).easing = parse_easing(name)?;
        Ok(())
    }

    /// Set CSS cubic-bezier easing.
    #[wasm_bindgen(js_name = setCubicBezier)]
    pub fn set_cubic_bezier(&self, x1: f32, y1: f32, x2: f32, y2: f32) {
        lock(&self.inner).easing = animato_core::Easing::CubicBezier(x1, y1, x2, y2);
    }

    /// Set playback time scale.
    #[wasm_bindgen(js_name = setTimeScale)]
    pub fn set_time_scale(&self, scale: f32) {
        lock(&self.inner).time_scale = non_negative(scale, 1.0);
    }

    /// Set start delay in seconds.
    #[wasm_bindgen(js_name = setDelay)]
    pub fn set_delay(&self, delay: f32) {
        lock(&self.inner).delay = non_negative(delay, 0.0);
    }

    /// Loop a fixed number of passes.
    #[wasm_bindgen(js_name = setLoopCount)]
    pub fn set_loop_count(&self, count: u32) {
        lock(&self.inner).looping = loop_from_count(count);
    }

    /// Set loop mode by string: `once`, `forever`, `pingPong`, or `timesN`.
    #[wasm_bindgen(js_name = setLoopMode)]
    pub fn set_loop_mode(&self, mode: &str) -> Result<(), JsValue> {
        lock(&self.inner).looping = parse_loop_mode(mode)?;
        Ok(())
    }

    /// Use ping-pong looping.
    #[wasm_bindgen(js_name = setPingPong)]
    pub fn set_ping_pong(&self) {
        lock(&self.inner).looping = Loop::PingPong;
    }

    /// Use infinite looping.
    #[wasm_bindgen(js_name = setForever)]
    pub fn set_forever(&self) {
        lock(&self.inner).looping = Loop::Forever;
    }

    pub(crate) fn shared(&self) -> SharedTween {
        SharedTween::new(Arc::clone(&self.inner))
    }
}

macro_rules! vector_tween {
    (
        $class:ident,
        $js_name:ident,
        $shared:ident,
        $value_ty:ty,
        [$($from:ident),+],
        [$($to:ident),+],
        $array_fn:ident,
        [$($component:ident : $index:tt),+]
    ) => {
        /// Vector tween.
        #[wasm_bindgen(js_name = $js_name)]
        #[derive(Clone, Debug)]
        pub struct $class {
            inner: Shared<$value_ty>,
        }

        #[wasm_bindgen(js_class = $js_name)]
        impl $class {
            /// Create a vector tween.
            #[wasm_bindgen(constructor)]
            #[allow(clippy::too_many_arguments)]
            pub fn new($($from: f32,)+ $($to: f32,)+ duration: f32) -> Self {
                Self {
                    inner: Arc::new(Mutex::new(
                        CoreTween::new([$($from),+], [$($to),+])
                            .duration(non_negative(duration, 1.0))
                            .build(),
                    )),
                }
            }

            /// Advance by `dt` seconds.
            pub fn update(&self, dt: f32) -> bool {
                lock(&self.inner).update(dt)
            }

            /// Return all vector components.
            #[wasm_bindgen(js_name = toArray)]
            pub fn to_array(&self) -> Float32Array {
                let value = lock(&self.inner).value();
                f32_array(&value)
            }

            $(
                /// Current vector component.
                pub fn $component(&self) -> f32 {
                    lock(&self.inner).value()[$index]
                }
            )+

            /// Current raw progress in `[0, 1]`.
            pub fn progress(&self) -> f32 {
                lock(&self.inner).progress()
            }

            /// Current eased progress in `[0, 1]`.
            #[wasm_bindgen(js_name = easedProgress)]
            pub fn eased_progress(&self) -> f32 {
                lock(&self.inner).eased_progress()
            }

            /// Whether playback is complete.
            #[wasm_bindgen(js_name = isComplete)]
            pub fn is_complete(&self) -> bool {
                lock(&self.inner).is_complete()
            }

            /// Pause playback.
            pub fn pause(&self) {
                lock(&self.inner).pause();
            }

            /// Resume playback.
            pub fn resume(&self) {
                lock(&self.inner).resume();
            }

            /// Reset to the beginning.
            pub fn reset(&self) {
                lock(&self.inner).reset();
            }

            /// Reverse direction while preserving progress.
            pub fn reverse(&self) {
                lock(&self.inner).reverse();
            }

            /// Seek to normalized progress.
            pub fn seek(&self, progress: f32) {
                lock(&self.inner).seek(progress);
            }

            /// Set easing by name.
            #[wasm_bindgen(js_name = setEasing)]
            pub fn set_easing(&self, name: &str) -> Result<(), JsValue> {
                lock(&self.inner).easing = parse_easing(name)?;
                Ok(())
            }

            /// Set playback time scale.
            #[wasm_bindgen(js_name = setTimeScale)]
            pub fn set_time_scale(&self, scale: f32) {
                lock(&self.inner).time_scale = non_negative(scale, 1.0);
            }

            /// Set loop mode by string.
            #[wasm_bindgen(js_name = setLoopMode)]
            pub fn set_loop_mode(&self, mode: &str) -> Result<(), JsValue> {
                lock(&self.inner).looping = parse_loop_mode(mode)?;
                Ok(())
            }

            pub(crate) fn shared(&self) -> $shared {
                $shared::new(Arc::clone(&self.inner))
            }
        }
    };
}

vector_tween!(
    Tween2D,
    Tween2D,
    SharedTween2D,
    [f32; 2],
    [from_x, from_y],
    [to_x, to_y],
    vec2,
    [x: 0, y: 1]
);

vector_tween!(
    Tween3D,
    Tween3D,
    SharedTween3D,
    [f32; 3],
    [from_x, from_y, from_z],
    [to_x, to_y, to_z],
    vec3,
    [x: 0, y: 1, z: 2]
);

vector_tween!(
    Tween4D,
    Tween4D,
    SharedTween4D,
    [f32; 4],
    [from_x, from_y, from_z, from_w],
    [to_x, to_y, to_z, to_w],
    vec4,
    [x: 0, y: 1, z: 2, w: 3]
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_tween_updates() {
        let tween = Tween::new(0.0, 10.0, 1.0);
        tween.set_easing("linear").unwrap();
        assert!(tween.update(0.5));
        assert_eq!(tween.value(), 5.0);
    }

    #[test]
    fn vector_tween_updates() {
        let tween = Tween2D::new(0.0, 0.0, 10.0, 20.0, 1.0);
        tween.update(0.5);
        assert_eq!(tween.x(), 5.0);
        assert_eq!(tween.y(), 10.0);
    }
}
