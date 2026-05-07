//! # Motus
//!
//! > *Latin: mōtus — motion, movement, impulse.*
//!
//! A professional-grade, renderer-agnostic animation library for Rust.
//! Zero mandatory dependencies. `no_std`-ready.
//!
//! Works everywhere: TUIs, Web (WASM), Bevy games, embedded targets, and native apps.
//!
//! ## Quick Start
//!
//! ```rust
//! use motus::{Tween, Easing, Update};
//!
//! let mut tween = Tween::new(0.0_f32, 100.0)
//!     .duration(1.0)
//!     .easing(Easing::EaseOutCubic)
//!     .build();
//!
//! tween.update(1.0);
//! assert_eq!(tween.value(), 100.0);
//! assert!(tween.is_complete());
//! ```
//!
//! ## Spring Physics
//!
//! ```rust
//! use motus::{Spring, SpringConfig, Update};
//!
//! let mut spring = Spring::new(SpringConfig::wobbly());
//! spring.set_target(200.0);
//!
//! while !spring.is_settled() {
//!     spring.update(1.0 / 60.0);
//! }
//! assert!((spring.position() - 200.0).abs() < 0.01);
//! ```
//!
//! ## AnimationDriver
//!
//! ```rust
//! use motus::{Tween, Easing, AnimationDriver, WallClock, Clock};
//!
//! let mut driver = AnimationDriver::new();
//! let id = driver.add(
//!     Tween::new(0.0_f32, 1.0).duration(2.0).easing(Easing::EaseInOutSine).build()
//! );
//!
//! let mut clock = WallClock::new();
//! // In your loop: driver.tick(clock.delta());
//! ```
//!
//! ## `no_std` Usage
//!
//! For `no_std` targets, depend on the sub-crates directly:
//!
//! ```toml
//! [dependencies]
//! motus-core   = { version = "0.1", default-features = false }
//! motus-tween  = { version = "0.1", default-features = false }
//! motus-spring = { version = "0.1", default-features = false }
//! ```
//!
//! ## Feature Flags
//!
//! | Feature | What it adds |
//! |---------|-------------|
//! | `default` | `std` + `tween` + `spring` + `driver` |
//! | `std` | Wall clock, heap-backed types |
//! | `tween` | [`Tween<T>`], [`Loop`] |
//! | `spring` | [`Spring`], [`SpringConfig`], [`SpringN<T>`] |
//! | `driver` | [`AnimationDriver`], all [`Clock`] variants |
//! | `serde` | `Serialize`/`Deserialize` on all public types |

// ── Core — always present ────────────────────────────────────────────────────
pub use motus_core::{Animatable, Easing, Interpolate, Update};

/// All 31 free easing functions (`ease_out_cubic`, etc.) re-exported at crate root.
///
/// These are `#[inline]` free functions — use them when you want zero-overhead
/// easing without the `Easing` enum indirection.
pub mod easing {
    pub use motus_core::easing::*;
}

// ── Tween ────────────────────────────────────────────────────────────────────
#[cfg(feature = "tween")]
pub use motus_tween::{Loop, Tween, TweenBuilder, TweenState, round_to, snap_to};

// ── Spring ───────────────────────────────────────────────────────────────────
#[cfg(feature = "spring")]
pub use motus_spring::{Integrator, Spring, SpringConfig};

#[cfg(feature = "spring")]
pub use motus_spring::SpringN;

// ── Driver ───────────────────────────────────────────────────────────────────
#[cfg(feature = "driver")]
pub use motus_driver::{AnimationDriver, AnimationId, Clock, ManualClock, MockClock, WallClock};
