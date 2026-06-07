//! Timeline bindings.

use crate::advanced::{AngleTween, AnimationGroup, Mat4Tween, QuaternionTween};
use crate::error::{JsResult, js_error, non_negative};
use crate::keyframe::{KeyframeTrack, KeyframeTrack2D, KeyframeTrack3D, KeyframeTrack4D};
use crate::path::MotionPath;
use crate::tween::{Tween, Tween2D, Tween3D, Tween4D, lock};
use crate::types::parse_loop_mode;
use animato_core::{Playable, Update};
use animato_timeline::{At, Timeline as CoreTimeline, TimelineState};
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;

fn state_name(state: TimelineState) -> &'static str {
    match state {
        TimelineState::Idle => "idle",
        TimelineState::Playing => "playing",
        TimelineState::Paused => "paused",
        TimelineState::Completed => "completed",
    }
}

pub(crate) fn parse_at(input: &str) -> JsResult<At<'_>> {
    let at = input.trim();
    if at.eq_ignore_ascii_case("start") {
        return Ok(At::Start);
    }
    if at.eq_ignore_ascii_case("end") {
        return Ok(At::End);
    }
    if let Some(label) = at.strip_prefix("label:") {
        return Ok(At::Label(label));
    }
    if let Some(offset) = at.strip_prefix('+') {
        return Ok(At::Offset(offset.parse::<f32>().map_err(|_| {
            js_error(format!("invalid timeline offset `{input}`"))
        })?));
    }
    if at.starts_with('-') {
        return Ok(At::Offset(at.parse::<f32>().map_err(|_| {
            js_error(format!("invalid timeline offset `{input}`"))
        })?));
    }
    if let Some(abs) = at.strip_prefix('@') {
        return Ok(At::Absolute(non_negative(
            abs.parse::<f32>()
                .map_err(|_| js_error(format!("invalid absolute timeline time `{input}`")))?,
            0.0,
        )));
    }
    if let Ok(abs) = at.parse::<f32>() {
        return Ok(At::Absolute(non_negative(abs, 0.0)));
    }
    Ok(At::Label(at))
}

/// Timeline composition of shared JS animations.
#[wasm_bindgen(js_name = Timeline)]
#[derive(Clone, Debug)]
pub struct Timeline {
    inner: Arc<Mutex<CoreTimeline>>,
}

