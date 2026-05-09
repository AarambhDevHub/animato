//! # animato-wasm
//!
//! WASM and browser integration helpers for Animato.
//!
//! [`RafDriver`] is target-independent and easy to test on native targets. DOM
//! helpers are available behind the `wasm-dom` feature on `wasm32` targets.

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

use animato_core::Update;
use animato_driver::{AnimationDriver, AnimationId};

#[cfg(all(feature = "wasm-dom", target_arch = "wasm32"))]
mod draggable;
#[cfg(all(feature = "wasm-dom", target_arch = "wasm32"))]
mod flip;
#[cfg(all(feature = "wasm-dom", target_arch = "wasm32"))]
mod observer;
mod scroll_smoother;
#[cfg(all(feature = "wasm-dom", target_arch = "wasm32"))]
mod split_text;

#[cfg(all(feature = "wasm-dom", target_arch = "wasm32"))]
pub use draggable::Draggable;
#[cfg(all(feature = "wasm-dom", target_arch = "wasm32"))]
pub use flip::{FlipAnimation, FlipState};
#[cfg(all(feature = "wasm-dom", target_arch = "wasm32"))]
pub use observer::{Observer, ObserverEvent};
pub use scroll_smoother::ScrollSmoother;
#[cfg(all(feature = "wasm-dom", target_arch = "wasm32"))]
pub use split_text::{SplitMode, SplitText};

/// Drives an [`AnimationDriver`] from `requestAnimationFrame` timestamps.
///
/// Browser rAF callbacks pass a monotonically increasing timestamp in
/// milliseconds. `RafDriver` converts that value into a clamped seconds `dt`,
/// applies time scale, and ticks the inner [`AnimationDriver`].
#[derive(Debug)]
pub struct RafDriver {
    driver: AnimationDriver,
    last_timestamp_ms: Option<f64>,
    paused: bool,
    time_scale: f32,
    max_dt: f32,
}

impl Default for RafDriver {
    fn default() -> Self {
        Self::new()
    }
}

impl RafDriver {
    /// Default maximum frame delta in seconds.
    pub const DEFAULT_MAX_DT: f32 = 0.25;

    /// Create an empty rAF driver.
    pub fn new() -> Self {
        Self {
            driver: AnimationDriver::new(),
            last_timestamp_ms: None,
            paused: false,
            time_scale: 1.0,
            max_dt: Self::DEFAULT_MAX_DT,
        }
    }

    /// Create a rAF driver from an existing [`AnimationDriver`].
    pub fn with_driver(driver: AnimationDriver) -> Self {
        Self {
            driver,
            ..Self::new()
        }
    }

    /// Register an animation with the inner driver.
    pub fn add<A: Update + Send + 'static>(&mut self, animation: A) -> AnimationId {
        self.driver.add(animation)
    }

    /// Convert an rAF timestamp in milliseconds to seconds and tick animations.
    ///
    /// The first tick returns `0.0` because no previous timestamp exists.
    /// Invalid or backward timestamps are treated as zero delta.
    pub fn tick(&mut self, timestamp_ms: f64) -> f32 {
        if !timestamp_ms.is_finite() {
            return 0.0;
        }

        let raw_dt = match self.last_timestamp_ms.replace(timestamp_ms) {
            Some(last) => ((timestamp_ms - last) / 1000.0).max(0.0) as f32,
            None => 0.0,
        };

        if self.paused {
            return 0.0;
        }

        let dt = raw_dt.min(self.max_dt) * self.time_scale;
        self.driver.tick(dt);
        dt
    }

    /// Pause ticking while still consuming timestamps to avoid resume jumps.
    pub fn pause(&mut self) {
        self.paused = true;
    }

    /// Resume ticking.
    pub fn resume(&mut self) {
        self.paused = false;
    }

    /// Returns `true` when the driver is paused.
    pub fn is_paused(&self) -> bool {
        self.paused
    }

    /// Set the time scale. Non-finite values become `1.0`; negative values clamp to `0.0`.
    pub fn set_time_scale(&mut self, time_scale: f32) {
        self.time_scale = if time_scale.is_finite() {
            time_scale.max(0.0)
        } else {
            1.0
        };
    }

    /// Current time scale.
    pub fn time_scale(&self) -> f32 {
        self.time_scale
    }

    /// Set the maximum accepted frame delta in seconds.
    pub fn set_max_dt(&mut self, max_dt: f32) {
        self.max_dt = if max_dt.is_finite() {
            max_dt.max(0.0)
        } else {
            Self::DEFAULT_MAX_DT
        };
    }

    /// Current maximum frame delta in seconds.
    pub fn max_dt(&self) -> f32 {
        self.max_dt
    }

    /// Forget the previous rAF timestamp.
    pub fn reset_timestamp(&mut self) {
        self.last_timestamp_ms = None;
    }

    /// Borrow the inner animation driver.
    pub fn driver(&self) -> &AnimationDriver {
        &self.driver
    }

    /// Mutably borrow the inner animation driver.
    pub fn driver_mut(&mut self) -> &mut AnimationDriver {
        &mut self.driver
    }

    /// Cancel an animation by id.
    pub fn cancel(&mut self, id: AnimationId) {
        self.driver.cancel(id);
    }

    /// Cancel all animations.
    pub fn cancel_all(&mut self) {
        self.driver.cancel_all();
    }

    /// Number of active animations.
    pub fn active_count(&self) -> usize {
        self.driver.active_count()
    }

    /// Returns `true` if an animation id is still active.
    pub fn is_active(&self, id: AnimationId) -> bool {
        self.driver.is_active(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use animato_tween::Tween;

    #[test]
    fn first_tick_has_zero_delta() {
        let mut driver = RafDriver::new();
        assert_eq!(driver.tick(1000.0), 0.0);
    }

    #[test]
    fn converts_milliseconds_to_seconds() {
        let mut driver = RafDriver::new();
        driver.tick(1000.0);
        assert!((driver.tick(1016.0) - 0.016).abs() < 0.0001);
    }

    #[test]
    fn time_scale_multiplies_delta() {
        let mut driver = RafDriver::new();
        driver.set_time_scale(2.0);
        driver.tick(1000.0);
        assert!((driver.tick(1016.0) - 0.032).abs() < 0.0001);
    }

    #[test]
    fn max_dt_clamps_large_gaps() {
        let mut driver = RafDriver::new();
        driver.set_max_dt(0.1);
        driver.tick(1000.0);
        assert!((driver.tick(5000.0) - 0.1).abs() < 0.0001);
    }

    #[test]
    fn pause_consumes_timestamp_without_ticking() {
        let mut driver = RafDriver::new();
        let id = driver.add(Tween::new(0.0_f32, 1.0).duration(0.01).build());
        driver.tick(1000.0);
        driver.pause();
        assert_eq!(driver.tick(5000.0), 0.0);
        assert!(driver.is_active(id));
        driver.resume();
        driver.tick(5016.0);
        assert!(!driver.is_active(id));
    }

    #[test]
    fn invalid_timestamp_is_ignored() {
        let mut driver = RafDriver::new();
        assert_eq!(driver.tick(f64::NAN), 0.0);
        assert_eq!(driver.tick(f64::INFINITY), 0.0);
    }
}
