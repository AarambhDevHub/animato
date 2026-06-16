//! Timeline inspection state.

use animato_core::{AnimationKind, Easing, PlaybackState};
use animato_driver::{AnimationDriver, AnimationId};

/// Snapshot of one running animation for DevTools rendering.
#[derive(Clone, Debug, PartialEq)]
pub struct AnimationSnapshot {
    /// Stable animation id returned by the driver.
    pub id: AnimationId,
    /// Optional user-facing label.
    pub label: Option<String>,
    /// High-level animation category.
    pub kind: AnimationKind,
    /// Normalized progress in `[0.0, 1.0]`.
    pub progress: f32,
    /// Elapsed seconds.
    pub elapsed: f32,
    /// Finite duration in seconds, if known.
    pub duration: Option<f32>,
    /// Coarse playback state.
    pub state: PlaybackState,
    /// Active easing curve, if applicable.
    pub easing: Option<Easing>,
}

impl AnimationSnapshot {
    /// Return a stable color name for the animation kind.
    pub fn color_name(&self) -> &'static str {
        match self.kind {
            AnimationKind::Tween => "blue",
            AnimationKind::Spring => "green",
            AnimationKind::Keyframe => "violet",
            AnimationKind::Timeline => "amber",
            AnimationKind::Group => "cyan",
            AnimationKind::Custom => "gray",
        }
    }

    /// Render an ASCII progress bar with a stable width.
    pub fn progress_bar(&self, width: usize) -> String {
        let width = width.max(1);
        let filled = ((self.progress.clamp(0.0, 1.0) * width as f32).round() as usize).min(width);
        let mut out = String::with_capacity(width);
        out.extend(core::iter::repeat_n('#', filled));
        out.extend(core::iter::repeat_n('-', width - filled));
        out
    }
}

/// Live animation timeline inspector.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TimelineInspector {
    snapshots: Vec<AnimationSnapshot>,
    completed_count: usize,
}

impl TimelineInspector {
    /// Create an empty inspector.
    pub fn new() -> Self {
        Self::default()
    }

    /// Capture all inspectable animations from a driver.
    pub fn capture(&mut self, driver: &AnimationDriver) {
        self.snapshots.clear();
        self.snapshots
            .extend(driver.snapshots().into_iter().map(|snapshot| {
                let introspection = snapshot.introspection;
                AnimationSnapshot {
                    id: snapshot.id,
                    label: snapshot.label,
                    kind: introspection.kind,
                    progress: introspection.progress,
                    elapsed: introspection.elapsed,
                    duration: introspection.duration,
                    state: introspection.state,
                    easing: introspection.easing,
                }
            }));
        self.completed_count = driver.completed_count();
    }

    /// Write snapshots into a reusable output buffer.
    pub fn capture_into(&self, out: &mut Vec<AnimationSnapshot>) {
        out.clear();
        out.extend_from_slice(&self.snapshots);
    }

    /// Captured snapshots.
    pub fn snapshots(&self) -> &[AnimationSnapshot] {
        &self.snapshots
    }

    /// Number of currently active inspectable animations.
    pub fn active_count(&self) -> usize {
        self.snapshots
            .iter()
            .filter(|snapshot| snapshot.state != PlaybackState::Complete)
            .count()
    }

    /// Number of animations completed by the driver.
    pub fn completed_count(&self) -> usize {
        self.completed_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use animato_core::{AnimationKind, PlaybackState};
    use animato_driver::AnimationDriver;
    use animato_tween::Tween;

    #[test]
    fn captures_driver_snapshots() {
        let mut driver = AnimationDriver::new();
        driver.add_inspectable("fade", Tween::new(0.0_f32, 1.0).duration(1.0).build());
        driver.tick(0.25);

        let mut inspector = TimelineInspector::new();
        inspector.capture(&driver);

        assert_eq!(inspector.snapshots().len(), 1);
        let snapshot = &inspector.snapshots()[0];
        assert_eq!(snapshot.label.as_deref(), Some("fade"));
        assert_eq!(snapshot.kind, AnimationKind::Tween);
        assert_eq!(snapshot.state, PlaybackState::Playing);
        assert!((snapshot.progress - 0.25).abs() < 0.001);
        assert_eq!(snapshot.progress_bar(4), "#---");
    }

    #[test]
    fn reports_completed_count() {
        let mut driver = AnimationDriver::new();
        driver.add_inspectable("fade", Tween::new(0.0_f32, 1.0).duration(0.1).build());
        driver.tick(1.0);

        let mut inspector = TimelineInspector::new();
        inspector.capture(&driver);
        assert_eq!(inspector.completed_count(), 1);
        assert_eq!(inspector.active_count(), 0);
    }
}
