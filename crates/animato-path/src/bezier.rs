//! Bezier curves, CatmullRom splines, and the [`PathEvaluate`] trait.

use crate::math;

#[cfg(any(feature = "std", feature = "alloc"))]
use alloc::vec::Vec;

const ARC_TABLE_SIZE: usize = 32;

/// Shared interface for evaluating a two-dimensional path.
///
/// Implementations interpret `t` as normalized distance along the path,
/// clamped to `[0.0, 1.0]`. This means `position(0.5)` is halfway by arc
/// length, not necessarily halfway by the curve's raw parameter.
pub trait PathEvaluate {
    /// Position at normalized path progress `t`.
    fn position(&self, t: f32) -> [f32; 2];

    /// Normalized direction at normalized path progress `t`.
    fn tangent(&self, t: f32) -> [f32; 2];

    /// Heading in degrees at normalized path progress `t`.
    fn rotation_deg(&self, t: f32) -> f32 {
        math::rotation_deg(self.tangent(t))
    }

    /// Approximate total path length in the same units as the path points.
    fn arc_length(&self) -> f32;
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct ArcTable {
    cumulative: [f32; ARC_TABLE_SIZE],
    length: f32,
}

impl ArcTable {
    fn build(mut sample: impl FnMut(f32) -> [f32; 2]) -> Self {
        let mut cumulative = [0.0_f32; ARC_TABLE_SIZE];
        let mut previous = sample(0.0);
        let mut length = 0.0;

        for (index, value) in cumulative.iter_mut().enumerate().skip(1) {
            let raw_t = index as f32 / (ARC_TABLE_SIZE - 1) as f32;
            let current = sample(raw_t);
            length += math::distance(previous, current);
            *value = length;
            previous = current;
        }

        Self { cumulative, length }
    }

    fn raw_t(&self, t: f32) -> f32 {
        let t = math::clamp01(t);
        if self.length <= f32::EPSILON {
            return t;
        }

        let target = self.length * t;
        if target <= 0.0 {
            return 0.0;
        }
        if target >= self.length {
            return 1.0;
        }

        let mut lo = 1_usize;
        let mut hi = ARC_TABLE_SIZE - 1;
        while lo < hi {
            let mid = (lo + hi) / 2;
            if self.cumulative[mid] < target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        let upper = lo;
        let lower = upper - 1;
        let lower_len = self.cumulative[lower];
        let upper_len = self.cumulative[upper];
        let span = (upper_len - lower_len).max(f32::EPSILON);
        let local = ((target - lower_len) / span).clamp(0.0, 1.0);
        (lower as f32 + local) / (ARC_TABLE_SIZE - 1) as f32
    }
}

/// A quadratic Bezier curve defined by start, control, and end points.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QuadBezier {
    start: [f32; 2],
    control: [f32; 2],
    end: [f32; 2],
    table: ArcTable,
}

impl QuadBezier {
    /// Create a quadratic Bezier curve.
    pub fn new(start: [f32; 2], control: [f32; 2], end: [f32; 2]) -> Self {
        let table = ArcTable::build(|t| Self::sample(start, control, end, t));
        Self {
            start,
            control,
            end,
            table,
        }
    }

    /// Start point.
    pub fn start(&self) -> [f32; 2] {
        self.start
    }

    /// Control point.
    pub fn control(&self) -> [f32; 2] {
        self.control
    }

    /// End point.
    pub fn end(&self) -> [f32; 2] {
        self.end
    }

    #[inline]
    fn sample(start: [f32; 2], control: [f32; 2], end: [f32; 2], t: f32) -> [f32; 2] {
        let t = math::clamp01(t);
        let a = math::lerp_point(start, control, t);
        let b = math::lerp_point(control, end, t);
        math::lerp_point(a, b, t)
    }

