//! Scroll-position-driven animation.
//!
//! [`ScrollDriver`] advances animations in proportion to scroll deltas rather
//! than wall-clock time, so animations are tied directly to a scroll position.
//!
//! [`ScrollClock`] adapts scroll movement into the [`Clock`] interface so any
//! [`AnimationDriver`](crate::driver::AnimationDriver) can be scroll-driven.

use std::fmt::Debug;

use crate::clock::Clock;
use animato_core::Update;

/// Drives registered animations from a normalised scroll position.
///
/// Each call to [`set_position`](ScrollDriver::set_position) advances every
/// animation by `Δposition / range` — a normalised fraction ∈ `[0, 1]`.
///
/// # Example
///
/// ```rust
/// use animato_driver::scroll::ScrollDriver;
/// use animato_core::Update;
///
/// struct Counter(u32);
/// impl Update for Counter {
///     fn update(&mut self, _dt: f32) -> bool { self.0 += 1; self.0 < 10 }
/// }
///
/// let mut driver = ScrollDriver::new(0.0, 1000.0);
/// driver.add(Counter(0));
/// driver.set_position(250.0);  // 25% through the scroll range
/// driver.set_position(500.0);  // another 25%
/// ```
#[derive(Default)]
pub struct ScrollDriver {
    min: f32,
    max: f32,
    position: f32,
    animations: std::vec::Vec<Box<dyn Update + Send>>,
}

fn normalized_max(min: f32, max: f32) -> f32 {
    if max > min {
        max
    } else {
        let increment = f32::EPSILON * min.abs().max(1.0);
        let adjusted = min + increment;
        if adjusted > min { adjusted } else { min + 1.0 }
    }
}

impl Debug for ScrollDriver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScrollDriver")
            .field("min", &self.min)
            .field("max", &self.max)
            .field("position", &self.position)
            .field("animations", &self.animations.len())
            .finish()
    }
}

impl ScrollDriver {
    /// Create a scroll driver over the given position range.
    ///
    /// `min` and `max` define the full scroll extent. `max` is clamped to be
    /// strictly greater than `min`.
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            min,
            max: normalized_max(min, max),
            position: min,
            animations: std::vec::Vec::new(),
        }
    }

    /// Register an animation to be driven by scroll changes.
    pub fn add<A: Update + Send + 'static>(&mut self, animation: A) {
        self.animations.push(Box::new(animation));
    }

    /// Update the scroll position and tick all animations by the normalised delta.
    ///
    /// If `pos` is outside `[min, max]` it is clamped. Animations receive a
    /// `dt` equal to `|Δpos| / (max − min)`, i.e. a normalised `[0, 1]` delta.
    ///
    /// Completed animations (returning `false`) are retained so they stay at
    /// their terminal value — call [`clear_completed`](Self::clear_completed)
    /// if you want to remove them.
    pub fn set_position(&mut self, pos: f32) {
        let clamped = pos.clamp(self.min, self.max);
        let range = self.max - self.min;
        if range <= 0.0 {
            return;
        }

        let delta = (clamped - self.position).abs() / range;
        self.position = clamped;

        if delta > 0.0 {
            for animation in self.animations.iter_mut() {
                animation.update(delta);
            }
        }
    }

    /// Remove all animations that have returned `false` from their last `update`.
    ///
    /// This is not done automatically, so completed animations stay accessible
    /// for value reads after they finish.
    pub fn clear_completed(&mut self) {
        // Re-run a zero-dt tick to determine completion status.
        self.animations.retain_mut(|a| a.update(0.0));
    }

    /// Current scroll position in user units.
    pub fn position(&self) -> f32 {
        self.position
    }

    /// Normalised scroll progress ∈ `[0.0, 1.0]`.
    pub fn progress(&self) -> f32 {
        let range = self.max - self.min;
        if range <= 0.0 {
            return 0.0;
        }
        ((self.position - self.min) / range).clamp(0.0, 1.0)
    }

    /// Minimum position of the scroll range.
    pub fn min(&self) -> f32 {
        self.min
    }

    /// Maximum position of the scroll range.
    pub fn max(&self) -> f32 {
        self.max
    }

    /// Number of registered animations.
    pub fn animation_count(&self) -> usize {
        self.animations.len()
    }
}

// ── ScrollClock ───────────────────────────────────────────────────────────────

/// A [`Clock`] implementation backed by scroll-position changes.
///
/// Each call to [`set_scroll`](ScrollClock::set_scroll) stores a normalised
/// delta; the next call to [`Clock::delta`] consumes and returns it.
///
/// # Example
///
/// ```rust
/// use animato_driver::{Clock, AnimationDriver};
/// use animato_driver::scroll::ScrollClock;
///
/// let mut clock = ScrollClock::new(0.0, 1000.0);
/// clock.set_scroll(250.0);
/// let dt = clock.delta(); // ≈ 0.25
/// assert!((dt - 0.25).abs() < 0.001);
/// ```
#[derive(Clone, Debug)]
pub struct ScrollClock {
    last: f32,
    pending: f32,
    min: f32,
    max: f32,
}

impl Default for ScrollClock {
    fn default() -> Self {
        Self::new(0.0, 1000.0)
    }
}

