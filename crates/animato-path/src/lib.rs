//! # animato-path
//!
//! Motion-path primitives for Animato.
//!
//! This crate provides Bezier curves, CatmullRom splines, compound paths,
//! SVG `d` attribute parsing, and [`MotionPathTween`] for driving an object
//! along a path with the existing tween system.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use animato_core::{Easing, Update};
//! use animato_path::{CubicBezierCurve, MotionPathTween};
//!
//! let path = CubicBezierCurve::new(
//!     [0.0, 0.0],
//!     [50.0, 100.0],
//!     [150.0, -100.0],
//!     [200.0, 0.0],
//! );
//! let mut motion = MotionPathTween::new(path)
//!     .duration(1.0)
//!     .easing(Easing::EaseInOutSine)
//!     .auto_rotate(true)
//!     .build();
//!
//! motion.update(0.5);
//! let position = motion.value();
//! assert!(position[0] > 0.0);
//! ```
//!
//! ## Feature Flags
//!
//! | Feature | Effect |
//! |---------|--------|
//! | `std`   | Enables all path types and forwards `std` to dependencies |
//! | `alloc` | Enables heap-backed paths, SVG parser, and `MotionPathTween` |
//! | `serde` | Derives `Serialize`/`Deserialize` on supported public types |

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

#[cfg(any(feature = "std", feature = "alloc"))]
extern crate alloc;

pub mod bezier;
pub mod draw;
pub(crate) mod math;

#[cfg(any(feature = "std", feature = "alloc"))]
pub mod morph;
#[cfg(any(feature = "std", feature = "alloc"))]
pub mod motion;
#[cfg(any(feature = "std", feature = "alloc"))]
pub mod poly;
#[cfg(any(feature = "std", feature = "alloc"))]
pub mod svg;

pub use bezier::{CubicBezierCurve, PathEvaluate, QuadBezier};
pub use draw::{DrawSvg, DrawValues};

#[cfg(any(feature = "std", feature = "alloc"))]
pub use bezier::CatmullRomSpline;
#[cfg(any(feature = "std", feature = "alloc"))]
pub use morph::{MorphPath, resample};
#[cfg(any(feature = "std", feature = "alloc"))]
pub use motion::{MotionPath, MotionPathTween, MotionPathTweenBuilder};
#[cfg(any(feature = "std", feature = "alloc"))]
pub use poly::{CompoundPath, EllipticalArc, LineSegment, PathCommand, PathSegment, PolyPath};
#[cfg(any(feature = "std", feature = "alloc"))]
pub use svg::{SvgPathError, SvgPathParser};
