//! Physics and gesture bindings.

use crate::error::{JsResult, js_error, non_negative};
use crate::tween::lock;
use crate::types::{f32_array, normalize_name, vec2};
use animato_core::Update;
use animato_physics::{
    DragAxis, DragConstraints, DragState as CoreDragState, Gesture, GestureConfig,
    GestureRecognizer as CoreGestureRecognizer, Inertia as CoreInertia, InertiaBounds,
    InertiaConfig, InertiaN, PointerData, SwipeDirection,
};
use js_sys::Float32Array;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;

fn inertia_config(friction: f32, min_velocity: f32) -> InertiaConfig<f32> {
    InertiaConfig::new(
        non_negative(friction, 1400.0),
        non_negative(min_velocity, 2.0),
    )
}

fn inertia_config_2d(friction: f32, min_velocity: f32) -> InertiaConfig<[f32; 2]> {
    InertiaConfig::new(
        non_negative(friction, 1400.0),
        non_negative(min_velocity, 2.0),
    )
}

fn axis(name: &str) -> JsResult<DragAxis> {
    match normalize_name(name).as_str() {
        "both" | "xy" => Ok(DragAxis::Both),
        "x" => Ok(DragAxis::X),
        "y" => Ok(DragAxis::Y),
        _ => Err(js_error(format!("unknown drag axis `{name}`"))),
    }
}

/// Scalar inertia animation.
#[wasm_bindgen(js_name = Inertia)]
#[derive(Clone, Debug)]
pub struct Inertia {
    inner: Arc<Mutex<CoreInertia>>,
}

#[wasm_bindgen(js_class = Inertia)]
impl Inertia {
    /// Create inertia with initial position, velocity, friction, and min velocity.
    #[wasm_bindgen(constructor)]
    pub fn new(position: f32, velocity: f32, friction: f32, min_velocity: f32) -> Self {
        let mut inertia =
            CoreInertia::with_position(inertia_config(friction, min_velocity), position);
        inertia.kick(velocity);
        Self {
            inner: Arc::new(Mutex::new(inertia)),
        }
    }

    /// Use a named preset: `smooth`, `snappy`, or `heavy`.
    #[wasm_bindgen(js_name = withPreset)]
    pub fn with_preset(position: f32, velocity: f32, preset: &str) -> Result<Inertia, JsValue> {
        let config = match normalize_name(preset).as_str() {
            "smooth" => InertiaConfig::smooth(),
            "snappy" => InertiaConfig::snappy(),
            "heavy" => InertiaConfig::heavy(),
            _ => return Err(js_error(format!("unknown inertia preset `{preset}`"))),
        };
        let mut inertia = CoreInertia::with_position(config, position);
        inertia.kick(velocity);
        Ok(Self {
            inner: Arc::new(Mutex::new(inertia)),
        })
    }

    /// Set inclusive bounds.
    #[wasm_bindgen(js_name = setBounds)]
    pub fn set_bounds(&self, min: f32, max: f32) {
        lock(&self.inner).config.bounds = Some(InertiaBounds::new(min, max));
    }

    /// Start from a new velocity.
    pub fn kick(&self, velocity: f32) {
        lock(&self.inner).kick(velocity);
    }

    /// Advance by `dt` seconds.
    pub fn update(&self, dt: f32) -> bool {
        lock(&self.inner).update(dt)
    }

    /// Current position.
    pub fn position(&self) -> f32 {
        lock(&self.inner).position()
    }

    /// Current velocity.
    pub fn velocity(&self) -> f32 {
        lock(&self.inner).velocity()
    }

    /// Snap instantly.
    #[wasm_bindgen(js_name = snapTo)]
    pub fn snap_to(&self, position: f32) {
        lock(&self.inner).snap_to(position);
    }

    /// Whether inertia has settled.
    #[wasm_bindgen(js_name = isSettled)]
    pub fn is_settled(&self) -> bool {
        lock(&self.inner).is_settled()
    }

    pub(crate) fn shared(&self) -> SharedInertia {
        SharedInertia {
            inner: Arc::clone(&self.inner),
        }
    }
}

