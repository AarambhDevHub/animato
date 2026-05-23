//! CPU batch tween evaluation for JavaScript.

use crate::easing::parse_easing;
use crate::error::require_index;
use crate::types::f32_array;
use animato_core::Update;
use animato_tween::Tween as CoreTween;
use js_sys::Float32Array;
use wasm_bindgen::prelude::*;

/// Batched scalar tween evaluator.
#[wasm_bindgen(js_name = TweenBatch)]
#[derive(Clone, Debug, Default)]
pub struct TweenBatch {
    tweens: Vec<CoreTween<f32>>,
    values: Vec<f32>,
}

#[wasm_bindgen(js_class = TweenBatch)]
impl TweenBatch {
    /// Create an empty batch.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Push a tween and return its batch index.
    pub fn push(
        &mut self,
        from: f32,
        to: f32,
        duration: f32,
        easing: &str,
    ) -> Result<u32, JsValue> {
        let tween = CoreTween::new(from, to)
            .duration(duration.max(0.0))
            .easing(parse_easing(easing)?)
            .build();
        let index = self.tweens.len();
        self.values.push(tween.value());
        self.tweens.push(tween);
        Ok(index as u32)
    }

    /// Advance every tween by `dt` seconds.
    pub fn tick(&mut self, dt: f32) {
        for (index, tween) in self.tweens.iter_mut().enumerate() {
            tween.update(dt);
            self.values[index] = tween.value();
        }
    }

    /// Current value at index.
    pub fn value(&self, index: u32) -> Result<f32, JsValue> {
        Ok(self.values[require_index(index, self.values.len(), "batch")?])
    }

    /// All current values.
    pub fn values(&self) -> Float32Array {
        f32_array(&self.values)
    }

    /// Number of tweens.
    pub fn len(&self) -> usize {
        self.tweens.len()
    }

    /// Whether the batch is empty.
    #[wasm_bindgen(js_name = isEmpty)]
    pub fn is_empty(&self) -> bool {
        self.tweens.is_empty()
    }

    /// Clear all tweens.
    pub fn clear(&mut self) {
        self.tweens.clear();
        self.values.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn batch_updates_values() {
        let mut batch = TweenBatch::new();
        let id = batch.push(0.0, 10.0, 1.0, "linear").unwrap();
        batch.tick(0.5);
        assert_eq!(batch.value(id).unwrap(), 5.0);
    }
}
