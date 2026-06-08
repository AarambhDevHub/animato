//! Advanced engine JavaScript bindings.

use crate::easing::parse_easing;
use crate::error::{JsResult, js_error, non_negative};
use crate::keyframe::KeyframeTrack;
use crate::timeline::{Timeline, parse_at};
use crate::tween::{Tween, lock};
use crate::types::f32_array;
use animato_core::{
    Angle as CoreAngle, Mat4 as CoreMat4, Playable, Quaternion as CoreQuaternion, Update,
};
use animato_timeline::{AnimationGroup as CoreAnimationGroup, At, Timeline as CoreTimeline};
use animato_tween::{
    StaggerPattern as CoreStaggerPattern, Tween as CoreTween, Waveform as CoreWaveform,
};
use js_sys::Float32Array;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;

type SharedAdvancedTween<T> = Arc<Mutex<CoreTween<T>>>;

macro_rules! shared_advanced_tween {
    ($name:ident, $value_ty:ty) => {
        #[derive(Clone, Debug)]
        pub(crate) struct $name {
            inner: SharedAdvancedTween<$value_ty>,
        }

        impl $name {
            pub(crate) fn new(inner: SharedAdvancedTween<$value_ty>) -> Self {
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

shared_advanced_tween!(SharedAngleTween, CoreAngle);
shared_advanced_tween!(SharedQuaternionTween, CoreQuaternion);
shared_advanced_tween!(SharedMat4Tween, CoreMat4);

/// Angle value stored in degrees.
#[wasm_bindgen(js_name = Angle)]
#[derive(Clone, Copy, Debug)]
pub struct Angle {
    inner: CoreAngle,
}

#[wasm_bindgen(js_class = Angle)]
impl Angle {
    /// Create an angle from degrees.
    #[wasm_bindgen(constructor)]
    pub fn new(degrees: f32) -> Self {
        Self {
            inner: CoreAngle::from_degrees(degrees),
        }
    }

    /// Create an angle from radians.
    #[wasm_bindgen(js_name = fromRadians)]
    pub fn from_radians(radians: f32) -> Self {
        Self {
            inner: CoreAngle::from_radians(radians),
        }
    }

    /// Return degrees.
    pub fn degrees(&self) -> f32 {
        self.inner.degrees()
    }

    /// Return radians.
    pub fn radians(&self) -> f32 {
        self.inner.radians()
    }

    /// Return normalized degrees in `[0, 360)`.
    pub fn normalized(&self) -> Self {
        Self {
            inner: self.inner.normalized(),
        }
    }
}

/// Unit quaternion value for 3D rotations.
#[wasm_bindgen(js_name = Quaternion)]
#[derive(Clone, Copy, Debug)]
pub struct Quaternion {
    inner: CoreQuaternion,
}

#[wasm_bindgen(js_class = Quaternion)]
impl Quaternion {
    /// Create a quaternion from components.
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self {
            inner: CoreQuaternion::new(x, y, z, w),
        }
    }

    /// Create an identity rotation.
    pub fn identity() -> Self {
        Self {
            inner: CoreQuaternion::IDENTITY,
        }
    }

    /// Create a quaternion from an axis and angle in degrees.
    #[wasm_bindgen(js_name = fromAxisAngle)]
    pub fn from_axis_angle(x: f32, y: f32, z: f32, degrees: f32) -> Self {
        Self {
            inner: CoreQuaternion::from_axis_angle([x, y, z], CoreAngle::from_degrees(degrees)),
        }
    }

    /// Spherical interpolation to another quaternion.
    pub fn slerp(&self, other: &Quaternion, t: f32) -> Self {
        Self {
            inner: self.inner.slerp(other.inner, t),
        }
    }

    /// Return components as `[x, y, z, w]`.
    #[wasm_bindgen(js_name = toArray)]
    pub fn to_array(&self) -> Float32Array {
        let q = self.inner;
        f32_array(&[q.x, q.y, q.z, q.w])
    }

    /// X component.
    pub fn x(&self) -> f32 {
        self.inner.x
    }

    /// Y component.
    pub fn y(&self) -> f32 {
        self.inner.y
    }

    /// Z component.
    pub fn z(&self) -> f32 {
        self.inner.z
    }

    /// W component.
    pub fn w(&self) -> f32 {
        self.inner.w
    }
}

/// Column-major affine 4x4 matrix.
#[wasm_bindgen(js_name = Mat4)]
#[derive(Clone, Copy, Debug)]
pub struct Mat4 {
    inner: CoreMat4,
}

#[wasm_bindgen(js_class = Mat4)]
impl Mat4 {
    /// Create a matrix from a 16-value typed array.
    #[wasm_bindgen(constructor)]
    pub fn new(values: &Float32Array) -> Result<Mat4, JsValue> {
        Ok(Self {
            inner: read_mat4(values)?,
        })
    }

    /// Create an identity matrix.
    pub fn identity() -> Self {
        Self {
            inner: CoreMat4::IDENTITY,
        }
    }

    /// Create from translation, quaternion rotation, and scale arrays.
    #[wasm_bindgen(js_name = fromTranslationRotationScale)]
    pub fn from_translation_rotation_scale(
        translation: &Float32Array,
        rotation: &Quaternion,
        scale: &Float32Array,
    ) -> Result<Self, JsValue> {
        Ok(Self {
            inner: CoreMat4::from_translation_rotation_scale(
                read_vec3(translation, "translation")?,
                rotation.inner,
                read_vec3(scale, "scale")?,
            ),
        })
    }

    /// Return matrix values as a typed array.
    #[wasm_bindgen(js_name = toArray)]
    pub fn to_array(&self) -> Float32Array {
        f32_array(&self.inner.0)
    }
}

/// Tween for angle values using shortest-path interpolation.
#[wasm_bindgen(js_name = AngleTween)]
#[derive(Clone, Debug)]
pub struct AngleTween {
    inner: SharedAdvancedTween<CoreAngle>,
}

#[wasm_bindgen(js_class = AngleTween)]
impl AngleTween {
    /// Create an angle tween from degrees.
    #[wasm_bindgen(constructor)]
    pub fn new(from_degrees: f32, to_degrees: f32, duration: f32) -> Self {
        Self {
            inner: Arc::new(Mutex::new(
                CoreTween::new(
                    CoreAngle::from_degrees(from_degrees),
                    CoreAngle::from_degrees(to_degrees),
                )
                .duration(non_negative(duration, 1.0))
                .build(),
            )),
        }
    }

    /// Advance by seconds.
    pub fn update(&self, dt: f32) -> bool {
        lock(&self.inner).update(dt)
    }

    /// Current angle in degrees.
    pub fn degrees(&self) -> f32 {
        lock(&self.inner).value().degrees()
    }

    /// Current angle in radians.
    pub fn radians(&self) -> f32 {
        lock(&self.inner).value().radians()
    }

    /// Set easing by name.
    #[wasm_bindgen(js_name = setEasing)]
    pub fn set_easing(&self, easing: &str) -> Result<(), JsValue> {
        lock(&self.inner).easing = parse_easing(easing)?;
        Ok(())
    }

    pub(crate) fn shared(&self) -> SharedAngleTween {
        SharedAngleTween::new(Arc::clone(&self.inner))
    }
}

/// Tween for quaternion rotations using slerp.
#[wasm_bindgen(js_name = QuaternionTween)]
#[derive(Clone, Debug)]
pub struct QuaternionTween {
    inner: SharedAdvancedTween<CoreQuaternion>,
}

#[wasm_bindgen(js_class = QuaternionTween)]
impl QuaternionTween {
    /// Create a quaternion tween.
    #[wasm_bindgen(constructor)]
    pub fn new(from: &Quaternion, to: &Quaternion, duration: f32) -> Self {
        Self {
            inner: Arc::new(Mutex::new(
                CoreTween::new(from.inner, to.inner)
                    .duration(non_negative(duration, 1.0))
                    .build(),
            )),
        }
    }

    /// Advance by seconds.
    pub fn update(&self, dt: f32) -> bool {
        lock(&self.inner).update(dt)
    }

    /// Current quaternion value.
    pub fn value(&self) -> Quaternion {
        Quaternion {
            inner: lock(&self.inner).value(),
        }
    }

    /// Current value as `[x, y, z, w]`.
    #[wasm_bindgen(js_name = toArray)]
    pub fn to_array(&self) -> Float32Array {
        self.value().to_array()
    }

    /// Set easing by name.
    #[wasm_bindgen(js_name = setEasing)]
    pub fn set_easing(&self, easing: &str) -> Result<(), JsValue> {
        lock(&self.inner).easing = parse_easing(easing)?;
        Ok(())
    }

    pub(crate) fn shared(&self) -> SharedQuaternionTween {
        SharedQuaternionTween::new(Arc::clone(&self.inner))
    }
}

/// Tween for affine 4x4 matrices.
#[wasm_bindgen(js_name = Mat4Tween)]
#[derive(Clone, Debug)]
pub struct Mat4Tween {
    inner: SharedAdvancedTween<CoreMat4>,
}

#[wasm_bindgen(js_class = Mat4Tween)]
impl Mat4Tween {
    /// Create a matrix tween.
    #[wasm_bindgen(constructor)]
    pub fn new(from: &Mat4, to: &Mat4, duration: f32) -> Self {
        Self {
            inner: Arc::new(Mutex::new(
                CoreTween::new(from.inner, to.inner)
                    .duration(non_negative(duration, 1.0))
                    .build(),
            )),
        }
    }

    /// Advance by seconds.
    pub fn update(&self, dt: f32) -> bool {
        lock(&self.inner).update(dt)
    }

    /// Current value as a typed array.
    #[wasm_bindgen(js_name = toArray)]
    pub fn to_array(&self) -> Float32Array {
        f32_array(&lock(&self.inner).value().0)
    }

    /// Set easing by name.
    #[wasm_bindgen(js_name = setEasing)]
    pub fn set_easing(&self, easing: &str) -> Result<(), JsValue> {
        lock(&self.inner).easing = parse_easing(easing)?;
        Ok(())
    }

    pub(crate) fn shared(&self) -> SharedMat4Tween {
        SharedMat4Tween::new(Arc::clone(&self.inner))
    }
}

/// Procedural scalar waveform.
#[wasm_bindgen(js_name = Waveform)]
#[derive(Clone, Copy, Debug)]
pub struct Waveform {
    inner: CoreWaveform,
}

#[wasm_bindgen(js_class = Waveform)]
impl Waveform {
    /// Create a named default waveform.
    #[wasm_bindgen(constructor)]
    pub fn new(kind: &str) -> Result<Self, JsValue> {
        match crate::types::normalize_name(kind).as_str() {
            "sine" => Ok(Self::sine(1.0, 1.0, 0.0)),
            "sawtooth" => Ok(Self::sawtooth(1.0, 1.0)),
            "square" => Ok(Self::square(1.0, 1.0, 0.5)),
            "triangle" => Ok(Self::triangle(1.0, 1.0)),
            "noise" => Ok(Self::noise(1, 0.25)),
            _ => Err(js_error(format!("unknown waveform `{kind}`"))),
        }
    }

    /// Create a sine waveform.
    pub fn sine(frequency: f32, amplitude: f32, phase: f32) -> Self {
        Self {
            inner: CoreWaveform::Sine {
                frequency,
                amplitude,
                phase,
            },
        }
    }

    /// Create a sawtooth waveform.
    pub fn sawtooth(frequency: f32, amplitude: f32) -> Self {
        Self {
            inner: CoreWaveform::Sawtooth {
                frequency,
                amplitude,
            },
        }
    }

    /// Create a square waveform.
    pub fn square(frequency: f32, amplitude: f32, duty_cycle: f32) -> Self {
        Self {
            inner: CoreWaveform::Square {
                frequency,
                amplitude,
                duty_cycle,
            },
        }
    }

    /// Create a triangle waveform.
    pub fn triangle(frequency: f32, amplitude: f32) -> Self {
        Self {
            inner: CoreWaveform::Triangle {
                frequency,
                amplitude,
            },
        }
    }

    /// Create deterministic smoothed noise.
    pub fn noise(seed: u32, smoothness: f32) -> Self {
        Self {
            inner: CoreWaveform::Noise { seed, smoothness },
        }
    }

    /// Sample the waveform at seconds.
    pub fn sample(&self, time: f32) -> f32 {
        self.inner.sample(time)
    }

    /// Convert the waveform to a scalar keyframe track.
    #[wasm_bindgen(js_name = toKeyframes)]
    pub fn to_keyframes(&self, duration: f32, sample_rate: f32) -> KeyframeTrack {
        KeyframeTrack::from_core(self.inner.to_keyframe_track(duration, sample_rate))
    }
}

/// Stagger delay pattern.
#[wasm_bindgen(js_name = StaggerPattern)]
#[derive(Debug)]
pub struct StaggerPattern {
    inner: CoreStaggerPattern,
}

#[wasm_bindgen(js_class = StaggerPattern)]
impl StaggerPattern {
    /// Create a center-origin grid pattern.
    #[wasm_bindgen(constructor)]
    pub fn new(cols: usize, rows: usize, step: f32) -> Self {
        Self::grid(cols, rows, "center", step)
    }

    /// Create a grid pattern. Origin accepts `center`, corners, or edges.
    pub fn grid(cols: usize, rows: usize, origin: &str, step: f32) -> Self {
        Self {
            inner: CoreStaggerPattern::Grid {
                cols,
                rows,
                origin: parse_grid_origin(origin),
                step,
            },
        }
    }

    /// Create a deterministic random pattern.
    pub fn random(seed: u32, min_delay: f32, max_delay: f32) -> Self {
        Self {
            inner: CoreStaggerPattern::Random {
                seed,
                min_delay,
                max_delay,
            },
        }
    }

    /// Create a center-out pattern.
    #[wasm_bindgen(js_name = centerOut)]
    pub fn center_out(count: usize, step: f32) -> Self {
        Self {
            inner: CoreStaggerPattern::CenterOut { count, step },
        }
    }

    /// Create an edges-in pattern.
    #[wasm_bindgen(js_name = edgesIn)]
    pub fn edges_in(count: usize, step: f32) -> Self {
        Self {
            inner: CoreStaggerPattern::EdgesIn { count, step },
        }
    }

    /// Return the delay for an item.
    pub fn delay(&self, index: usize, total: usize) -> f32 {
        self.inner.delay(index, total)
    }
}

/// Animation group wrapper.
#[wasm_bindgen(js_name = AnimationGroup)]
#[derive(Clone, Debug)]
pub struct AnimationGroup {
    inner: Arc<Mutex<CoreAnimationGroup>>,
}

#[wasm_bindgen(js_class = AnimationGroup)]
impl AnimationGroup {
    /// Create an empty animation group.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(CoreAnimationGroup::from_timeline(
                CoreTimeline::new(),
            ))),
        }
    }

    /// Add a scalar tween.
    #[wasm_bindgen(js_name = addTween)]
    pub fn add_tween(&self, label: &str, tween: &Tween, at: &str) -> Result<(), JsValue> {
        self.add_playable(label, tween.shared(), at)
    }

    /// Add an angle tween.
    #[wasm_bindgen(js_name = addAngleTween)]
    pub fn add_angle_tween(
        &self,
        label: &str,
        tween: &AngleTween,
        at: &str,
    ) -> Result<(), JsValue> {
        self.add_playable(label, tween.shared(), at)
    }

    /// Add a quaternion tween.
    #[wasm_bindgen(js_name = addQuaternionTween)]
    pub fn add_quaternion_tween(
        &self,
        label: &str,
        tween: &QuaternionTween,
        at: &str,
    ) -> Result<(), JsValue> {
        self.add_playable(label, tween.shared(), at)
    }

    /// Add a matrix tween.
    #[wasm_bindgen(js_name = addMat4Tween)]
    pub fn add_mat4_tween(&self, label: &str, tween: &Mat4Tween, at: &str) -> Result<(), JsValue> {
        self.add_playable(label, tween.shared(), at)
    }

    /// Add a nested timeline.
    #[wasm_bindgen(js_name = addTimeline)]
    pub fn add_timeline(&self, label: &str, timeline: &Timeline, at: &str) -> Result<(), JsValue> {
        self.add_playable(label, timeline.shared(), at)
    }

    /// Add a tween at the delay calculated by a stagger pattern.
    #[wasm_bindgen(js_name = addStaggeredTween)]
    pub fn add_staggered_tween(
        &self,
        label: &str,
        tween: &Tween,
        index: usize,
        total: usize,
        pattern: &StaggerPattern,
    ) -> Result<(), JsValue> {
        self.add_playable_at(
            label,
            tween.shared(),
            At::Absolute(pattern.delay(index, total).max(0.0)),
        )
    }

    /// Begin playback.
    pub fn play(&self) {
        lock(&self.inner).play();
    }

    /// Pause playback.
    pub fn pause(&self) {
        lock(&self.inner).pause();
    }

    /// Resume playback.
    pub fn resume(&self) {
        lock(&self.inner).resume();
    }

    /// Reset playback.
    pub fn reset(&self) {
        lock(&self.inner).reset();
    }

    /// Advance by seconds.
    pub fn update(&self, dt: f32) -> bool {
        lock(&self.inner).update(dt)
    }

    /// Seek normalized progress.
    pub fn seek(&self, progress: f32) {
        lock(&self.inner).seek(progress);
    }

    /// Seek to mirrored progress.
    pub fn reverse(&self) {
        lock(&self.inner).reverse();
    }

    /// Set group time scale.
    #[wasm_bindgen(js_name = setTimeScale)]
    pub fn set_time_scale(&self, scale: f32) {
        lock(&self.inner).set_time_scale(scale);
    }

    /// Group duration.
    pub fn duration(&self) -> f32 {
        lock(&self.inner).duration()
    }

    /// Group progress.
    pub fn progress(&self) -> f32 {
        lock(&self.inner).progress()
    }

    /// Whether the group completed.
    #[wasm_bindgen(js_name = isComplete)]
    pub fn is_complete(&self) -> bool {
        lock(&self.inner).is_complete()
    }

    pub(crate) fn shared(&self) -> SharedAnimationGroup {
        SharedAnimationGroup::new(Arc::clone(&self.inner))
    }

    fn add_playable<A>(&self, label: &str, animation: A, at: &str) -> Result<(), JsValue>
    where
        A: Playable + Send + 'static,
    {
        let at = parse_at(at)?;
        self.add_playable_at(label, animation, at)
    }

    fn add_playable_at<A>(&self, label: &str, animation: A, at: At<'_>) -> Result<(), JsValue>
    where
        A: Playable + Send + 'static,
    {
        let mut group = lock(&self.inner);
        let timeline = core::mem::take(group.timeline_mut()).add(label, animation, at);
        *group.timeline_mut() = timeline;
        Ok(())
    }
}

