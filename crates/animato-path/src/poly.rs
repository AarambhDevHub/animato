//! Polyline, compound path, and path command types.

use crate::bezier::{CatmullRomSpline, CubicBezierCurve, PathEvaluate, QuadBezier};
use crate::math;
use alloc::vec::Vec;
use core::f32::consts::PI;

const ARC_TABLE_SIZE: usize = 32;

/// A straight line segment.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LineSegment {
    start: [f32; 2],
    end: [f32; 2],
    length: f32,
}

impl LineSegment {
    /// Create a line segment from `start` to `end`.
    pub fn new(start: [f32; 2], end: [f32; 2]) -> Self {
        Self {
            start,
            end,
            length: math::distance(start, end),
        }
    }

    /// Start point.
    pub fn start(&self) -> [f32; 2] {
        self.start
    }

    /// End point.
    pub fn end(&self) -> [f32; 2] {
        self.end
    }
}

impl PathEvaluate for LineSegment {
    fn position(&self, t: f32) -> [f32; 2] {
        math::lerp_point(self.start, self.end, math::clamp01(t))
    }

    fn tangent(&self, _t: f32) -> [f32; 2] {
        math::normalize(math::sub(self.end, self.start))
    }

    fn arc_length(&self) -> f32 {
        self.length
    }
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

/// An SVG-compatible elliptical arc segment.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct EllipticalArc {
    start: [f32; 2],
    end: [f32; 2],
    center: [f32; 2],
    radii: [f32; 2],
    x_axis_rotation: f32,
    start_angle: f32,
    delta_angle: f32,
    line_fallback: bool,
    table: ArcTable,
}

impl EllipticalArc {
    /// Create an SVG endpoint-parameterized elliptical arc.
    pub fn from_svg(
        start: [f32; 2],
        radii: [f32; 2],
        x_axis_rotation: f32,
        large_arc: bool,
        sweep: bool,
        end: [f32; 2],
    ) -> Self {
        let mut rx = radii[0].abs();
        let mut ry = radii[1].abs();
        if math::distance(start, end) <= f32::EPSILON || rx <= f32::EPSILON || ry <= f32::EPSILON {
            return Self::line_fallback(start, end, x_axis_rotation);
        }

        let phi = math::deg_to_rad(x_axis_rotation);
        let cos_phi = math::cos(phi);
        let sin_phi = math::sin(phi);
        let dx = (start[0] - end[0]) * 0.5;
        let dy = (start[1] - end[1]) * 0.5;
        let x1p = cos_phi * dx + sin_phi * dy;
        let y1p = -sin_phi * dx + cos_phi * dy;

        let lambda = (x1p * x1p) / (rx * rx) + (y1p * y1p) / (ry * ry);
        if lambda > 1.0 {
            let scale = math::sqrt(lambda);
            rx *= scale;
            ry *= scale;
        }

        let rx2 = rx * rx;
        let ry2 = ry * ry;
        let x1p2 = x1p * x1p;
        let y1p2 = y1p * y1p;
        let denom = rx2 * y1p2 + ry2 * x1p2;
        let sign = if large_arc == sweep { -1.0 } else { 1.0 };
        let coef = if denom <= f32::EPSILON {
            0.0
        } else {
            sign * math::sqrt(((rx2 * ry2 - rx2 * y1p2 - ry2 * x1p2) / denom).max(0.0))
        };

        let cxp = coef * (rx * y1p / ry);
        let cyp = coef * (-ry * x1p / rx);
        let center = [
            cos_phi * cxp - sin_phi * cyp + (start[0] + end[0]) * 0.5,
            sin_phi * cxp + cos_phi * cyp + (start[1] + end[1]) * 0.5,
        ];

        let v1 = [(x1p - cxp) / rx, (y1p - cyp) / ry];
        let v2 = [(-x1p - cxp) / rx, (-y1p - cyp) / ry];
        let start_angle = math::angle_between([1.0, 0.0], v1);
        let mut delta_angle = math::angle_between(v1, v2);

        if !sweep && delta_angle > 0.0 {
            delta_angle -= PI * 2.0;
        } else if sweep && delta_angle < 0.0 {
            delta_angle += PI * 2.0;
        }

        let table =
            ArcTable::build(|t| Self::sample(center, [rx, ry], phi, start_angle, delta_angle, t));

        Self {
            start,
            end,
            center,
            radii: [rx, ry],
            x_axis_rotation,
            start_angle,
            delta_angle,
            line_fallback: false,
            table,
        }
    }

    /// Start point.
    pub fn start(&self) -> [f32; 2] {
        self.start
    }

    /// End point.
    pub fn end(&self) -> [f32; 2] {
        self.end
    }

