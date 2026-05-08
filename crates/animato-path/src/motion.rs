//! Motion paths and tween-driven path animation.

use crate::bezier::{CubicBezierCurve, PathEvaluate, QuadBezier};
use crate::math;
use crate::poly::{CompoundPath, EllipticalArc, LineSegment, PathCommand, PathSegment};
use animato_core::{Easing, Playable, Update};
use animato_tween::{Loop, Tween};

/// A unified motion path built from one or more drawable path segments.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MotionPath {
    inner: CompoundPath,
}

impl MotionPath {
    /// Create an empty motion path.
    pub fn new() -> Self {
        Self::default()
    }

    /// Build a motion path from canonical path commands.
    pub fn from_commands(commands: &[PathCommand]) -> Self {
        Self {
            inner: CompoundPath::from_commands(commands),
        }
    }

    /// Parse an SVG `d` attribute into a motion path.
    pub fn from_svg(d: &str) -> Self {
        Self {
            inner: CompoundPath::from_svg(d),
        }
    }

    /// Parse an SVG `d` attribute into a motion path with error reporting.
    pub fn try_from_svg(d: &str) -> Result<Self, crate::svg::SvgPathError> {
        Ok(Self {
            inner: CompoundPath::try_from_svg(d)?,
        })
    }

    /// Append a path segment and return the path.
    pub fn push_segment(mut self, segment: PathSegment) -> Self {
        self.inner = self.inner.push_segment(segment);
        self
    }

    /// Segments in drawing order.
    pub fn segments(&self) -> &[PathSegment] {
        self.inner.segments()
    }

    /// Number of drawable segments.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// `true` when the path has no drawable segments.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl From<CompoundPath> for MotionPath {
    fn from(inner: CompoundPath) -> Self {
        Self { inner }
    }
}

impl From<LineSegment> for MotionPath {
    fn from(segment: LineSegment) -> Self {
        Self {
            inner: CompoundPath::new().push_segment(PathSegment::Line(segment)),
        }
    }
}

impl From<QuadBezier> for MotionPath {
    fn from(segment: QuadBezier) -> Self {
        Self {
            inner: CompoundPath::new().push_segment(PathSegment::Quad(segment)),
        }
    }
}

impl From<CubicBezierCurve> for MotionPath {
    fn from(segment: CubicBezierCurve) -> Self {
        Self {
            inner: CompoundPath::new().push_segment(PathSegment::Cubic(segment)),
        }
    }
}

impl From<EllipticalArc> for MotionPath {
    fn from(segment: EllipticalArc) -> Self {
        Self {
            inner: CompoundPath::new().push_segment(PathSegment::Arc(segment)),
        }
    }
}

impl From<PathSegment> for MotionPath {
    fn from(segment: PathSegment) -> Self {
        Self {
            inner: CompoundPath::new().push_segment(segment),
        }
    }
}

impl PathEvaluate for MotionPath {
    fn position(&self, t: f32) -> [f32; 2] {
        self.inner.position(t)
    }

    fn tangent(&self, t: f32) -> [f32; 2] {
        self.inner.tangent(t)
    }

    fn arc_length(&self) -> f32 {
        self.inner.arc_length()
    }
}

/// Builder for [`MotionPathTween`].
#[derive(Clone, Debug)]
pub struct MotionPathTweenBuilder {
    path: MotionPath,
    duration: f32,
    easing: Easing,
    delay: f32,
    time_scale: f32,
    looping: Loop,
    auto_rotate: bool,
    start_offset: f32,
    end_offset: f32,
}

impl MotionPathTweenBuilder {
    /// Create a builder from a motion path.
    pub fn new(path: impl Into<MotionPath>) -> Self {
        Self {
            path: path.into(),
            duration: 1.0,
            easing: Easing::Linear,
            delay: 0.0,
            time_scale: 1.0,
            looping: Loop::Once,
            auto_rotate: false,
            start_offset: 0.0,
            end_offset: 1.0,
        }
    }

    /// Set the animation duration in seconds.
    pub fn duration(mut self, secs: f32) -> Self {
        self.duration = secs.max(0.0);
        self
    }

    /// Set the easing curve for progress along the path.
    pub fn easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }

    /// Set delay before motion begins.
    pub fn delay(mut self, secs: f32) -> Self {
        self.delay = secs.max(0.0);
        self
    }

    /// Set the time-scale multiplier.
    pub fn time_scale(mut self, scale: f32) -> Self {
        self.time_scale = scale.max(0.0);
        self
    }

    /// Set looping behavior.
    pub fn looping(mut self, mode: Loop) -> Self {
        self.looping = mode;
        self
    }

    /// Enable or disable auto-rotation.
    pub fn auto_rotate(mut self, yes: bool) -> Self {
        self.auto_rotate = yes;
        self
    }

    /// Set the normalized start offset along the path.
    pub fn start_offset(mut self, offset: f32) -> Self {
        self.start_offset = math::clamp01(offset);
        self
    }

    /// Set the normalized end offset along the path.
    pub fn end_offset(mut self, offset: f32) -> Self {
        self.end_offset = math::clamp01(offset);
        self
    }

    /// Build the configured motion path tween.
    pub fn build(self) -> MotionPathTween {
        let tween = Tween::new(0.0_f32, 1.0)
            .duration(self.duration)
            .easing(self.easing)
            .delay(self.delay)
            .time_scale(self.time_scale)
            .looping(self.looping)
            .build();
        MotionPathTween {
            path: self.path,
            tween,
            auto_rotate: self.auto_rotate,
            start_offset: self.start_offset,
            end_offset: self.end_offset,
        }
    }
}

