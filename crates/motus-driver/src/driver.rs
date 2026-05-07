//! [`AnimationDriver`] — owns and ticks multiple animations simultaneously.

use motus_core::Update;

/// An opaque handle to an animation registered with [`AnimationDriver`].
///
/// Returned by [`AnimationDriver::add`]. Use it to cancel or query animations.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct AnimationId(u64);

struct Slot {
    id: AnimationId,
    animation: Box<dyn Update + Send>,
    remove: bool,
}

impl std::fmt::Debug for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Slot")
            .field("id", &self.id)
            .field("remove", &self.remove)
            .finish()
    }
}

/// Manages a collection of animations, ticking them each frame and
/// automatically removing completed ones.
///
/// # Example
///
/// ```rust
/// use motus_driver::{AnimationDriver, MockClock, Clock};
/// use motus_tween::Tween;
/// use motus_core::Easing;
///
/// let mut driver = AnimationDriver::new();
/// let id = driver.add(
///     Tween::new(0.0_f32, 1.0).duration(1.0).build()
/// );
///
/// assert!(driver.is_active(id));
/// assert_eq!(driver.active_count(), 1);
///
/// // Tick past completion:
/// driver.tick(2.0);
/// assert!(!driver.is_active(id));
/// assert_eq!(driver.active_count(), 0);
/// ```
#[derive(Debug, Default)]
pub struct AnimationDriver {
    slots: Vec<Slot>,
    next_id: u64,
}

impl AnimationDriver {
    /// Create a new, empty driver.
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            next_id: 0,
        }
    }

    /// Register an animation and return its [`AnimationId`].
    ///
    /// The animation will be ticked on every call to [`tick`](Self::tick)
    /// and automatically removed when it returns `false` from `update`.
    pub fn add<A: Update + Send + 'static>(&mut self, anim: A) -> AnimationId {
        let id = AnimationId(self.next_id);
        self.next_id += 1;
        self.slots.push(Slot {
            id,
            animation: Box::new(anim),
            remove: false,
        });
        id
    }

    /// Advance all active animations by `dt` seconds.
    ///
    /// Animations that return `false` from `update` are marked and removed
    /// at the end of the tick — no allocation during the tick itself.
    pub fn tick(&mut self, dt: f32) {
        for slot in self.slots.iter_mut() {
            if slot.remove {
                continue;
            }
            let still_running = slot.animation.update(dt);
            if !still_running {
                slot.remove = true;
            }
        }
        // Drain completed slots in one pass — O(n), no realloc.
        self.slots.retain(|s| !s.remove);
    }

    /// Cancel an animation by id.
    ///
    /// The animation is removed immediately, before the next tick.
    /// No-op if the id is not found (e.g. already completed).
    pub fn cancel(&mut self, id: AnimationId) {
        self.slots.retain(|s| s.id != id);
    }

    /// Cancel all active animations.
    pub fn cancel_all(&mut self) {
        self.slots.clear();
    }

    /// Number of currently active animations.
    pub fn active_count(&self) -> usize {
        self.slots.len()
    }

    /// `true` if the animation with the given id is still active.
    pub fn is_active(&self, id: AnimationId) -> bool {
        self.slots.iter().any(|s| s.id == id)
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Tests
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use motus_core::Easing;
    use motus_tween::Tween;

    #[test]
    fn auto_removes_completed() {
        let mut driver = AnimationDriver::new();
        let id = driver.add(Tween::new(0.0_f32, 1.0).duration(1.0).build());
        assert!(driver.is_active(id));
        driver.tick(2.0); // well past duration
        assert!(!driver.is_active(id));
        assert_eq!(driver.active_count(), 0);
    }

    #[test]
    fn cancel_removes_mid_animation() {
        let mut driver = AnimationDriver::new();
        let id = driver.add(Tween::new(0.0_f32, 1.0).duration(10.0).build());
        driver.tick(1.0);
        assert!(driver.is_active(id));
        driver.cancel(id);
        assert!(!driver.is_active(id));
    }

    #[test]
    fn cancel_noop_on_missing_id() {
        let mut driver = AnimationDriver::new();
        let id = driver.add(Tween::new(0.0_f32, 1.0).duration(1.0).build());
        driver.tick(2.0); // completes, auto-removed
        driver.cancel(id); // should not panic
        assert_eq!(driver.active_count(), 0);
    }

    #[test]
    fn active_count_tracks_correctly() {
        let mut driver = AnimationDriver::new();
        let _a = driver.add(Tween::new(0.0_f32, 1.0).duration(1.0).build());
        let _b = driver.add(Tween::new(0.0_f32, 1.0).duration(2.0).build());
        let _c = driver.add(Tween::new(0.0_f32, 1.0).duration(3.0).build());
        assert_eq!(driver.active_count(), 3);

        driver.tick(1.5); // first one completes
        assert_eq!(driver.active_count(), 2);

        driver.tick(1.0); // second completes
        assert_eq!(driver.active_count(), 1);
    }

    #[test]
    fn cancel_all_clears_everything() {
        let mut driver = AnimationDriver::new();
        for _ in 0..10 {
            driver.add(Tween::new(0.0_f32, 1.0).duration(5.0).build());
        }
        assert_eq!(driver.active_count(), 10);
        driver.cancel_all();
        assert_eq!(driver.active_count(), 0);
    }

    #[test]
    fn multiple_concurrent_animations_tick_independently() {
        let mut driver = AnimationDriver::new();
        let slow = driver.add(Tween::new(0.0_f32, 1.0).duration(2.0).build());
        let fast = driver.add(
            Tween::new(0.0_f32, 1.0)
                .duration(0.5)
                .easing(Easing::Linear)
                .build(),
        );

        driver.tick(1.0); // fast completes, slow still running
        assert!(!driver.is_active(fast));
        assert!(driver.is_active(slow));

        driver.tick(2.0); // slow completes
        assert!(!driver.is_active(slow));
    }

    #[test]
    fn animation_id_is_unique() {
        let mut driver = AnimationDriver::new();
        let ids: Vec<_> = (0..100)
            .map(|_| driver.add(Tween::new(0.0_f32, 1.0).duration(1.0).build()))
            .collect();
        let unique: std::collections::HashSet<_> = ids.iter().collect();
        assert_eq!(unique.len(), 100);
    }
}
