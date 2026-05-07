//! All easing functions for the Animato animation library.
//!
//! Two ways to use easing:
//!
//! 1. **`Easing` enum** — storable, passable, optionally serializable:
//!    ```rust
//!    use animato_core::Easing;
//!    let e = Easing::EaseOutCubic;
//!    let v = e.apply(0.5);
//!    ```
//!
//! 2. **Free functions** — zero-overhead, inlined at call site:
//!    ```rust
//!    use animato_core::easing::ease_out_cubic;
//!    let v = ease_out_cubic(0.5);
//!    ```
//!
//! ## Invariants
//!
//! All named variants satisfy:
//! - `apply(0.0) == 0.0`
//! - `apply(1.0) == 1.0`
//! - `t` outside `[0.0, 1.0]` is clamped — no panic

use crate::math::{cos, powf, powi, sin, sqrt};
use core::f32::consts::PI;

/// All 31 classic easing variants plus an escape-hatch `Custom` function pointer.
///
/// # `PartialEq` behaviour
///
/// `Custom(_)` never equals anything (including itself) because function
/// pointers aren't meaningfully comparable by identity in this context.
/// All other variants use structural equality.
///
/// # Serialization
///
/// With the `serde` feature, all variants except `Custom` are serializable.
/// `Custom` is skipped — function pointers cannot be serialized.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Easing {
    /// Constant velocity — no acceleration.
    Linear,

    // ── Polynomial ────────────────────────────────────────────────────────────
    /// Quadratic ease-in (accelerates from zero velocity).
    EaseInQuad,
    /// Quadratic ease-out (decelerates to zero velocity).
    EaseOutQuad,
    /// Quadratic ease-in-out (accelerates then decelerates).
    EaseInOutQuad,

    /// Cubic ease-in.
    EaseInCubic,
    /// Cubic ease-out.
    EaseOutCubic,
    /// Cubic ease-in-out.
    EaseInOutCubic,

    /// Quartic ease-in.
    EaseInQuart,
    /// Quartic ease-out.
    EaseOutQuart,
    /// Quartic ease-in-out.
    EaseInOutQuart,

    /// Quintic ease-in.
    EaseInQuint,
    /// Quintic ease-out.
    EaseOutQuint,
    /// Quintic ease-in-out.
    EaseInOutQuint,

    // ── Sinusoidal ────────────────────────────────────────────────────────────
    /// Sinusoidal ease-in.
    EaseInSine,
    /// Sinusoidal ease-out.
    EaseOutSine,
    /// Sinusoidal ease-in-out.
    EaseInOutSine,

    // ── Exponential ───────────────────────────────────────────────────────────
    /// Exponential ease-in.
    EaseInExpo,
    /// Exponential ease-out.
    EaseOutExpo,
    /// Exponential ease-in-out.
    EaseInOutExpo,

    // ── Circular ──────────────────────────────────────────────────────────────
    /// Circular ease-in.
    EaseInCirc,
    /// Circular ease-out.
    EaseOutCirc,
    /// Circular ease-in-out.
    EaseInOutCirc,

    // ── Back (overshoot) ──────────────────────────────────────────────────────
    /// Back ease-in — slight overshoot at the start.
    EaseInBack,
    /// Back ease-out — slight overshoot at the end.
    EaseOutBack,
    /// Back ease-in-out.
    EaseInOutBack,

    // ── Elastic ───────────────────────────────────────────────────────────────
    /// Elastic ease-in — spring-like oscillation at the start.
    EaseInElastic,
    /// Elastic ease-out — spring-like oscillation at the end.
    EaseOutElastic,
    /// Elastic ease-in-out.
    EaseInOutElastic,

    // ── Bounce ────────────────────────────────────────────────────────────────
    /// Bounce ease-in — ball bounce at the start.
    EaseInBounce,
    /// Bounce ease-out — ball bounces to rest at the end.
    EaseOutBounce,
    /// Bounce ease-in-out.
    EaseInOutBounce,

    // ── Escape hatch ──────────────────────────────────────────────────────────
    /// Custom function pointer — zero overhead, no allocation.
    ///
    /// Not serializable and does not participate in `PartialEq` (always `false`).
    ///
    /// ```rust
    /// use animato_core::Easing;
    /// let e = Easing::Custom(|t| t * t * t);
    /// assert_eq!(e.apply(0.5), 0.125);
    /// ```
    #[cfg_attr(feature = "serde", serde(skip))]
    Custom(fn(f32) -> f32),
}

