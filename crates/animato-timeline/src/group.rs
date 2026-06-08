//! Animation groups built on top of [`Timeline`].

use crate::{At, Sequence, Timeline};
use alloc::format;
use alloc::vec::Vec;
use animato_core::{Playable, Update};
use animato_tween::StaggerPattern;
use core::fmt;

/// A group of animations controlled as a single playable unit.
pub struct AnimationGroup {
    inner: Timeline,
}

impl fmt::Debug for AnimationGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnimationGroup")
            .field("inner", &self.inner)
            .finish()
    }
}

impl AnimationGroup {
    /// Create a group from an existing timeline.
    pub fn from_timeline(timeline: Timeline) -> Self {
        Self { inner: timeline }
    }

    /// Play all animations at the same time.
    pub fn parallel<A>(animations: Vec<A>) -> Self
    where
        A: Playable + Send + 'static,
    {
        let mut timeline = Timeline::new();
        for (index, animation) in animations.into_iter().enumerate() {
            timeline = timeline.add(format!("item_{index}"), animation, At::Start);
        }
        Self::from_timeline(timeline)
    }

    /// Play animations one after another.
    pub fn sequence<A>(animations: Vec<A>) -> Self
    where
        A: Playable + Send + 'static,
    {
        let mut sequence = Sequence::new();
        for (index, animation) in animations.into_iter().enumerate() {
            sequence = sequence.then(format!("item_{index}"), animation);
        }
        Self::from_timeline(sequence.build())
    }

    /// Play animations with start delays from a [`StaggerPattern`].
    pub fn stagger<A>(animations: Vec<A>, pattern: StaggerPattern) -> Self
    where
        A: Playable + Send + 'static,
    {
        let total = animations.len();
        let mut timeline = Timeline::new();
        for (index, animation) in animations.into_iter().enumerate() {
            timeline = timeline.add(
                format!("item_{index}"),
                animation,
                At::Absolute(pattern.delay(index, total)),
            );
        }
        Self::from_timeline(timeline)
    }

    /// Access the wrapped timeline.
    pub fn timeline(&self) -> &Timeline {
        &self.inner
    }

    /// Access the wrapped timeline mutably.
    pub fn timeline_mut(&mut self) -> &mut Timeline {
        &mut self.inner
    }

    /// Begin group playback.
    pub fn play(&mut self) {
        self.inner.play();
    }

    /// Pause group playback.
    pub fn pause(&mut self) {
        self.inner.pause();
    }

    /// Resume group playback.
    pub fn resume(&mut self) {
        self.inner.resume();
    }

    /// Reset group playback.
    pub fn reset(&mut self) {
        self.inner.reset();
    }

    /// Seek by normalized progress.
    pub fn seek(&mut self, progress: f32) {
        self.inner.seek(progress);
    }

    /// Seek to the mirrored progress position.
    pub fn reverse(&mut self) {
        let progress = self.inner.progress();
        self.inner.seek(1.0 - progress);
    }

    /// Set time scale for the entire group.
    pub fn set_time_scale(&mut self, scale: f32) {
        self.inner.set_time_scale(scale);
    }

    /// Group duration in seconds.
    pub fn duration(&self) -> f32 {
        self.inner.duration()
    }

    /// Normalized progress.
    pub fn progress(&self) -> f32 {
        self.inner.progress()
    }

    /// `true` when the group has completed.
    pub fn is_complete(&self) -> bool {
        self.inner.is_complete()
    }

    /// Register a callback fired once when the group completes.
    #[cfg(feature = "std")]
    pub fn on_complete(mut self, f: impl FnMut() + Send + 'static) -> Self {
        self.inner = self.inner.on_complete(f);
        self
    }
}

impl Update for AnimationGroup {
    fn update(&mut self, dt: f32) -> bool {
        self.inner.update(dt)
    }
}

impl Playable for AnimationGroup {
    fn duration(&self) -> f32 {
        self.inner.duration()
    }

    fn reset(&mut self) {
        AnimationGroup::reset(self);
    }

    fn seek_to(&mut self, progress: f32) {
        self.seek(progress);
    }

    fn is_complete(&self) -> bool {
        AnimationGroup::is_complete(self)
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
    use animato_tween::Tween;

    #[test]
    fn parallel_completes_when_last_child_finishes() {
        let mut group = AnimationGroup::parallel(alloc::vec![
            Tween::new(0.0_f32, 1.0).duration(0.5).build(),
            Tween::new(0.0_f32, 1.0).duration(1.0).build(),
        ]);
        group.play();
        assert!(group.update(0.75));
        assert!(!group.update(0.25));
    }

    #[test]
    fn sequence_orders_children() {
        let mut group = AnimationGroup::sequence(alloc::vec![
            Tween::new(0.0_f32, 10.0).duration(1.0).build(),
            Tween::new(0.0_f32, 20.0).duration(1.0).build(),
        ]);
        group.play();
        group.update(1.5);
        assert_eq!(
            group
                .timeline()
                .get::<Tween<f32>>("item_0")
                .expect("first")
                .value(),
            10.0
        );
        assert_eq!(
            group
                .timeline()
                .get::<Tween<f32>>("item_1")
                .expect("second")
                .value(),
            10.0
        );
    }
}