    #[inline]
    fn derivative(&self, raw_t: f32) -> [f32; 2] {
        let a = math::scale(math::sub(self.control, self.start), 2.0 * (1.0 - raw_t));
        let b = math::scale(math::sub(self.end, self.control), 2.0 * raw_t);
        math::add(a, b)
    }
}

impl PathEvaluate for QuadBezier {
    fn position(&self, t: f32) -> [f32; 2] {
        Self::sample(self.start, self.control, self.end, self.table.raw_t(t))
    }

    fn tangent(&self, t: f32) -> [f32; 2] {
        let raw_t = self.table.raw_t(t);
        let tangent = math::normalize(self.derivative(raw_t));
        if tangent == [0.0, 0.0] {
            math::normalize(math::sub(
                self.position((t + 0.001).min(1.0)),
                self.position(t),
            ))
        } else {
            tangent
        }
    }

    fn arc_length(&self) -> f32 {
        self.table.length
    }
}

/// A cubic Bezier curve defined by start, two control points, and end.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CubicBezierCurve {
    start: [f32; 2],
    control1: [f32; 2],
    control2: [f32; 2],
    end: [f32; 2],
    table: ArcTable,
}

impl CubicBezierCurve {
    /// Create a cubic Bezier curve.
    pub fn new(start: [f32; 2], control1: [f32; 2], control2: [f32; 2], end: [f32; 2]) -> Self {
        let table = ArcTable::build(|t| Self::sample(start, control1, control2, end, t));
        Self {
            start,
            control1,
            control2,
            end,
            table,
        }
    }

    /// Start point.
    pub fn start(&self) -> [f32; 2] {
        self.start
    }

    /// First control point.
    pub fn control1(&self) -> [f32; 2] {
        self.control1
    }

    /// Second control point.
    pub fn control2(&self) -> [f32; 2] {
        self.control2
    }

    /// End point.
    pub fn end(&self) -> [f32; 2] {
        self.end
    }

    #[inline]
    fn sample(
        start: [f32; 2],
        control1: [f32; 2],
        control2: [f32; 2],
        end: [f32; 2],
        t: f32,
    ) -> [f32; 2] {
        let t = math::clamp01(t);
        let a = math::lerp_point(start, control1, t);
        let b = math::lerp_point(control1, control2, t);
        let c = math::lerp_point(control2, end, t);
        let d = math::lerp_point(a, b, t);
        let e = math::lerp_point(b, c, t);
        math::lerp_point(d, e, t)
    }

    #[inline]
    fn derivative(&self, raw_t: f32) -> [f32; 2] {
        let mt = 1.0 - raw_t;
        let a = math::scale(math::sub(self.control1, self.start), 3.0 * mt * mt);
        let b = math::scale(math::sub(self.control2, self.control1), 6.0 * mt * raw_t);
        let c = math::scale(math::sub(self.end, self.control2), 3.0 * raw_t * raw_t);
        math::add(math::add(a, b), c)
    }
}

impl PathEvaluate for CubicBezierCurve {
    fn position(&self, t: f32) -> [f32; 2] {
        Self::sample(
            self.start,
            self.control1,
            self.control2,
            self.end,
            self.table.raw_t(t),
        )
    }

    fn tangent(&self, t: f32) -> [f32; 2] {
        let raw_t = self.table.raw_t(t);
        let tangent = math::normalize(self.derivative(raw_t));
        if tangent == [0.0, 0.0] {
            math::normalize(math::sub(
                self.position((t + 0.001).min(1.0)),
                self.position(t),
            ))
        } else {
            tangent
        }
    }

    fn arc_length(&self) -> f32 {
        self.table.length
    }
}

