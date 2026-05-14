//! Shared-element transition between two captured DOM positions.

use crate::flip::{FlipAnimation, FlipState};
use animato_core::Easing;
use wasm_bindgen::JsValue;
use web_sys::Element;

/// Animates a DOM element between two captured layout positions.
///
/// This is a single-element variant of [`LayoutAnimator`](crate::LayoutAnimator)
/// suitable for hero animations where one element transitions between two
/// visual states (e.g. a thumbnail expanding to full size).
///
/// # Example
///
/// ```rust,ignore
/// // Capture "before" state.
/// let first = FlipState::capture(&thumbnail);
///
/// // Mutate DOM: show the full-size version.
/// show_full_view();
///
/// // Capture "after" state.
/// let last = FlipState::capture(&full_view);
///
/// let mut transition = SharedElementTransition::new(first, last, 0.5, Easing::EaseInOutCubic);
///
/// // In rAF loop:
/// transition.update(dt);
/// transition.apply_to(&full_view).unwrap();
/// ```
#[derive(Debug)]
pub struct SharedElementTransition {
    animation: FlipAnimation,
    complete: bool,
}

impl SharedElementTransition {
    /// Build a transition from two captured states.
    pub fn new(first: FlipState, last: FlipState, duration: f32, easing: Easing) -> Self {
        Self {
            animation: FlipAnimation::new(first, last)
                .duration(duration)
                .easing(easing),
            complete: false,
        }
    }

    /// Capture both states from DOM elements and build a transition.
    pub fn capture(from: &Element, to: &Element, duration: f32, easing: Easing) -> Self {
        Self::new(
            FlipState::capture(from),
            FlipState::capture(to),
            duration,
            easing,
        )
    }

    /// Advance by `dt` seconds.
    ///
    /// Returns `true` while still running, `false` when complete.
    pub fn update(&mut self, dt: f32) -> bool {
        if self.complete {
            return false;
        }
        let running = self.animation.update(dt);
        if !running {
            self.complete = true;
        }
        running
    }

    /// Apply the current CSS transform to a DOM element.
    pub fn apply_to(&self, element: &Element) -> Result<(), JsValue> {
        self.animation.apply_to(element)
    }

    /// Current CSS `transform` string.
    pub fn css_transform(&self) -> String {
        self.animation.css_transform()
    }

    /// Normalised progress ∈ `[0.0, 1.0]`.
    pub fn progress(&self) -> f32 {
        self.animation.progress()
    }

    /// `true` after the transition has completed.
    pub fn is_complete(&self) -> bool {
        self.complete
    }

    /// Reset to the beginning.
    pub fn reset(&mut self) {
        self.complete = false;
        self.animation.reset();
    }
}
