//! Timeline composition primitives.

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use animato_core::{Playable, Update};
use animato_tween::Loop;
use core::fmt;

/// Positioning rule for a timeline entry.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum At<'a> {
    /// Start at an explicit absolute time in seconds.
    Absolute(f32),
    /// Start at timeline time `0.0`.
    Start,
    /// Start when the current last entry ends.
    End,
    /// Start at the same time as an existing labeled entry.
    Label(&'a str),
    /// Start relative to the current timeline end.
    Offset(f32),
}

/// Current playback state of a [`Timeline`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TimelineState {
    /// Ready but not advancing.
    Idle,
    /// Actively advancing.
    Playing,
    /// Paused mid-playback.
    Paused,
    /// Finished all finite playback.
    Completed,
}

struct TimelineEntry {
    label: String,
    animation: Box<dyn Playable + Send>,
    start_at: f32,
    duration: f32,
    completed: bool,
}

impl fmt::Debug for TimelineEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TimelineEntry")
            .field("label", &self.label)
            .field("start_at", &self.start_at)
            .field("duration", &self.duration)
            .field("completed", &self.completed)
            .finish()
    }
}

impl TimelineEntry {
    fn end_at(&self) -> f32 {
        self.start_at + self.duration
    }
}

#[cfg(feature = "std")]
struct EntryCallback {
    label: String,
    callback: Box<dyn FnMut() + Send + 'static>,
}

#[cfg(feature = "std")]
impl fmt::Debug for EntryCallback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EntryCallback")
            .field("label", &self.label)
            .finish()
    }
}

#[cfg(feature = "std")]
#[derive(Default)]
struct TimelineCallbacks {
    entry_complete: Vec<EntryCallback>,
    complete: Option<Box<dyn FnMut() + Send + 'static>>,
    complete_fired: bool,
}

#[cfg(feature = "std")]
impl fmt::Debug for TimelineCallbacks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TimelineCallbacks")
            .field("entry_complete", &self.entry_complete)
            .field("has_complete", &self.complete.is_some())
            .field("complete_fired", &self.complete_fired)
            .finish()
    }
}

#[cfg(feature = "std")]
impl TimelineCallbacks {
    fn fire_entry_complete(&mut self, completed_labels: &[String]) {
        for completed_label in completed_labels {
            for callback in self.entry_complete.iter_mut() {
                if callback.label == *completed_label {
                    (callback.callback)();
                }
            }
        }
    }

    fn fire_complete(&mut self) {
        if self.complete_fired {
            return;
        }
        self.complete_fired = true;
        if let Some(callback) = self.complete.as_mut() {
            callback();
        }
    }

    fn reset_completion(&mut self) {
        self.complete_fired = false;
    }
}

/// Composes multiple animations on one shared clock.
///
/// Entries are stored by label, absolute start time, and cached duration.
/// Normal one-shot playback advances children incrementally. Seeking and
/// timeline-level loops resynchronize children through [`Playable::seek_to`].
pub struct Timeline {
    entries: Vec<TimelineEntry>,
    elapsed: f32,
    state: TimelineState,
    /// Timeline-level looping behavior.
    pub looping: Loop,
    /// Timeline time scale. `1.0` = normal speed, `2.0` = double speed.
    pub time_scale: f32,
    #[cfg(feature = "std")]
    callbacks: TimelineCallbacks,
    #[cfg(feature = "tokio")]
    completion_tx: tokio::sync::watch::Sender<bool>,
}

impl fmt::Debug for Timeline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Timeline")
            .field("entries", &self.entries)
            .field("elapsed", &self.elapsed)
            .field("state", &self.state)
            .field("looping", &self.looping)
            .field("time_scale", &self.time_scale)
            .field("callbacks", &{
                #[cfg(feature = "std")]
                {
                    &self.callbacks
                }
                #[cfg(not(feature = "std"))]
                {
                    &"disabled"
                }
            })
            .finish()
    }
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}