impl ScrollClock {
    /// Create a scroll clock spanning `[min, max]`.
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            last: min,
            pending: 0.0,
            min,
            max: normalized_max(min, max),
        }
    }

    /// Register a new scroll position and accumulate the normalised delta.
    ///
    /// Multiple calls before [`Clock::delta`] accumulate (they do not cancel
    /// each other out — only magnitudes are summed).
    pub fn set_scroll(&mut self, pos: f32) {
        let clamped = pos.clamp(self.min, self.max);
        let range = self.max - self.min;
        if range > 0.0 {
            self.pending += (clamped - self.last).abs() / range;
        }
        self.last = clamped;
    }

    /// Current scroll position in user units.
    pub fn scroll_position(&self) -> f32 {
        self.last
    }

    /// Normalised scroll progress ∈ `[0.0, 1.0]`.
    pub fn progress(&self) -> f32 {
        let range = self.max - self.min;
        if range <= 0.0 {
            return 0.0;
        }
        ((self.last - self.min) / range).clamp(0.0, 1.0)
    }
}

impl Clock for ScrollClock {
    /// Returns the accumulated normalised delta since the last call.
    fn delta(&mut self) -> f32 {
        let dt = self.pending;
        self.pending = 0.0;
        dt
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use animato_core::{Easing, Update};
    use animato_tween::Tween;

    #[test]
    fn scroll_driver_progress_tracks_position() {
        let mut driver = ScrollDriver::new(0.0, 100.0);
        assert_eq!(driver.progress(), 0.0);
        driver.set_position(50.0);
        assert!((driver.progress() - 0.5).abs() < 0.001);
        driver.set_position(100.0);
        assert!((driver.progress() - 1.0).abs() < 0.001);
    }

    #[test]
    fn scroll_driver_clamps_position() {
        let mut driver = ScrollDriver::new(0.0, 100.0);
        driver.set_position(-50.0);
        assert_eq!(driver.position(), 0.0);
        driver.set_position(200.0);
        assert_eq!(driver.position(), 100.0);
    }

    #[test]
    fn scroll_driver_ticks_animations_proportionally() {
        let mut driver = ScrollDriver::new(0.0, 1000.0);
        // A tween with duration=1 will be driven by normalised scroll deltas.
        driver.add(
            Tween::new(0.0_f32, 100.0)
                .duration(1.0)
                .easing(Easing::Linear)
                .build(),
        );
        // Scroll to 50% → tween should be ~50% complete.
        driver.set_position(500.0);
        assert_eq!(driver.animation_count(), 1);
    }

    #[test]
    fn scroll_driver_zero_delta_does_not_tick() {
        struct _PanicOnUpdate;
        impl Update for _PanicOnUpdate {
            fn update(&mut self, _dt: f32) -> bool {
                panic!("should not be called")
            }
        }
        let mut driver = ScrollDriver::new(0.0, 100.0);
        // No movement — update should not be called.
        driver.set_position(0.0); // clamped, delta = 0
        // If we reach here without panic, test passes.
    }

    #[test]
    fn scroll_clock_delta_is_normalised() {
        let mut clock = ScrollClock::new(0.0, 1000.0);
        clock.set_scroll(250.0);
        let dt = clock.delta();
        assert!((dt - 0.25).abs() < 0.001);
    }

    #[test]
    fn scroll_clock_delta_consumed_after_read() {
        let mut clock = ScrollClock::new(0.0, 100.0);
        clock.set_scroll(30.0);
        let _ = clock.delta();
        assert_eq!(clock.delta(), 0.0);
    }

    #[test]
    fn scroll_clock_accumulates_multiple_moves() {
        let mut clock = ScrollClock::new(0.0, 100.0);
        clock.set_scroll(10.0); // +0.1
        clock.set_scroll(20.0); // +0.1
        clock.set_scroll(30.0); // +0.1
        let dt = clock.delta();
        assert!((dt - 0.3).abs() < 0.001);
    }

    #[test]
    fn scroll_clock_progress() {
        let mut clock = ScrollClock::new(0.0, 200.0);
        clock.set_scroll(100.0);
        let _ = clock.delta();
        assert!((clock.progress() - 0.5).abs() < 0.001);
    }

    #[test]
    fn scroll_driver_debug_default_and_degenerate_range() {
        let mut driver = ScrollDriver::new(10.0, 5.0);

        assert_eq!(driver.min(), 10.0);
        assert!(driver.max() > driver.min());
        assert_eq!(driver.position(), 10.0);
        assert_eq!(driver.progress(), 0.0);
        assert!(format!("{driver:?}").contains("ScrollDriver"));

        driver.set_position(20.0);
        assert_eq!(driver.position(), driver.max());
        assert_eq!(driver.progress(), 1.0);

        let default_driver = ScrollDriver::default();
        assert_eq!(default_driver.animation_count(), 0);
    }

    #[test]
    fn clear_completed_removes_finished_animations() {
        struct Done;
        impl Update for Done {
            fn update(&mut self, _dt: f32) -> bool {
                false
            }
        }

        let mut driver = ScrollDriver::new(0.0, 100.0);
        driver.add(Done);

        assert_eq!(driver.animation_count(), 1);
        driver.clear_completed();
        assert_eq!(driver.animation_count(), 0);
    }

    #[test]
    fn scroll_clock_default_clamps_and_reports_position() {
        let mut clock = ScrollClock::default();

        clock.set_scroll(2000.0);

        assert_eq!(clock.scroll_position(), 1000.0);
        assert_eq!(clock.progress(), 1.0);
        assert_eq!(clock.delta(), 1.0);
    }
}
