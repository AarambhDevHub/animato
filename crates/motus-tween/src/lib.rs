//! # motus-tween
//!
//! `Tween<T>` — single-value animation from a start to an end value over time.
//!
//! This crate is fully `no_std`-compatible and requires no heap allocation.
//!
//! ## Quick Start
//!
//! ```rust
//! use motus_tween::{Tween, Loop};
//! use motus_core::{Easing, Update};
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
//! | `std`   | Enables std-dependent features (forwarded to `motus-core`) |
//! | `serde` | Derives `Serialize`/`Deserialize` on public types |

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

pub mod builder;
pub mod loop_mode;
pub mod tween;
pub mod modifiers;

pub use builder::TweenBuilder;
pub use loop_mode::Loop;
pub use tween::{Tween, TweenState};
pub use modifiers::{snap_to, round_to};