/// Shared scalar inertia update adapter.
#[derive(Clone, Debug)]
pub(crate) struct SharedInertia {
    inner: Arc<Mutex<CoreInertia>>,
}

impl Update for SharedInertia {
    fn update(&mut self, dt: f32) -> bool {
        lock(&self.inner).update(dt)
    }
}

/// 2D inertia animation.
#[wasm_bindgen(js_name = Inertia2D)]
#[derive(Clone, Debug)]
pub struct Inertia2D {
    inner: Arc<Mutex<InertiaN<[f32; 2]>>>,
}

impl Inertia2D {
    fn from_core(inner: InertiaN<[f32; 2]>) -> Self {
        Self {
            inner: Arc::new(Mutex::new(inner)),
        }
    }
}

#[wasm_bindgen(js_class = Inertia2D)]
impl Inertia2D {
    /// Create 2D inertia.
    #[wasm_bindgen(constructor)]
    pub fn new(
        x: f32,
        y: f32,
        velocity_x: f32,
        velocity_y: f32,
        friction: f32,
        min_velocity: f32,
    ) -> Self {
        let mut inertia = InertiaN::new(inertia_config_2d(friction, min_velocity), [x, y]);
        inertia.kick([velocity_x, velocity_y]);
        Self::from_core(inertia)
    }

    /// Set inclusive 2D bounds.
    #[wasm_bindgen(js_name = setBounds)]
    pub fn set_bounds(&self, min_x: f32, max_x: f32, min_y: f32, max_y: f32) {
        let mut current = lock(&self.inner);
        let pos = current.position();
        let vel = current.velocity();
        let mut config = inertia_config_2d(1400.0, 2.0);
        config.bounds = Some(InertiaBounds::new([min_x, min_y], [max_x, max_y]));
        let mut next = InertiaN::new(config, pos);
        next.kick(vel);
        *current = next;
    }

    /// Start from a new velocity.
    pub fn kick(&self, velocity_x: f32, velocity_y: f32) {
        lock(&self.inner).kick([velocity_x, velocity_y]);
    }

    /// Advance by `dt` seconds.
    pub fn update(&self, dt: f32) -> bool {
        lock(&self.inner).update(dt)
    }

    /// Current position.
    #[wasm_bindgen(js_name = toArray)]
    pub fn to_array(&self) -> Float32Array {
        let pos = lock(&self.inner).position();
        f32_array(&pos)
    }

    /// Current velocity.
    #[wasm_bindgen(js_name = velocityArray)]
    pub fn velocity_array(&self) -> Float32Array {
        let velocity = lock(&self.inner).velocity();
        f32_array(&velocity)
    }

    /// Whether inertia has settled.
    #[wasm_bindgen(js_name = isSettled)]
    pub fn is_settled(&self) -> bool {
        lock(&self.inner).is_settled()
    }

    pub(crate) fn shared(&self) -> SharedInertia2D {
        SharedInertia2D {
            inner: Arc::clone(&self.inner),
        }
    }
}

/// Shared 2D inertia update adapter.
#[derive(Clone, Debug)]
pub(crate) struct SharedInertia2D {
    inner: Arc<Mutex<InertiaN<[f32; 2]>>>,
}

impl Update for SharedInertia2D {
    fn update(&mut self, dt: f32) -> bool {
        lock(&self.inner).update(dt)
    }
}

/// Pointer drag tracker.
#[wasm_bindgen(js_name = DragState)]
#[derive(Clone, Debug)]
pub struct DragState {
    inner: Arc<Mutex<CoreDragState>>,
}

