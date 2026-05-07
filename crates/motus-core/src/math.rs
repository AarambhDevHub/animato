//! Math shim — see individual function docs below.
#![allow(missing_docs)]

//! both `std` and `no_std` targets.
//!
//! With `std`: delegates to the compiler-intrinsic `f32` methods.  
//! Without `std`: delegates to [`libm`], which provides software-float
//! implementations that work on bare-metal / embedded targets.
//!
//! All functions are `#[inline]` so the compiler can constant-fold and
//! eliminate the call entirely when the argument is known at compile time.

/// Sine of `x` (radians).
#[cfg(feature = "std")]
#[inline] pub(crate) fn sin(x: f32)          -> f32 { x.sin() }
/// Cosine of `x` (radians).
#[cfg(feature = "std")]
#[inline] pub(crate) fn cos(x: f32)          -> f32 { x.cos() }
/// `x` raised to the floating-point power `n`.
#[cfg(feature = "std")]
#[inline] pub(crate) fn powf(x: f32, n: f32) -> f32 { x.powf(n) }
/// `x` raised to the integer power `n`.
#[cfg(feature = "std")]
#[inline] pub(crate) fn powi(x: f32, n: i32) -> f32 { x.powi(n) }
/// Square root of `x`.
#[cfg(feature = "std")]
#[inline] pub(crate) fn sqrt(x: f32)          -> f32 { x.sqrt() }
/// Round `x` to the nearest integer (as f32).
#[cfg(feature = "std")]
#[inline] pub(crate) fn round(x: f32)         -> f32 { x.round() }

/// Sine of `x` (radians).
#[cfg(not(feature = "std"))]
#[inline] pub(crate) fn sin(x: f32)          -> f32 { libm::sinf(x) }
/// Cosine of `x` (radians).
#[cfg(not(feature = "std"))]
#[inline] pub(crate) fn cos(x: f32)          -> f32 { libm::cosf(x) }
/// `x` raised to the floating-point power `n`.
#[cfg(not(feature = "std"))]
#[inline] pub(crate) fn powf(x: f32, n: f32) -> f32 { libm::powf(x, n) }
/// `x` raised to the integer power `n`.
#[cfg(not(feature = "std"))]
#[inline] pub(crate) fn powi(x: f32, n: i32) -> f32 { libm::powf(x, n as f32) }
/// Square root of `x`.
#[cfg(not(feature = "std"))]
#[inline] pub(crate) fn sqrt(x: f32)          -> f32 { libm::sqrtf(x) }
/// Round `x` to the nearest integer (as f32).
#[cfg(not(feature = "std"))]
#[inline] pub(crate) fn round(x: f32)         -> f32 { libm::roundf(x) }