impl Timeline {
    /// Create an empty timeline.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            elapsed: 0.0,
            state: TimelineState::Idle,
            looping: Loop::Once,
            time_scale: 1.0,
            #[cfg(feature = "std")]
            callbacks: TimelineCallbacks::default(),
            #[cfg(feature = "tokio")]
            completion_tx: tokio::sync::watch::channel(false).0,
        }
    }

    /// Add an animation at the requested position.
    ///
    /// Missing [`At::Label`] references fall back to [`At::End`] behavior.
    pub fn add<A>(mut self, label: impl Into<String>, animation: A, at: At<'_>) -> Self
    where
        A: Playable + Send + 'static,
    {
        let start_at = self.resolve_start(at);
        let duration = animation.duration().max(0.0);
        self.entries.push(TimelineEntry {
            label: label.into(),
            animation: Box::new(animation),
            start_at,
            duration,
            completed: false,
        });
        self
    }

    pub(crate) fn add_boxed_with_duration(
        mut self,
        label: impl Into<String>,
        animation: Box<dyn Playable + Send>,
        at: At<'_>,
        duration: f32,
    ) -> Self {
        let start_at = self.resolve_start(at);
        self.entries.push(TimelineEntry {
            label: label.into(),
            animation,
            start_at,
            duration: duration.max(0.0),
            completed: false,
        });
        self
    }

    /// Set timeline-level looping behavior.
    pub fn looping(mut self, mode: Loop) -> Self {
        self.looping = mode;
        self
    }

    /// Set the timeline time scale.
    ///
    /// Negative values are clamped to `0.0`.
    pub fn time_scale(mut self, scale: f32) -> Self {
        self.set_time_scale(scale);
        self
    }

    /// Change the timeline time scale at runtime.
    ///
    /// `1.0` is normal speed, `2.0` is double speed, and `0.0` freezes time.
    pub fn set_time_scale(&mut self, scale: f32) {
        self.time_scale = scale.max(0.0);
    }

    /// Register a callback fired when the labeled entry completes during `update`.
    ///
    /// This is available with the `std` feature. Seeking and resetting do not
    /// fire callbacks.
    #[cfg(feature = "std")]
    pub fn on_entry_complete(
        mut self,
        label: impl Into<String>,
        f: impl FnMut() + Send + 'static,
    ) -> Self {
        self.callbacks.entry_complete.push(EntryCallback {
            label: label.into(),
            callback: Box::new(f),
        });
        self
    }

    /// Register a callback fired once when finite timeline playback completes during `update`.
    ///
    /// This is available with the `std` feature. Seeking to the end does not
    /// fire this callback.
    #[cfg(feature = "std")]
    pub fn on_complete(mut self, f: impl FnMut() + Send + 'static) -> Self {
        self.callbacks.complete = Some(Box::new(f));
        self
    }

    /// Return a future that resolves when the timeline reaches `Completed`.
    ///
    /// The future only observes completion; it does not drive the timeline.
    /// Continue calling [`update`](Update::update) from your runtime loop.
    #[cfg(feature = "tokio")]
    pub fn wait(&self) -> impl core::future::Future<Output = ()> + Send + 'static {
        let mut rx = self.completion_tx.subscribe();
        async move {
            loop {
                if *rx.borrow() {
                    return;
                }
                if rx.changed().await.is_err() {
                    return;
                }
            }
        }
    }

    /// Begin playback.
    pub fn play(&mut self) {
        if self.state == TimelineState::Completed {
            self.reset();
        }
        if self.duration() == 0.0 {
            self.state = TimelineState::Completed;
            self.notify_completion_state(true);
        } else {
            self.state = TimelineState::Playing;
            self.notify_completion_state(false);
            self.sync_to_elapsed();
        }
    }

    /// Pause playback.
    pub fn pause(&mut self) {
        if self.state == TimelineState::Playing {
            self.state = TimelineState::Paused;
        }
    }

    /// Resume playback after a pause.
    pub fn resume(&mut self) {
        if self.state == TimelineState::Paused {
            self.state = TimelineState::Playing;
        }
    }

    /// Reset the timeline and all children to the beginning.
    pub fn reset(&mut self) {
        self.elapsed = 0.0;
        self.state = TimelineState::Idle;
        self.reset_completion_callbacks();
        self.notify_completion_state(false);
        for entry in self.entries.iter_mut() {
            entry.animation.reset();
            entry.completed = false;
        }
    }

    /// Seek by normalized progress through the timeline.
    pub fn seek(&mut self, progress: f32) {
        let total = self.playback_duration();
        let seek_duration = if total.is_finite() {
            total
        } else {
            self.duration()
        };
        self.seek_abs(seek_duration * progress.clamp(0.0, 1.0));
    }

    /// Seek to an absolute time in seconds.
    pub fn seek_abs(&mut self, secs: f32) {
        let total = self.playback_duration();
        let secs = secs.max(0.0);
        self.elapsed = if total.is_finite() {
            secs.min(total)
        } else {
            secs
        };
        self.sync_to_elapsed();
        if total.is_finite() && self.elapsed >= total {
            self.state = TimelineState::Completed;
            self.notify_completion_state(true);
        } else if self.state == TimelineState::Completed {
            self.state = TimelineState::Playing;
            self.notify_completion_state(false);
        }
    }

    /// Base duration in seconds, equal to the last finishing entry.
    pub fn duration(&self) -> f32 {
        self.entries
            .iter()
            .map(TimelineEntry::end_at)
            .fold(0.0, f32::max)
    }

    /// Current normalized progress through finite playback.
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
                (self.local_time_for_elapsed(self.elapsed) / base).clamp(0.0, 1.0)
            }
        }
    }

    /// `true` when the timeline has finished all finite playback.
    pub fn is_complete(&self) -> bool {
        self.state == TimelineState::Completed
    }

    /// Current timeline state.
    pub fn state(&self) -> TimelineState {
        self.state
    }

    /// Current total elapsed timeline time in seconds.
    pub fn elapsed(&self) -> f32 {
        self.elapsed
    }

    /// Number of entries in the timeline.
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    /// Find a child animation by label and concrete type.
    pub fn get<T>(&self, label: &str) -> Option<&T>
    where
        T: Playable + 'static,
    {
        self.entries
            .iter()
            .find(|entry| entry.label == label)
            .and_then(|entry| entry.animation.as_any().downcast_ref::<T>())
    }

    /// Find a mutable child animation by label and concrete type.
    pub fn get_mut<T>(&mut self, label: &str) -> Option<&mut T>
    where
        T: Playable + 'static,
    {
        self.entries
            .iter_mut()
            .find(|entry| entry.label == label)
            .and_then(|entry| entry.animation.as_any_mut().downcast_mut::<T>())
    }

    fn fire_entry_callbacks(&mut self, completed_labels: &[String]) {
        #[cfg(feature = "std")]
        self.callbacks.fire_entry_complete(completed_labels);

        #[cfg(not(feature = "std"))]
        let _ = completed_labels;
    }

    fn fire_complete_callback(&mut self) {
        #[cfg(feature = "std")]
        self.callbacks.fire_complete();
    }

    fn reset_completion_callbacks(&mut self) {
        #[cfg(feature = "std")]
        self.callbacks.reset_completion();
    }

    fn notify_completion_state(&self, complete: bool) {
        #[cfg(feature = "tokio")]
        let _ = self.completion_tx.send_replace(complete);

        #[cfg(not(feature = "tokio"))]
        let _ = complete;
    }

    fn complete_from_update(&mut self) -> bool {
        self.state = TimelineState::Completed;
        self.fire_complete_callback();
        self.notify_completion_state(true);
        false
    }

    fn resolve_start(&self, at: At<'_>) -> f32 {
        match at {
            At::Absolute(secs) => secs.max(0.0),
            At::Start => 0.0,
            At::End => self.duration(),
            At::Label(label) => self
                .entries
                .iter()
                .find(|entry| entry.label == label)
                .map_or_else(|| self.duration(), |entry| entry.start_at),
            At::Offset(offset) => (self.duration() + offset).max(0.0),
        }
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

    fn local_time_for_elapsed(&self, elapsed: f32) -> f32 {
        let base = self.duration();
        if base == 0.0 {
            return 0.0;
        }

        match self.looping {
            Loop::Once => elapsed.min(base),
            Loop::Times(n) => {
                let total = base * n.max(1) as f32;
                if elapsed >= total {
                    base
                } else {
                    elapsed % base
                }
            }
            Loop::Forever => elapsed % base,
            Loop::PingPong => {
                let cycle = elapsed % (base * 2.0);
                if cycle <= base {
                    cycle
                } else {
                    base * 2.0 - cycle
                }
            }
        }
    }

    fn entry_completion_labels_between(&self, prev: f32, next: f32, base: f32) -> Vec<String> {
        let mut labels = Vec::new();
        if next <= prev || base <= 0.0 {
            return labels;
        }

        let (max_cycles, period) = match self.looping {
            Loop::Once => (Some(1), base),
            Loop::Times(n) => (Some(n.max(1)), base),
            Loop::Forever => (None, base),
            Loop::PingPong => (None, base * 2.0),
        };

        if period <= 0.0 {
            return labels;
        }

        let mut cycle = (prev / period).max(0.0) as u32;
        loop {
            if let Some(max_cycles) = max_cycles
                && cycle >= max_cycles
            {
                break;
            }

            let cycle_start = cycle as f32 * period;
            if cycle_start > next {
                break;
            }

            for entry in self.entries.iter() {
                let completion = cycle_start + entry.end_at();
                if prev < completion && completion <= next {
                    labels.push(entry.label.clone());
                }
            }

            cycle = cycle.saturating_add(1);
            if cycle == u32::MAX {
                break;
            }
        }

        labels
    }

    fn tick_forward(&mut self, prev: f32, next: f32) -> Vec<String> {
        let mut completed_labels = Vec::new();
        for entry in self.entries.iter_mut() {
            let start = entry.start_at;
            let end = entry.end_at();
            let was_completed = entry.completed;

            if next < start {
                entry.animation.reset();
                entry.completed = false;
                continue;
            }

            if prev <= start && next >= start {
                entry.animation.reset();
                entry.completed = false;
            }

            if entry.duration == 0.0 {
                if next >= start {
                    entry.animation.seek_to(1.0);
                    entry.completed = true;
                }
                if !was_completed && entry.completed {
                    completed_labels.push(entry.label.clone());
                }
                continue;
            }

            let overlap_start = prev.max(start);
            let overlap_end = next.min(end);
            if overlap_end > overlap_start {
                let still_running = entry.animation.update(overlap_end - overlap_start);
                if !still_running {
                    entry.completed = true;
                }
            }

            if next >= end {
                entry.animation.seek_to(1.0);
                entry.completed = true;
            }

            if !was_completed && entry.completed {
                completed_labels.push(entry.label.clone());
            }
        }
        completed_labels
    }

    fn sync_to_elapsed(&mut self) {
        let local_time = self.local_time_for_elapsed(self.elapsed);
        for entry in self.entries.iter_mut() {
            let start = entry.start_at;
            let end = entry.end_at();

            if local_time <= start {
                entry.animation.reset();
                entry.completed = false;
            } else if local_time >= end || entry.duration == 0.0 {
                entry.animation.seek_to(1.0);
                entry.completed = true;
            } else {
                let progress = (local_time - start) / entry.duration;
                entry.animation.seek_to(progress);
                entry.completed = false;
            }
        }
    }
}

