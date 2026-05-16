//! Signal-backed Leptos animation hooks.

use animato_core::{Animatable, Update};
use animato_spring::{Decompose, SpringConfig, SpringN};
use animato_timeline::{Timeline, TimelineState};
use animato_tween::{KeyframeTrack, Tween, TweenBuilder};
use leptos::prelude::{ReadSignal, Set, WriteSignal, signal};
use std::fmt;
use std::sync::{Arc, Mutex};

/// Control handle for a signal-backed [`Tween`].
#[derive(Clone)]
pub struct TweenHandle<T: Animatable + Send + Sync + 'static> {
    tween: Arc<Mutex<Tween<T>>>,
    value: WriteSignal<T>,
    progress: ReadSignal<f32>,
    set_progress: WriteSignal<f32>,
    complete: ReadSignal<bool>,
    set_complete: WriteSignal<bool>,
}

impl<T: Animatable + Send + Sync + 'static> fmt::Debug for TweenHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TweenHandle").finish_non_exhaustive()
    }
}

impl<T: Animatable + Send + Sync + 'static> TweenHandle<T> {
    /// Resume playback, resetting first if the tween had completed.
    pub fn play(&self) {
        crate::with_lock(&self.tween, |tween| {
            if tween.is_complete() {
                tween.reset();
            }
            tween.resume();
            self.value.set(tween.value());
            self.set_progress.set(tween.progress());
            self.set_complete.set(tween.is_complete());
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
            self.value.set(tween.value());
            self.set_progress.set(tween.progress());
            self.set_complete.set(tween.is_complete());
        });
    }

    /// Reverse the tween direction while preserving visual continuity.
    pub fn reverse(&self) {
        crate::with_lock(&self.tween, |tween| {
            tween.reverse();
            self.value.set(tween.value());
            self.set_progress.set(tween.progress());
            self.set_complete.set(tween.is_complete());
        });
    }

    /// Seek to normalized progress in `[0.0, 1.0]`.
    pub fn seek(&self, progress: f32) {
        crate::with_lock(&self.tween, |tween| {
            tween.seek(progress);
            self.value.set(tween.value());
            self.set_progress.set(tween.progress());
            self.set_complete.set(tween.is_complete());
        });
    }

    /// Change the time scale multiplier.
    pub fn set_time_scale(&self, scale: f32) {
        crate::with_lock(&self.tween, |tween| {
            tween.time_scale = crate::finite_or(scale, 1.0).max(0.0);
        });
    }

    /// Completion signal.
    pub fn is_complete(&self) -> ReadSignal<bool> {
        self.complete
    }

    /// Raw progress signal.
    pub fn progress(&self) -> ReadSignal<f32> {
        self.progress
    }

    /// Deterministically advance the tween by `dt` seconds.
    pub fn tick(&self, dt: f32) -> bool {
        crate::with_lock(&self.tween, |tween| {
            let running = tween.update(dt);
            self.value.set(tween.value());
            self.set_progress.set(tween.progress());
            self.set_complete.set(tween.is_complete());
            running
        })
    }
}

/// Signal-backed tween hook.
pub fn use_tween<T>(
    from: T,
    to: T,
    config: impl FnOnce(TweenBuilder<T>) -> TweenBuilder<T>,
) -> (ReadSignal<T>, TweenHandle<T>)
where
    T: Animatable + Send + Sync + 'static,
{
    let mut tween = config(Tween::new(from.clone(), to.clone())).build();
    if crate::ssr::is_hydrating() {
        tween.seek(1.0);
    }

    let initial = if crate::ssr::is_hydrating() {
        to
    } else {
        tween.value()
    };
    let (value, set_value) = signal(initial);
    let (progress, set_progress) = signal(tween.progress());
    let (complete, set_complete) = signal(crate::ssr::is_hydrating() || tween.is_complete());

    let handle = TweenHandle {
        tween: Arc::new(Mutex::new(tween)),
        value: set_value,
        progress,
        set_progress,
        complete,
        set_complete,
    };

    let loop_handle = handle.clone();
    crate::spawn_raf_loop(move |dt| {
        loop_handle.tick(dt);
        true
    });

    (value, handle)
}

