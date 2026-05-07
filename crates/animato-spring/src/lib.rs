//! # animato-spring
//!
//! Physics-based spring animations using a damped harmonic oscillator.
//!
//! - [`Spring`] — 1D spring, stack-allocated, `no_std`-compatible.
//! - [`SpringConfig`] — stiffness / damping / mass / epsilon, with named presets.
//! - [`SpringN<T>`] — multi-dimensional spring (requires `alloc` feature).
//!
//! ## Quick Start
//!
//! ```rust
//! use animato_spring::{Spring, SpringConfig};
//! use animato_core::Update;
//!
//! let mut spring = Spring::new(SpringConfig::wobbly());
//! spring.set_target(200.0);
//!
//! let mut steps = 0;
//! while !spring.is_settled() {
//!     spring.update(1.0 / 60.0);
//!     steps += 1;
//!     assert!(steps < 10_000, "spring should settle");
//! }
//! let pos = spring.position();
//! assert!((pos - 200.0).abs() < 0.01);
//! ```
//!
//! ## Feature Flags
//!
//! | Feature | Effect |
//! |---------|--------|
//! | `std`   | Enables `alloc` and `animato-core/std` |
//! | `alloc` | Enables [`SpringN<T>`] (multi-dimensional spring) |
//! | `serde` | Derives `Serialize`/`Deserialize` on public types |

#![cfg_attr(not(any(feature = "std", feature = "alloc")), no_std)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

#[cfg(any(feature = "std", feature = "alloc"))]
extern crate alloc;

pub mod config;
pub mod spring;

#[cfg(any(feature = "std", feature = "alloc"))]
pub(crate) mod decompose;
#[cfg(any(feature = "std", feature = "alloc"))]
pub mod spring_n;

pub use config::SpringConfig;
pub use spring::{Integrator, Spring};

#[cfg(any(feature = "std", feature = "alloc"))]
pub use spring_n::SpringN;
