//! Value modifier free functions for post-processing tween output.

use animato_core::math::{powi, round};

/// Snap a value to the nearest multiple of `grid`.
///
/// # Example
///
/// ```rust
/// use animato_tween::snap_to;
/// assert_eq!(snap_to(13.4_f32, 5.0), 15.0);
/// assert_eq!(snap_to(12.0_f32, 5.0), 10.0);
/// ```
#[inline]
pub fn snap_to(value: f32, grid: f32) -> f32 {
    if grid == 0.0 {
        return value;
    }
    round(value / grid) * grid
}

/// Round a value to a given number of decimal places.
///
/// # Example
///
/// ```rust
/// use animato_tween::round_to;
/// assert_eq!(round_to(3.14159_f32, 2), 3.14);
/// ```
#[inline]
pub fn round_to(value: f32, decimals: u32) -> f32 {
    let factor = powi(10.0_f32, decimals as i32);
    round(value * factor) / factor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snap_to_grid() {
        assert_eq!(snap_to(13.4, 5.0), 15.0);
        assert_eq!(snap_to(12.0, 5.0), 10.0);
        assert_eq!(snap_to(7.5, 5.0), 10.0);
        assert_eq!(snap_to(0.0, 5.0), 0.0);
    }

    #[test]
    fn snap_to_zero_grid_noop() {
        assert_eq!(snap_to(13.4, 0.0), 13.4);
    }

    #[test]
    fn round_to_decimals() {
        assert_eq!(round_to(3.14159, 2), 3.14);
        assert_eq!(round_to(3.14159, 0), 3.0);
        assert_eq!(round_to(3.145, 2), 3.15);
    }
}