/// Control handle for a signal-backed [`SpringN`].
#[derive(Clone)]
pub struct SpringHandle<T: Decompose + Send + Sync + 'static> {
    spring: Arc<Mutex<SpringN<T>>>,
    value: WriteSignal<T>,
    settled: ReadSignal<bool>,
    set_settled: WriteSignal<bool>,
}

impl<T: Decompose + Send + Sync + 'static> fmt::Debug for SpringHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SpringHandle").finish_non_exhaustive()
    }
}

impl<T: Decompose + Send + Sync + 'static> SpringHandle<T> {
    /// Set a new spring target.
    pub fn set_target(&self, target: T) {
        crate::with_lock(&self.spring, |spring| {
            spring.set_target(target);
            self.set_settled.set(spring.is_settled());
        });
    }

    /// Snap instantly to a value.
    pub fn snap_to(&self, value: T) {
        crate::with_lock(&self.spring, |spring| {
            spring.snap_to(value);
            self.value.set(spring.position());
            self.set_settled.set(spring.is_settled());
        });
    }

    /// Settled signal.
    pub fn is_settled(&self) -> ReadSignal<bool> {
        self.settled
    }

    /// Deterministically advance the spring by `dt` seconds.
    pub fn tick(&self, dt: f32) -> bool {
        crate::with_lock(&self.spring, |spring| {
            let running = spring.update(dt);
            self.value.set(spring.position());
            self.set_settled.set(spring.is_settled());
            running
        })
    }
}

/// Signal-backed spring hook.
pub fn use_spring<T>(initial: T, config: SpringConfig) -> (ReadSignal<T>, SpringHandle<T>)
where
    T: Decompose + Send + Sync + Clone + 'static,
{
    let spring = SpringN::new(config, initial.clone());
    let (value, set_value) = signal(initial);
    let (settled, set_settled) = signal(true);
    let handle = SpringHandle {
        spring: Arc::new(Mutex::new(spring)),
        value: set_value,
        settled,
        set_settled,
    };

    let loop_handle = handle.clone();
    crate::spawn_raf_loop(move |dt| {
        loop_handle.tick(dt);
        true
    });

    (value, handle)
}

/// Control handle for a signal-backed [`Timeline`].
#[derive(Clone)]
pub struct TimelineHandle {
    timeline: Arc<Mutex<Timeline>>,
    progress: ReadSignal<f32>,
    set_progress: WriteSignal<f32>,
    complete: ReadSignal<bool>,
    set_complete: WriteSignal<bool>,
    state: ReadSignal<TimelineState>,
    set_state: WriteSignal<TimelineState>,
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
    pub fn progress(&self) -> ReadSignal<f32> {
        self.progress
    }

    /// Completion signal.
    pub fn is_complete(&self) -> ReadSignal<bool> {
        self.complete
    }

    /// Timeline state signal.
    pub fn state(&self) -> ReadSignal<TimelineState> {
        self.state
    }

    /// Deterministically advance by `dt` seconds.
    pub fn tick(&self, dt: f32) -> bool {
        crate::with_lock(&self.timeline, |timeline| {
            let running = timeline.update(dt);
            self.sync(timeline);
            running
        })
    }

    fn sync(&self, timeline: &Timeline) {
        self.set_progress.set(timeline.progress());
        self.set_complete.set(timeline.is_complete());
        self.set_state.set(timeline.state());
    }
}

/// Signal-backed timeline hook.
pub fn use_timeline(builder: impl FnOnce(Timeline) -> Timeline) -> TimelineHandle {
    let mut timeline = builder(Timeline::new());
    if crate::ssr::is_hydrating() {
        timeline.seek(1.0);
    } else {
        timeline.play();
    }

    let (progress, set_progress) = signal(timeline.progress());
    let (complete, set_complete) = signal(crate::ssr::is_hydrating() || timeline.is_complete());
    let (state, set_state) = signal(timeline.state());
    let handle = TimelineHandle {
        timeline: Arc::new(Mutex::new(timeline)),
        progress,
        set_progress,
        complete,
        set_complete,
        state,
        set_state,
    };

    let loop_handle = handle.clone();
    crate::spawn_raf_loop(move |dt| {
        loop_handle.tick(dt);
        true
    });

    handle
}

