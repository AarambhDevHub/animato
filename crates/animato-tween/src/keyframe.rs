//! Keyframe tracks for multi-stop value animation.

use crate::loop_mode::Loop;
use alloc::vec::Vec;
use animato_core::{Animatable, Easing, Playable, Update};
use core::cmp::Ordering;

/// A value sample in a [`KeyframeTrack`].
///
/// `easing` is applied to the segment that starts at this keyframe and ends at
/// the next keyframe.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Keyframe<T: Animatable> {
    /// Time in seconds from the start of the track.
    pub time: f32,
    /// Value at this keyframe.
    pub value: T,
    /// Easing applied from this keyframe to the next one.
    pub easing: Easing,
}

impl<T: Animatable> Keyframe<T> {
    /// Create a keyframe at `time` with `value` and `easing`.
    pub fn new(time: f32, value: T, easing: Easing) -> Self {
        Self {
            time: time.max(0.0),
            value,
            easing,
        }
    }
}

/// A sorted collection of keyframes that evaluates a value over time.
///
/// Empty tracks are valid. They evaluate to `None` instead of requiring
/// `T: Default` or panicking.
///
/// # Example
///
/// ```rust
/// use animato_core::{Easing, Update};
/// use animato_tween::KeyframeTrack;
///
/// let mut track = KeyframeTrack::new()
///     .push_eased(0.0, 0.0_f32, Easing::EaseOutCubic)
///     .push(1.0, 100.0);
///
/// track.update(0.5);
/// assert!(track.value().unwrap() > 50.0);
/// ```
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KeyframeTrack<T: Animatable> {
    frames: Vec<Keyframe<T>>,
    elapsed: f32,
    /// Looping behavior for this track.
    pub looping: Loop,
    loop_count: u32,
    complete: bool,
}

impl<T: Animatable> Default for KeyframeTrack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Animatable> KeyframeTrack<T> {
    /// Create an empty keyframe track.
    pub fn new() -> Self {
        Self {
            frames: Vec::new(),
            elapsed: 0.0,
            looping: Loop::Once,
            loop_count: 0,
            complete: false,
        }
    }

    /// Insert a linear keyframe and keep the track sorted by time.
    ///
    /// If another frame already exists at the same time, it is replaced.
    pub fn push(self, time: f32, value: T) -> Self {
        self.push_eased(time, value, Easing::Linear)
    }

    /// Insert a keyframe with an explicit segment easing.
    ///
    /// If another frame already exists at the same time, it is replaced.
    pub fn push_eased(mut self, time: f32, value: T, easing: Easing) -> Self {
        let frame = Keyframe::new(time, value, easing);
        match self.frames.binary_search_by(|existing| {
            existing
                .time
                .partial_cmp(&frame.time)
                .unwrap_or(Ordering::Equal)
        }) {
            Ok(index) => self.frames[index] = frame,
            Err(index) => self.frames.insert(index, frame),
        }
        self.complete = false;
        self
    }

    /// Set the looping behavior.
    pub fn looping(mut self, mode: Loop) -> Self {
        self.looping = mode;
        self.complete = false;
        self
    }

    /// All frames in sorted order.
    pub fn frames(&self) -> &[Keyframe<T>] {
        &self.frames
    }

    /// Current elapsed time in seconds.
    pub fn elapsed(&self) -> f32 {
        self.elapsed
    }

    /// Base duration in seconds, equal to the last keyframe time.
    pub fn duration(&self) -> f32 {
        self.frames.last().map_or(0.0, |frame| frame.time)
    }

    /// Normalized progress through the current finite playback span.
    pub fn progress(&self) -> f32 {
        let total = self.playback_duration();
        if total == 0.0 {
            return 1.0;
        }
        if total.is_finite() {
            (self.elapsed / total).clamp(0.0, 1.0)
        } else {
            let base = self.duration();
            if base == 0.0 {
                1.0
            } else {
                (self.active_time() / base).clamp(0.0, 1.0)
            }
        }
    }

    /// Evaluate the track at an absolute time in seconds.
    pub fn value_at(&self, secs: f32) -> Option<T> {
        if self.frames.is_empty() {
            return None;
        }
        if self.frames.len() == 1 {
            return Some(self.frames[0].value.clone());
        }

        let duration = self.duration();
        let t = secs.max(0.0).min(duration);

        if t <= self.frames[0].time {
            return Some(self.frames[0].value.clone());
        }

        let last_index = self.frames.len() - 1;
        if t >= self.frames[last_index].time {
            return Some(self.frames[last_index].value.clone());
        }

        let upper = self.frames.partition_point(|frame| frame.time <= t);
        let index = upper.saturating_sub(1);
        let current = &self.frames[index];
        let next = &self.frames[index + 1];
        let span = (next.time - current.time).max(f32::EPSILON);
        let local_t = ((t - current.time) / span).clamp(0.0, 1.0);
        let curved_t = current.easing.apply(local_t);
        Some(current.value.lerp(&next.value, curved_t))
    }

