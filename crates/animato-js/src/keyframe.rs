//! Keyframe track bindings.

use crate::easing::parse_easing;
use crate::tween::lock;
use crate::types::{f32_array, parse_loop_mode};
use animato_core::{Playable, Update};
use animato_tween::KeyframeTrack as CoreKeyframeTrack;
use js_sys::Float32Array;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;

type Shared<T> = Arc<Mutex<CoreKeyframeTrack<T>>>;

macro_rules! shared_keyframes {
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
                lock(&self.inner).duration()
            }

            fn reset(&mut self) {
                lock(&self.inner).reset();
            }

            fn seek_to(&mut self, progress: f32) {
                let mut track = lock(&self.inner);
                Playable::seek_to(&mut *track, progress);
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

shared_keyframes!(SharedKeyframeTrack, f32);
shared_keyframes!(SharedKeyframeTrack2D, [f32; 2]);
shared_keyframes!(SharedKeyframeTrack3D, [f32; 3]);
shared_keyframes!(SharedKeyframeTrack4D, [f32; 4]);

/// Scalar keyframe track.
#[wasm_bindgen(js_name = KeyframeTrack)]
#[derive(Clone, Debug)]
pub struct KeyframeTrack {
    inner: Shared<f32>,
}

#[wasm_bindgen(js_class = KeyframeTrack)]
impl KeyframeTrack {
    /// Create an empty keyframe track.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(CoreKeyframeTrack::new())),
        }
    }

    /// Add a linear keyframe.
    pub fn push(&self, time: f32, value: f32) {
        let mut track = lock(&self.inner);
        let next = core::mem::take(&mut *track).push(time, value);
        *track = next;
    }

    /// Add a keyframe with easing applied to the following segment.
    #[wasm_bindgen(js_name = pushEased)]
    pub fn push_eased(&self, time: f32, value: f32, easing: &str) -> Result<(), JsValue> {
        let easing = parse_easing(easing)?;
        let mut track = lock(&self.inner);
        let next = core::mem::take(&mut *track).push_eased(time, value, easing);
        *track = next;
        Ok(())
    }

    /// Advance by `dt` seconds.
    pub fn update(&self, dt: f32) -> bool {
        lock(&self.inner).update(dt)
    }

    /// Current value, or `NaN` if the track is empty.
    pub fn value(&self) -> f32 {
        lock(&self.inner).value().unwrap_or(f32::NAN)
    }

    /// Value at an absolute time, or `NaN` if the track is empty.
    #[wasm_bindgen(js_name = valueAt)]
    pub fn value_at(&self, seconds: f32) -> f32 {
        lock(&self.inner).value_at(seconds).unwrap_or(f32::NAN)
    }

    /// Track duration in seconds.
    pub fn duration(&self) -> f32 {
        lock(&self.inner).duration()
    }

    /// Normalized track progress.
    pub fn progress(&self) -> f32 {
        lock(&self.inner).progress()
    }

    /// Whether playback is complete.
    #[wasm_bindgen(js_name = isComplete)]
    pub fn is_complete(&self) -> bool {
        lock(&self.inner).is_complete()
    }

    /// Reset playback.
    pub fn reset(&self) {
        lock(&self.inner).reset();
    }

    /// Set loop mode by string.
    #[wasm_bindgen(js_name = setLoopMode)]
    pub fn set_loop_mode(&self, mode: &str) -> Result<(), JsValue> {
        let mut track = lock(&self.inner);
        let next = core::mem::take(&mut *track).looping(parse_loop_mode(mode)?);
        *track = next;
        Ok(())
    }

    pub(crate) fn shared(&self) -> SharedKeyframeTrack {
        SharedKeyframeTrack::new(Arc::clone(&self.inner))
    }
}

impl Default for KeyframeTrack {
    fn default() -> Self {
        Self::new()
    }
}

macro_rules! vector_track {
    (
        $class:ident,
        $js_name:ident,
        $shared:ident,
        $value_ty:ty,
        [$($value:ident),+]
    ) => {
        /// Vector keyframe track.
        #[wasm_bindgen(js_name = $js_name)]
        #[derive(Clone, Debug)]
        pub struct $class {
            inner: Shared<$value_ty>,
        }

        #[wasm_bindgen(js_class = $js_name)]
        impl $class {
            /// Create an empty vector keyframe track.
            #[wasm_bindgen(constructor)]
            pub fn new() -> Self {
                Self {
                    inner: Arc::new(Mutex::new(CoreKeyframeTrack::new())),
                }
            }

            /// Add a linear vector keyframe.
            pub fn push(&self, time: f32, $($value: f32),+) {
                let mut track = lock(&self.inner);
                let next = core::mem::take(&mut *track).push(time, [$($value),+]);
                *track = next;
            }

            /// Add a vector keyframe with easing.
            #[wasm_bindgen(js_name = pushEased)]
            pub fn push_eased(&self, time: f32, $($value: f32,)+ easing: &str) -> Result<(), JsValue> {
                let easing = parse_easing(easing)?;
                let mut track = lock(&self.inner);
                let next = core::mem::take(&mut *track).push_eased(time, [$($value),+], easing);
                *track = next;
                Ok(())
            }

            /// Advance by `dt` seconds.
            pub fn update(&self, dt: f32) -> bool {
                lock(&self.inner).update(dt)
            }

            /// Current vector value.
            #[wasm_bindgen(js_name = toArray)]
            pub fn to_array(&self) -> Float32Array {
                let values = lock(&self.inner).value().unwrap_or_default();
                f32_array(&values)
            }

            /// Value at an absolute time.
            #[wasm_bindgen(js_name = valueAt)]
            pub fn value_at(&self, seconds: f32) -> Float32Array {
                let values = lock(&self.inner).value_at(seconds).unwrap_or_default();
                f32_array(&values)
            }

            /// Track duration in seconds.
            pub fn duration(&self) -> f32 {
                lock(&self.inner).duration()
            }

            /// Normalized track progress.
            pub fn progress(&self) -> f32 {
                lock(&self.inner).progress()
            }

            /// Whether playback is complete.
            #[wasm_bindgen(js_name = isComplete)]
            pub fn is_complete(&self) -> bool {
                lock(&self.inner).is_complete()
            }

            /// Reset playback.
            pub fn reset(&self) {
                lock(&self.inner).reset();
            }

            /// Set loop mode by string.
            #[wasm_bindgen(js_name = setLoopMode)]
            pub fn set_loop_mode(&self, mode: &str) -> Result<(), JsValue> {
                let mut track = lock(&self.inner);
                let next = core::mem::take(&mut *track).looping(parse_loop_mode(mode)?);
                *track = next;
                Ok(())
            }

            pub(crate) fn shared(&self) -> $shared {
                $shared::new(Arc::clone(&self.inner))
            }
        }

        impl Default for $class {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

vector_track!(
    KeyframeTrack2D,
    KeyframeTrack2D,
    SharedKeyframeTrack2D,
    [f32; 2],
    [x, y]
);
vector_track!(
    KeyframeTrack3D,
    KeyframeTrack3D,
    SharedKeyframeTrack3D,
    [f32; 3],
    [x, y, z]
);
vector_track!(
    KeyframeTrack4D,
    KeyframeTrack4D,
    SharedKeyframeTrack4D,
    [f32; 4],
    [x, y, z, w]
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyframes_interpolate() {
        let track = KeyframeTrack::new();
        track.push(0.0, 0.0);
        track.push_eased(1.0, 100.0, "linear").unwrap();
        track.update(0.5);
        assert_eq!(track.value(), 50.0);
    }
}