    /// Center point after SVG endpoint conversion.
    pub fn center(&self) -> [f32; 2] {
        self.center
    }

    /// Effective radii after SVG normalization.
    pub fn radii(&self) -> [f32; 2] {
        self.radii
    }

    fn line_fallback(start: [f32; 2], end: [f32; 2], x_axis_rotation: f32) -> Self {
        let line = LineSegment::new(start, end);
        let table = ArcTable {
            cumulative: {
                let mut cumulative = [0.0_f32; ARC_TABLE_SIZE];
                for (index, value) in cumulative.iter_mut().enumerate() {
                    *value = line.arc_length() * index as f32 / (ARC_TABLE_SIZE - 1) as f32;
                }
                cumulative
            },
            length: line.arc_length(),
        };
        Self {
            start,
            end,
            center: [0.0, 0.0],
            radii: [0.0, 0.0],
            x_axis_rotation,
            start_angle: 0.0,
            delta_angle: 0.0,
            line_fallback: true,
            table,
        }
    }

    fn sample(
        center: [f32; 2],
        radii: [f32; 2],
        phi: f32,
        start_angle: f32,
        delta_angle: f32,
        raw_t: f32,
    ) -> [f32; 2] {
        let angle = start_angle + delta_angle * math::clamp01(raw_t);
        let cos_phi = math::cos(phi);
        let sin_phi = math::sin(phi);
        let cos_angle = math::cos(angle);
        let sin_angle = math::sin(angle);
        [
            center[0] + cos_phi * radii[0] * cos_angle - sin_phi * radii[1] * sin_angle,
            center[1] + sin_phi * radii[0] * cos_angle + cos_phi * radii[1] * sin_angle,
        ]
    }
}

impl PathEvaluate for EllipticalArc {
    fn position(&self, t: f32) -> [f32; 2] {
        if self.line_fallback {
            return LineSegment::new(self.start, self.end).position(t);
        }
        Self::sample(
            self.center,
            self.radii,
            math::deg_to_rad(self.x_axis_rotation),
            self.start_angle,
            self.delta_angle,
            self.table.raw_t(t),
        )
    }

    fn tangent(&self, t: f32) -> [f32; 2] {
        if self.line_fallback {
            return LineSegment::new(self.start, self.end).tangent(t);
        }

        let raw_t = self.table.raw_t(t);
        let angle = self.start_angle + self.delta_angle * raw_t;
        let phi = math::deg_to_rad(self.x_axis_rotation);
        let cos_phi = math::cos(phi);
        let sin_phi = math::sin(phi);
        let cos_angle = math::cos(angle);
        let sin_angle = math::sin(angle);
        math::normalize([
            self.delta_angle
                * (-cos_phi * self.radii[0] * sin_angle - sin_phi * self.radii[1] * cos_angle),
            self.delta_angle
                * (-sin_phi * self.radii[0] * sin_angle + cos_phi * self.radii[1] * cos_angle),
        ])
    }

    fn arc_length(&self) -> f32 {
        self.table.length
    }
}

/// A heterogeneous segment in a [`CompoundPath`].
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PathSegment {
    /// Straight line segment.
    Line(LineSegment),
    /// Quadratic Bezier segment.
    Quad(QuadBezier),
    /// Cubic Bezier segment.
    Cubic(CubicBezierCurve),
    /// Elliptical arc segment.
    Arc(EllipticalArc),
}

impl PathEvaluate for PathSegment {
    fn position(&self, t: f32) -> [f32; 2] {
        match self {
            PathSegment::Line(segment) => segment.position(t),
            PathSegment::Quad(segment) => segment.position(t),
            PathSegment::Cubic(segment) => segment.position(t),
            PathSegment::Arc(segment) => segment.position(t),
        }
    }

    fn tangent(&self, t: f32) -> [f32; 2] {
        match self {
            PathSegment::Line(segment) => segment.tangent(t),
            PathSegment::Quad(segment) => segment.tangent(t),
            PathSegment::Cubic(segment) => segment.tangent(t),
            PathSegment::Arc(segment) => segment.tangent(t),
        }
    }

    fn arc_length(&self) -> f32 {
        match self {
            PathSegment::Line(segment) => segment.arc_length(),
            PathSegment::Quad(segment) => segment.arc_length(),
            PathSegment::Cubic(segment) => segment.arc_length(),
            PathSegment::Arc(segment) => segment.arc_length(),
        }
    }
}