#[wasm_bindgen(js_class = DragState)]
impl DragState {
    /// Create a drag tracker at an initial position.
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            inner: Arc::new(Mutex::new(CoreDragState::new([x, y]))),
        }
    }

    /// Set axis filter.
    #[wasm_bindgen(js_name = setAxis)]
    pub fn set_axis(&self, axis_name: &str) -> Result<(), JsValue> {
        let mut drag = lock(&self.inner);
        let next =
            core::mem::replace(&mut *drag, CoreDragState::new([0.0, 0.0])).axis(axis(axis_name)?);
        *drag = next;
        Ok(())
    }

    /// Set rectangular bounds.
    #[wasm_bindgen(js_name = setBounds)]
    pub fn set_bounds(&self, min_x: f32, max_x: f32, min_y: f32, max_y: f32) {
        lock(&self.inner).set_constraints(DragConstraints::bounded(min_x, max_x, min_y, max_y));
    }

    /// Set grid snap size.
    #[wasm_bindgen(js_name = setGridSnap)]
    pub fn set_grid_snap(&self, grid: f32) {
        lock(&self.inner).set_constraints(DragConstraints::unbounded().with_grid_snap(grid));
    }

    /// Current position.
    #[wasm_bindgen(js_name = toArray)]
    pub fn to_array(&self) -> Float32Array {
        let pos = lock(&self.inner).position();
        vec2(pos[0], pos[1])
    }

    /// Current velocity.
    #[wasm_bindgen(js_name = velocityArray)]
    pub fn velocity_array(&self) -> Float32Array {
        let velocity = lock(&self.inner).velocity();
        vec2(velocity[0], velocity[1])
    }

    /// Whether pointer is captured.
    #[wasm_bindgen(js_name = isDragging)]
    pub fn is_dragging(&self) -> bool {
        lock(&self.inner).is_dragging()
    }

    /// Pointer down.
    #[wasm_bindgen(js_name = pointerDown)]
    pub fn pointer_down(&self, x: f32, y: f32, pointer_id: u32) {
        lock(&self.inner).on_pointer_down(PointerData::new(x, y, pointer_id as u64));
    }

    /// Pointer move with seconds delta.
    #[wasm_bindgen(js_name = pointerMove)]
    pub fn pointer_move(&self, x: f32, y: f32, pointer_id: u32, dt: f32) {
        lock(&self.inner).on_pointer_move(
            PointerData::new(x, y, pointer_id as u64),
            non_negative(dt, 0.0),
        );
    }

    /// Pointer up. Returns inertia when release velocity is high enough.
    #[wasm_bindgen(js_name = pointerUp)]
    pub fn pointer_up(&self, x: f32, y: f32, pointer_id: u32) -> Option<Inertia2D> {
        lock(&self.inner)
            .on_pointer_up(PointerData::new(x, y, pointer_id as u64))
            .map(Inertia2D::from_core)
    }

    /// Snap instantly to a position.
    #[wasm_bindgen(js_name = snapTo)]
    pub fn snap_to(&self, x: f32, y: f32) {
        lock(&self.inner).snap_to([x, y]);
    }
}

/// Pointer gesture recognizer.
#[wasm_bindgen(js_name = GestureRecognizer)]
#[derive(Clone, Debug)]
pub struct GestureRecognizer {
    inner: Arc<Mutex<CoreGestureRecognizer>>,
}

#[wasm_bindgen(js_class = GestureRecognizer)]
impl GestureRecognizer {
    /// Create a recognizer with default thresholds.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(CoreGestureRecognizer::default())),
        }
    }

    /// Set custom thresholds.
    #[wasm_bindgen(js_name = setConfig)]
    pub fn set_config(
        &self,
        tap_max_distance: f32,
        tap_max_duration: f32,
        swipe_min_distance: f32,
        long_press_duration: f32,
        double_tap_max_interval: f32,
    ) {
        *lock(&self.inner) = CoreGestureRecognizer::new(GestureConfig {
            tap_max_distance: non_negative(tap_max_distance, 8.0),
            tap_max_duration: non_negative(tap_max_duration, 0.25),
            swipe_min_distance: non_negative(swipe_min_distance, 40.0),
            long_press_duration: non_negative(long_press_duration, 0.5),
            double_tap_max_interval: non_negative(double_tap_max_interval, 0.3),
        });
    }

    /// Pointer down at timestamp seconds.
    #[wasm_bindgen(js_name = pointerDown)]
    pub fn pointer_down(&self, x: f32, y: f32, pointer_id: u32, time_seconds: f32) {
        lock(&self.inner).on_pointer_down(
            PointerData::new(x, y, pointer_id as u64),
            non_negative(time_seconds, 0.0),
        );
    }

    /// Pointer move at timestamp seconds.
    #[wasm_bindgen(js_name = pointerMove)]
    pub fn pointer_move(&self, x: f32, y: f32, pointer_id: u32, time_seconds: f32) {
        lock(&self.inner).on_pointer_move(
            PointerData::new(x, y, pointer_id as u64),
            non_negative(time_seconds, 0.0),
        );
    }

    /// Pointer up. Returns a gesture object or `undefined`.
    #[wasm_bindgen(js_name = pointerUp)]
    pub fn pointer_up(
        &self,
        x: f32,
        y: f32,
        pointer_id: u32,
        time_seconds: f32,
    ) -> Result<JsValue, JsValue> {
        let gesture = lock(&self.inner).on_pointer_up(
            PointerData::new(x, y, pointer_id as u64),
            non_negative(time_seconds, 0.0),
        );
        match gesture {
            Some(gesture) => Ok(gesture_to_value(gesture)),
            None => Ok(JsValue::UNDEFINED),
        }
    }
}

