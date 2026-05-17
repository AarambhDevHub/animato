//! Dioxus `Signal`-backed animation hooks.

use animato_core::{Animatable, Update};
use animato_spring::{Decompose, SpringConfig, SpringN};
use animato_timeline::{Timeline, TimelineState};
use animato_tween::{KeyframeTrack, Tween, TweenBuilder};
use dioxus::prelude::{Signal, use_signal};
use std::fmt;
use std::sync::{Arc, Mutex};

/// Control handle for a Dioxus signal-backed [`Tween`].
#[derive(Clone)]
pub struct TweenHandle<T: Animatable + Send + Sync + 'static> {
    tween: Arc<Mutex<Tween<T>>>,
    value: Signal<T>,
    progress: Signal<f32>,
    complete: Signal<bool>,
}

impl<T: Animatable + Send + Sync + 'static> fmt::Debug for TweenHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TweenHandle").finish_non_exhaustive()
    }
}

impl<T: Animatable + Send + Sync + 'static> TweenHandle<T> {
    /// Resume playback, resetting first if the tween has completed.
    pub fn play(&self) {
        crate::with_lock(&self.tween, |tween| {
            if tween.is_complete() {
                tween.reset();
            }
            tween.resume();
            self.sync(tween);
        });
    }

    /// Pause playback.
    pub fn pause(&self) {
        crate::with_lock(&self.tween, Tween::pause);
    }

    /// Resume playback.
    pub fn resume(&self) {
        crate::with_lock(&self.tween, Tween::resume);
    }

    /// Reset the tween to the beginning.
    pub fn reset(&self) {
        crate::with_lock(&self.tween, |tween| {
            tween.reset();
            self.sync(tween);
        });
    }

    /// Reverse direction while preserving the current visual progress.
    pub fn reverse(&self) {
        crate::with_lock(&self.tween, |tween| {
            tween.reverse();
            self.sync(tween);
        });
    }

    /// Seek to normalized progress in `[0.0, 1.0]`.
    pub fn seek(&self, progress: f32) {
        crate::with_lock(&self.tween, |tween| {
            tween.seek(progress);
            self.sync(tween);
        });
    }

    /// Set the playback time scale. Non-finite values become `1.0`.
    pub fn set_time_scale(&self, scale: f32) {
        crate::with_lock(&self.tween, |tween| {
            tween.time_scale = crate::finite_or(scale, 1.0).max(0.0);
        });
    }

    /// Current value signal.
    pub fn value(&self) -> Signal<T> {
        self.value
    }

    /// Completion signal.
    pub fn is_complete(&self) -> Signal<bool> {
        self.complete
    }

    /// Raw normalized progress signal.
    pub fn progress(&self) -> Signal<f32> {
        self.progress
    }

    /// Deterministically advance the tween by `dt` seconds.
    pub fn tick(&self, dt: f32) -> bool {
        crate::with_lock(&self.tween, |tween| {
            let running = tween.update(dt.max(0.0));
            self.sync(tween);
            running
        })
    }

    fn sync(&self, tween: &Tween<T>) {
        crate::set_signal(self.value, tween.value());
        crate::set_signal(self.progress, tween.progress());
        crate::set_signal(self.complete, tween.is_complete());
    }
}

/// Create a signal-backed tween hook.
pub fn use_tween<T>(
    from: T,
    to: T,
    config: impl FnOnce(TweenBuilder<T>) -> TweenBuilder<T>,
) -> (Signal<T>, TweenHandle<T>)
where
    T: Animatable + Send + Sync + 'static,
{
    let tween = config(Tween::new(from, to)).build();
    let value = use_signal({
        let initial = tween.value();
        move || initial
    });
    let progress = use_signal({
        let initial = tween.progress();
        move || initial
    });
    let complete = use_signal({
        let initial = tween.is_complete();
        move || initial
    });

    let handle = TweenHandle {
        tween: Arc::new(Mutex::new(tween)),
        value,
        progress,
        complete,
    };

    let loop_handle = handle.clone();
    crate::spawn_animation_loop(move |dt| {
        loop_handle.tick(dt);
        true
    });

    (value, handle)
}

/// Control handle for a Dioxus signal-backed [`SpringN`].
#[derive(Clone)]
pub struct SpringHandle<T: Decompose + Send + Sync + Clone + 'static> {
    spring: Arc<Mutex<SpringN<T>>>,
    value: Signal<T>,
    settled: Signal<bool>,
}