// Manual PartialEq: Custom never equals anything; all others use discriminant.
impl PartialEq for Easing {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Easing::Custom(_), _) | (_, Easing::Custom(_)) => false,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Easing {
    /// Evaluate the easing function at `t`.
    ///
    /// `t` is clamped to `[0.0, 1.0]` before evaluation — out-of-range
    /// values never panic and never extrapolate for named variants.
    ///
    /// # Example
    ///
    /// ```rust
    /// use animato_core::Easing;
    /// assert_eq!(Easing::Linear.apply(0.5), 0.5);
    /// assert_eq!(Easing::EaseInQuad.apply(0.0), 0.0);
    /// assert_eq!(Easing::EaseInQuad.apply(1.0), 1.0);
    /// // Out-of-range — no panic:
    /// let _ = Easing::EaseOutCubic.apply(-1.0);
    /// let _ = Easing::EaseOutCubic.apply(2.0);
    /// ```
    #[inline]
    pub fn apply(&self, t: f32) -> f32 {
        match self {
            Easing::Custom(f) => f(t),
            _ => {
                let t = t.clamp(0.0, 1.0);
                match self {
                    Easing::Linear => t,
                    Easing::EaseInQuad => ease_in_quad(t),
                    Easing::EaseOutQuad => ease_out_quad(t),
                    Easing::EaseInOutQuad => ease_in_out_quad(t),
                    Easing::EaseInCubic => ease_in_cubic(t),
                    Easing::EaseOutCubic => ease_out_cubic(t),
                    Easing::EaseInOutCubic => ease_in_out_cubic(t),
                    Easing::EaseInQuart => ease_in_quart(t),
                    Easing::EaseOutQuart => ease_out_quart(t),
                    Easing::EaseInOutQuart => ease_in_out_quart(t),
                    Easing::EaseInQuint => ease_in_quint(t),
                    Easing::EaseOutQuint => ease_out_quint(t),
                    Easing::EaseInOutQuint => ease_in_out_quint(t),
                    Easing::EaseInSine => ease_in_sine(t),
                    Easing::EaseOutSine => ease_out_sine(t),
                    Easing::EaseInOutSine => ease_in_out_sine(t),
                    Easing::EaseInExpo => ease_in_expo(t),
                    Easing::EaseOutExpo => ease_out_expo(t),
                    Easing::EaseInOutExpo => ease_in_out_expo(t),
                    Easing::EaseInCirc => ease_in_circ(t),
                    Easing::EaseOutCirc => ease_out_circ(t),
                    Easing::EaseInOutCirc => ease_in_out_circ(t),
                    Easing::EaseInBack => ease_in_back(t),
                    Easing::EaseOutBack => ease_out_back(t),
                    Easing::EaseInOutBack => ease_in_out_back(t),
                    Easing::EaseInElastic => ease_in_elastic(t),
                    Easing::EaseOutElastic => ease_out_elastic(t),
                    Easing::EaseInOutElastic => ease_in_out_elastic(t),
                    Easing::EaseInBounce => ease_in_bounce(t),
                    Easing::EaseOutBounce => ease_out_bounce(t),
                    Easing::EaseInOutBounce => ease_in_out_bounce(t),
                    Easing::Custom(_) => unreachable!(),
                }
            }
        }
    }