    /// Current value based on the track's elapsed time.
    pub fn value(&self) -> Option<T> {
        self.value_at(self.active_time())
    }

    /// `true` when the track has finished all finite playback.
    pub fn is_complete(&self) -> bool {
        self.frames.len() < 2 || self.complete
    }

    /// Reset to the beginning of the track.
    pub fn reset(&mut self) {
        self.elapsed = 0.0;
        self.loop_count = 0;
        self.complete = false;
    }

    fn playback_duration(&self) -> f32 {
        let base = self.duration();
        if base == 0.0 {
            return 0.0;
        }
        match self.looping {
            Loop::Once => base,
            Loop::Times(n) => base * n.max(1) as f32,
            Loop::Forever | Loop::PingPong => f32::INFINITY,
        }
    }

    fn active_time(&self) -> f32 {
        let base = self.duration();
        if base == 0.0 {
            return 0.0;
        }

        match self.looping {
            Loop::Once => self.elapsed.min(base),
            Loop::Times(_) => {
                if self.complete {
                    base
                } else {
                    self.elapsed % base
                }
            }
            Loop::Forever => self.elapsed % base,
            Loop::PingPong => {
                let cycle = self.elapsed % (base * 2.0);
                if cycle <= base {
                    cycle
                } else {
                    base * 2.0 - cycle
                }
            }
        }
    }
}

impl<T: Animatable> Update for KeyframeTrack<T> {
    fn update(&mut self, dt: f32) -> bool {
        if self.is_complete() {
            return false;
        }

        let base = self.duration();
        if base == 0.0 {
            self.complete = true;
            return false;
        }

        self.elapsed += dt.max(0.0);
        self.loop_count = (self.elapsed / base) as u32;

        match self.looping {
            Loop::Once => {
                if self.elapsed >= base {
                    self.elapsed = base;
                    self.complete = true;
                    return false;
                }
            }
            Loop::Times(n) => {
                let total = base * n.max(1) as f32;
                if self.elapsed >= total {
                    self.elapsed = total;
                    self.loop_count = n.max(1);
                    self.complete = true;
                    return false;
                }
            }
            Loop::Forever | Loop::PingPong => {}
        }

        true
    }
}

impl<T: Animatable> Playable for KeyframeTrack<T> {
    fn duration(&self) -> f32 {
        self.playback_duration()
    }

    fn reset(&mut self) {
        KeyframeTrack::reset(self);
    }

    fn seek_to(&mut self, progress: f32) {
        let progress = progress.clamp(0.0, 1.0);
        let total = self.playback_duration();
        let seek_duration = if total.is_finite() {
            total
        } else {
            self.duration()
        };

        if seek_duration == 0.0 {
            self.elapsed = 0.0;
            self.loop_count = 0;
            self.complete = true;
            return;
        }

        self.elapsed = seek_duration * progress;
        self.loop_count = (self.elapsed / self.duration().max(f32::EPSILON)) as u32;
        self.complete = total.is_finite() && progress >= 1.0;
    }

