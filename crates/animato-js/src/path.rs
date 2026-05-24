//! Motion path and shape morphing bindings.

use crate::easing::parse_easing;
use crate::error::non_negative;
use crate::tween::lock;
use crate::types::{f32_array, flat_points, points_to_array, vec2};
use animato_core::{Playable, Update};
use animato_path::{
    DrawSvg, LineSegment, MorphPath as CoreMorphPath, MotionPath as CoreMotionPath,
    MotionPathTween as CoreMotionPathTween, PathEvaluate,
};
use js_sys::Float32Array;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;

/// Shared motion path update adapter.
#[derive(Clone, Debug)]
pub(crate) struct SharedMotionPath {
    inner: Arc<Mutex<CoreMotionPathTween>>,
}

impl SharedMotionPath {
    pub(crate) fn new(inner: Arc<Mutex<CoreMotionPathTween>>) -> Self {
        Self { inner }
    }
}

impl Update for SharedMotionPath {
    fn update(&mut self, dt: f32) -> bool {
        lock(&self.inner).update(dt)
    }
}

impl Playable for SharedMotionPath {
    fn duration(&self) -> f32 {
        lock(&self.inner).tween().duration
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

/// Tween-driven motion path.
#[wasm_bindgen(js_name = MotionPath)]
#[derive(Clone, Debug)]
pub struct MotionPath {
    inner: Arc<Mutex<CoreMotionPathTween>>,
}

#[wasm_bindgen(js_class = MotionPath)]
impl MotionPath {
    /// Create a motion path from an SVG `d` string.
    #[wasm_bindgen(constructor)]
    pub fn new(svg_path: &str, duration: f32) -> Result<Self, JsValue> {
        let path = CoreMotionPath::try_from_svg(svg_path)
            .map_err(|err| JsValue::from_str(&err.to_string()))?;
        Ok(Self {
            inner: Arc::new(Mutex::new(
                CoreMotionPathTween::new(path)
                    .duration(non_negative(duration, 1.0))
                    .build(),
            )),
        })
    }

    /// Create a straight-line motion path.
    #[wasm_bindgen(js_name = line)]
    pub fn line(from_x: f32, from_y: f32, to_x: f32, to_y: f32, duration: f32) -> Self {
        Self {
            inner: Arc::new(Mutex::new(
                CoreMotionPathTween::new(LineSegment::new([from_x, from_y], [to_x, to_y]))
                    .duration(non_negative(duration, 1.0))
                    .build(),
            )),
        }
    }

    /// Advance by `dt` seconds.
    pub fn update(&self, dt: f32) -> bool {
        lock(&self.inner).update(dt)
    }

    /// Current x position.
    pub fn x(&self) -> f32 {
        lock(&self.inner).value()[0]
    }

    /// Current y position.
    pub fn y(&self) -> f32 {
        lock(&self.inner).value()[1]
    }

    /// Current position as a typed array.
    #[wasm_bindgen(js_name = toArray)]
    pub fn to_array(&self) -> Float32Array {
        let pos = lock(&self.inner).value();
        vec2(pos[0], pos[1])
    }

    /// Current auto-rotation heading in degrees.
    #[wasm_bindgen(js_name = rotationDeg)]
    pub fn rotation_deg(&self) -> f32 {
        lock(&self.inner).rotation_deg()
    }

    /// Current normalized path progress after offsets.
    pub fn progress(&self) -> f32 {
        lock(&self.inner).path_progress()
    }

    /// Whether the motion tween is complete.
    #[wasm_bindgen(js_name = isComplete)]
    pub fn is_complete(&self) -> bool {
        lock(&self.inner).is_complete()
    }

    /// Reset playback.
    pub fn reset(&self) {
        lock(&self.inner).reset();
    }

    /// Seek normalized progress.
    pub fn seek(&self, progress: f32) {
        lock(&self.inner).seek(progress);
    }

    /// Set easing by name.
    #[wasm_bindgen(js_name = setEasing)]
    pub fn set_easing(&self, easing: &str) -> Result<(), JsValue> {
        lock(&self.inner).tween_mut().easing = parse_easing(easing)?;
        Ok(())
    }

    /// Enable or disable auto-rotation.
    #[wasm_bindgen(js_name = setAutoRotate)]
    pub fn set_auto_rotate(&self, yes: bool) {
        lock(&self.inner).set_auto_rotate(yes);
    }

    /// Set normalized path offsets.
    #[wasm_bindgen(js_name = setOffsets)]
    pub fn set_offsets(&self, start: f32, end: f32) {
        lock(&self.inner).set_offsets(start, end);
    }

    /// SVG draw values as `[dashArray, dashOffset, progress]`.
    #[wasm_bindgen(js_name = drawOn)]
    pub fn draw_on(&self, progress: f32) -> Float32Array {
        let values = lock(&self.inner).path().draw_on(progress);
        f32_array(&[values.dash_array, values.dash_offset, values.progress()])
    }

    /// SVG reverse draw values as `[dashArray, dashOffset, progress]`.
    #[wasm_bindgen(js_name = drawOnReverse)]
    pub fn draw_on_reverse(&self, progress: f32) -> Float32Array {
        let values = lock(&self.inner).path().draw_on_reverse(progress);
        f32_array(&[values.dash_array, values.dash_offset, values.progress()])
    }

    /// Total path length.
    #[wasm_bindgen(js_name = totalLength)]
    pub fn total_length(&self) -> f32 {
        lock(&self.inner).path().arc_length()
    }

    pub(crate) fn shared(&self) -> SharedMotionPath {
        SharedMotionPath::new(Arc::clone(&self.inner))
    }
}

/// Shape morphing between two point lists.
#[wasm_bindgen(js_name = MorphPath)]
#[derive(Clone, Debug)]
pub struct MorphPath {
    inner: CoreMorphPath,
}

#[wasm_bindgen(js_class = MorphPath)]
impl MorphPath {
    /// Create a morph from flat `[x0, y0, x1, y1, ...]` point arrays.
    #[wasm_bindgen(constructor)]
    pub fn new(from: &Float32Array, to: &Float32Array, resolution: usize) -> Result<Self, JsValue> {
        Ok(Self {
            inner: CoreMorphPath::with_resolution(
                flat_points(from)?,
                flat_points(to)?,
                resolution.max(2),
            ),
        })
    }

    /// Evaluate points at normalized progress.
    pub fn evaluate(&self, progress: f32) -> Float32Array {
        points_to_array(&self.inner.evaluate(progress))
    }

    /// Bounds at progress as `[minX, minY, maxX, maxY]`.
    #[wasm_bindgen(js_name = boundsAt)]
    pub fn bounds_at(&self, progress: f32) -> Float32Array {
        f32_array(&self.inner.bounds_at(progress))
    }

    /// Point count after resampling.
    #[wasm_bindgen(js_name = pointCount)]
    pub fn point_count(&self) -> usize {
        self.inner.point_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn motion_path_line_updates() {
        let path = MotionPath::line(0.0, 0.0, 100.0, 0.0, 1.0);
        path.update(0.5);
        assert_eq!(path.x(), 50.0);
    }
}
