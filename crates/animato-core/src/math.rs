//! Math shim — provides `sin`, `cos`, `powf`, `powi`, `sqrt`, `round` for
//! both `std` and `no_std` targets.
//!
//! With `std`: delegates to the compiler-intrinsic `f32` methods.
//! Without `std`: delegates to [`libm`], which provides software-float
//! implementations that work on bare-metal / embedded targets.
//!
//! All functions are `#[inline]` so the compiler can constant-fold and
//! eliminate the call entirely when the argument is known at compile time.
//!
//! This module is `#[doc(hidden)]` — it is an implementation detail.
//! Direct use by downstream crates is allowed but not part of the public API.

#[cfg(feature = "std")]
#[inline]
pub fn sin(x: f32) -> f32 {
    x.sin()
}
#[cfg(feature = "std")]
#[inline]
pub fn cos(x: f32) -> f32 {
    x.cos()
}
#[cfg(feature = "std")]
#[inline]
pub fn powf(x: f32, n: f32) -> f32 {
    x.powf(n)
}
#[cfg(feature = "std")]
#[inline]
pub fn powi(x: f32, n: i32) -> f32 {
    x.powi(n)
}
#[cfg(feature = "std")]
#[inline]
pub fn sqrt(x: f32) -> f32 {
    x.sqrt()
}
#[cfg(feature = "std")]
#[inline]
pub fn round(x: f32) -> f32 {
    x.round()
}
#[cfg(feature = "std")]
#[inline]
pub fn ceil(x: f32) -> f32 {
    x.ceil()
}

#[cfg(not(feature = "std"))]
#[inline]
pub fn sin(x: f32) -> f32 {
    libm::sinf(x)
}
#[cfg(not(feature = "std"))]
#[inline]
pub fn cos(x: f32) -> f32 {
    libm::cosf(x)
}
#[cfg(not(feature = "std"))]
#[inline]
pub fn powf(x: f32, n: f32) -> f32 {
    libm::powf(x, n)
}
#[cfg(not(feature = "std"))]
#[inline]
pub fn powi(x: f32, n: i32) -> f32 {
    libm::powf(x, n as f32)
}
#[cfg(not(feature = "std"))]
#[inline]
pub fn sqrt(x: f32) -> f32 {
    libm::sqrtf(x)
}
#[cfg(not(feature = "std"))]
#[inline]
pub fn round(x: f32) -> f32 {
    libm::roundf(x)
}
#[cfg(not(feature = "std"))]
#[inline]
pub fn ceil(x: f32) -> f32 {
    libm::ceilf(x)
}
