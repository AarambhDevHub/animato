//! Core traits for the Motus animation system.
//!
//! This module defines the three fundamental traits:
//! - [`Interpolate`] — a value that can be linearly blended between two states
//! - [`Animatable`] — blanket marker for any `Interpolate + Clone + 'static` type
//! - [`Update`] — anything that advances through time when given a `dt`

/// A value that supports linear interpolation between two instances.
///
/// Implement this trait for any type you want to animate with Motus.
/// The library ships blanket impls for `f32`, `f64`, `[f32; 2]`,
/// `[f32; 3]`, `[f32; 4]`, `i32`, and `u8`.
///
/// # Contract
///
/// - `self.lerp(other, 0.0)` must equal `*self`
/// - `self.lerp(other, 1.0)` must equal `*other`
/// - `t` outside `[0.0, 1.0]` is allowed — implementations may extrapolate or clamp
///
/// # Example
///
/// ```rust
/// use motus_core::Interpolate;
///
/// let a = 0.0_f32;
/// let b = 100.0_f32;
/// assert_eq!(a.lerp(&b, 0.0), 0.0);
/// assert_eq!(a.lerp(&b, 1.0), 100.0);
/// assert_eq!(a.lerp(&b, 0.5), 50.0);
/// ```
pub trait Interpolate: Sized {
    /// Linearly interpolate from `self` to `other` by factor `t`.
    ///
    /// `t = 0.0` returns `self`, `t = 1.0` returns `other`.
    fn lerp(&self, other: &Self, t: f32) -> Self;
}

/// Marker trait for types that can be used as animation targets.
///
/// Any type implementing `Interpolate + Clone + 'static` automatically
/// satisfies `Animatable` through a blanket impl. You never implement
/// this trait manually.
///
/// # Example
///
/// ```rust
/// use motus_core::Animatable;
///
/// fn animate<T: Animatable>(start: T, end: T) {
///     let mid = start.lerp(&end, 0.5);
///     let _ = mid;
/// }
///
/// animate(0.0_f32, 1.0_f32);
/// animate([0.0_f32; 3], [1.0_f32; 3]);
/// ```
pub trait Animatable: Interpolate + Clone + 'static {}

/// Blanket impl — every `Interpolate + Clone + 'static` is automatically `Animatable`.
impl<T: Interpolate + Clone + 'static> Animatable for T {}

/// A value that advances through time.
///
/// Implemented by [`Tween`](motus_tween::Tween), `Spring`, `SpringN`,
/// `Timeline`, and any user-defined animation type.
/// The [`AnimationDriver`](motus_driver::AnimationDriver) calls this each frame.
///
/// # Contract
///
/// - Returns `true` while the animation is still running.
/// - Returns `false` when the animation has completed (or is settled, for springs).
/// - Calling `update` after returning `false` is a no-op and returns `false`.
/// - `dt < 0.0` is treated as `0.0` — time does not run backward.
///
/// # Example
///
/// ```rust
/// use motus_core::Update;
///
/// struct Counter { count: u32 }
///
/// impl Update for Counter {
///     fn update(&mut self, _dt: f32) -> bool {
///         self.count += 1;
///         self.count < 10
///     }
/// }
///
/// let mut c = Counter { count: 0 };
/// while c.update(0.016) {}
/// assert_eq!(c.count, 10);
/// ```
pub trait Update {
    /// Advance the animation by `dt` seconds.
    ///
    /// Returns `true` while still running, `false` when complete.
    fn update(&mut self, dt: f32) -> bool;
}

// ──────────────────────────────────────────────────────────────────────────────
// Blanket `Interpolate` implementations
// ──────────────────────────────────────────────────────────────────────────────

impl Interpolate for f32 {
    /// Direct float lerp — `self + (other - self) * t`.
    #[inline]
    fn lerp(&self, other: &Self, t: f32) -> Self {
        self + (other - self) * t
    }
}

impl Interpolate for f64 {
    /// Full-precision f64 lerp — `t` is cast to f64.
    #[inline]
    fn lerp(&self, other: &Self, t: f32) -> Self {
        let t64 = t as f64;
        self + (other - self) * t64
    }
}

impl Interpolate for [f32; 2] {
    /// Per-component lerp for 2D vectors.
    #[inline]
    fn lerp(&self, other: &Self, t: f32) -> Self {
        [
            self[0] + (other[0] - self[0]) * t,
            self[1] + (other[1] - self[1]) * t,
        ]
    }
}

