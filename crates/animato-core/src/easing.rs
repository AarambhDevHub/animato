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

use crate::math::{ceil, cos, log, powf, powi, sin, sqrt};
use core::f32::consts::PI;

/// All 31 classic easing variants, CSS-compatible parameterized variants,
/// and an escape-hatch `Custom` function pointer.
///
/// # `PartialEq` behaviour
///
/// `Custom(_)` never equals anything (including itself) because function
/// pointers aren't meaningfully comparable by identity in this context.
/// All other variants use structural equality, including parameter values for
/// `CubicBezier` and `Steps`.
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

    // ── CSS-compatible ──────────────────────────────────────────────────────
    /// CSS-compatible cubic Bezier easing: `(x1, y1, x2, y2)`.
    ///
    /// The x control points are clamped to `[0.0, 1.0]` before evaluation,
    /// matching the valid CSS timing-function domain. The y control points may
    /// overshoot to support anticipation and bounce-like curves.
    CubicBezier(f32, f32, f32, f32),

    /// CSS `steps(n, jump-end)` easing.
    ///
    /// `0` is treated as `1` step so invalid input remains safe.
    Steps(u32),

    // ── Advanced parameterised (v0.8.0) ───────────────────────────────────────
    /// Rough, organic-feeling easing with deterministic sine-based noise.
    ///
    /// `strength` in `[0.0, 1.0]` controls how rough the motion is.
    /// `points` controls the number of noise harmonics (2–20).
    RoughEase {
        /// Noise amplitude: `0.0` = linear, `1.0` = very rough.
        strength: f32,
        /// Number of noise harmonics: more points = more complex noise.
        points: u32,
    },

    /// Motion that accelerates at the edges and crawls through the middle.
    ///
    /// `linear_ratio` (`0.0`–`1.0`) is the fraction of time spent slow.
    /// `power` controls how sharply the edges accelerate.
    SlowMo {
        /// Fraction of time spent slow: `0.0` = no slow-down, `1.0` = fully slow.
        linear_ratio: f32,
        /// Controls how sharply the edges accelerate: higher values = sharper acceleration.
        power: f32,
    },

    /// Wiggling oscillation that fades in and out around the linear trend.
    ///
    /// `wiggles` is the number of complete oscillations.
    Wiggle {
        /// Number of complete oscillations.
        wiggles: u32,
    },

    /// Configurable bounce easing.
    ///
    /// `strength` in `[0.0, 1.0]`: `0.0` = no bounce (linear), `1.0` = full EaseOutBounce.
    CustomBounce {
        /// Bounce strength: `0.0` = no bounce (linear), `1.0` = full EaseOutBounce.
        strength: f32,
    },

    /// Exponential progression from `start` scale to `end` scale.
    ///
    /// Both `start` and `end` must be positive. Values near `1.0` approach linear.
    ExpoScale {
        /// Starting scale: must be positive.
        start: f32,
        /// Ending scale: must be positive.
        end: f32,
    },

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

