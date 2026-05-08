//! # animato-color
//!
//! Perceptual color interpolation wrappers for Animato.
//!
//! This crate adapts [`palette`] color types to Animato's [`animato_core::Interpolate`]
//! trait. Wrap the color in the space you want to interpolate through, then
//! use it with `Tween<T>`, `KeyframeTrack<T>`, or any other Animato primitive.
//!
//! ## Quick Start
//!
//! ```rust
//! use animato_color::InLab;
//! use animato_core::Interpolate;
//! use palette::Srgb;
//!
//! let red = InLab::new(Srgb::new(1.0, 0.0, 0.0));
//! let blue = InLab::new(Srgb::new(0.0, 0.0, 1.0));
//! let midpoint = red.lerp(&blue, 0.5).into_inner();
//!
//! assert!(midpoint.red > 0.0);
//! assert!(midpoint.blue > 0.0);
//! ```
//!
//! ## Feature Flags
//!
//! | Feature | Effect |
//! |---------|--------|
//! | `std`   | Enables `std` support in dependencies |
//! | `serde` | Derives `Serialize`/`Deserialize` on wrapper types |

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

pub mod spaces;

pub use spaces::{InLab, InLinear, InOklch};