impl<T: Decompose + Send + Sync + Clone + 'static> fmt::Debug for SpringHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SpringHandle").finish_non_exhaustive()
    }
}

impl<T: Decompose + Send + Sync + Clone + 'static> SpringHandle<T> {
    /// Set a new spring target.
    pub fn set_target(&self, target: T) {
        crate::with_lock(&self.spring, |spring| {
            spring.set_target(target);
            crate::set_signal(self.settled, spring.is_settled());
        });
    }

    /// Snap instantly to a value.
    pub fn snap_to(&self, value: T) {
        crate::with_lock(&self.spring, |spring| {
            spring.snap_to(value);
            self.sync(spring);
        });
    }

    /// Current value signal.
    pub fn value(&self) -> Signal<T> {
        self.value
    }

    /// Settled-state signal.
    pub fn is_settled(&self) -> Signal<bool> {
        self.settled
    }

    /// Deterministically advance the spring by `dt` seconds.
    pub fn tick(&self, dt: f32) -> bool {
        crate::with_lock(&self.spring, |spring| {
            let running = spring.update(dt.max(0.0));
            self.sync(spring);
            running
        })
    }

    fn sync(&self, spring: &SpringN<T>) {
        crate::set_signal(self.value, spring.position());
        crate::set_signal(self.settled, spring.is_settled());
    }
}

/// Create a signal-backed spring hook.
pub fn use_spring<T>(initial: T, config: SpringConfig) -> (Signal<T>, SpringHandle<T>)
where
    T: Decompose + Send + Sync + Clone + 'static,
{
    let spring = SpringN::new(config, initial.clone());
    let value = use_signal(move || initial);
    let settled = use_signal(|| true);
    let handle = SpringHandle {
        spring: Arc::new(Mutex::new(spring)),
        value,
        settled,
    };

    let loop_handle = handle.clone();
    crate::spawn_animation_loop(move |dt| {
        loop_handle.tick(dt);
        true
    });

    (value, handle)
}

/// Control handle for a Dioxus signal-backed [`Timeline`].
#[derive(Clone)]
pub struct TimelineHandle {
    timeline: Arc<Mutex<Timeline>>,
    progress: Signal<f32>,
    complete: Signal<bool>,
    state: Signal<TimelineState>,
}

impl fmt::Debug for TimelineHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TimelineHandle").finish_non_exhaustive()
    }
}

impl TimelineHandle {
    /// Start timeline playback.
    pub fn play(&self) {
        crate::with_lock(&self.timeline, |timeline| {
            timeline.play();
            self.sync(timeline);
        });
    }

    /// Pause playback.
    pub fn pause(&self) {
        crate::with_lock(&self.timeline, |timeline| {
            timeline.pause();
            self.sync(timeline);
        });
    }

    /// Resume playback.
    pub fn resume(&self) {
        crate::with_lock(&self.timeline, |timeline| {
            timeline.resume();
            self.sync(timeline);
        });
    }

    /// Reset to the beginning.
    pub fn reset(&self) {
        crate::with_lock(&self.timeline, |timeline| {
            timeline.reset();
            self.sync(timeline);
        });
    }

    /// Seek by normalized progress.
    pub fn seek(&self, progress: f32) {
        crate::with_lock(&self.timeline, |timeline| {
            timeline.seek(progress);
            self.sync(timeline);
        });
    }

    /// Change the time scale multiplier.
    pub fn set_time_scale(&self, scale: f32) {
        crate::with_lock(&self.timeline, |timeline| {
            timeline.set_time_scale(scale);
            self.sync(timeline);
        });
    }

    /// Progress signal.
    pub fn progress(&self) -> Signal<f32> {
        self.progress
    }

    /// Completion signal.
    pub fn is_complete(&self) -> Signal<bool> {
        self.complete
    }

    /// Timeline state signal.
    pub fn state(&self) -> Signal<TimelineState> {
        self.state
    }

    /// Deterministically advance by `dt` seconds.
    pub fn tick(&self, dt: f32) -> bool {
        crate::with_lock(&self.timeline, |timeline| {
            let running = timeline.update(dt.max(0.0));
            self.sync(timeline);
            running
        })
    }

    fn sync(&self, timeline: &Timeline) {
        crate::set_signal(self.progress, timeline.progress());
        crate::set_signal(self.complete, timeline.is_complete());
        crate::set_signal(self.state, timeline.state());
    }
}

