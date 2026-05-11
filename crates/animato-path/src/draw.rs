//! SVG stroke-dashoffset animation helpers.
//!
//! The [`DrawSvg`] trait is automatically implemented for every type that
//! implements [`PathEvaluate`], providing `draw_on` and `draw_on_reverse`
//! methods for CSS stroke-dash animation.

use crate::bezier::PathEvaluate;

/// CSS stroke-dash values for animating SVG path drawing.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DrawValues {
    /// Value for `stroke-dasharray` in SVG user units.
    pub dash_array: f32,
    /// Value for `stroke-dashoffset` in SVG user units.
    pub dash_offset: f32,
}

impl DrawValues {
    /// Normalised draw progress derived from these values.
    pub fn progress(&self) -> f32 {
        if self.dash_array <= f32::EPSILON {
            return 1.0;
        }
        (1.0 - self.dash_offset / self.dash_array).clamp(0.0, 1.0)
    }

    /// Format as a CSS inline style fragment.
    ///
    /// ```text
    /// stroke-dasharray: 314.159; stroke-dashoffset: 157.080
    /// ```
    #[cfg(any(feature = "std", feature = "alloc"))]
    pub fn to_css(&self) -> alloc::string::String {
        alloc::format!(
            "stroke-dasharray: {:.3}; stroke-dashoffset: {:.3}",
            self.dash_array,
            self.dash_offset
        )
    }
}

/// Trait for animating SVG path drawing via `stroke-dashoffset`.
///
/// Automatically implemented for every type that implements [`PathEvaluate`].
///
/// # Example
///
/// ```rust,ignore
/// use animato_path::{CubicBezierCurve, DrawSvg};
///
/// let path = CubicBezierCurve::new([0.0, 0.0], [50.0, 100.0], [150.0, -100.0], [200.0, 0.0]);
/// let values = path.draw_on(0.5); // 50% drawn
/// println!("dasharray={} dashoffset={}", values.dash_array, values.dash_offset);
/// ```
pub trait DrawSvg {
    /// Total arc length of the drawable path.
    fn total_length(&self) -> f32;

    /// Compute `stroke-dasharray` / `stroke-dashoffset` values to draw the
    /// path forward from the start to `progress` ∈ `[0.0, 1.0]`.
    ///
    /// At `0.0` the path is invisible; at `1.0` it is fully drawn.
    fn draw_on(&self, progress: f32) -> DrawValues {
        let p = progress.clamp(0.0, 1.0);
        let length = self.total_length();
        DrawValues {
            dash_array: length,
            dash_offset: length * (1.0 - p),
        }
    }

    /// Compute values for erasing the path from the end back toward the start.
    ///
    /// At `0.0` the path is fully drawn; at `1.0` it is invisible.
    fn draw_on_reverse(&self, progress: f32) -> DrawValues {
        let p = progress.clamp(0.0, 1.0);
        let length = self.total_length();
        DrawValues {
            dash_array: length,
            dash_offset: length * p,
        }
    }
}

/// Blanket implementation: every [`PathEvaluate`] type is also [`DrawSvg`].
impl<T: PathEvaluate> DrawSvg for T {
    #[inline]
    fn total_length(&self) -> f32 {
        self.arc_length()
    }
}

#[cfg(all(test, any(feature = "std", feature = "alloc")))]
mod tests {
    use super::*;
    use crate::poly::LineSegment;

    #[test]
    fn draw_on_zero_is_invisible() {
        let line = LineSegment::new([0.0, 0.0], [100.0, 0.0]);
        let v = line.draw_on(0.0);
        assert_eq!(v.dash_array, 100.0);
        assert_eq!(v.dash_offset, 100.0);
        assert_eq!(v.progress(), 0.0);
    }

    #[test]
    fn draw_on_one_is_fully_visible() {
        let line = LineSegment::new([0.0, 0.0], [100.0, 0.0]);
        let v = line.draw_on(1.0);
        assert_eq!(v.dash_offset, 0.0);
        assert_eq!(v.progress(), 1.0);
    }

    #[test]
    fn draw_on_half() {
        let line = LineSegment::new([0.0, 0.0], [100.0, 0.0]);
        let v = line.draw_on(0.5);
        assert!((v.dash_array - 100.0).abs() < 0.001);
        assert!((v.dash_offset - 50.0).abs() < 0.001);
        assert!((v.progress() - 0.5).abs() < 0.001);
    }

    #[test]
    fn draw_on_reverse_inverts_progress() {
        let line = LineSegment::new([0.0, 0.0], [200.0, 0.0]);
        let fwd = line.draw_on(0.3);
        let rev = line.draw_on_reverse(0.7);
        // Both should expose 30% of the path.
        assert!((fwd.dash_offset - rev.dash_offset).abs() < 0.001);
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn to_css_formats_correctly() {
        let v = DrawValues {
            dash_array: 314.159,
            dash_offset: 157.080,
        };
        let css = v.to_css();
        assert!(css.contains("stroke-dasharray"));
        assert!(css.contains("stroke-dashoffset"));
    }
}
