//! Advanced stagger delay patterns.

#[cfg(any(feature = "std", feature = "alloc"))]
use alloc::boxed::Box;
use core::fmt;

/// Origin used by grid-based stagger patterns.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GridOrigin {
    /// Top-left cell starts first.
    TopLeft,
    /// Top-right cell starts first.
    TopRight,
    /// Bottom-left cell starts first.
    BottomLeft,
    /// Bottom-right cell starts first.
    BottomRight,
    /// Center cell or center cells start first.
    Center,
    /// Top edge starts first.
    Top,
    /// Bottom edge starts first.
    Bottom,
    /// Left edge starts first.
    Left,
    /// Right edge starts first.
    Right,
}

/// Deterministic delay pattern for staggered animation starts.
pub enum StaggerPattern {
    /// 2D grid delay based on cell distance from an origin.
    Grid {
        /// Number of columns.
        cols: usize,
        /// Number of rows.
        rows: usize,
        /// Starting origin.
        origin: GridOrigin,
        /// Seconds per distance step.
        step: f32,
    },
    /// Deterministic random delay inside `[min_delay, max_delay]`.
    Random {
        /// Deterministic seed.
        seed: u32,
        /// Minimum seconds delay.
        min_delay: f32,
        /// Maximum seconds delay.
        max_delay: f32,
    },
    /// Start from the center index and move outward.
    CenterOut {
        /// Number of items.
        count: usize,
        /// Seconds per distance step.
        step: f32,
    },
    /// Start from both edges and move inward.
    EdgesIn {
        /// Number of items.
        count: usize,
        /// Seconds per distance step.
        step: f32,
    },
    /// User-defined delay function.
    #[cfg(any(feature = "std", feature = "alloc"))]
    Custom(
        /// Function receiving `(index, total)` and returning seconds delay.
        Box<dyn Fn(usize, usize) -> f32 + Send + Sync + 'static>,
    ),
}

impl fmt::Debug for StaggerPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Grid {
                cols,
                rows,
                origin,
                step,
            } => f
                .debug_struct("Grid")
                .field("cols", cols)
                .field("rows", rows)
                .field("origin", origin)
                .field("step", step)
                .finish(),
            Self::Random {
                seed,
                min_delay,
                max_delay,
            } => f
                .debug_struct("Random")
                .field("seed", seed)
                .field("min_delay", min_delay)
                .field("max_delay", max_delay)
                .finish(),
            Self::CenterOut { count, step } => f
                .debug_struct("CenterOut")
                .field("count", count)
                .field("step", step)
                .finish(),
            Self::EdgesIn { count, step } => f
                .debug_struct("EdgesIn")
                .field("count", count)
                .field("step", step)
                .finish(),
            #[cfg(any(feature = "std", feature = "alloc"))]
            Self::Custom(_) => f.debug_tuple("Custom").field(&"<function>").finish(),
        }
    }
}

impl StaggerPattern {
    /// Return the delay for `index` out of `total` items.
    pub fn delay(&self, index: usize, _total: usize) -> f32 {
        match self {
            Self::Grid {
                cols,
                rows,
                origin,
                step,
            } => grid_delay(*cols, *rows, *origin, *step, index),
            Self::Random {
                seed,
                min_delay,
                max_delay,
            } => random_delay(*seed, *min_delay, *max_delay, index),
            Self::CenterOut { count, step } => {
                center_distance(index.min(count.saturating_sub(1)), *count) as f32
                    * finite_or(*step, 0.0).max(0.0)
            }
            Self::EdgesIn { count, step } => {
                let count = *count;
                if count == 0 {
                    return 0.0;
                }
                let index = index.min(count - 1);
                index.min(count - 1 - index) as f32 * finite_or(*step, 0.0).max(0.0)
            }
            #[cfg(any(feature = "std", feature = "alloc"))]
            Self::Custom(callback) => finite_or(callback(index, _total), 0.0).max(0.0),
        }
    }
}

fn grid_delay(cols: usize, rows: usize, origin: GridOrigin, step: f32, index: usize) -> f32 {
    let cols = cols.max(1);
    let rows = rows.max(1);
    let index = index.min(cols.saturating_mul(rows).saturating_sub(1));
    let x = index % cols;
    let y = index / cols;
    let distance = match origin {
        GridOrigin::TopLeft => x + y,
        GridOrigin::TopRight => (cols - 1 - x) + y,
        GridOrigin::BottomLeft => x + (rows - 1 - y),
        GridOrigin::BottomRight => (cols - 1 - x) + (rows - 1 - y),
        GridOrigin::Center => {
            let left = x.abs_diff((cols - 1) / 2).min(x.abs_diff(cols / 2));
            let top = y.abs_diff((rows - 1) / 2).min(y.abs_diff(rows / 2));
            left + top
        }
        GridOrigin::Top => y,
        GridOrigin::Bottom => rows - 1 - y,
        GridOrigin::Left => x,
        GridOrigin::Right => cols - 1 - x,
    };
    distance as f32 * finite_or(step, 0.0).max(0.0)
}

fn center_distance(index: usize, count: usize) -> usize {
    if count <= 1 {
        return 0;
    }
    index
        .abs_diff((count - 1) / 2)
        .min(index.abs_diff(count / 2))
}

fn random_delay(seed: u32, min_delay: f32, max_delay: f32, index: usize) -> f32 {
    let min = finite_or(min_delay, 0.0).max(0.0);
    let max = finite_or(max_delay, min).max(min);
    let mut x = seed ^ (index as u32).wrapping_mul(0x9E37_79B9);
    x ^= x >> 16;
    x = x.wrapping_mul(0x7FEB_352D);
    x ^= x >> 15;
    x = x.wrapping_mul(0x846C_A68B);
    x ^= x >> 16;
    min + (max - min) * (x as f32 / u32::MAX as f32)
}

fn finite_or(value: f32, fallback: f32) -> f32 {
    if value.is_finite() { value } else { fallback }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_center_is_symmetric() {
        let pattern = StaggerPattern::Grid {
            cols: 3,
            rows: 3,
            origin: GridOrigin::Center,
            step: 0.1,
        };
        assert_eq!(pattern.delay(4, 9), 0.0);
        assert_eq!(pattern.delay(0, 9), pattern.delay(8, 9));
        assert_eq!(pattern.delay(2, 9), pattern.delay(6, 9));
    }

    #[test]
    fn random_is_deterministic_and_bounded() {
        let pattern = StaggerPattern::Random {
            seed: 9,
            min_delay: 0.2,
            max_delay: 0.5,
        };
        let delay = pattern.delay(3, 10);
        assert_eq!(delay, pattern.delay(3, 10));
        assert!((0.2..=0.5).contains(&delay));
    }

    #[test]
    fn center_out_and_edges_in_order() {
        let center = StaggerPattern::CenterOut {
            count: 5,
            step: 0.1,
        };
        assert_eq!(center.delay(2, 5), 0.0);
        assert_eq!(center.delay(0, 5), 0.2);

        let edges = StaggerPattern::EdgesIn {
            count: 5,
            step: 0.1,
        };
        assert_eq!(edges.delay(0, 5), 0.0);
        assert_eq!(edges.delay(4, 5), 0.0);
        assert_eq!(edges.delay(2, 5), 0.2);
    }
}
