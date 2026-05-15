//! Momentum-style scroll smoothing state.

/// Smooths scroll deltas into a damped position.
#[derive(Clone, Copy, Debug)]
pub struct ScrollSmoother {
    current: f32,
    target: f32,
    velocity: f32,
    stiffness: f32,
    damping: f32,
    epsilon: f32,
}

impl Default for ScrollSmoother {
    fn default() -> Self {
        Self::new()
    }
}

impl ScrollSmoother {
    /// Create a scroll smoother with balanced defaults.
    pub fn new() -> Self {
        Self {
            current: 0.0,
            target: 0.0,
            velocity: 0.0,
            stiffness: 180.0,
            damping: 24.0,
            epsilon: 0.01,
        }
    }

    /// Set the current and target scroll position immediately.
    pub fn snap_to(&mut self, value: f32) {
        self.current = value;
        self.target = value;
        self.velocity = 0.0;
    }

    /// Set the target scroll position.
    pub fn scroll_to(&mut self, value: f32) {
        self.target = value.max(0.0);
    }

    /// Add a wheel delta to the target scroll position.
    pub fn on_wheel(&mut self, delta_y: f32) {
        self.scroll_to(self.target + delta_y);
    }

    /// Advance smoothing by `dt` seconds. Returns `true` while moving.
    pub fn update(&mut self, dt: f32) -> bool {
        let dt = dt.max(0.0);
        let displacement = self.target - self.current;
        let acceleration = displacement * self.stiffness - self.velocity * self.damping;
        self.velocity += acceleration * dt;
        self.current += self.velocity * dt;

        if self.is_settled() {
            self.current = self.target;
            self.velocity = 0.0;
            return false;
        }

        true
    }

    /// Current smoothed scroll position.
    pub fn current(&self) -> f32 {
        self.current
    }

    /// Target scroll position.
    pub fn target(&self) -> f32 {
        self.target
    }

    /// Returns `true` when position and velocity are below epsilon.
    pub fn is_settled(&self) -> bool {
        (self.target - self.current).abs() <= self.epsilon && self.velocity.abs() <= self.epsilon
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_starts_settled_at_zero() {
        let smoother = ScrollSmoother::default();

        assert_eq!(smoother.current(), 0.0);
        assert_eq!(smoother.target(), 0.0);
        assert!(smoother.is_settled());
    }

    #[test]
    fn snap_to_sets_current_target_and_stops_motion() {
        let mut smoother = ScrollSmoother::new();

        smoother.scroll_to(100.0);
        smoother.update(1.0 / 60.0);
        smoother.snap_to(42.0);

        assert_eq!(smoother.current(), 42.0);
        assert_eq!(smoother.target(), 42.0);
        assert!(!smoother.update(0.0));
        assert!(smoother.is_settled());
    }

    #[test]
    fn scroll_to_clamps_negative_targets() {
        let mut smoother = ScrollSmoother::new();

        smoother.scroll_to(-20.0);

        assert_eq!(smoother.target(), 0.0);
    }

    #[test]
    fn wheel_delta_accumulates_into_target() {
        let mut smoother = ScrollSmoother::new();

        smoother.on_wheel(25.0);
        smoother.on_wheel(15.0);

        assert_eq!(smoother.target(), 40.0);
    }

    #[test]
    fn update_moves_toward_target_and_settles() {
        let mut smoother = ScrollSmoother::new();

        smoother.scroll_to(120.0);
        assert!(smoother.update(1.0 / 60.0));
        assert!(smoother.current() > 0.0);

        for _ in 0..600 {
            if !smoother.update(1.0 / 60.0) {
                break;
            }
        }

        assert!((smoother.current() - 120.0).abs() <= 0.01);
        assert!(smoother.is_settled());
    }
}
