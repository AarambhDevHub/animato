//! FLIP layout transition helpers for DOM elements.

use animato_core::{Easing, Update};
use animato_tween::Tween;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Element, HtmlElement};

/// Captured DOM element box used by FLIP layout animation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlipState {
    /// Left position in CSS pixels.
    pub x: f32,
    /// Top position in CSS pixels.
    pub y: f32,
    /// Width in CSS pixels.
    pub width: f32,
    /// Height in CSS pixels.
    pub height: f32,
}

impl FlipState {
    /// Capture the current bounding client rect of an element.
    pub fn capture(element: &Element) -> Self {
        let rect = element.get_bounding_client_rect();
        Self {
            x: rect.x() as f32,
            y: rect.y() as f32,
            width: rect.width() as f32,
            height: rect.height() as f32,
        }
    }
}

/// A FLIP transition from one captured DOM box to another.
#[derive(Clone, Debug)]
pub struct FlipAnimation {
    first: FlipState,
    last: FlipState,
    tween: Tween<f32>,
}

impl FlipAnimation {
    /// Create a FLIP animation from `first` to `last`.
    pub fn new(first: FlipState, last: FlipState) -> Self {
        Self {
            first,
            last,
            tween: Tween::new(0.0_f32, 1.0).duration(0.3).build(),
        }
    }

    /// Set animation duration in seconds.
    pub fn duration(mut self, seconds: f32) -> Self {
        self.tween.duration = seconds.max(0.0);
        self
    }

    /// Set the easing curve.
    pub fn easing(mut self, easing: Easing) -> Self {
        self.tween.easing = easing;
        self
    }

    /// Advance the FLIP animation.
    pub fn update(&mut self, dt: f32) -> bool {
        self.tween.update(dt)
    }

    /// Reset the FLIP animation to its first frame.
    pub fn reset(&mut self) {
        self.tween.reset();
    }

    /// Current normalized progress.
    pub fn progress(&self) -> f32 {
        self.tween.value()
    }

    /// Return the CSS transform for the current progress.
    pub fn css_transform(&self) -> String {
        self.css_transform_at(self.progress())
    }

    /// Return the CSS transform for a specific normalized progress.
    pub fn css_transform_at(&self, progress: f32) -> String {
        let p = progress.clamp(0.0, 1.0);
        let dx = (self.first.x - self.last.x) * (1.0 - p);
        let dy = (self.first.y - self.last.y) * (1.0 - p);
        let sx = scale_ratio(self.first.width, self.last.width, p);
        let sy = scale_ratio(self.first.height, self.last.height, p);
        format!("translate({dx:.3}px, {dy:.3}px) scale({sx:.5}, {sy:.5})")
    }

    /// Apply the current CSS transform to an element.
    pub fn apply_to(&self, element: &Element) -> Result<(), JsValue> {
        let html = element
            .dyn_ref::<HtmlElement>()
            .ok_or_else(|| JsValue::from_str("element is not an HtmlElement"))?;
        html.style()
            .set_property("transform", &self.css_transform())
    }
}

#[inline]
fn scale_ratio(first: f32, last: f32, progress: f32) -> f32 {
    if last.abs() <= f32::EPSILON {
        1.0
    } else {
        let inverted = first / last;
        inverted + (1.0 - inverted) * progress
    }
}