/// Canonical path command used by SVG parsing and compound path construction.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PathCommand {
    /// Move the current point without drawing.
    MoveTo([f32; 2]),
    /// Draw a line to the given point.
    LineTo([f32; 2]),
    /// Draw a quadratic Bezier to `end`.
    QuadTo {
        /// Control point.
        control: [f32; 2],
        /// End point.
        end: [f32; 2],
    },
    /// Draw a cubic Bezier to `end`.
    CubicTo {
        /// First control point.
        control1: [f32; 2],
        /// Second control point.
        control2: [f32; 2],
        /// End point.
        end: [f32; 2],
    },
    /// Draw an SVG elliptical arc to `end`.
    ArcTo {
        /// Arc radii.
        radii: [f32; 2],
        /// X-axis rotation in degrees.
        x_axis_rotation: f32,
        /// SVG large-arc flag.
        large_arc: bool,
        /// SVG sweep flag.
        sweep: bool,
        /// End point.
        end: [f32; 2],
    },
    /// Close the current subpath.
    ClosePath,
}

/// A sequence of line, Bezier, and arc segments with arc-length evaluation.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CompoundPath {
    segments: Vec<PathSegment>,
    cumulative: Vec<f32>,
    length: f32,
}

impl CompoundPath {
    /// Create an empty compound path.
    pub fn new() -> Self {
        Self::default()
    }

    /// Build a path from canonical commands.
    pub fn from_commands(commands: &[PathCommand]) -> Self {
        let mut path = Self::new();
        let mut current = [0.0, 0.0];
        let mut subpath_start = [0.0, 0.0];
        let mut has_current = false;

        for command in commands {
            match *command {
                PathCommand::MoveTo(point) => {
                    current = point;
                    subpath_start = point;
                    has_current = true;
                }
                PathCommand::LineTo(end) => {
                    if has_current {
                        path.push_segment_mut(PathSegment::Line(LineSegment::new(current, end)));
                    }
                    current = end;
                    has_current = true;
                }
                PathCommand::QuadTo { control, end } => {
                    if has_current {
                        path.push_segment_mut(PathSegment::Quad(QuadBezier::new(
                            current, control, end,
                        )));
                    }
                    current = end;
                    has_current = true;
                }
                PathCommand::CubicTo {
                    control1,
                    control2,
                    end,
                } => {
                    if has_current {
                        path.push_segment_mut(PathSegment::Cubic(CubicBezierCurve::new(
                            current, control1, control2, end,
                        )));
                    }
                    current = end;
                    has_current = true;
                }
                PathCommand::ArcTo {
                    radii,
                    x_axis_rotation,
                    large_arc,
                    sweep,
                    end,
                } => {
                    if has_current {
                        path.push_segment_mut(PathSegment::Arc(EllipticalArc::from_svg(
                            current,
                            radii,
                            x_axis_rotation,
                            large_arc,
                            sweep,
                            end,
                        )));
                    }
                    current = end;
                    has_current = true;
                }
                PathCommand::ClosePath => {
                    if has_current {
                        path.push_segment_mut(PathSegment::Line(LineSegment::new(
                            current,
                            subpath_start,
                        )));
                        current = subpath_start;
                    }
                }
            }
        }

        path
    }

    /// Parse an SVG `d` attribute into a compound path.
    pub fn from_svg(d: &str) -> Self {
        Self::from_commands(&crate::svg::SvgPathParser::parse(d))
    }

    /// Parse an SVG `d` attribute into a compound path with error reporting.
    pub fn try_from_svg(d: &str) -> Result<Self, crate::svg::SvgPathError> {
        Ok(Self::from_commands(&crate::svg::SvgPathParser::try_parse(
            d,
        )?))
    }

    /// Append a segment and return the path.
    pub fn push_segment(mut self, segment: PathSegment) -> Self {
        self.push_segment_mut(segment);
        self
    }

    /// Append a line segment and return the path.
    pub fn line_to(self, start: [f32; 2], end: [f32; 2]) -> Self {
        self.push_segment(PathSegment::Line(LineSegment::new(start, end)))
    }

    /// Append a quadratic Bezier segment and return the path.
    pub fn quad_to(self, start: [f32; 2], control: [f32; 2], end: [f32; 2]) -> Self {
        self.push_segment(PathSegment::Quad(QuadBezier::new(start, control, end)))
    }

    /// Append a cubic Bezier segment and return the path.
    pub fn cubic_to(
        self,
        start: [f32; 2],
        control1: [f32; 2],
        control2: [f32; 2],
        end: [f32; 2],
    ) -> Self {
        self.push_segment(PathSegment::Cubic(CubicBezierCurve::new(
            start, control1, control2, end,
        )))
    }

    /// Append an SVG arc segment and return the path.
    pub fn arc_to(
        self,
        start: [f32; 2],
        radii: [f32; 2],
        x_axis_rotation: f32,
        large_arc: bool,
        sweep: bool,
        end: [f32; 2],
    ) -> Self {
        self.push_segment(PathSegment::Arc(EllipticalArc::from_svg(
            start,
            radii,
            x_axis_rotation,
            large_arc,
            sweep,
            end,
        )))
    }