impl Interpolate for [f32; 3] {
    /// Per-component lerp for 3D vectors.
    #[inline]
    fn lerp(&self, other: &Self, t: f32) -> Self {
        [
            self[0] + (other[0] - self[0]) * t,
            self[1] + (other[1] - self[1]) * t,
            self[2] + (other[2] - self[2]) * t,
        ]
    }
}

impl Interpolate for [f32; 4] {
    /// Per-component lerp for 4D vectors (e.g. RGBA colors in linear space).
    #[inline]
    fn lerp(&self, other: &Self, t: f32) -> Self {
        [
            self[0] + (other[0] - self[0]) * t,
            self[1] + (other[1] - self[1]) * t,
            self[2] + (other[2] - self[2]) * t,
            self[3] + (other[3] - self[3]) * t,
        ]
    }
}

impl Interpolate for i32 {
    /// Lerps as `f32` and rounds to the nearest integer.
    #[inline]
    fn lerp(&self, other: &Self, t: f32) -> Self {
        let a = *self as f32;
        let b = *other as f32;
        (a + (b - a) * t).round() as i32
    }
}

impl Interpolate for u8 {
    /// Lerps as `f32`, rounds, and clamps to `[0, 255]`.
    #[inline]
    fn lerp(&self, other: &Self, t: f32) -> Self {
        let a = *self as f32;
        let b = *other as f32;
        (a + (b - a) * t).round().clamp(0.0, 255.0) as u8
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Tests
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // --- f32 ---
    #[test]
    fn f32_lerp_start() {
        assert_eq!(0.0_f32.lerp(&100.0, 0.0), 0.0);
    }

    #[test]
    fn f32_lerp_end() {
        assert_eq!(0.0_f32.lerp(&100.0, 1.0), 100.0);
    }

    #[test]
    fn f32_lerp_mid() {
        assert_eq!(0.0_f32.lerp(&100.0, 0.5), 50.0);
    }

    // --- f64 ---
    #[test]
    fn f64_lerp_precision() {
        let result = 0.0_f64.lerp(&1.0, 0.5);
        assert!((result - 0.5).abs() < 1e-10);
    }

    // --- [f32; 2] ---
    #[test]
    fn vec2_lerp() {
        let a = [0.0_f32, 0.0];
        let b = [10.0_f32, 20.0];
        let mid = a.lerp(&b, 0.5);
        assert_eq!(mid, [5.0, 10.0]);
    }

    // --- [f32; 3] ---
    #[test]
    fn vec3_lerp_endpoints() {
        let a = [1.0_f32, 2.0, 3.0];
        let b = [4.0_f32, 5.0, 6.0];
        assert_eq!(a.lerp(&b, 0.0), a);
        assert_eq!(a.lerp(&b, 1.0), b);
    }

    // --- [f32; 4] component independence ---
    #[test]
    fn vec4_components_independent() {
        let a = [0.0_f32; 4];
        let b = [1.0_f32, 2.0, 3.0, 4.0];
        let mid = a.lerp(&b, 0.5);
        assert_eq!(mid, [0.5, 1.0, 1.5, 2.0]);
    }

    // --- i32 rounding ---
    #[test]
    fn i32_rounds_correctly() {
        assert_eq!(0_i32.lerp(&10, 0.55), 6); // 5.5 → rounds to 6
        assert_eq!(0_i32.lerp(&10, 0.44), 4); // 4.4 → rounds to 4
    }

    // --- u8 clamping ---
    #[test]
    fn u8_clamps_at_255() {
        assert_eq!(200_u8.lerp(&255, 2.0), 255); // extrapolated, clamped
    }

    #[test]
    fn u8_clamps_at_0() {
        assert_eq!(50_u8.lerp(&0, 2.0), 0); // extrapolated below 0, clamped
    }

    // --- Update trait contract ---
    #[test]
    fn update_returns_false_when_done() {
        struct OneShot { done: bool }
        impl Update for OneShot {
            fn update(&mut self, _dt: f32) -> bool {
                if self.done { return false; }
                self.done = true;
                false
            }
        }
        let mut s = OneShot { done: false };
        assert!(!s.update(0.016));
        assert!(!s.update(0.016)); // idempotent after done
    }
}