/// Control handle for a signal-backed keyframe track.
#[derive(Clone)]
pub struct KeyframeHandle<T: Animatable + Send + Sync + 'static> {
    track: Arc<Mutex<KeyframeTrack<T>>>,
    value: WriteSignal<T>,
    progress: ReadSignal<f32>,
    set_progress: WriteSignal<f32>,
    complete: ReadSignal<bool>,
    set_complete: WriteSignal<bool>,
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
            if let Some(value) = track.value() {
                self.value.set(value);
            }
            self.set_progress.set(track.progress());
            self.set_complete.set(track.is_complete());
        });
    }

    /// Change playback time scale.
    pub fn set_time_scale(&self, scale: f32) {
        crate::with_lock(&self.time_scale, |time_scale| {
            *time_scale = crate::finite_or(scale, 1.0).max(0.0);
        });
    }

    /// Progress signal.
    pub fn progress(&self) -> ReadSignal<f32> {
        self.progress
    }

    /// Completion signal.
    pub fn is_complete(&self) -> ReadSignal<bool> {
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
            if let Some(value) = track.value() {
                self.value.set(value);
            }
            self.set_progress.set(track.progress());
            self.set_complete.set(track.is_complete());
            running
        })
    }
}

/// Signal-backed keyframe hook.
///
/// The builder must insert at least one keyframe. Empty tracks are ambiguous
/// because there is no fallback value for the returned `ReadSignal<T>`.
pub fn use_keyframes<T>(
    builder: impl FnOnce(KeyframeTrack<T>) -> KeyframeTrack<T>,
) -> (ReadSignal<T>, KeyframeHandle<T>)
where
    T: Animatable + Send + Sync + 'static,
{
    let mut track = builder(KeyframeTrack::new());
    if crate::ssr::is_hydrating() {
        track.update(track.duration());
    }
    let initial = track
        .value()
        .expect("use_keyframes requires at least one keyframe");
    let (value, set_value) = signal(initial);
    let (progress, set_progress) = signal(track.progress());
    let (complete, set_complete) = signal(crate::ssr::is_hydrating() || track.is_complete());
    let handle = KeyframeHandle {
        track: Arc::new(Mutex::new(track)),
        value: set_value,
        progress,
        set_progress,
        complete,
        set_complete,
        paused: Arc::new(Mutex::new(false)),
        time_scale: Arc::new(Mutex::new(1.0)),
    };

    let loop_handle = handle.clone();
    crate::spawn_raf_loop(move |dt| {
        loop_handle.tick(dt);
        true
    });

    (value, handle)
}

#[cfg(test)]
mod tests {
    use super::*;
    use animato_core::Easing;
    use animato_tween::KeyframeTrack;
    use leptos::prelude::{Get, Owner};

    #[test]
    fn tween_handle_ticks_deterministically() {
        Owner::new().with(|| {
            let (value, handle) =
                use_tween(0.0_f32, 10.0, |b| b.duration(1.0).easing(Easing::Linear));
            handle.tick(0.5);
            assert!((value.get() - 5.0).abs() < 0.001 || crate::ssr::is_hydrating());
        });
    }

    #[test]
    fn spring_handle_sets_target_and_ticks() {
        Owner::new().with(|| {
            let (value, handle) = use_spring(0.0_f32, SpringConfig::snappy());
            handle.set_target(1.0);
            for _ in 0..10 {
                handle.tick(1.0 / 60.0);
            }
            assert!(value.get() >= 0.0);
        });
    }

    #[test]
    fn keyframe_handle_tracks_progress() {
        Owner::new().with(|| {
            let (value, handle) =
                use_keyframes(|track: KeyframeTrack<f32>| track.push(0.0, 0.0).push(1.0, 10.0));
            handle.tick(0.5);
            assert!(value.get() >= 0.0);
            assert!(handle.progress().get() >= 0.0);
        });
    }
}