impl Default for AnimationGroup {
    fn default() -> Self {
        Self::new()
    }
}

/// Shared animation group update adapter.
#[derive(Clone, Debug)]
pub(crate) struct SharedAnimationGroup {
    inner: Arc<Mutex<CoreAnimationGroup>>,
}

impl SharedAnimationGroup {
    pub(crate) fn new(inner: Arc<Mutex<CoreAnimationGroup>>) -> Self {
        Self { inner }
    }
}

impl Update for SharedAnimationGroup {
    fn update(&mut self, dt: f32) -> bool {
        lock(&self.inner).update(dt)
    }
}

impl Playable for SharedAnimationGroup {
    fn duration(&self) -> f32 {
        lock(&self.inner).duration()
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

fn read_mat4(values: &Float32Array) -> JsResult<CoreMat4> {
    if values.length() != 16 {
        return Err(js_error("Mat4 requires exactly 16 values"));
    }
    let mut out = [0.0; 16];
    for (index, value) in out.iter_mut().enumerate() {
        *value = values.get_index(index as u32);
    }
    Ok(CoreMat4(out))
}

fn read_vec3(values: &Float32Array, name: &str) -> JsResult<[f32; 3]> {
    if values.length() != 3 {
        return Err(js_error(format!("{name} requires exactly 3 values")));
    }
    Ok([
        values.get_index(0),
        values.get_index(1),
        values.get_index(2),
    ])
}

fn parse_grid_origin(origin: &str) -> animato_tween::GridOrigin {
    match crate::types::normalize_name(origin).as_str() {
        "topleft" => animato_tween::GridOrigin::TopLeft,
        "topright" => animato_tween::GridOrigin::TopRight,
        "bottomleft" => animato_tween::GridOrigin::BottomLeft,
        "bottomright" => animato_tween::GridOrigin::BottomRight,
        "top" => animato_tween::GridOrigin::Top,
        "bottom" => animato_tween::GridOrigin::Bottom,
        "left" => animato_tween::GridOrigin::Left,
        "right" => animato_tween::GridOrigin::Right,
        _ => animato_tween::GridOrigin::Center,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quaternion_tween_returns_normalized_value() {
        let from = Quaternion::identity();
        let to = Quaternion::from_axis_angle(0.0, 0.0, 1.0, 180.0);
        let tween = QuaternionTween::new(&from, &to, 1.0);
        tween.update(0.5);
        let value = tween.value();
        let length = (value.x() * value.x()
            + value.y() * value.y()
            + value.z() * value.z()
            + value.w() * value.w())
        .sqrt();
        assert!((length - 1.0).abs() < 0.0001);
    }

    #[test]
    fn waveform_to_keyframes_works() {
        let wave = Waveform::sine(1.0, 1.0, 0.0);
        let track = wave.to_keyframes(1.0, 4.0);
        assert_eq!(track.duration(), 1.0);
    }
}