impl Update for Timeline {
    fn update(&mut self, dt: f32) -> bool {
        match self.state {
            TimelineState::Completed => return false,
            TimelineState::Paused | TimelineState::Idle => return true,
            TimelineState::Playing => {}
        }

        let base = self.duration();
        if base == 0.0 {
            return self.complete_from_update();
        }

        let dt = dt.max(0.0) * self.time_scale;
        let previous_elapsed = self.elapsed;
        let next_elapsed = previous_elapsed + dt;

        match self.looping {
            Loop::Once => {
                let prev_local = previous_elapsed.min(base);
                let next_local = next_elapsed.min(base);
                let completed_labels = self.tick_forward(prev_local, next_local);
                self.fire_entry_callbacks(&completed_labels);
                self.elapsed = next_elapsed.min(base);
                if next_elapsed >= base {
                    return self.complete_from_update();
                }
            }
            Loop::Times(n) => {
                let total = base * n.max(1) as f32;
                let completed_labels =
                    self.entry_completion_labels_between(previous_elapsed, next_elapsed, base);
                self.elapsed = next_elapsed.min(total);
                self.sync_to_elapsed();
                self.fire_entry_callbacks(&completed_labels);
                if next_elapsed >= total {
                    return self.complete_from_update();
                }
            }
            Loop::Forever | Loop::PingPong => {
                let completed_labels =
                    self.entry_completion_labels_between(previous_elapsed, next_elapsed, base);
                self.elapsed = next_elapsed;
                self.sync_to_elapsed();
                self.fire_entry_callbacks(&completed_labels);
            }
        }

        true
    }
}

