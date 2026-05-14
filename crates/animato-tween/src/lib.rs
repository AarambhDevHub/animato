//! # animato-tween
//!
//! `Tween<T>` — single-value animation from a start to an end value over time.
//!
//! This crate is fully `no_std`-compatible and requires no heap allocation.
//!
//! ## Quick Start
//!
//! ```rust
//! use animato_tween::{Tween, Loop};
//! use animato_core::{Easing, Update};
//!
//! let mut tween = Tween::new(0.0_f32, 100.0)
//!     .duration(1.0)
//!     .easing(Easing::EaseOutCubic)
//!     .build();
//!
//! // Advance by 0.5 seconds (half the duration):
//! tween.update(0.5);
//! assert!(tween.value() > 50.0); // EaseOut front-loads motion
//! assert!(!tween.is_complete());
//!
//! tween.update(0.5);
//! assert!(tween.is_complete());
//! assert_eq!(tween.value(), 100.0);
//! ```
//!
//! ## Feature Flags
//!
//! | Feature | Effect |
//! |---------|--------|
//! | `std`   | Enables std-dependent features (forwarded to `animato-core`) |
//! | `serde` | Derives `Serialize`/`Deserialize` on public types |

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

#[cfg(any(feature = "std", feature = "alloc"))]
extern crate alloc;

pub mod builder;
#[cfg(any(feature = "std", feature = "alloc"))]
pub mod keyframe;
pub mod loop_mode;
pub mod modifiers;
pub mod tween;

pub use builder::TweenBuilder;
#[cfg(any(feature = "std", feature = "alloc"))]
pub use keyframe::{Keyframe, KeyframeTrack};
pub use loop_mode::Loop;
pub use modifiers::{round_to, snap_to};
pub use tween::{Tween, TweenSnapshot, TweenState};
