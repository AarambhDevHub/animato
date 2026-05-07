//! Time sources for driving animations.

/// Abstracts any time source that can produce a `dt` (delta-time) value in seconds.
///
/// Implement this for custom timers, game-engine time sources, or scroll-based drivers.
///
/// # Example
///
/// ```rust
/// use motus_driver::{Clock, MockClock};
///
/// let mut clk = MockClock::new(1.0 / 60.0);
/// assert!((clk.delta() - 1.0 / 60.0).abs() < 1e-6);
/// ```
pub trait Clock {
    /// Returns seconds elapsed since the last call.
    ///
    /// The first call returns `0.0` for `WallClock`, or the configured step
    /// for `MockClock` / `ManualClock`.
    fn delta(&mut self) -> f32;
}

// ──────────────────────────────────────────────────────────────────────────────
// WallClock
// ──────────────────────────────────────────────────────────────────────────────

/// A real-time clock backed by [`std::time::Instant`].
///
/// The first call to [`delta`](Clock::delta) returns `0.0` (no elapsed time since creation).
///
/// # Example
///
/// ```rust
/// use motus_driver::{Clock, WallClock};
///
/// let mut clk = WallClock::new();
/// let dt = clk.delta(); // first call: ~0.0
/// assert!(dt >= 0.0);
/// ```
#[derive(Debug)]
pub struct WallClock {
    last: std::time::Instant,
}

impl WallClock {
    /// Create a new `WallClock` set to the current instant.
    pub fn new() -> Self {
        Self {
            last: std::time::Instant::now(),
        }
    }
}

impl Default for WallClock {
    fn default() -> Self {
        Self::new()
    }
}

impl Clock for WallClock {
    fn delta(&mut self) -> f32 {
        let now = std::time::Instant::now();
        let dt = now.duration_since(self.last).as_secs_f32();
        self.last = now;
        dt
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// ManualClock
// ──────────────────────────────────────────────────────────────────────────────

/// A clock where the caller explicitly sets the dt each frame.
///
/// Call [`advance`](ManualClock::advance) to set the pending dt,
/// then [`delta`](Clock::delta) to consume it (resets to `0.0`).
///
/// Useful for custom game loops that already compute their own dt.
///
/// # Example
///
/// ```rust
/// use motus_driver::{Clock, ManualClock};
///
/// let mut clk = ManualClock::new();
/// clk.advance(0.016);
/// assert!((clk.delta() - 0.016).abs() < 1e-6);
/// assert_eq!(clk.delta(), 0.0); // consumed
/// ```
#[derive(Debug, Default)]
pub struct ManualClock {
    pending: f32,
}

impl ManualClock {
    /// Create a new `ManualClock` with zero pending time.
    pub fn new() -> Self {
        Self { pending: 0.0 }
    }

    /// Set the pending dt that [`delta`](Clock::delta) will return on next call.
    pub fn advance(&mut self, dt: f32) {
        self.pending = dt.max(0.0);
    }
}

impl Clock for ManualClock {
    fn delta(&mut self) -> f32 {
        let dt = self.pending;
        self.pending = 0.0;
        dt
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// MockClock
// ──────────────────────────────────────────────────────────────────────────────

/// A fixed-step clock for deterministic tests.
///
/// Always returns the same `step` value from [`delta`](Clock::delta),
/// making tests independent of wall-clock speed.
///
/// # Example
///
/// ```rust
/// use motus_driver::{Clock, MockClock};
///
/// let mut clk = MockClock::new(1.0 / 60.0);
/// for _ in 0..5 {
///     let dt = clk.delta();
///     assert!((dt - 1.0 / 60.0).abs() < 1e-6);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct MockClock {
    step: f32,
}

impl MockClock {
    /// Create a `MockClock` that always returns `step_seconds` from [`delta`](Clock::delta).
    pub fn new(step_seconds: f32) -> Self {
        Self {
            step: step_seconds.max(0.0),
        }
    }
}

impl Clock for MockClock {
    fn delta(&mut self) -> f32 {
        self.step
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Tests
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_clock_always_returns_step() {
        let mut clk = MockClock::new(1.0 / 60.0);
        for _ in 0..10 {
            let dt = clk.delta();
            assert!((dt - 1.0 / 60.0).abs() < 1e-6);
        }
    }

    #[test]
    fn manual_clock_advance_and_consume() {
        let mut clk = ManualClock::new();
        clk.advance(0.016);
        assert!((clk.delta() - 0.016).abs() < 1e-6);
        // After consumption, returns 0.0
        assert_eq!(clk.delta(), 0.0);
    }

    #[test]
    fn manual_clock_negative_clamped() {
        let mut clk = ManualClock::new();
        clk.advance(-5.0);
        assert_eq!(clk.delta(), 0.0);
    }

    #[test]
    fn wall_clock_non_negative() {
        let mut clk = WallClock::new();
        let dt = clk.delta();
        assert!(dt >= 0.0);
    }

    #[test]
    fn mock_clock_zero_step() {
        let mut clk = MockClock::new(0.0);
        assert_eq!(clk.delta(), 0.0);
    }
}