impl Playable for Timeline {
    fn duration(&self) -> f32 {
        self.playback_duration()
    }

    fn reset(&mut self) {
        Timeline::reset(self);
    }

    fn seek_to(&mut self, progress: f32) {
        Timeline::seek(self, progress);
    }

    fn is_complete(&self) -> bool {
        Timeline::is_complete(self)
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
    use animato_core::Easing;
    use animato_tween::Tween;

    fn tween(end: f32, duration: f32) -> Tween<f32> {
        Tween::new(0.0_f32, end)
            .duration(duration)
            .easing(Easing::Linear)
            .build()
    }

    #[test]
    fn concurrent_entries_advance_together() {
        let mut timeline = Timeline::new().add("a", tween(1.0, 1.0), At::Start).add(
            "b",
            tween(100.0, 1.0),
            At::Label("a"),
        );

        timeline.play();
        timeline.update(0.5);

        assert_eq!(timeline.get::<Tween<f32>>("a").unwrap().value(), 0.5);
        assert_eq!(timeline.get::<Tween<f32>>("b").unwrap().value(), 50.0);
    }

    #[test]
    fn end_and_offset_position_entries() {
        let timeline = Timeline::new()
            .add("first", tween(1.0, 1.0), At::Start)
            .add("second", tween(1.0, 0.5), At::End)
            .add("third", tween(1.0, 0.25), At::Offset(0.25));

        assert_eq!(timeline.duration(), 2.0);
    }

    #[test]
    fn seek_abs_synchronizes_children() {
        let mut timeline = Timeline::new().add("a", tween(100.0, 2.0), At::Start);

        timeline.seek_abs(0.5);

        assert_eq!(timeline.get::<Tween<f32>>("a").unwrap().value(), 25.0);
    }

    #[test]
    fn pause_stops_timeline_progress() {
        let mut timeline = Timeline::new().add("a", tween(100.0, 1.0), At::Start);
        timeline.play();
        timeline.update(0.25);
        timeline.pause();
        timeline.update(0.5);

        assert_eq!(timeline.elapsed(), 0.25);
        assert_eq!(timeline.get::<Tween<f32>>("a").unwrap().value(), 25.0);
    }

    #[test]
    fn resume_continues_after_pause() {
        let mut timeline = Timeline::new().add("a", tween(100.0, 1.0), At::Start);
        timeline.play();
        timeline.update(0.25);
        timeline.pause();
        timeline.resume();
        timeline.update(0.25);

        assert_eq!(timeline.get::<Tween<f32>>("a").unwrap().value(), 50.0);
    }

    #[test]
    fn once_timeline_completes() {
        let mut timeline = Timeline::new().add("a", tween(1.0, 1.0), At::Start);
        timeline.play();

        assert!(!timeline.update(1.0));
        assert!(timeline.is_complete());
    }

    #[test]
    fn times_loop_repeats_then_completes() {
        let mut timeline = Timeline::new()
            .add("a", tween(100.0, 1.0), At::Start)
            .looping(Loop::Times(2));
        timeline.play();

        timeline.update(1.25);
        assert_eq!(timeline.get::<Tween<f32>>("a").unwrap().value(), 25.0);

        assert!(!timeline.update(1.0));
        assert!(timeline.is_complete());
        assert_eq!(timeline.get::<Tween<f32>>("a").unwrap().value(), 100.0);
    }

    #[test]
    fn ping_pong_reflects_timeline_time() {
        let mut timeline = Timeline::new()
            .add("a", tween(100.0, 1.0), At::Start)
            .looping(Loop::PingPong);
        timeline.play();
        timeline.update(1.25);

        assert_eq!(timeline.get::<Tween<f32>>("a").unwrap().value(), 75.0);
        assert!(!timeline.is_complete());
    }

    #[test]
    fn time_scale_speeds_up_timeline() {
        let mut timeline = Timeline::new()
            .add("a", tween(100.0, 1.0), At::Start)
            .time_scale(2.0);
        timeline.play();
        timeline.update(0.25);

        assert_eq!(timeline.elapsed(), 0.5);
        assert_eq!(timeline.get::<Tween<f32>>("a").unwrap().value(), 50.0);
    }

    #[test]
    fn set_time_scale_clamps_negative_to_zero() {
        let mut timeline = Timeline::new().add("a", tween(100.0, 1.0), At::Start);
        timeline.set_time_scale(-1.0);
        timeline.play();
        timeline.update(0.5);

        assert_eq!(timeline.elapsed(), 0.0);
        assert_eq!(timeline.get::<Tween<f32>>("a").unwrap().value(), 0.0);
    }

    #[cfg(feature = "std")]
    #[test]
    fn callbacks_fire_once_during_update() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicUsize, Ordering};

