//! rAF and scroll driver bindings.

use crate::advanced::{AngleTween, AnimationGroup, Mat4Tween, QuaternionTween};
use crate::error::non_negative;
use crate::keyframe::{KeyframeTrack, KeyframeTrack2D, KeyframeTrack3D, KeyframeTrack4D};
use crate::path::MotionPath;
use crate::physics::{Inertia, Inertia2D};
use crate::spring::{Spring, Spring2D, Spring3D, Spring4D};
use crate::timeline::Timeline;
use crate::tween::{Tween, Tween2D, Tween3D, Tween4D};
use animato_core::Update;
use animato_driver::ScrollDriver as CoreScrollDriver;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;

struct DriverSlot {
    id: u32,
    animation: Box<dyn Update + Send>,
    active: bool,
}

impl core::fmt::Debug for DriverSlot {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DriverSlot").field("id", &self.id).finish()
    }
}

/// requestAnimationFrame timestamp driver for JavaScript-owned animations.
#[wasm_bindgen(js_name = RafDriver)]
#[derive(Debug)]
pub struct RafDriver {
    slots: Vec<DriverSlot>,
    next_id: u32,
    last_timestamp_ms: Option<f64>,
    paused: bool,
    time_scale: f32,
    max_dt: f32,
}

#[wasm_bindgen(js_class = RafDriver)]
impl RafDriver {
    /// Create an empty rAF driver.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            next_id: 1,
            last_timestamp_ms: None,
            paused: false,
            time_scale: 1.0,
            max_dt: 0.25,
        }
    }

    /// Register a scalar tween and return its id.
    #[wasm_bindgen(js_name = addTween)]
    pub fn add_tween(&mut self, tween: &Tween) -> u32 {
        self.add_boxed(Box::new(tween.shared()))
    }

    /// Register a 2D tween.
    #[wasm_bindgen(js_name = addTween2D)]
    pub fn add_tween_2d(&mut self, tween: &Tween2D) -> u32 {
        self.add_boxed(Box::new(tween.shared()))
    }

    /// Register a 3D tween.
    #[wasm_bindgen(js_name = addTween3D)]
    pub fn add_tween_3d(&mut self, tween: &Tween3D) -> u32 {
        self.add_boxed(Box::new(tween.shared()))
    }

    /// Register a 4D tween.
    #[wasm_bindgen(js_name = addTween4D)]
    pub fn add_tween_4d(&mut self, tween: &Tween4D) -> u32 {
        self.add_boxed(Box::new(tween.shared()))
    }

    /// Register an angle tween.
    #[wasm_bindgen(js_name = addAngleTween)]
    pub fn add_angle_tween(&mut self, tween: &AngleTween) -> u32 {
        self.add_boxed(Box::new(tween.shared()))
    }

    /// Register a quaternion tween.
    #[wasm_bindgen(js_name = addQuaternionTween)]
    pub fn add_quaternion_tween(&mut self, tween: &QuaternionTween) -> u32 {
        self.add_boxed(Box::new(tween.shared()))
    }

    /// Register a matrix tween.
    #[wasm_bindgen(js_name = addMat4Tween)]
    pub fn add_mat4_tween(&mut self, tween: &Mat4Tween) -> u32 {
        self.add_boxed(Box::new(tween.shared()))
    }

    /// Register a scalar spring.
    #[wasm_bindgen(js_name = addSpring)]
    pub fn add_spring(&mut self, spring: &Spring) -> u32 {
        self.add_boxed(Box::new(spring.shared()))
    }

    /// Register a 2D spring.
    #[wasm_bindgen(js_name = addSpring2D)]
    pub fn add_spring_2d(&mut self, spring: &Spring2D) -> u32 {
        self.add_boxed(Box::new(spring.shared()))
    }

    /// Register a 3D spring.
    #[wasm_bindgen(js_name = addSpring3D)]
    pub fn add_spring_3d(&mut self, spring: &Spring3D) -> u32 {
        self.add_boxed(Box::new(spring.shared()))
    }

    /// Register a 4D spring.
    #[wasm_bindgen(js_name = addSpring4D)]
    pub fn add_spring_4d(&mut self, spring: &Spring4D) -> u32 {
        self.add_boxed(Box::new(spring.shared()))
    }

    /// Register a scalar keyframe track.
    #[wasm_bindgen(js_name = addKeyframes)]
    pub fn add_keyframes(&mut self, track: &KeyframeTrack) -> u32 {
        self.add_boxed(Box::new(track.shared()))
    }

    /// Register a 2D keyframe track.
    #[wasm_bindgen(js_name = addKeyframes2D)]
    pub fn add_keyframes_2d(&mut self, track: &KeyframeTrack2D) -> u32 {
        self.add_boxed(Box::new(track.shared()))
    }

    /// Register a 3D keyframe track.
    #[wasm_bindgen(js_name = addKeyframes3D)]
    pub fn add_keyframes_3d(&mut self, track: &KeyframeTrack3D) -> u32 {
        self.add_boxed(Box::new(track.shared()))
    }

    /// Register a 4D keyframe track.
    #[wasm_bindgen(js_name = addKeyframes4D)]
    pub fn add_keyframes_4d(&mut self, track: &KeyframeTrack4D) -> u32 {
        self.add_boxed(Box::new(track.shared()))
    }

    /// Register a timeline.
    #[wasm_bindgen(js_name = addTimeline)]
    pub fn add_timeline(&mut self, timeline: &Timeline) -> u32 {
        self.add_boxed(Box::new(timeline.shared()))
    }

    /// Register an animation group.
    #[wasm_bindgen(js_name = addAnimationGroup)]
    pub fn add_animation_group(&mut self, group: &AnimationGroup) -> u32 {
        self.add_boxed(Box::new(group.shared()))
    }

    /// Register a motion path.
    #[wasm_bindgen(js_name = addMotionPath)]
    pub fn add_motion_path(&mut self, motion: &MotionPath) -> u32 {
        self.add_boxed(Box::new(motion.shared()))
    }

    /// Register scalar inertia.
    #[wasm_bindgen(js_name = addInertia)]
    pub fn add_inertia(&mut self, inertia: &Inertia) -> u32 {
        self.add_boxed(Box::new(inertia.shared()))
    }

    /// Register 2D inertia.
    #[wasm_bindgen(js_name = addInertia2D)]
    pub fn add_inertia_2d(&mut self, inertia: &Inertia2D) -> u32 {
        self.add_boxed(Box::new(inertia.shared()))
    }

    /// Tick from a browser rAF timestamp in milliseconds.
    ///
    /// Returns the seconds delta applied to animations.
    pub fn tick(&mut self, timestamp_ms: f64) -> f32 {
        if !timestamp_ms.is_finite() {
            return 0.0;
        }
        let raw_dt = match self.last_timestamp_ms.replace(timestamp_ms) {
            Some(last) => ((timestamp_ms - last) / 1000.0).max(0.0) as f32,
            None => 0.0,
        };
        if self.paused {
            return 0.0;
        }
        let dt = raw_dt.min(self.max_dt) * self.time_scale;
        self.tick_dt(dt);
        dt
    }

    /// Tick by an explicit seconds delta.
    #[wasm_bindgen(js_name = tickDt)]
    pub fn tick_dt(&mut self, dt: f32) {
        let dt = non_negative(dt, 0.0);
        for slot in &mut self.slots {
            slot.active = slot.animation.update(dt);
        }
        self.slots.retain(|slot| slot.active);
    }

    /// Pause ticking.
    pub fn pause(&mut self) {
        self.paused = true;
    }

    /// Resume ticking.
    pub fn resume(&mut self) {
        self.paused = false;
    }

    /// Whether ticking is paused.
    #[wasm_bindgen(js_name = isPaused)]
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    /// Reset stored timestamp.
    #[wasm_bindgen(js_name = resetTimestamp)]
    pub fn reset_timestamp(&mut self) {
        self.last_timestamp_ms = None;
    }

    /// Set time scale.
    #[wasm_bindgen(js_name = setTimeScale)]
    pub fn set_time_scale(&mut self, scale: f32) {
        self.time_scale = non_negative(scale, 1.0);
    }

    /// Set maximum accepted frame delta.
    #[wasm_bindgen(js_name = setMaxDt)]
    pub fn set_max_dt(&mut self, max_dt: f32) {
        self.max_dt = non_negative(max_dt, 0.25);
    }

    /// Cancel an animation by id.
    pub fn cancel(&mut self, id: u32) {
        self.slots.retain(|slot| slot.id != id);
    }

    /// Cancel all animations.
    #[wasm_bindgen(js_name = cancelAll)]
    pub fn cancel_all(&mut self) {
        self.slots.clear();
    }

    /// Number of active animations.
    #[wasm_bindgen(js_name = activeCount)]
    pub fn active_count(&self) -> usize {
        self.slots.len()
    }

    /// Whether an animation id is active.
    #[wasm_bindgen(js_name = isActive)]
    pub fn is_active(&self, id: u32) -> bool {
        self.slots.iter().any(|slot| slot.id == id)
    }

    fn add_boxed(&mut self, animation: Box<dyn Update + Send>) -> u32 {
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1).max(1);
        self.slots.push(DriverSlot {
            id,
            animation,
            active: true,
        });
        id
    }
}