// Manual PartialEq: Custom never equals anything; parameterized variants compare data.
impl PartialEq for Easing {
    fn eq(&self, other: &Self) -> bool {
        use Easing::*;
        match (self, other) {
            (Custom(_), _) | (_, Custom(_)) => false,
            (CubicBezier(ax1, ay1, ax2, ay2), CubicBezier(bx1, by1, bx2, by2)) => {
                ax1 == bx1 && ay1 == by1 && ax2 == bx2 && ay2 == by2
            }
            (Steps(a), Steps(b)) => a == b,
            (
                RoughEase {
                    strength: sa,
                    points: pa,
                },
                RoughEase {
                    strength: sb,
                    points: pb,
                },
            ) => sa == sb && pa == pb,
            (
                SlowMo {
                    linear_ratio: la,
                    power: pa,
                },
                SlowMo {
                    linear_ratio: lb,
                    power: pb,
                },
            ) => la == lb && pa == pb,
            (Wiggle { wiggles: a }, Wiggle { wiggles: b }) => a == b,
            (CustomBounce { strength: a }, CustomBounce { strength: b }) => a == b,
            (ExpoScale { start: sa, end: ea }, ExpoScale { start: sb, end: eb }) => {
                sa == sb && ea == eb
            }
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
                    Easing::CubicBezier(x1, y1, x2, y2) => cubic_bezier(t, *x1, *y1, *x2, *y2),
                    Easing::Steps(count) => steps(t, *count),
                    Easing::RoughEase { strength, points } => rough_ease(t, *strength, *points),
                    Easing::SlowMo {
                        linear_ratio,
                        power,
                    } => slow_mo(t, *linear_ratio, *power),
                    Easing::Wiggle { wiggles } => wiggle(t, *wiggles),
                    Easing::CustomBounce { strength } => custom_bounce(t, *strength),
                    Easing::ExpoScale { start, end } => expo_scale(t, *start, *end),
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
            Easing::CubicBezier(0.25, 0.1, 0.25, 1.0),
            Easing::Steps(1),
            // ── v0.8.0 advanced ─────────────────────────────────────
            Easing::RoughEase {
                strength: 0.5,
                points: 8,
            },
            Easing::SlowMo {
                linear_ratio: 0.5,
                power: 0.7,
            },
            Easing::Wiggle { wiggles: 5 },
            Easing::CustomBounce { strength: 0.7 },
            Easing::ExpoScale {
                start: 0.5,
                end: 2.0,
            },
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

/// CSS-compatible cubic Bezier easing.
///
/// `x1` and `x2` are clamped to `[0.0, 1.0]` because CSS timing functions
/// require monotonic x control points. `y1` and `y2` are left unconstrained so
/// curves can overshoot.
#[inline]
pub fn cubic_bezier(t: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t == 0.0 || t == 1.0 {
        return t;
    }

    let x1 = x1.clamp(0.0, 1.0);
    let x2 = x2.clamp(0.0, 1.0);
    let mut u = t;

    for _ in 0..6 {
        let x = sample_cubic(x1, x2, u) - t;
        if x.abs() < 1e-6 {
            return sample_cubic(y1, y2, u);
        }
        let derivative = sample_cubic_derivative(x1, x2, u);
        if derivative.abs() < 1e-6 {
            break;
        }
        u = (u - x / derivative).clamp(0.0, 1.0);
    }

    let mut low = 0.0;
    let mut high = 1.0;
    u = t;
    for _ in 0..10 {
        let x = sample_cubic(x1, x2, u);
        if (x - t).abs() < 1e-6 {
            break;
        }
        if x < t {
            low = u;
        } else {
            high = u;
        }
        u = (low + high) * 0.5;
    }

    sample_cubic(y1, y2, u)
}

/// CSS `steps(n, jump-end)` easing.
///
/// `count = 0` is treated as one step.
#[inline]
pub fn steps(t: f32, count: u32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t == 0.0 {
        return 0.0;
    }
    let count = count.max(1) as f32;
    (ceil(t * count) / count).clamp(0.0, 1.0)
}

// ── v0.8.0 Advanced free functions ───────────────────────────────────────────

/// Rough, organic easing using deterministic sine harmonics.
///
/// Adds noise that is zero at `t = 0.0` and `t = 1.0`, preserving endpoints.
/// `strength` controls amplitude (`0.0` = linear), `points` controls harmonics.
///
/// ```rust
/// use animato_core::easing::rough_ease;
/// assert_eq!(rough_ease(0.0, 0.5, 8), 0.0);
/// assert_eq!(rough_ease(1.0, 0.5, 8), 1.0);
/// ```
#[inline]
pub fn rough_ease(t: f32, strength: f32, points: u32) -> f32 {
    if t <= 0.0 {
        return 0.0;
    }
    if t >= 1.0 {
        return 1.0;
    }

    let n = points.clamp(2, 20);
    // Boundary envelope: zero at t=0 and t=1, peaks at midpoint.
    let boundary = 4.0 * t * (1.0 - t);
    let mut noise = 0.0_f32;
    for i in 1..=n {
        let freq = i as f32 * PI;
        noise += sin(freq * t) / i as f32;
    }
    // Normalise amplitude so it's roughly in [-1, 1] regardless of `n`.
    let norm = log(n as f32).max(1.0);
    noise /= norm;

    t + boundary * strength.clamp(0.0, 2.0) * noise
}

/// Slow-motion easing: fast at edges, slow in the middle.
///
/// `linear_ratio` ∈ `[0.0, 1.0]` is the fraction of the animation spent slow.
/// `power` ≥ `0.0` controls how sharply edges accelerate (`0.0` = linear edges).
///
/// ```rust
/// use animato_core::easing::slow_mo;
/// assert_eq!(slow_mo(0.0, 0.5, 0.7), 0.0);
/// assert_eq!(slow_mo(1.0, 0.5, 0.7), 1.0);
/// ```
#[inline]
pub fn slow_mo(t: f32, linear_ratio: f32, power: f32) -> f32 {
    if t <= 0.0 {
        return 0.0;
    }
    if t >= 1.0 {
        return 1.0;
    }

    let lr = linear_ratio.clamp(0.0, 1.0);
    let p = power.max(0.0);

    if lr >= 1.0 {
        return t;
    }

    // Time-domain split: middle lr-fraction of time, edges (1-lr)/2 each.
    let t_mid_start = (1.0 - lr) * 0.5;
    let t_mid_end = t_mid_start + lr;

    // Speed ratio: edges are (p+1)x faster than the middle.
    // Derive slopes so that the output integrates to 1.0 over [0,1]:
    //   s_mid * lr + s_edge * (1 - lr) = 1
    //   s_edge = (p + 1) * s_mid
    // => s_mid = 1 / (1 + p * (1 - lr))
    let s_mid = 1.0 / (1.0 + p * (1.0 - lr));
    let s_edge = if (1.0 - lr) > f32::EPSILON {
        (1.0 - s_mid * lr) / (1.0 - lr)
    } else {
        1.0
    };

    if t < t_mid_start {
        // Fast leading edge (linear ramp)
        t * s_edge
    } else if t > t_mid_end {
        // Fast trailing edge (linear ramp)
        let out_at_mid_end = t_mid_start * s_edge + lr * s_mid;
        out_at_mid_end + (t - t_mid_end) * s_edge
    } else {
        // Slow middle (compressed linear progress)
        t_mid_start * s_edge + (t - t_mid_start) * s_mid
    }
}

/// Wiggle easing: oscillates around the linear trend with a sine envelope.
///
/// `wiggles` is the number of full oscillation cycles. The trend still goes
/// from `0.0` to `1.0` with overshoot in between.
///
/// ```rust
/// use animato_core::easing::wiggle;
/// assert_eq!(wiggle(0.0, 5), 0.0);
/// assert_eq!(wiggle(1.0, 5), 1.0);
/// ```
#[inline]
pub fn wiggle(t: f32, wiggles: u32) -> f32 {
    if t <= 0.0 {
        return 0.0;
    }
    if t >= 1.0 {
        return 1.0;
    }
    let n = wiggles.max(1) as f32;
    // Envelope peaks at the midpoint and is zero at both ends.
    let envelope = sin(t * PI);
    let oscillation = sin(t * n * PI * 2.0) * envelope;
    t + oscillation * 0.25
}

/// Configurable bounce easing.
///
/// `strength` ∈ `[0.0, 1.0]`: blends between linear (`0.0`) and `EaseOutBounce` (`1.0`).
///
/// ```rust
/// use animato_core::easing::custom_bounce;
/// assert_eq!(custom_bounce(0.0, 0.7), 0.0);
/// assert_eq!(custom_bounce(1.0, 0.7), 1.0);
/// ```
#[inline]
pub fn custom_bounce(t: f32, strength: f32) -> f32 {
    if t <= 0.0 {
        return 0.0;
    }
    if t >= 1.0 {
        return 1.0;
    }
    let s = strength.clamp(0.0, 1.0);
    // Blend between linear and ease_out_bounce.
    t * (1.0 - s) + ease_out_bounce(t) * s
}

/// Exponential scale easing: warps time according to an exponential curve.
///
/// At `start = 1.0, end = 1.0` the result is linear.
/// `start` and `end` must be positive; values below `0.001` are clamped.
///
/// ```rust
/// use animato_core::easing::expo_scale;
/// assert_eq!(expo_scale(0.0, 0.5, 2.0), 0.0);
/// assert_eq!(expo_scale(1.0, 0.5, 2.0), 1.0);
/// ```
#[inline]
pub fn expo_scale(t: f32, start: f32, end: f32) -> f32 {
    if t <= 0.0 {
        return 0.0;
    }
    if t >= 1.0 {
        return 1.0;
    }

    let s = start.max(0.001_f32);
    let e = end.max(0.001_f32);

    // If start ≈ end the curve degenerates to linear.
    if (s - e).abs() < 0.001 {
        return t;
    }

    let k = e / s;
    if (k - 1.0).abs() < 0.001 {
        return t;
    }

    // Exponential interpolation: f(t) = (k^t - 1) / (k - 1).
    // f(0) = 0 ✓   f(1) = (k-1)/(k-1) = 1 ✓
    (powf(k, t) - 1.0) / (k - 1.0)
}

#[inline]
fn sample_cubic(a1: f32, a2: f32, t: f32) -> f32 {
    let c = 3.0 * a1;
    let b = 3.0 * (a2 - a1) - c;
    let a = 1.0 - c - b;
    ((a * t + b) * t + c) * t
}

#[inline]
fn sample_cubic_derivative(a1: f32, a2: f32, t: f32) -> f32 {
    let c = 3.0 * a1;
    let b = 3.0 * (a2 - a1) - c;
    let a = 1.0 - c - b;
    (3.0 * a * t + 2.0 * b) * t + c
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

    #[test]
    fn all_named_count() {
        assert_eq!(Easing::all_named().len(), 38);
    }

    #[test]
    fn custom_variant_applies_fn() {
        let e = Easing::Custom(|t| t * t);
        assert_eq!(e.apply(0.5), 0.25);
    }

    #[test]
    fn custom_never_equals() {
        let a = Easing::Custom(|t| t);
        let b = Easing::Custom(|t| t);
        assert!(a != b);
        assert!(a != Easing::Linear);
    }

    #[test]
    fn named_equality() {
        assert_eq!(Easing::Linear, Easing::Linear);
        assert_eq!(Easing::EaseOutCubic, Easing::EaseOutCubic);
        assert_eq!(
            Easing::CubicBezier(0.25, 0.1, 0.25, 1.0),
            Easing::CubicBezier(0.25, 0.1, 0.25, 1.0)
        );
        assert_eq!(Easing::Steps(4), Easing::Steps(4));
        assert_ne!(Easing::EaseInQuad, Easing::EaseOutQuad);
        // Advanced variants
        assert_eq!(
            Easing::RoughEase {
                strength: 0.5,
                points: 8
            },
            Easing::RoughEase {
                strength: 0.5,
                points: 8
            }
        );
        assert_ne!(
            Easing::RoughEase {
                strength: 0.5,
                points: 8
            },
            Easing::RoughEase {
                strength: 0.5,
                points: 4
            }
        );
        assert_eq!(Easing::Wiggle { wiggles: 5 }, Easing::Wiggle { wiggles: 5 });
        assert_eq!(
            Easing::CustomBounce { strength: 0.7 },
            Easing::CustomBounce { strength: 0.7 }
        );
        assert_eq!(
            Easing::ExpoScale {
                start: 0.5,
                end: 2.0
            },
            Easing::ExpoScale {
                start: 0.5,
                end: 2.0
            }
        );
    }

    #[test]
    fn rough_ease_monotonic_bias() {
        // Should be a "roughly" increasing function on average.
        let sum: f32 = (1..10).map(|i| rough_ease(i as f32 / 10.0, 0.3, 6)).sum();
        assert!(sum > 0.0, "rough ease should have positive trend");
    }

    #[test]
    fn slow_mo_middle_is_slow() {
        // The derivative at the midpoint should be smaller than at the edges.
        let dt = 0.01_f32;
        let mid_vel = (slow_mo(0.5 + dt, 0.5, 1.0) - slow_mo(0.5, 0.5, 1.0)) / dt;
        let edge_vel = (slow_mo(0.05 + dt, 0.5, 1.0) - slow_mo(0.05, 0.5, 1.0)) / dt;
        assert!(
            mid_vel < edge_vel,
            "middle should be slower than edges: mid={mid_vel}, edge={edge_vel}"
        );
    }

    #[test]
    fn slow_mo_zero_linear_ratio() {
        // With lr=0.0 there is no linear portion, should still satisfy endpoints.
        assert!(approx_eq(slow_mo(0.0, 0.0, 1.0), 0.0));
        assert!(approx_eq(slow_mo(1.0, 0.0, 1.0), 1.0));
    }

    #[test]
    fn wiggle_stays_finite() {
        for i in 0..=100 {
            let t = i as f32 / 100.0;
            let v = wiggle(t, 5);
            assert!(v.is_finite(), "wiggle at t={t} produced non-finite {v}");
        }
    }

    #[test]
    fn custom_bounce_blends_correctly() {
        // At strength=0.0 it should be linear.
        for i in 1..10 {
            let t = i as f32 / 10.0;
            assert!(
                approx_eq(custom_bounce(t, 0.0), t),
                "strength=0 should be linear at t={t}"
            );
        }
        // At strength=1.0 it should equal ease_out_bounce.
        for i in 1..10 {
            let t = i as f32 / 10.0;
            assert!(
                approx_eq(custom_bounce(t, 1.0), ease_out_bounce(t)),
                "strength=1 should equal ease_out_bounce at t={t}"
            );
        }
    }

    #[test]
    fn expo_scale_is_monotonic() {
        let mut prev = 0.0_f32;
        for i in 1..=20 {
            let t = i as f32 / 20.0;
            let v = expo_scale(t, 0.5, 2.0);
            assert!(v >= prev - 1e-5, "expo_scale should be monotonic at t={t}");
            prev = v;
        }
    }

    #[test]
    fn expo_scale_equal_start_end_is_linear() {
        for i in 1..10 {
            let t = i as f32 / 10.0;
            assert!(approx_eq(expo_scale(t, 1.0, 1.0), t));
        }
    }

    #[test]
    fn free_functions_match_enum() {
        type EasingCase = (Easing, fn(f32) -> f32);

        let cases: &[EasingCase] = &[
            (Easing::EaseInQuad, ease_in_quad),
            (Easing::EaseOutCubic, ease_out_cubic),
            (Easing::EaseOutBounce, ease_out_bounce),
        ];
        for t in [0.1, 0.5, 0.9] {
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

    #[test]
    fn cubic_bezier_linear_is_identity() {
        let easing = Easing::CubicBezier(0.0, 0.0, 1.0, 1.0);
        for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
            assert!(
                approx_eq(easing.apply(t), t),
                "linear cubic-bezier at t={t} was {}",
                easing.apply(t)
            );
        }
    }

    #[test]
    fn steps_jump_end_behavior() {
        let easing = Easing::Steps(4);
        assert_eq!(easing.apply(0.0), 0.0);
        assert_eq!(easing.apply(0.01), 0.25);
        assert_eq!(easing.apply(1.0), 1.0);
    }

    #[test]
    fn linear_is_identity() {
        for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
            assert_eq!(Easing::Linear.apply(t), t);
        }
    }
}