    /// Returns a slice of all named (non-`Custom`) variants.
    ///
    /// Useful for picker UIs and exhaustive test sweeps.
    ///
    /// ```rust
    /// use animato_core::Easing;
    /// for e in Easing::all_named() {
    ///     assert_eq!(e.apply(0.0), 0.0);
    ///     assert_eq!(e.apply(1.0), 1.0);
    /// }
    /// ```
    pub fn all_named() -> &'static [Easing] {
        &[
            Easing::Linear,
            Easing::EaseInQuad,
            Easing::EaseOutQuad,
            Easing::EaseInOutQuad,
            Easing::EaseInCubic,
            Easing::EaseOutCubic,
            Easing::EaseInOutCubic,
            Easing::EaseInQuart,
            Easing::EaseOutQuart,
            Easing::EaseInOutQuart,
            Easing::EaseInQuint,
            Easing::EaseOutQuint,
            Easing::EaseInOutQuint,
            Easing::EaseInSine,
            Easing::EaseOutSine,
            Easing::EaseInOutSine,
            Easing::EaseInExpo,
            Easing::EaseOutExpo,
            Easing::EaseInOutExpo,
            Easing::EaseInCirc,
            Easing::EaseOutCirc,
            Easing::EaseInOutCirc,
            Easing::EaseInBack,
            Easing::EaseOutBack,
            Easing::EaseInOutBack,
            Easing::EaseInElastic,
            Easing::EaseOutElastic,
            Easing::EaseInOutElastic,
            Easing::EaseInBounce,
            Easing::EaseOutBounce,
            Easing::EaseInOutBounce,
        ]
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Free easing functions — #[inline] for zero-overhead direct calls
// ──────────────────────────────────────────────────────────────────────────────

/// Quadratic ease-in: `t²`
#[inline]
pub fn ease_in_quad(t: f32) -> f32 {
    t * t
}

/// Quadratic ease-out: `1 - (1-t)²`
#[inline]
pub fn ease_out_quad(t: f32) -> f32 {
    1.0 - (1.0 - t) * (1.0 - t)
}

/// Quadratic ease-in-out.
#[inline]
pub fn ease_in_out_quad(t: f32) -> f32 {
    if t < 0.5 {
        2.0 * t * t
    } else {
        1.0 - powi(-2.0 * t + 2.0, 2) / 2.0
    }
}

/// Cubic ease-in: `t³`
#[inline]
pub fn ease_in_cubic(t: f32) -> f32 {
    t * t * t
}

/// Cubic ease-out: `1 - (1-t)³`
#[inline]
pub fn ease_out_cubic(t: f32) -> f32 {
    1.0 - powi(1.0 - t, 3)
}

/// Cubic ease-in-out.
#[inline]
pub fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - powi(-2.0 * t + 2.0, 3) / 2.0
    }
}

/// Quartic ease-in: `t⁴`
#[inline]
pub fn ease_in_quart(t: f32) -> f32 {
    t * t * t * t
}

/// Quartic ease-out: `1 - (1-t)⁴`
#[inline]
pub fn ease_out_quart(t: f32) -> f32 {
    1.0 - powi(1.0 - t, 4)
}

/// Quartic ease-in-out.
#[inline]
pub fn ease_in_out_quart(t: f32) -> f32 {
    if t < 0.5 {
        8.0 * t * t * t * t
    } else {
        1.0 - powi(-2.0 * t + 2.0, 4) / 2.0
    }
}

/// Quintic ease-in: `t⁵`
#[inline]
pub fn ease_in_quint(t: f32) -> f32 {
    t * t * t * t * t
}

/// Quintic ease-out: `1 - (1-t)⁵`
#[inline]
pub fn ease_out_quint(t: f32) -> f32 {
    1.0 - powi(1.0 - t, 5)
}

/// Quintic ease-in-out.
#[inline]
pub fn ease_in_out_quint(t: f32) -> f32 {
    if t < 0.5 {
        16.0 * t * t * t * t * t
    } else {
        1.0 - powi(-2.0 * t + 2.0, 5) / 2.0
    }
}

/// Sinusoidal ease-in.
#[inline]
pub fn ease_in_sine(t: f32) -> f32 {
    1.0 - cos(t * PI / 2.0)
}
/// Sinusoidal ease-out.
#[inline]
pub fn ease_out_sine(t: f32) -> f32 {
    sin(t * PI / 2.0)
}
/// Sinusoidal ease-in-out.
#[inline]
pub fn ease_in_out_sine(t: f32) -> f32 {
    -(cos(t * PI) - 1.0) / 2.0
}

/// Exponential ease-in.
#[inline]
pub fn ease_in_expo(t: f32) -> f32 {
    if t == 0.0 {
        0.0
    } else {
        powf(2.0, 10.0 * t - 10.0)
    }
}

/// Exponential ease-out.
#[inline]
pub fn ease_out_expo(t: f32) -> f32 {
    if t == 1.0 {
        1.0
    } else {
        1.0 - powf(2.0, -10.0 * t)
    }
}