    fn is_complete(&self) -> bool {
        KeyframeTrack::is_complete(self)
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
    fn empty_track_returns_none() {
        let track: KeyframeTrack<f32> = KeyframeTrack::new();
        assert!(track.value().is_none());
        assert!(track.value_at(0.5).is_none());
        assert!(track.is_complete());
    }

    #[test]
    fn single_frame_returns_that_value() {
        let track = KeyframeTrack::new().push(0.5, 42.0_f32);
        assert_eq!(track.value_at(0.0), Some(42.0));
        assert_eq!(track.value_at(2.0), Some(42.0));
    }

    #[test]
    fn push_sorts_and_replaces_duplicate_times() {
        let track = KeyframeTrack::new()
            .push(1.0, 10.0_f32)
            .push(0.0, 0.0)
            .push(1.0, 20.0);

        assert_eq!(track.frames().len(), 2);
        assert_eq!(track.frames()[0].time, 0.0);
        assert_eq!(track.frames()[1].time, 1.0);
        assert_eq!(track.frames()[1].value, 20.0);
    }

    #[test]
    fn two_frames_interpolate_linearly() {
        let track = KeyframeTrack::new().push(0.0, 0.0_f32).push(1.0, 100.0);
        assert_eq!(track.value_at(0.5), Some(50.0));
    }

    #[test]
    fn easing_applies_from_current_frame() {
        let track = KeyframeTrack::new()
            .push_eased(0.0, 0.0_f32, Easing::EaseOutCubic)
            .push(1.0, 100.0);
        assert!(track.value_at(0.5).unwrap() > 50.0);
    }

    #[test]
    fn update_completes_once_track() {
        let mut track = KeyframeTrack::new().push(0.0, 0.0_f32).push(1.0, 1.0);
        assert!(track.update(0.5));
        assert_eq!(track.value(), Some(0.5));
        assert!(!track.update(0.5));
        assert!(track.is_complete());
        assert_eq!(track.value(), Some(1.0));
    }

    #[test]
    fn forever_loop_stays_in_bounds() {
        let mut track = KeyframeTrack::new()
            .push(0.0, 0.0_f32)
            .push(1.0, 1.0)
            .looping(Loop::Forever);
        track.update(10.25);
        assert!(!track.is_complete());
        let value = track.value().unwrap();
        assert!((0.0..=1.0).contains(&value));
    }

    #[test]
    fn ping_pong_reflects_after_duration() {
        let mut track = KeyframeTrack::new()
            .push(0.0, 0.0_f32)
            .push(1.0, 100.0)
            .looping(Loop::PingPong);
        track.update(1.25);
        assert_eq!(track.value(), Some(75.0));
    }

    #[test]
    fn negative_dt_does_not_move() {
        let mut track = KeyframeTrack::new().push(0.0, 0.0_f32).push(1.0, 100.0);
        track.update(-1.0);
        assert_eq!(track.value(), Some(0.0));
    }

    #[test]
    fn large_dt_completes_without_panic() {
        let mut track = KeyframeTrack::new().push(0.0, 0.0_f32).push(1.0, 100.0);
        assert!(!track.update(1_000_000.0));
        assert_eq!(track.value(), Some(100.0));
    }

    #[test]
    fn playable_seek_sets_position() {
        let mut track = KeyframeTrack::new().push(0.0, 0.0_f32).push(2.0, 100.0);
        Playable::seek_to(&mut track, 0.25);
        assert_eq!(track.value(), Some(25.0));
    }

    #[test]
    fn keyframe_clamps_negative_time_and_default_track_is_empty() {
        let frame = Keyframe::new(-1.0, 5.0_f32, Easing::EaseInQuad);
        let track = KeyframeTrack::<f32>::default();

        assert_eq!(frame.time, 0.0);
        assert_eq!(frame.value, 5.0);
        assert_eq!(track.duration(), 0.0);
        assert_eq!(track.progress(), 1.0);
    }

    #[test]
    fn accessors_progress_reset_and_times_loop_work() {
        let mut track = KeyframeTrack::new()
            .push(0.0, 0.0_f32)
            .push(1.0, 100.0)
            .looping(Loop::Times(2));

        assert_eq!(track.duration(), 1.0);
        assert_eq!(track.elapsed(), 0.0);
        assert_eq!(Playable::duration(&track), 2.0);

        track.update(1.25);
        assert_eq!(track.progress(), 0.625);
        assert_eq!(track.value(), Some(25.0));

        assert!(!track.update(1.0));
        assert!(track.is_complete());
        assert_eq!(track.value(), Some(100.0));

        track.reset();
        assert_eq!(track.elapsed(), 0.0);
        assert!(!track.is_complete());
    }

    #[test]
    fn value_at_clamps_before_first_and_after_last() {
        let track = KeyframeTrack::new().push(0.5, 10.0_f32).push(1.5, 30.0_f32);

        assert_eq!(track.value_at(-1.0), Some(10.0));
        assert_eq!(track.value_at(3.0), Some(30.0));
    }

    #[test]
    fn playable_seek_handles_empty_and_infinite_tracks() {
        let mut empty = KeyframeTrack::<f32>::new();
        Playable::seek_to(&mut empty, 0.5);
        assert!(empty.is_complete());
        assert_eq!(empty.elapsed(), 0.0);

        let mut forever = KeyframeTrack::new()
            .push(0.0, 0.0_f32)
            .push(1.0, 10.0)
            .looping(Loop::Forever);
        Playable::seek_to(&mut forever, 0.5);
        assert_eq!(forever.value(), Some(5.0));
        assert!(!Playable::is_complete(&forever));

        let mut ping_pong = KeyframeTrack::new()
            .push(0.0, 0.0_f32)
            .push(1.0, 10.0)
            .looping(Loop::PingPong);
        Playable::seek_to(&mut ping_pong, 1.0);
        assert_eq!(ping_pong.value(), Some(10.0));
        assert!(Playable::as_any(&ping_pong).is::<KeyframeTrack<f32>>());
        assert!(Playable::as_any_mut(&mut ping_pong).is::<KeyframeTrack<f32>>());
    }
}
