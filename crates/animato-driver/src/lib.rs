//! # animato-driver
//!
//! Runtime management for Animato animations.
//!
//! - [`AnimationDriver`] — owns and ticks many animations; auto-removes completed ones.
//! - [`Clock`] — trait abstracting time sources.
//! - [`WallClock`] — real wall-clock time via `std::time::Instant`.
//! - [`ManualClock`] — caller-driven time.
//! - [`MockClock`] — fixed-step clock for tests.
//! - [`scroll::ScrollDriver`] — animations driven by scroll position (v0.8.0).
//! - [`scroll::ScrollClock`] — scroll-backed [`Clock`] implementation (v0.8.0).
//!
//! ## Quick Start
//!
//! ```rust
//! use animato_driver::{AnimationDriver, MockClock, Clock};
//! use animato_tween::Tween;
//! use animato_core::{Easing, Update};
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
//! for _ in 0..61 {
//!     driver.tick(clock.delta());
//! }
//! assert!(!driver.is_active(id));
//! ```

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

pub mod clock;
pub mod driver;
pub mod scroll;

pub use clock::{Clock, ManualClock, MockClock, WallClock};
pub use driver::{AnimationDriver, AnimationId};
pub use scroll::{ScrollClock, ScrollDriver};
