//! # animato-physics
//!
//! Input-driven physics for Animato: friction inertia, drag tracking with
//! velocity estimation, and gesture recognition.
//!
//! ## Quick Start
//!
//! ```rust
//! use animato_core::Update;
//! use animato_physics::{Inertia, InertiaConfig};
//!
//! let mut inertia = Inertia::new(InertiaConfig::smooth());
//! inertia.kick(500.0);
//!
//! while inertia.update(1.0 / 60.0) {}
//! assert!(inertia.is_settled());
//! ```
//!
//! ## Feature Flags
//!
//! | Feature | Effect |
//! |---------|--------|
//! | `std`   | Enables `alloc` and forwards `animato-core/std` |
//! | `alloc` | Enables [`InertiaN<T>`] and [`DragState`] |
//! | `serde` | Derives `Serialize`/`Deserialize` on public types |

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

#[cfg(any(feature = "std", feature = "alloc"))]
extern crate alloc;

#[cfg(any(feature = "std", feature = "alloc"))]
pub(crate) mod decompose;

pub mod drag;
pub mod gesture;
pub mod inertia;

#[cfg(any(feature = "std", feature = "alloc"))]
pub use drag::DragState;
pub use drag::{DragAxis, DragConstraints, PointerData};
pub use gesture::{Gesture, GestureConfig, GestureRecognizer, SwipeDirection};
#[cfg(any(feature = "std", feature = "alloc"))]
pub use inertia::InertiaN;
pub use inertia::{Inertia, InertiaBounds, InertiaConfig};