impl Default for RafDriver {
    fn default() -> Self {
        Self::new()
    }
}

/// Scroll position driver for scroll-linked animations.
#[wasm_bindgen(js_name = ScrollDriver)]
#[derive(Clone, Debug)]
pub struct ScrollDriver {
    inner: Arc<Mutex<CoreScrollDriver>>,
}

#[wasm_bindgen(js_class = ScrollDriver)]
impl ScrollDriver {
    /// Create a scroll driver with a min and max scroll position.
    #[wasm_bindgen(constructor)]
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            inner: Arc::new(Mutex::new(CoreScrollDriver::new(min, max))),
        }
    }

    /// Register a tween.
    #[wasm_bindgen(js_name = addTween)]
    pub fn add_tween(&self, tween: &Tween) {
        self.inner
            .lock()
            .expect("animato-js scroll driver lock poisoned")
            .add(tween.shared());
    }

    /// Set the current scroll position.
    #[wasm_bindgen(js_name = setPosition)]
    pub fn set_position(&self, position: f32) {
        self.inner
            .lock()
            .expect("animato-js scroll driver lock poisoned")
            .set_position(position);
    }

    /// Current normalized scroll progress.
    pub fn progress(&self) -> f32 {
        self.inner
            .lock()
            .expect("animato-js scroll driver lock poisoned")
            .progress()
    }

    /// Current scroll position.
    pub fn position(&self) -> f32 {
        self.inner
            .lock()
            .expect("animato-js scroll driver lock poisoned")
            .position()
    }

    /// Number of registered animations.
    #[wasm_bindgen(js_name = animationCount)]
    pub fn animation_count(&self) -> usize {
        self.inner
            .lock()
            .expect("animato-js scroll driver lock poisoned")
            .animation_count()
    }

    /// Remove completed animations.
    #[wasm_bindgen(js_name = clearCompleted)]
    pub fn clear_completed(&self) {
        self.inner
            .lock()
            .expect("animato-js scroll driver lock poisoned")
            .clear_completed();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::require_index;

    #[test]
    fn raf_driver_ticks_tween() {
        let tween = Tween::new(0.0, 1.0, 1.0);
        let mut driver = RafDriver::new();
        let id = driver.add_tween(&tween);
        driver.tick_dt(0.5);
        assert!(driver.is_active(id));
        assert_eq!(tween.value(), 0.5);
    }

    #[test]
    fn invalid_index_helper_errors() {
        assert!(require_index(4, 2, "test").is_err());
    }
}
