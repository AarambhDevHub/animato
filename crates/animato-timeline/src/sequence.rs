//! Sequence builder for back-to-back timeline entries.

use crate::timeline::{At, Timeline};
use alloc::boxed::Box;
use alloc::string::String;
use animato_core::Playable;

/// Builder for timeline entries that play one after another.
#[derive(Debug)]
pub struct Sequence {
    inner: Timeline,
    cursor: f32,
}

impl Default for Sequence {
    fn default() -> Self {
        Self::new()
    }
}

impl Sequence {
    /// Create an empty sequence.
    pub fn new() -> Self {
        Self {
            inner: Timeline::new(),
            cursor: 0.0,
        }
    }

    /// Add an animation at the current sequence cursor.
    pub fn then<A>(mut self, label: impl Into<String>, animation: A) -> Self
    where
        A: Playable + Send + 'static,
    {
        let duration = animation.duration().max(0.0);
        self.inner = self.inner.add(label, animation, At::Absolute(self.cursor));
        self.cursor += duration;
        self
    }

    /// Add an animation and advance the cursor by an explicit duration.
    ///
    /// This is useful when the child has a longer internal duration but should
    /// reserve a shorter or longer slot in the sequence.
    pub fn then_for<A>(mut self, label: impl Into<String>, animation: A, duration: f32) -> Self
    where
        A: Playable + Send + 'static,
    {
        let duration = duration.max(0.0);
        self.inner = self.inner.add_boxed_with_duration(
            label,
            Box::new(animation),
            At::Absolute(self.cursor),
            duration,
        );
        self.cursor += duration;
        self
    }

    /// Insert a silent gap before the next entry.
    pub fn gap(mut self, seconds: f32) -> Self {
        self.cursor += seconds.max(0.0);
        self
    }

    /// Consume the builder and return the completed timeline.
    pub fn build(self) -> Timeline {
        self.inner
    }

    /// Current cursor position in seconds.
    pub fn cursor(&self) -> f32 {
        self.cursor
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use animato_core::Update;
    use animato_tween::Tween;

    #[test]
    fn sequence_places_entries_back_to_back() {
        let mut timeline = Sequence::new()
            .then("first", Tween::new(0.0_f32, 10.0).duration(1.0).build())
            .then("second", Tween::new(0.0_f32, 20.0).duration(1.0).build())
            .build();

        timeline.play();
        timeline.update(1.5);

        assert_eq!(timeline.get::<Tween<f32>>("first").unwrap().value(), 10.0);
        assert_eq!(timeline.get::<Tween<f32>>("second").unwrap().value(), 10.0);
    }

    #[test]
    fn gap_delays_next_entry() {
        let mut timeline = Sequence::new()
            .then("first", Tween::new(0.0_f32, 10.0).duration(1.0).build())
            .gap(0.5)
            .then("second", Tween::new(0.0_f32, 20.0).duration(1.0).build())
            .build();

        timeline.play();
        timeline.update(1.25);

        assert_eq!(timeline.get::<Tween<f32>>("first").unwrap().value(), 10.0);
        assert_eq!(timeline.get::<Tween<f32>>("second").unwrap().value(), 0.0);
    }
}
