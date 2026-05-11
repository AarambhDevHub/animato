//! FLIP-style layout transition orchestration for multiple DOM elements.

use crate::flip::{FlipAnimation, FlipState};
use animato_core::Easing;
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use web_sys::Element;

/// Orchestrates FLIP layout transitions for a named set of DOM elements.
///
/// Usage:
/// 1. Call [`snapshot`] to record current positions.
/// 2. Mutate the DOM (reorder, show/hide, resize).
/// 3. Call [`compute_transitions`] to build animations from snapshot to new positions.
/// 4. In your rAF loop: call [`update`] then [`apply`].
///
/// # Example
///
/// ```rust,ignore
/// let mut animator = LayoutAnimator::new();
///
/// // Before DOM mutation — capture positions.
/// animator.snapshot("card-a", &element_a);
/// animator.snapshot("card-b", &element_b);
///
/// // Mutate DOM here…
///
/// // Build animations from old → new positions.
/// animator.compute_transitions(
///     &[("card-a", &element_a), ("card-b", &element_b)],
///     0.4,
///     Easing::EaseInOutCubic,
/// );
///
/// // In rAF loop:
/// animator.update(dt);
/// animator.apply(&[("card-a", &element_a), ("card-b", &element_b)]).unwrap();
/// ```
#[derive(Debug, Default)]
pub struct LayoutAnimator {
    snapshots: HashMap<String, FlipState>,
    animations: HashMap<String, FlipAnimation>,
}

impl LayoutAnimator {
    /// Create an empty [`LayoutAnimator`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Capture the current bounding rect of an element under a key.
    ///
    /// The snapshot is stored until [`compute_transitions`] is called.
    pub fn snapshot(&mut self, key: impl Into<String>, element: &Element) {
        self.snapshots
            .insert(key.into(), FlipState::capture(element));
    }

    /// Build FLIP animations from stored snapshots to current element positions.
    ///
    /// Elements whose key has no stored snapshot are silently skipped.
    /// All snapshots are cleared after this call.
    pub fn compute_transitions(
        &mut self,
        elements: &[(&str, &Element)],
        duration: f32,
        easing: Easing,
    ) {
        for (key, element) in elements {
            if let Some(&first) = self.snapshots.get(*key) {
                let last = FlipState::capture(element);
                let animation = FlipAnimation::new(first, last)
                    .duration(duration)
                    .easing(easing.clone());
                self.animations.insert((*key).to_owned(), animation);
            }
        }
        self.snapshots.clear();
    }

    /// Advance all animations by `dt` seconds.
    pub fn update(&mut self, dt: f32) {
        for animation in self.animations.values_mut() {
            animation.update(dt);
        }
    }

    /// Apply the current CSS transforms to the corresponding elements.
    pub fn apply(&self, elements: &[(&str, &Element)]) -> Result<(), JsValue> {
        for (key, element) in elements {
            if let Some(animation) = self.animations.get(*key) {
                animation.apply_to(element)?;
            }
        }
        Ok(())
    }

    /// `true` when all tracked animations have finished (progress ≥ 1.0).
    pub fn is_complete(&self) -> bool {
        self.animations.values().all(|a| a.progress() >= 1.0)
    }

    /// Get the CSS `transform` string for a named element, if it exists.
    pub fn css_transform(&self, key: &str) -> Option<String> {
        self.animations.get(key).map(|a| a.css_transform())
    }

    /// Number of active animations.
    pub fn animation_count(&self) -> usize {
        self.animations.len()
    }

    /// Remove all completed animations to free memory.
    pub fn clear_completed(&mut self) {
        self.animations.retain(|_, a| a.progress() < 1.0);
    }
}