        let entry_count = Arc::new(AtomicUsize::new(0));
        let complete_count = Arc::new(AtomicUsize::new(0));
        let entry_seen = Arc::clone(&entry_count);
        let complete_seen = Arc::clone(&complete_count);

        let mut timeline = Timeline::new()
            .add("a", tween(100.0, 1.0), At::Start)
            .on_entry_complete("a", move || {
                entry_seen.fetch_add(1, Ordering::SeqCst);
            })
            .on_complete(move || {
                complete_seen.fetch_add(1, Ordering::SeqCst);
            });

        timeline.play();
        assert!(!timeline.update(1.0));
        assert!(!timeline.update(1.0));

        assert_eq!(entry_count.load(Ordering::SeqCst), 1);
        assert_eq!(complete_count.load(Ordering::SeqCst), 1);
    }

    #[cfg(feature = "std")]
    #[test]
    fn callbacks_do_not_fire_on_seek_or_reset() {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicUsize, Ordering};

        let entry_count = Arc::new(AtomicUsize::new(0));
        let complete_count = Arc::new(AtomicUsize::new(0));
        let entry_seen = Arc::clone(&entry_count);
        let complete_seen = Arc::clone(&complete_count);

        let mut timeline = Timeline::new()
            .add("a", tween(100.0, 1.0), At::Start)
            .on_entry_complete("a", move || {
                entry_seen.fetch_add(1, Ordering::SeqCst);
            })
            .on_complete(move || {
                complete_seen.fetch_add(1, Ordering::SeqCst);
            });

        timeline.seek(1.0);
        timeline.reset();

        assert_eq!(entry_count.load(Ordering::SeqCst), 0);
        assert_eq!(complete_count.load(Ordering::SeqCst), 0);
    }

    #[cfg(feature = "tokio")]
    #[test]
    fn wait_is_ready_after_completion() {
        use core::future::Future;
        use std::sync::Arc;
        use std::task::{Context, Poll, Wake, Waker};

        struct NoopWaker;
        impl Wake for NoopWaker {
            fn wake(self: Arc<Self>) {}
        }

        let mut timeline = Timeline::new().add("a", tween(1.0, 1.0), At::Start);
        timeline.play();
        timeline.update(1.0);

        let mut wait = Box::pin(timeline.wait());
        let waker = Waker::from(Arc::new(NoopWaker));
        let mut cx = Context::from_waker(&waker);

        assert!(matches!(wait.as_mut().poll(&mut cx), Poll::Ready(())));
    }

    #[test]
    fn empty_timeline_completes_on_play_and_reports_progress() {
        let mut timeline = Timeline::default();

        assert_eq!(timeline.state(), TimelineState::Idle);
        assert_eq!(timeline.progress(), 1.0);
        timeline.play();

        assert_eq!(timeline.state(), TimelineState::Completed);
        assert!(timeline.is_complete());
        assert!(!timeline.update(1.0));
    }

    #[test]
    fn completed_timeline_restarts_when_played_again() {
        let mut timeline = Timeline::new().add("a", tween(1.0, 1.0), At::Start);

        timeline.play();
        assert!(!timeline.update(1.0));
        timeline.play();

        assert_eq!(timeline.state(), TimelineState::Playing);
        assert_eq!(timeline.elapsed(), 0.0);
        assert_eq!(timeline.get::<Tween<f32>>("a").unwrap().value(), 0.0);
    }

    #[test]
    fn absolute_and_missing_label_positions_are_resolved() {
        let timeline = Timeline::new()
            .add("first", tween(1.0, 0.5), At::Absolute(-1.0))
            .add("second", tween(1.0, 0.5), At::Label("missing"));

        assert_eq!(timeline.entry_count(), 2);
        assert_eq!(timeline.duration(), 1.0);
    }

    #[test]
    fn get_mut_can_edit_child_animation() {
        let mut timeline = Timeline::new().add("a", tween(100.0, 1.0), At::Start);

        timeline.get_mut::<Tween<f32>>("a").unwrap().seek(0.75);

        assert_eq!(timeline.get::<Tween<f32>>("a").unwrap().value(), 75.0);
        assert!(timeline.get::<Tween<f32>>("missing").is_none());
    }

    #[test]
    fn seek_clamps_and_uncompletes_when_returning_from_end() {
        let mut timeline = Timeline::new().add("a", tween(100.0, 1.0), At::Start);

        timeline.seek(2.0);
        assert!(timeline.is_complete());
        timeline.seek_abs(0.25);

        assert_eq!(timeline.state(), TimelineState::Playing);
        assert_eq!(timeline.get::<Tween<f32>>("a").unwrap().value(), 25.0);
        timeline.seek_abs(-5.0);
        assert_eq!(timeline.elapsed(), 0.0);
    }

    #[test]
    fn forever_loop_progress_uses_local_time() {
        let mut timeline = Timeline::new()
            .add("a", tween(100.0, 1.0), At::Start)
            .looping(Loop::Forever);

        timeline.play();
        timeline.update(2.25);

        assert!((timeline.progress() - 0.25).abs() < 0.001);
        assert_eq!(timeline.get::<Tween<f32>>("a").unwrap().value(), 25.0);
        assert!(!timeline.is_complete());
    }

    #[test]
    fn zero_duration_entry_completes_and_fires_once() {
        let mut timeline = Timeline::new().add("instant", tween(1.0, 0.0), At::Start);

        timeline.play();

        assert!(!timeline.update(0.0));
        assert_eq!(timeline.get::<Tween<f32>>("instant").unwrap().value(), 1.0);
    }

    #[test]
    fn negative_update_delta_does_not_advance() {
        let mut timeline = Timeline::new().add("a", tween(100.0, 1.0), At::Start);

        timeline.play();
        assert!(timeline.update(-1.0));

        assert_eq!(timeline.elapsed(), 0.0);
        assert_eq!(timeline.get::<Tween<f32>>("a").unwrap().value(), 0.0);
    }

    #[test]
    fn playable_trait_for_timeline_exposes_downcast_hooks() {
        let mut timeline = Timeline::new().add("a", tween(100.0, 1.0), At::Start);

        assert_eq!(Playable::duration(&timeline), 1.0);
        Playable::seek_to(&mut timeline, 0.5);
        assert_eq!(timeline.get::<Tween<f32>>("a").unwrap().value(), 50.0);
        assert!(Playable::as_any(&timeline).is::<Timeline>());
        assert!(Playable::as_any_mut(&mut timeline).is::<Timeline>());
        Playable::reset(&mut timeline);
        assert_eq!(timeline.state(), TimelineState::Idle);
    }

    #[cfg(feature = "std")]
    #[test]
    fn debug_formats_entries_and_callbacks() {
        let timeline = Timeline::new()
            .add("a", tween(1.0, 1.0), At::Start)
            .on_entry_complete("a", || {})
            .on_complete(|| {});

        let debug = format!("{timeline:?}");

        assert!(debug.contains("Timeline"));
        assert!(debug.contains("entry_complete"));
        assert!(debug.contains("has_complete"));
    }
}