/// Exponential ease-in-out.
#[inline]
pub fn ease_in_out_expo(t: f32) -> f32 {
    if t == 0.0 {
        return 0.0;
    }
    if t == 1.0 {
        return 1.0;
    }
    if t < 0.5 {
        powf(2.0, 20.0 * t - 10.0) / 2.0
    } else {
        (2.0 - powf(2.0, -20.0 * t + 10.0)) / 2.0
    }
}

/// Circular ease-in.
#[inline]
pub fn ease_in_circ(t: f32) -> f32 {
    1.0 - sqrt(1.0 - t * t)
}
/// Circular ease-out.
#[inline]
pub fn ease_out_circ(t: f32) -> f32 {
    sqrt(1.0 - (t - 1.0) * (t - 1.0))
}

/// Circular ease-in-out.
#[inline]
pub fn ease_in_out_circ(t: f32) -> f32 {
    if t < 0.5 {
        (1.0 - sqrt(1.0 - powi(2.0 * t, 2))) / 2.0
    } else {
        (sqrt(1.0 - powi(-2.0 * t + 2.0, 2)) + 1.0) / 2.0
    }
}

const BACK_C1: f32 = 1.701_58;
const BACK_C2: f32 = BACK_C1 * 1.525;
const BACK_C3: f32 = BACK_C1 + 1.0;

/// Back ease-in — overshoots slightly then pulls back.
#[inline]
pub fn ease_in_back(t: f32) -> f32 {
    BACK_C3 * t * t * t - BACK_C1 * t * t
}

/// Back ease-out — overshoots the target then settles.
#[inline]
pub fn ease_out_back(t: f32) -> f32 {
    let t = t - 1.0;
    1.0 + BACK_C3 * t * t * t + BACK_C1 * t * t
}

/// Back ease-in-out.
#[inline]
pub fn ease_in_out_back(t: f32) -> f32 {
    if t < 0.5 {
        (powi(2.0 * t, 2) * ((BACK_C2 + 1.0) * 2.0 * t - BACK_C2)) / 2.0
    } else {
        (powi(2.0 * t - 2.0, 2) * ((BACK_C2 + 1.0) * (2.0 * t - 2.0) + BACK_C2) + 2.0) / 2.0
    }
}

const ELASTIC_C4: f32 = (2.0 * PI) / 3.0;
const ELASTIC_C5: f32 = (2.0 * PI) / 4.5;

/// Elastic ease-in — spring-like oscillation at the beginning.
#[inline]
pub fn ease_in_elastic(t: f32) -> f32 {
    if t == 0.0 {
        return 0.0;
    }
    if t == 1.0 {
        return 1.0;
    }
    -powf(2.0, 10.0 * t - 10.0) * sin((10.0 * t - 10.75) * ELASTIC_C4)
}

/// Elastic ease-out — spring-like oscillation at the end.
#[inline]
pub fn ease_out_elastic(t: f32) -> f32 {
    if t == 0.0 {
        return 0.0;
    }
    if t == 1.0 {
        return 1.0;
    }
    powf(2.0, -10.0 * t) * sin((10.0 * t - 0.75) * ELASTIC_C4) + 1.0
}

/// Elastic ease-in-out.
#[inline]
pub fn ease_in_out_elastic(t: f32) -> f32 {
    if t == 0.0 {
        return 0.0;
    }
    if t == 1.0 {
        return 1.0;
    }
    if t < 0.5 {
        -(powf(2.0, 20.0 * t - 10.0) * sin((20.0 * t - 11.125) * ELASTIC_C5)) / 2.0
    } else {
        (powf(2.0, -20.0 * t + 10.0) * sin((20.0 * t - 11.125) * ELASTIC_C5)) / 2.0 + 1.0
    }
}

/// Bounce ease-out — ball bouncing to rest.
#[inline]
pub fn ease_out_bounce(t: f32) -> f32 {
    const N1: f32 = 7.5625;
    const D1: f32 = 2.75;
    let t = &mut { t };
    if *t < 1.0 / D1 {
        N1 * *t * *t
    } else if *t < 2.0 / D1 {
        *t -= 1.5 / D1;
        N1 * *t * *t + 0.75
    } else if *t < 2.5 / D1 {
        *t -= 2.25 / D1;
        N1 * *t * *t + 0.9375
    } else {
        *t -= 2.625 / D1;
        N1 * *t * *t + 0.984_375
    }
}

