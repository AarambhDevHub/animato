//! WASM counter example for Animato.

use animato::{Easing, RafDriver, Tween, Update};
use wasm_bindgen::prelude::*;

/// Small app driven by JavaScript `requestAnimationFrame`.
#[wasm_bindgen]
pub struct CounterApp {
    tween: Tween<f32>,
    driver: RafDriver,
}

#[wasm_bindgen]
impl CounterApp {
    /// Create a new counter animation.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            tween: Tween::new(0.0_f32, 500.0)
                .duration(1.5)
                .easing(Easing::EaseOutBounce)
                .build(),
            driver: RafDriver::new(),
        }
    }

    /// Advance from a `requestAnimationFrame` timestamp in milliseconds.
    pub fn tick(&mut self, timestamp_ms: f64) {
        let dt = self.driver.tick(timestamp_ms);
        self.tween.update(dt);
    }

    /// Current animated value.
    pub fn value(&self) -> f32 {
        self.tween.value()
    }
}

impl Default for CounterApp {
    fn default() -> Self {
        Self::new()
    }
}
