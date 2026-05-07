//! # Animato
//!
//! > *Italian: animato — animated, lively, with life and movement.*
//!
//! A professional-grade, renderer-agnostic animation library for Rust.
//! Zero mandatory dependencies. `no_std`-ready.
//!
//! Works everywhere: TUIs, Web (WASM), Bevy games, embedded targets, and native apps.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use animato::{Tween, Easing, Update};
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
//! ```rust,ignore
//! use animato::{Spring, SpringConfig, Update};
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
//! ```rust,ignore
//! use animato::{Tween, Easing, AnimationDriver, WallClock, Clock};
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
//! animato-core   = { version = "0.2", default-features = false }
//! animato-tween  = { version = "0.2", default-features = false }
//! animato-spring = { version = "0.2", default-features = false }
//! ```
//!
//! ## Feature Flags
//!
//! | Feature | What it adds |
//! |---------|-------------|
//! | `default` | `std` + `tween` + `timeline` + `spring` + `driver` |
//! | `std` | Wall clock, heap-backed types |
//! | `tween` | [`Tween<T>`], [`KeyframeTrack<T>`], [`Loop`] |
//! | `timeline` | [`Timeline`], [`Sequence`], [`stagger()`] |
//! | `spring` | [`Spring`], [`SpringConfig`], [`SpringN<T>`] |
//! | `driver` | [`AnimationDriver`], all [`Clock`] variants |
//! | `serde` | `Serialize`/`Deserialize` on all public types |

// ── Core — always present ────────────────────────────────────────────────────
pub use animato_core::{Animatable, Easing, Interpolate, Playable, Update};

/// All 31 free easing functions (`ease_out_cubic`, etc.) re-exported at crate root.
///
/// These are `#[inline]` free functions — use them when you want zero-overhead
/// easing without the `Easing` enum indirection.
pub mod easing {
    pub use animato_core::easing::*;
}

// ── Tween ────────────────────────────────────────────────────────────────────
#[cfg(feature = "tween")]
pub use animato_tween::{
    Keyframe, KeyframeTrack, Loop, Tween, TweenBuilder, TweenState, round_to, snap_to,
};

// ── Timeline ────────────────────────────────────────────────────────────────
#[cfg(feature = "timeline")]
pub use animato_timeline::{At, Sequence, Timeline, TimelineState, stagger};

// ── Spring ───────────────────────────────────────────────────────────────────
#[cfg(feature = "spring")]
pub use animato_spring::{Integrator, Spring, SpringConfig};

#[cfg(feature = "spring")]
pub use animato_spring::SpringN;

// ── Driver ───────────────────────────────────────────────────────────────────
#[cfg(feature = "driver")]
pub use animato_driver::{AnimationDriver, AnimationId, Clock, ManualClock, MockClock, WallClock};
