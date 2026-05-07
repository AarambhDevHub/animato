//! [`SpringConfig`] — parameters that control spring behaviour.

/// Configuration for a damped harmonic oscillator spring.
///
/// Use one of the named presets for common feels, or tune the parameters directly.
///
/// # Presets
///
/// | Preset | Stiffness | Damping | Feel |
/// |--------|-----------|---------|------|
/// | `gentle()` | 60 | 14 | Slow, soft |
/// | `wobbly()` | 180 | 12 | Bouncy, playful |
/// | `stiff()` | 210 | 20 | Fast, firm |
/// | `slow()` | 37 | 14 | Very slow, lazy |
/// | `snappy()` | 300 | 30 | Near-instant |
///
/// # Example
///
/// ```rust
/// use motus_spring::SpringConfig;
///
/// let cfg = SpringConfig::wobbly();
/// assert_eq!(cfg.stiffness, 180.0);
/// ```
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpringConfig {
    /// Restoring force. Higher = snappier. Default: `100.0`.
    pub stiffness: f32,
    /// Resistance to motion. Higher = less bouncy. Default: `10.0`.
    pub damping: f32,
    /// Mass of the simulated object. Default: `1.0`.
    pub mass: f32,
    /// Settle threshold — spring is considered at rest when both
    /// `|position - target| < epsilon` and `|velocity| < epsilon`. Default: `0.001`.
    pub epsilon: f32,
}

impl Default for SpringConfig {
    fn default() -> Self {
        Self {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
            epsilon: 0.001,
        }
    }
}

impl SpringConfig {
    /// Slow, soft spring — good for subtle UI elements.
    pub fn gentle() -> Self {
        Self {
            stiffness: 60.0,
            damping: 14.0,
            mass: 1.0,
            epsilon: 0.001,
        }
    }

    /// Bouncy, playful spring — great for icons and interactive elements.
    pub fn wobbly() -> Self {
        Self {
            stiffness: 180.0,
            damping: 12.0,
            mass: 1.0,
            epsilon: 0.001,
        }
    }

    /// Fast, firm spring — good for panels and drawers.
    pub fn stiff() -> Self {
        Self {
            stiffness: 210.0,
            damping: 20.0,
            mass: 1.0,
            epsilon: 0.001,
        }
    }

    /// Very slow, lazy spring — good for background animations.
    pub fn slow() -> Self {
        Self {
            stiffness: 37.0,
            damping: 14.0,
            mass: 1.0,
            epsilon: 0.001,
        }
    }

    /// Near-instant response — for time-critical feedback.
    pub fn snappy() -> Self {
        Self {
            stiffness: 300.0,
            damping: 30.0,
            mass: 1.0,
            epsilon: 0.001,
        }
    }
}