#[wasm_bindgen(js_class = Timeline)]
impl Timeline {
    /// Create an empty timeline.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(CoreTimeline::new())),
        }
    }

    /// Add a scalar tween at a string position.
    #[wasm_bindgen(js_name = addTween)]
    pub fn add_tween(&self, label: &str, tween: &Tween, at: &str) -> Result<(), JsValue> {
        self.add_playable(label, tween.shared(), at)
    }

    /// Add a 2D tween.
    #[wasm_bindgen(js_name = addTween2D)]
    pub fn add_tween_2d(&self, label: &str, tween: &Tween2D, at: &str) -> Result<(), JsValue> {
        self.add_playable(label, tween.shared(), at)
    }

    /// Add a 3D tween.
    #[wasm_bindgen(js_name = addTween3D)]
    pub fn add_tween_3d(&self, label: &str, tween: &Tween3D, at: &str) -> Result<(), JsValue> {
        self.add_playable(label, tween.shared(), at)
    }

    /// Add a 4D tween.
    #[wasm_bindgen(js_name = addTween4D)]
    pub fn add_tween_4d(&self, label: &str, tween: &Tween4D, at: &str) -> Result<(), JsValue> {
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

    /// Add a scalar keyframe track.
    #[wasm_bindgen(js_name = addKeyframes)]
    pub fn add_keyframes(
        &self,
        label: &str,
        track: &KeyframeTrack,
        at: &str,
    ) -> Result<(), JsValue> {
        self.add_playable(label, track.shared(), at)
    }

    /// Add a 2D keyframe track.
    #[wasm_bindgen(js_name = addKeyframes2D)]
    pub fn add_keyframes_2d(
        &self,
        label: &str,
        track: &KeyframeTrack2D,
        at: &str,
    ) -> Result<(), JsValue> {
        self.add_playable(label, track.shared(), at)
    }

    /// Add a 3D keyframe track.
    #[wasm_bindgen(js_name = addKeyframes3D)]
    pub fn add_keyframes_3d(
        &self,
        label: &str,
        track: &KeyframeTrack3D,
        at: &str,
    ) -> Result<(), JsValue> {
        self.add_playable(label, track.shared(), at)
    }

    /// Add a 4D keyframe track.
    #[wasm_bindgen(js_name = addKeyframes4D)]
    pub fn add_keyframes_4d(
        &self,
        label: &str,
        track: &KeyframeTrack4D,
        at: &str,
    ) -> Result<(), JsValue> {
        self.add_playable(label, track.shared(), at)
    }

    /// Add a motion path animation.
    #[wasm_bindgen(js_name = addMotionPath)]
    pub fn add_motion_path(
        &self,
        label: &str,
        motion: &MotionPath,
        at: &str,
    ) -> Result<(), JsValue> {
        self.add_playable(label, motion.shared(), at)
    }

    /// Add an animation group.
    #[wasm_bindgen(js_name = addAnimationGroup)]
    pub fn add_animation_group(
        &self,
        label: &str,
        group: &AnimationGroup,
        at: &str,
    ) -> Result<(), JsValue> {
        self.add_playable(label, group.shared(), at)
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

    /// Reset timeline and children.
    pub fn reset(&self) {
        lock(&self.inner).reset();
    }

    /// Advance by `dt` seconds.
    pub fn update(&self, dt: f32) -> bool {
        lock(&self.inner).update(dt)
    }

    /// Seek normalized progress.
    pub fn seek(&self, progress: f32) {
        lock(&self.inner).seek(progress);
    }

    /// Seek absolute seconds.
    #[wasm_bindgen(js_name = seekAbs)]
    pub fn seek_abs(&self, seconds: f32) {
        lock(&self.inner).seek_abs(seconds);
    }

    /// Timeline duration.
    pub fn duration(&self) -> f32 {
        lock(&self.inner).duration()
    }

    /// Normalized progress.
    pub fn progress(&self) -> f32 {
        lock(&self.inner).progress()
    }

    /// Current state.
    pub fn state(&self) -> String {
        state_name(lock(&self.inner).state()).to_owned()
    }

    /// Whether playback has completed.
    #[wasm_bindgen(js_name = isComplete)]
    pub fn is_complete(&self) -> bool {
        lock(&self.inner).is_complete()
    }

    /// Number of entries.
    #[wasm_bindgen(js_name = entryCount)]
    pub fn entry_count(&self) -> usize {
        lock(&self.inner).entry_count()
    }

    /// Set timeline time scale.
    #[wasm_bindgen(js_name = setTimeScale)]
    pub fn set_time_scale(&self, scale: f32) {
        lock(&self.inner).set_time_scale(non_negative(scale, 1.0));
    }

    /// Set timeline loop mode.
    #[wasm_bindgen(js_name = setLoopMode)]
    pub fn set_loop_mode(&self, mode: &str) -> Result<(), JsValue> {
        lock(&self.inner).looping = parse_loop_mode(mode)?;
        Ok(())
    }

    fn add_playable<A>(&self, label: &str, animation: A, at: &str) -> Result<(), JsValue>
    where
        A: Playable + Send + 'static,
    {
        let at = parse_at(at)?;
        let mut timeline = lock(&self.inner);
        let next = core::mem::take(&mut *timeline).add(label, animation, at);
        *timeline = next;
        Ok(())
    }
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub(crate) struct SharedTimeline {
    inner: Arc<Mutex<CoreTimeline>>,
}

impl SharedTimeline {
    pub(crate) fn new(inner: Arc<Mutex<CoreTimeline>>) -> Self {
        Self { inner }
    }
}

impl Update for SharedTimeline {
    fn update(&mut self, dt: f32) -> bool {
        lock(&self.inner).update(dt)
    }
}

impl Playable for SharedTimeline {
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

impl Timeline {
    pub(crate) fn shared(&self) -> SharedTimeline {
        SharedTimeline::new(Arc::clone(&self.inner))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timeline_updates_shared_tween() {
        let tween = Tween::new(0.0, 100.0, 1.0);
        let timeline = Timeline::new();
        timeline.add_tween("x", &tween, "start").unwrap();
        timeline.play();
        timeline.update(0.5);
        assert_eq!(tween.value(), 50.0);
    }
}
