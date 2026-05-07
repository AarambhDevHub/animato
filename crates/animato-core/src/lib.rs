//! # animato-core
//!
//! Core traits and easing system for the Animato animation library.
//!
//! This crate is the foundation every other Animato crate builds on.
//! It is fully `no_std`-compatible with zero external dependencies
//! (unless the optional `serde` feature is enabled).
//!
//! ## Quick Start
//!
//! ```rust
//! use animato_core::{Interpolate, Easing};
//!
//! // Easing a value from 0.0 to 1.0 at the midpoint
//! let t = 0.5_f32;
//! let eased = Easing::EaseOutCubic.apply(t);
//! assert!(eased > t); // EaseOut front-loads motion
//! ```
//!
//! ## Feature Flags
//!
//! | Feature | Effect |
//! |---------|--------|
//! | `std`   | Enables std-dependent extensions (none in this crate today — reserved for future use) |
//! | `serde` | Derives `Serialize`/`Deserialize` on `Easing` |
//!
//! ## `no_std` Usage
//!
//! ```toml
//! [dependencies]
//! animato-core = { version = "0.1", default-features = false }
//! ```
//!
//! All items in this crate are available in `no_std` environments.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

pub mod easing;
/// Internal math shim — `no_std`/`std` portable math functions.
#[doc(hidden)]
pub mod math;
pub mod traits;

// Flatten the most important items to the crate root for convenience.
pub use easing::Easing;
pub use traits::{Animatable, Interpolate, Playable, Update};