/// A smooth CatmullRom spline that interpolates its control points.
#[cfg(any(feature = "std", feature = "alloc"))]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CatmullRomSpline {
    points: Vec<[f32; 2]>,
    cumulative: Vec<f32>,
    length: f32,
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl CatmullRomSpline {
    /// Create a spline through the supplied control points.
    pub fn new(points: impl Into<Vec<[f32; 2]>>) -> Self {
        let points = points.into();
        let (cumulative, length) = Self::build_table(&points);
        Self {
            points,
            cumulative,
            length,
        }
    }

    /// Control points in input order.
    pub fn points(&self) -> &[[f32; 2]] {
        &self.points
    }

    fn build_table(points: &[[f32; 2]]) -> (Vec<f32>, f32) {
        if points.len() < 2 {
            return (alloc::vec![0.0], 0.0);
        }

        let sample_count = ((points.len() - 1) * 16 + 1).max(2);
        let mut cumulative = alloc::vec![0.0_f32; sample_count];
        let mut previous = Self::sample_raw(points, 0.0);
        let mut length = 0.0;

        for (index, value) in cumulative.iter_mut().enumerate().skip(1) {
            let raw_t = index as f32 / (sample_count - 1) as f32;
            let current = Self::sample_raw(points, raw_t);
            length += math::distance(previous, current);
            *value = length;
            previous = current;
        }

        (cumulative, length)
    }

    fn raw_t(&self, t: f32) -> f32 {
        let t = math::clamp01(t);
        if self.length <= f32::EPSILON || self.cumulative.len() < 2 {
            return t;
        }

        let target = self.length * t;
        if target <= 0.0 {
            return 0.0;
        }
        if target >= self.length {
            return 1.0;
        }

        let upper = self.cumulative.partition_point(|length| *length < target);
        let lower = upper.saturating_sub(1);
        let lower_len = self.cumulative[lower];
        let upper_len = self.cumulative[upper];
        let span = (upper_len - lower_len).max(f32::EPSILON);
        let local = ((target - lower_len) / span).clamp(0.0, 1.0);
        (lower as f32 + local) / (self.cumulative.len() - 1) as f32
    }

    fn sample_raw(points: &[[f32; 2]], raw_t: f32) -> [f32; 2] {
        match points.len() {
            0 => [0.0, 0.0],
            1 => points[0],
            2 => math::lerp_point(points[0], points[1], math::clamp01(raw_t)),
            len => {
                let segments = len - 1;
                let scaled = math::clamp01(raw_t) * segments as f32;
                let index = (scaled as usize).min(segments - 1);
                let t = scaled - index as f32;
                let t2 = t * t;
                let t3 = t2 * t;

                let p0 = if index == 0 {
                    points[0]
                } else {
                    points[index - 1]
                };
                let p1 = points[index];
                let p2 = points[index + 1];
                let p3 = if index + 2 < len {
                    points[index + 2]
                } else {
                    points[len - 1]
                };

                [
                    0.5 * (2.0 * p1[0]
                        + (-p0[0] + p2[0]) * t
                        + (2.0 * p0[0] - 5.0 * p1[0] + 4.0 * p2[0] - p3[0]) * t2
                        + (-p0[0] + 3.0 * p1[0] - 3.0 * p2[0] + p3[0]) * t3),
                    0.5 * (2.0 * p1[1]
                        + (-p0[1] + p2[1]) * t
                        + (2.0 * p0[1] - 5.0 * p1[1] + 4.0 * p2[1] - p3[1]) * t2
                        + (-p0[1] + 3.0 * p1[1] - 3.0 * p2[1] + p3[1]) * t3),
                ]
            }
        }
    }

    fn tangent_raw(points: &[[f32; 2]], raw_t: f32) -> [f32; 2] {
        if points.len() < 2 {
            return [0.0, 0.0];
        }
        if points.len() == 2 {
            return math::normalize(math::sub(points[1], points[0]));
        }

        let delta = 0.001;
        let before = Self::sample_raw(points, (raw_t - delta).max(0.0));
        let after = Self::sample_raw(points, (raw_t + delta).min(1.0));
        math::normalize(math::sub(after, before))
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl PathEvaluate for CatmullRomSpline {
    fn position(&self, t: f32) -> [f32; 2] {
        Self::sample_raw(&self.points, self.raw_t(t))
    }

    fn tangent(&self, t: f32) -> [f32; 2] {
        Self::tangent_raw(&self.points, self.raw_t(t))
    }

    fn arc_length(&self) -> f32 {
        self.length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quad_bezier_hits_endpoints() {
        let curve = QuadBezier::new([0.0, 0.0], [50.0, 100.0], [100.0, 0.0]);
        assert_eq!(curve.position(0.0), [0.0, 0.0]);
        assert_eq!(curve.position(1.0), [100.0, 0.0]);
    }

    #[test]
    fn cubic_bezier_hits_endpoints() {
        let curve = CubicBezierCurve::new([0.0, 0.0], [25.0, 50.0], [75.0, -50.0], [100.0, 0.0]);
        assert_eq!(curve.position(0.0), [0.0, 0.0]);
        assert_eq!(curve.position(1.0), [100.0, 0.0]);
    }

    #[test]
    fn straight_cubic_arc_length_matches_line_length() {
        let curve = CubicBezierCurve::new([0.0, 0.0], [33.0, 0.0], [66.0, 0.0], [100.0, 0.0]);
        assert!((curve.arc_length() - 100.0).abs() < 0.01);
        assert_eq!(curve.tangent(0.5), [1.0, 0.0]);
        assert_eq!(curve.rotation_deg(0.5), 0.0);
    }

    #[test]
    fn t_is_clamped() {
        let curve = QuadBezier::new([0.0, 0.0], [50.0, 0.0], [100.0, 0.0]);
        assert_eq!(curve.position(-10.0), [0.0, 0.0]);
        assert_eq!(curve.position(10.0), [100.0, 0.0]);
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn catmull_rom_interpolates_endpoints() {
        let spline = CatmullRomSpline::new(alloc::vec![[0.0, 0.0], [50.0, 100.0], [100.0, 0.0]]);
        assert_eq!(spline.position(0.0), [0.0, 0.0]);
        assert_eq!(spline.position(1.0), [100.0, 0.0]);
        assert!(spline.arc_length() > 100.0);
    }

    #[test]
    fn quad_accessors_and_degenerate_tangent_are_safe() {
        let curve = QuadBezier::new([1.0, 2.0], [1.0, 2.0], [1.0, 2.0]);

        assert_eq!(curve.start(), [1.0, 2.0]);
        assert_eq!(curve.control(), [1.0, 2.0]);
        assert_eq!(curve.end(), [1.0, 2.0]);
        assert_eq!(curve.position(0.5), [1.0, 2.0]);
        assert_eq!(curve.tangent(0.5), [0.0, 0.0]);
    }

    #[test]
    fn cubic_accessors_and_degenerate_tangent_are_safe() {
        let curve = CubicBezierCurve::new([2.0, 3.0], [2.0, 3.0], [2.0, 3.0], [2.0, 3.0]);

        assert_eq!(curve.start(), [2.0, 3.0]);
        assert_eq!(curve.control1(), [2.0, 3.0]);
        assert_eq!(curve.control2(), [2.0, 3.0]);
        assert_eq!(curve.end(), [2.0, 3.0]);
        assert_eq!(curve.position(0.5), [2.0, 3.0]);
        assert_eq!(curve.tangent(0.5), [0.0, 0.0]);
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn catmull_rom_handles_empty_single_and_two_point_inputs() {
        let empty = CatmullRomSpline::new(alloc::vec![]);
        assert!(empty.points().is_empty());
        assert_eq!(empty.position(0.5), [0.0, 0.0]);
        assert_eq!(empty.tangent(0.5), [0.0, 0.0]);

        let single = CatmullRomSpline::new(alloc::vec![[4.0, 5.0]]);
        assert_eq!(single.position(0.5), [4.0, 5.0]);
        assert_eq!(single.tangent(0.5), [0.0, 0.0]);

        let line = CatmullRomSpline::new(alloc::vec![[0.0, 0.0], [10.0, 0.0]]);
        assert_eq!(line.position(0.5), [5.0, 0.0]);
        assert_eq!(line.tangent(0.5), [1.0, 0.0]);
    }
}