    /// Segments in drawing order.
    pub fn segments(&self) -> &[PathSegment] {
        &self.segments
    }

    /// Number of drawable segments.
    pub fn len(&self) -> usize {
        self.segments.len()
    }

    /// `true` when the path has no drawable segments.
    pub fn is_empty(&self) -> bool {
        self.segments.is_empty()
    }

    fn push_segment_mut(&mut self, segment: PathSegment) {
        self.length += segment.arc_length();
        self.cumulative.push(self.length);
        self.segments.push(segment);
    }

    fn segment_at(&self, t: f32) -> Option<(&PathSegment, f32)> {
        if self.segments.is_empty() {
            return None;
        }
        if self.length <= f32::EPSILON {
            return self.segments.last().map(|segment| (segment, 1.0));
        }

        let target = self.length * math::clamp01(t);
        if target <= 0.0 {
            return self.segments.first().map(|segment| (segment, 0.0));
        }
        if target >= self.length {
            return self.segments.last().map(|segment| (segment, 1.0));
        }

        let index = self.cumulative.partition_point(|length| *length < target);
        let previous = if index == 0 {
            0.0
        } else {
            self.cumulative[index - 1]
        };
        let segment_length = (self.cumulative[index] - previous).max(f32::EPSILON);
        let local_t = ((target - previous) / segment_length).clamp(0.0, 1.0);
        Some((&self.segments[index], local_t))
    }
}

impl PathEvaluate for CompoundPath {
    fn position(&self, t: f32) -> [f32; 2] {
        self.segment_at(t)
            .map_or([0.0, 0.0], |(segment, local_t)| segment.position(local_t))
    }

    fn tangent(&self, t: f32) -> [f32; 2] {
        self.segment_at(t)
            .map_or([0.0, 0.0], |(segment, local_t)| segment.tangent(local_t))
    }

    fn arc_length(&self) -> f32 {
        self.length
    }
}

/// Smooth path through arbitrary points using a CatmullRom spline.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PolyPath {
    spline: CatmullRomSpline,
}

impl PolyPath {
    /// Create a smooth path through `points`.
    pub fn new(points: impl Into<Vec<[f32; 2]>>) -> Self {
        Self {
            spline: CatmullRomSpline::new(points),
        }
    }

    /// Input points.
    pub fn points(&self) -> &[[f32; 2]] {
        self.spline.points()
    }
}

impl PathEvaluate for PolyPath {
    fn position(&self, t: f32) -> [f32; 2] {
        self.spline.position(t)
    }

    fn tangent(&self, t: f32) -> [f32; 2] {
        self.spline.tangent(t)
    }

    fn arc_length(&self) -> f32 {
        self.spline.arc_length()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_segment_evaluates() {
        let line = LineSegment::new([0.0, 0.0], [100.0, 0.0]);
        assert_eq!(line.position(0.5), [50.0, 0.0]);
        assert_eq!(line.tangent(0.5), [1.0, 0.0]);
        assert_eq!(line.arc_length(), 100.0);
    }

    #[test]
    fn compound_path_uses_arc_length_across_segments() {
        let path = CompoundPath::new()
            .line_to([0.0, 0.0], [100.0, 0.0])
            .line_to([100.0, 0.0], [100.0, 100.0]);

        assert_eq!(path.position(0.25), [50.0, 0.0]);
        assert_eq!(path.position(0.75), [100.0, 50.0]);
        assert_eq!(path.arc_length(), 200.0);
    }

    #[test]
    fn commands_build_compound_path() {
        let commands = [
            PathCommand::MoveTo([0.0, 0.0]),
            PathCommand::LineTo([100.0, 0.0]),
            PathCommand::ClosePath,
        ];
        let path = CompoundPath::from_commands(&commands);
        assert_eq!(path.len(), 2);
        assert_eq!(path.position(1.0), [0.0, 0.0]);
    }

    #[test]
    fn arc_hits_endpoints() {
        let arc = EllipticalArc::from_svg([0.0, 0.0], [50.0, 50.0], 0.0, false, true, [100.0, 0.0]);
        assert!((arc.position(0.0)[0] - 0.0).abs() < 0.01);
        assert!((arc.position(1.0)[0] - 100.0).abs() < 0.01);
        assert!(arc.arc_length() > 100.0);
    }

    #[test]
    fn poly_path_is_smooth_spline() {
        let path = PolyPath::new(alloc::vec![[0.0, 0.0], [50.0, 100.0], [100.0, 0.0]]);
        assert_eq!(path.position(0.0), [0.0, 0.0]);
        assert_eq!(path.position(1.0), [100.0, 0.0]);
    }
}