impl Default for GestureRecognizer {
    fn default() -> Self {
        Self::new()
    }
}

fn gesture_to_value(gesture: Gesture) -> JsValue {
    match gesture {
        Gesture::Tap { position } => object(&[
            ("type", JsValue::from_str("tap")),
            ("x", JsValue::from_f64(position[0] as f64)),
            ("y", JsValue::from_f64(position[1] as f64)),
        ]),
        Gesture::DoubleTap { position } => object(&[
            ("type", JsValue::from_str("doubleTap")),
            ("x", JsValue::from_f64(position[0] as f64)),
            ("y", JsValue::from_f64(position[1] as f64)),
        ]),
        Gesture::LongPress { position, duration } => object(&[
            ("type", JsValue::from_str("longPress")),
            ("x", JsValue::from_f64(position[0] as f64)),
            ("y", JsValue::from_f64(position[1] as f64)),
            ("duration", JsValue::from_f64(duration as f64)),
        ]),
        Gesture::Swipe {
            direction,
            velocity,
            distance,
        } => object(&[
            ("type", JsValue::from_str("swipe")),
            (
                "direction",
                JsValue::from_str(match direction {
                    SwipeDirection::Up => "up",
                    SwipeDirection::Down => "down",
                    SwipeDirection::Left => "left",
                    SwipeDirection::Right => "right",
                }),
            ),
            ("velocity", JsValue::from_f64(velocity as f64)),
            ("distance", JsValue::from_f64(distance as f64)),
        ]),
        Gesture::Pinch { scale, center } => object(&[
            ("type", JsValue::from_str("pinch")),
            ("scale", JsValue::from_f64(scale as f64)),
            ("centerX", JsValue::from_f64(center[0] as f64)),
            ("centerY", JsValue::from_f64(center[1] as f64)),
        ]),
        Gesture::Rotation {
            angle_delta,
            center,
        } => object(&[
            ("type", JsValue::from_str("rotation")),
            ("angleDelta", JsValue::from_f64(angle_delta as f64)),
            ("centerX", JsValue::from_f64(center[0] as f64)),
            ("centerY", JsValue::from_f64(center[1] as f64)),
        ]),
    }
}

fn object(entries: &[(&str, JsValue)]) -> JsValue {
    let object = js_sys::Object::new();
    for (key, value) in entries {
        let _ = js_sys::Reflect::set(&object, &JsValue::from_str(key), value);
    }
    object.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inertia_moves() {
        let inertia = Inertia::new(0.0, 100.0, 1000.0, 1.0);
        inertia.update(0.016);
        assert!(inertia.position() > 0.0);
    }

    #[test]
    fn drag_tracks_position() {
        let drag = DragState::new(0.0, 0.0);
        drag.pointer_down(0.0, 0.0, 1);
        drag.pointer_move(20.0, 10.0, 1, 0.016);
        assert!(drag.is_dragging());
    }
}