/// Tween-driven animation along a [`MotionPath`].
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MotionPathTween {
    path: MotionPath,
    tween: Tween<f32>,
    auto_rotate: bool,
    start_offset: f32,
    end_offset: f32,
}

impl MotionPathTween {
    /// Start building a motion tween for `path`.
    #[allow(clippy::new_ret_no_self)]
    pub fn new(path: impl Into<MotionPath>) -> MotionPathTweenBuilder {
        MotionPathTweenBuilder::new(path)
    }

    /// Create a motion tween from an existing progress tween.
    pub fn from_tween(path: impl Into<MotionPath>, tween: Tween<f32>) -> Self {
        Self {
            path: path.into(),
            tween,
            auto_rotate: false,
            start_offset: 0.0,
            end_offset: 1.0,
        }
    }

    /// The underlying motion path.
    pub fn path(&self) -> &MotionPath {
        &self.path
    }

    /// The internal progress tween.
    pub fn tween(&self) -> &Tween<f32> {
        &self.tween
    }

    /// Mutable access to the internal progress tween.
    pub fn tween_mut(&mut self) -> &mut Tween<f32> {
        &mut self.tween
    }

    /// Current position on the path.
    pub fn value(&self) -> [f32; 2] {
        self.path.position(self.path_t())
    }

    /// Current heading in degrees when auto-rotation is enabled.
    ///
    /// Returns `0.0` when auto-rotation is disabled.
    pub fn rotation_deg(&self) -> f32 {
        if self.auto_rotate {
            self.path.rotation_deg(self.path_t())
        } else {
            0.0
        }
    }

    /// Current normalized path progress after offsets are applied.
    pub fn path_progress(&self) -> f32 {
        self.path_t()
    }

    /// `true` when auto-rotation is enabled.
    pub fn is_auto_rotate(&self) -> bool {
        self.auto_rotate
    }

    /// `true` when the internal tween is complete.
    pub fn is_complete(&self) -> bool {
        self.tween.is_complete()
    }

    /// Reset the internal tween to the beginning.
    pub fn reset(&mut self) {
        self.tween.reset();
    }

    /// Seek the internal tween to normalized progress.
    pub fn seek(&mut self, t: f32) {
        self.tween.seek(t);
    }

    fn path_t(&self) -> f32 {
        let progress = math::clamp01(self.tween.value());
        math::clamp01(self.start_offset + (self.end_offset - self.start_offset) * progress)
    }
}

impl Update for MotionPathTween {
    fn update(&mut self, dt: f32) -> bool {
        self.tween.update(dt)
    }
}

impl Playable for MotionPathTween {
    fn duration(&self) -> f32 {
        Playable::duration(&self.tween)
    }

    fn reset(&mut self) {
        MotionPathTween::reset(self);
    }

    fn seek_to(&mut self, progress: f32) {
        Playable::seek_to(&mut self.tween, progress);
    }

    fn is_complete(&self) -> bool {
        MotionPathTween::is_complete(self)
    }

    fn as_any(&self) -> &dyn core::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn core::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn motion_path_from_cubic_evaluates() {
        let curve = CubicBezierCurve::new([0.0, 0.0], [25.0, 50.0], [75.0, -50.0], [100.0, 0.0]);
        let path = MotionPath::from(curve);
        assert_eq!(path.position(0.0), [0.0, 0.0]);
        assert_eq!(path.position(1.0), [100.0, 0.0]);
    }

    #[test]
    fn motion_tween_updates_position() {
        let line = LineSegment::new([0.0, 0.0], [100.0, 0.0]);
        let mut tween = MotionPathTween::new(line).duration(1.0).build();
        tween.update(0.5);
        assert_eq!(tween.value(), [50.0, 0.0]);
    }

    #[test]
    fn offsets_trim_path() {
        let line = LineSegment::new([0.0, 0.0], [100.0, 0.0]);
        let mut tween = MotionPathTween::new(line)
            .duration(1.0)
            .start_offset(0.25)
            .end_offset(0.75)
            .build();
        assert_eq!(tween.value(), [25.0, 0.0]);
        tween.update(1.0);
        assert_eq!(tween.value(), [75.0, 0.0]);
    }

    #[test]
    fn auto_rotate_uses_path_heading() {
        let line = LineSegment::new([0.0, 0.0], [0.0, 100.0]);
        let tween = MotionPathTween::new(line).auto_rotate(true).build();
        assert!((tween.rotation_deg() - 90.0).abs() < 0.001);
    }
}