/// Bounce ease-in.
#[inline]
pub fn ease_in_bounce(t: f32) -> f32 {
    1.0 - ease_out_bounce(1.0 - t)
}

/// Bounce ease-in-out.
#[inline]
pub fn ease_in_out_bounce(t: f32) -> f32 {
    if t < 0.5 {
        (1.0 - ease_out_bounce(1.0 - 2.0 * t)) / 2.0
    } else {
        (1.0 + ease_out_bounce(2.0 * t - 1.0)) / 2.0
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Tests
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 1e-5;

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < EPSILON
    }

    /// Every named variant must satisfy apply(0) == 0 and apply(1) == 1.
    #[test]
    fn all_named_endpoints() {
        for easing in Easing::all_named() {
            let v0 = easing.apply(0.0);
            let v1 = easing.apply(1.0);
            assert!(
                approx_eq(v0, 0.0),
                "{:?}.apply(0.0) = {} (expected 0.0)",
                easing,
                v0
            );
            assert!(
                approx_eq(v1, 1.0),
                "{:?}.apply(1.0) = {} (expected 1.0)",
                easing,
                v1
            );
        }
    }

    /// Out-of-range t must not panic for any named variant.
    #[test]
    fn no_panic_out_of_range() {
        for easing in Easing::all_named() {
            let _ = easing.apply(-0.5);
            let _ = easing.apply(1.5);
            let _ = easing.apply(f32::INFINITY);
            let _ = easing.apply(f32::NEG_INFINITY);
            // Note: NaN input is outside the defined contract (t ∈ [0,1]).
            // f32::NAN.clamp(0.0, 1.0) == NaN per IEEE 754 — acceptable.
        }
    }

    /// Custom function pointer works and is not equal to anything.
    #[test]
    fn custom_variant() {
        let e = Easing::Custom(|t| t * t);
        assert_eq!(e.apply(0.5), 0.25);
    }

    #[test]
    fn custom_never_equals() {
        let a = Easing::Custom(|t| t);
        let b = Easing::Custom(|t| t);
        let c = Easing::Linear;
        assert!(a != b);
        assert!(a != c);
        assert!(c != a);
    }

    /// Named variants with same discriminant are equal.
    #[test]
    fn named_equality() {
        assert_eq!(Easing::Linear, Easing::Linear);
        assert_eq!(Easing::EaseOutCubic, Easing::EaseOutCubic);
        assert_ne!(Easing::EaseInQuad, Easing::EaseOutQuad);
    }

    /// Free functions match Easing::apply output.
    #[test]
    fn free_functions_match_enum() {
        let cases: &[(Easing, fn(f32) -> f32)] = &[
            (Easing::EaseInQuad, ease_in_quad),
            (Easing::EaseOutQuad, ease_out_quad),
            (Easing::EaseInCubic, ease_in_cubic),
            (Easing::EaseOutCubic, ease_out_cubic),
            (Easing::EaseInOutCubic, ease_in_out_cubic),
            (Easing::EaseOutBounce, ease_out_bounce),
            (Easing::EaseOutElastic, ease_out_elastic),
            (Easing::EaseOutBack, ease_out_back),
        ];
        for t in [0.1, 0.25, 0.5, 0.75, 0.9] {
            for (easing, f) in cases {
                let a = easing.apply(t);
                let b = f(t);
                assert!(
                    approx_eq(a, b),
                    "{:?} at t={}: enum={} free_fn={}",
                    easing,
                    t,
                    a,
                    b
                );
            }
        }
    }

    /// EaseOut variants produce values > t for t in (0, 1) — front-loaded motion.
    #[test]
    fn ease_out_frontloaded() {
        for t in [0.1_f32, 0.3, 0.5, 0.7] {
            assert!(
                Easing::EaseOutCubic.apply(t) > t,
                "EaseOutCubic at t={} should be > t",
                t
            );
        }
    }

    /// Linear is exactly t.
    #[test]
    fn linear_is_identity() {
        for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
            assert_eq!(Easing::Linear.apply(t), t);
        }
    }

    /// all_named() has exactly 31 entries.
    #[test]
    fn all_named_count() {
        assert_eq!(Easing::all_named().len(), 31);
    }
}
