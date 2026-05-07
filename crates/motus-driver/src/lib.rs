//! # motus-driver
//!
//! Runtime management for Motus animations.
//!
//! - [`AnimationDriver`] — owns and ticks many animations; auto-removes completed ones.
//! - [`Clock`] — trait abstracting time sources.
//! - [`WallClock`] — real wall-clock time via `std::time::Instant`.
//! - [`ManualClock`] — caller-driven time for custom loops.
//! - [`MockClock`] — fixed-step clock for deterministic tests.
//!
//! ## Quick Start
//!
//! ```rust
//! use motus_driver::{AnimationDriver, MockClock, Clock};
//! use motus_tween::Tween;
//! use motus_core::{Easing, Update};
//!
//! let mut driver = AnimationDriver::new();
//! let id = driver.add(
//!     Tween::new(0.0_f32, 100.0)
//!         .duration(1.0)
//!         .easing(Easing::EaseOutCubic)
//!         .build()
//! );
//!
//! let mut clock = MockClock::new(1.0 / 60.0);
//! assert!(driver.is_active(id));
//! for _ in 0..61 { // 61 × (1/60s) > 1.0s → tween completes and is auto-removed
//!     driver.tick(clock.delta());
//! }
//! assert!(!driver.is_active(id)); // auto-removed after completion
//! ```

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

pub mod clock;
pub mod driver;

pub use clock::{Clock, ManualClock, MockClock, WallClock};
pub use driver::{AnimationDriver, AnimationId};