/// Create a signal-backed timeline hook.
pub fn use_timeline(builder: impl FnOnce(Timeline) -> Timeline) -> TimelineHandle {
    let mut timeline = builder(Timeline::new());
    timeline.play();

    let progress = use_signal({
        let initial = timeline.progress();
        move || initial
    });
    let complete = use_signal({
        let initial = timeline.is_complete();
        move || initial
    });
    let state = use_signal({
        let initial = timeline.state();
        move || initial
    });
    let handle = TimelineHandle {
        timeline: Arc::new(Mutex::new(timeline)),
        progress,
        complete,
        state,
    };

    let loop_handle = handle.clone();
    crate::spawn_animation_loop(move |dt| {
        loop_handle.tick(dt);
        true
    });

    handle
}

/// Control handle for a signal-backed keyframe track.
#[derive(Clone)]
pub struct KeyframeHandle<T: Animatable + Send + Sync + 'static> {
    track: Arc<Mutex<KeyframeTrack<T>>>,
    value: Signal<T>,
    progress: Signal<f32>,
    complete: Signal<bool>,
    paused: Arc<Mutex<bool>>,
    time_scale: Arc<Mutex<f32>>,
}

impl<T: Animatable + Send + Sync + 'static> fmt::Debug for KeyframeHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KeyframeHandle").finish_non_exhaustive()
    }
}

impl<T: Animatable + Send + Sync + 'static> KeyframeHandle<T> {
    /// Start or resume playback.
    pub fn play(&self) {
        self.resume();
    }

    /// Pause playback.
    pub fn pause(&self) {
        crate::with_lock(&self.paused, |paused| *paused = true);
    }

    /// Resume playback.
    pub fn resume(&self) {
        crate::with_lock(&self.paused, |paused| *paused = false);
    }

    /// Reset to the beginning.
    pub fn reset(&self) {
        crate::with_lock(&self.track, |track| {
            track.reset();
            self.sync(track);
        });
    }

    /// Change playback time scale.
    pub fn set_time_scale(&self, scale: f32) {
        crate::with_lock(&self.time_scale, |time_scale| {
            *time_scale = crate::finite_or(scale, 1.0).max(0.0);
        });
    }

    /// Current value signal.
    pub fn value(&self) -> Signal<T> {
        self.value
    }

    /// Progress signal.
    pub fn progress(&self) -> Signal<f32> {
        self.progress
    }

    /// Completion signal.
    pub fn is_complete(&self) -> Signal<bool> {
        self.complete
    }

    /// Deterministically advance by `dt` seconds.
    pub fn tick(&self, dt: f32) -> bool {
        let paused = crate::with_lock(&self.paused, |paused| *paused);
        if paused {
            return true;
        }

        let time_scale = crate::with_lock(&self.time_scale, |scale| *scale);
        crate::with_lock(&self.track, |track| {
            let running = track.update(dt.max(0.0) * time_scale);
            self.sync(track);
            running
        })
    }

    fn sync(&self, track: &KeyframeTrack<T>) {
        if let Some(value) = track.value() {
            crate::set_signal(self.value, value);
        }
        crate::set_signal(self.progress, track.progress());
        crate::set_signal(self.complete, track.is_complete());
    }
}

/// Create a signal-backed keyframe track hook.
///
/// The builder must insert at least one keyframe. Empty tracks are ambiguous
/// because there is no fallback value for the returned `Signal<T>`.
pub fn use_keyframes<T>(
    builder: impl FnOnce(KeyframeTrack<T>) -> KeyframeTrack<T>,
) -> (Signal<T>, KeyframeHandle<T>)
where
    T: Animatable + Send + Sync + 'static,
{
    let track = builder(KeyframeTrack::new());
    let initial = track
        .value()
        .expect("use_keyframes requires at least one keyframe");
    let progress_value = track.progress();
    let complete_value = track.is_complete();

    let value = use_signal(move || initial);
    let progress = use_signal(move || progress_value);
    let complete = use_signal(move || complete_value);
    let handle = KeyframeHandle {
        track: Arc::new(Mutex::new(track)),
        value,
        progress,
        complete,
        paused: Arc::new(Mutex::new(false)),
        time_scale: Arc::new(Mutex::new(1.0)),
    };

    let loop_handle = handle.clone();
    crate::spawn_animation_loop(move |dt| {
        loop_handle.tick(dt);
        true
    });

    (value, handle)
}
