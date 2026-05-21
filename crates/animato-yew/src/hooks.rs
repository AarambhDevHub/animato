//! Yew `UseStateHandle`-backed animation hooks.

use crate::raf::RafLoop;
use animato_core::{Animatable, Update};
use animato_spring::{Decompose, SpringConfig, SpringN};
use animato_timeline::{Timeline, TimelineState};
use animato_tween::{KeyframeTrack, Tween, TweenBuilder, TweenState};
use std::cell::{Cell, RefCell};
use std::fmt;
use std::rc::Rc;
use yew::prelude::{UseStateHandle, hook, use_state_eq};

/// Control handle for a Yew state-backed [`Tween`].
#[derive(Clone)]
pub struct TweenHandle<T: Animatable + PartialEq + 'static> {
    tween: Rc<RefCell<Tween<T>>>,
    value: UseStateHandle<T>,
    progress: UseStateHandle<f32>,
    complete: UseStateHandle<bool>,
    state: UseStateHandle<TweenState>,
    loop_handle: RafLoop,
}

impl<T: Animatable + PartialEq + 'static> fmt::Debug for TweenHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TweenHandle").finish_non_exhaustive()
    }
}

impl<T: Animatable + PartialEq + 'static> TweenHandle<T> {
    /// Resume playback, resetting first if the tween has completed.
    pub fn play(&self) {
        self.with_tween(|tween| {
            if tween.is_complete() {
                tween.reset();
            }
            tween.resume();
        });
        self.loop_handle.kick();
    }

    /// Pause playback.
    pub fn pause(&self) {
        self.with_tween(Tween::pause);
        self.loop_handle.stop();
    }

    /// Resume playback.
    pub fn resume(&self) {
        self.with_tween(Tween::resume);
        self.loop_handle.kick();
    }

    /// Reset the tween to the beginning.
    pub fn reset(&self) {
        self.with_tween(Tween::reset);
        self.loop_handle.kick();
    }

    /// Reverse direction while preserving the current visual progress.
    pub fn reverse(&self) {
        self.with_tween(Tween::reverse);
        self.loop_handle.kick();
    }

    /// Seek to normalized progress in `[0.0, 1.0]`.
    pub fn seek(&self, progress: f32) {
        self.with_tween(|tween| tween.seek(progress));
        if !*self.complete {
            self.loop_handle.kick();
        }
    }

    /// Set the playback time scale. Non-finite values become `1.0`.
    pub fn set_time_scale(&self, scale: f32) {
        self.with_tween(|tween| {
            tween.time_scale = crate::finite_or(scale, 1.0).max(0.0);
        });
        self.loop_handle.kick();
    }

    /// Current value state handle.
    pub fn value(&self) -> UseStateHandle<T> {
        self.value.clone()
    }

    /// Completion state handle.
    pub fn is_complete(&self) -> UseStateHandle<bool> {
        self.complete.clone()
    }

    /// Raw normalized progress state handle.
    pub fn progress(&self) -> UseStateHandle<f32> {
        self.progress.clone()
    }

    /// Runtime tween state handle.
    pub fn state(&self) -> UseStateHandle<TweenState> {
        self.state.clone()
    }

    /// Deterministically advance the tween by `dt` seconds.
    pub fn tick(&self, dt: f32) -> bool {
        self.tick_inner(dt.max(0.0))
    }

    fn tick_inner(&self, dt: f32) -> bool {
        let running = {
            let mut tween = self.tween.borrow_mut();
            let running = tween.update(dt);
            self.sync(&tween);
            running
        };
        if !running {
            self.loop_handle.stop();
        }
        running
    }

    fn with_tween(&self, f: impl FnOnce(&mut Tween<T>)) {
        let mut tween = self.tween.borrow_mut();
        f(&mut tween);
        self.sync(&tween);
    }

    fn sync(&self, tween: &Tween<T>) {
        set_if_changed(&self.value, tween.value());
        set_if_changed(&self.progress, tween.progress());
        set_if_changed(&self.complete, tween.is_complete());
        set_if_changed(&self.state, tween.state().clone());
    }
}

/// Create a Yew state-backed tween hook.
#[hook]
pub fn use_tween<T>(
    from: T,
    to: T,
    config: impl FnOnce(TweenBuilder<T>) -> TweenBuilder<T>,
) -> (UseStateHandle<T>, TweenHandle<T>)
where
    T: Animatable + PartialEq + 'static,
{
    let tween = Rc::new(RefCell::new(config(Tween::new(from, to)).build()));
    let value = {
        let tween = Rc::clone(&tween);
        use_state_eq(move || tween.borrow().value())
    };
    let progress = {
        let tween = Rc::clone(&tween);
        use_state_eq(move || tween.borrow().progress())
    };
    let complete = {
        let tween = Rc::clone(&tween);
        use_state_eq(move || tween.borrow().is_complete())
    };
    let state = {
        let tween = Rc::clone(&tween);
        use_state_eq(move || tween.borrow().state().clone())
    };

    let loop_handle = RafLoop::new({
        let tween = Rc::clone(&tween);
        let value = value.clone();
        let progress = progress.clone();
        let complete = complete.clone();
        let state = state.clone();
        move |dt| {
            let mut tween = tween.borrow_mut();
            let running = tween.update(dt.max(0.0));
            set_if_changed(&value, tween.value());
            set_if_changed(&progress, tween.progress());
            set_if_changed(&complete, tween.is_complete());
            set_if_changed(&state, tween.state().clone());
            running
        }
    });

    let handle = TweenHandle {
        tween,
        value: value.clone(),
        progress,
        complete,
        state,
        loop_handle,
    };
    if !*handle.complete {
        handle.loop_handle.kick();
    }

    (value, handle)
}

/// Control handle for a Yew state-backed [`SpringN`].
#[derive(Clone)]
pub struct SpringHandle<T: Decompose + Clone + PartialEq + 'static> {
    spring: Rc<RefCell<SpringN<T>>>,
    value: UseStateHandle<T>,
    settled: UseStateHandle<bool>,
    loop_handle: RafLoop,
}

impl<T: Decompose + Clone + PartialEq + 'static> fmt::Debug for SpringHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SpringHandle").finish_non_exhaustive()
    }
}

impl<T: Decompose + Clone + PartialEq + 'static> SpringHandle<T> {
    /// Set a new spring target.
    pub fn set_target(&self, target: T) {
        {
            let mut spring = self.spring.borrow_mut();
            spring.set_target(target);
            self.sync(&spring);
        }
        self.loop_handle.kick();
    }

    /// Snap instantly to a value.
    pub fn snap_to(&self, value: T) {
        self.with_spring(|spring| spring.snap_to(value));
        self.loop_handle.stop();
    }

    /// Current value state handle.
    pub fn value(&self) -> UseStateHandle<T> {
        self.value.clone()
    }

    /// Settled-state handle.
    pub fn is_settled(&self) -> UseStateHandle<bool> {
        self.settled.clone()
    }

    /// Deterministically advance the spring by `dt` seconds.
    pub fn tick(&self, dt: f32) -> bool {
        self.tick_inner(dt.max(0.0))
    }

    fn tick_inner(&self, dt: f32) -> bool {
        let running = {
            let mut spring = self.spring.borrow_mut();
            let running = spring.update(dt);
            self.sync(&spring);
            running
        };
        if !running {
            self.loop_handle.stop();
        }
        running
    }

    fn with_spring(&self, f: impl FnOnce(&mut SpringN<T>)) {
        let mut spring = self.spring.borrow_mut();
        f(&mut spring);
        self.sync(&spring);
    }

    fn sync(&self, spring: &SpringN<T>) {
        set_if_changed(&self.value, spring.position());
        set_if_changed(&self.settled, spring.is_settled());
    }
}

/// Create a Yew state-backed spring hook.
#[hook]
pub fn use_spring<T>(initial: T, config: SpringConfig) -> (UseStateHandle<T>, SpringHandle<T>)
where
    T: Decompose + Clone + PartialEq + 'static,
{
    let spring = Rc::new(RefCell::new(SpringN::new(config, initial.clone())));
    let value = use_state_eq(move || initial);
    let settled = use_state_eq(|| true);
    let loop_handle = RafLoop::new({
        let spring = Rc::clone(&spring);
        let value = value.clone();
        let settled = settled.clone();
        move |dt| {
            let mut spring = spring.borrow_mut();
            let running = spring.update(dt.max(0.0));
            set_if_changed(&value, spring.position());
            set_if_changed(&settled, spring.is_settled());
            running
        }
    });

    let handle = SpringHandle {
        spring,
        value: value.clone(),
        settled,
        loop_handle,
    };

    (value, handle)
}

/// Control handle for a Yew state-backed [`Timeline`].
#[derive(Clone)]
pub struct TimelineHandle {
    timeline: Rc<RefCell<Timeline>>,
    progress: UseStateHandle<f32>,
    complete: UseStateHandle<bool>,
    state: UseStateHandle<TimelineState>,
    loop_handle: RafLoop,
}

impl fmt::Debug for TimelineHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TimelineHandle").finish_non_exhaustive()
    }
}

impl TimelineHandle {
    /// Start timeline playback.
    pub fn play(&self) {
        self.with_timeline(Timeline::play);
        self.loop_handle.kick();
    }

    /// Pause playback.
    pub fn pause(&self) {
        self.with_timeline(Timeline::pause);
        self.loop_handle.stop();
    }

    /// Resume playback.
    pub fn resume(&self) {
        self.with_timeline(Timeline::resume);
        self.loop_handle.kick();
    }

    /// Reset to the beginning.
    pub fn reset(&self) {
        self.with_timeline(Timeline::reset);
        self.loop_handle.kick();
    }

    /// Seek by normalized progress.
    pub fn seek(&self, progress: f32) {
        self.with_timeline(|timeline| timeline.seek(progress));
    }

    /// Change the time scale multiplier.
    pub fn set_time_scale(&self, scale: f32) {
        self.with_timeline(|timeline| timeline.set_time_scale(crate::finite_or(scale, 1.0)));
        self.loop_handle.kick();
    }

    /// Progress state handle.
    pub fn progress(&self) -> UseStateHandle<f32> {
        self.progress.clone()
    }

    /// Completion state handle.
    pub fn is_complete(&self) -> UseStateHandle<bool> {
        self.complete.clone()
    }

    /// Timeline state handle.
    pub fn state(&self) -> UseStateHandle<TimelineState> {
        self.state.clone()
    }

    /// Deterministically advance by `dt` seconds.
    pub fn tick(&self, dt: f32) -> bool {
        self.tick_inner(dt.max(0.0))
    }

    fn tick_inner(&self, dt: f32) -> bool {
        let running = {
            let mut timeline = self.timeline.borrow_mut();
            let running = timeline.update(dt);
            self.sync(&timeline);
            running
        };
        if !running {
            self.loop_handle.stop();
        }
        running
    }

    fn with_timeline(&self, f: impl FnOnce(&mut Timeline)) {
        let mut timeline = self.timeline.borrow_mut();
        f(&mut timeline);
        self.sync(&timeline);
    }

    fn sync(&self, timeline: &Timeline) {
        set_if_changed(&self.progress, timeline.progress());
        set_if_changed(&self.complete, timeline.is_complete());
        set_if_changed(&self.state, timeline.state());
    }
}

/// Create a Yew state-backed timeline hook.
#[hook]
pub fn use_timeline(builder: impl FnOnce(Timeline) -> Timeline) -> TimelineHandle {
    let mut timeline = builder(Timeline::new());
    timeline.play();

    let timeline = Rc::new(RefCell::new(timeline));
    let progress = {
        let timeline = Rc::clone(&timeline);
        use_state_eq(move || timeline.borrow().progress())
    };
    let complete = {
        let timeline = Rc::clone(&timeline);
        use_state_eq(move || timeline.borrow().is_complete())
    };
    let state = {
        let timeline = Rc::clone(&timeline);
        use_state_eq(move || timeline.borrow().state())
    };

    let loop_handle = RafLoop::new({
        let timeline = Rc::clone(&timeline);
        let progress = progress.clone();
        let complete = complete.clone();
        let state = state.clone();
        move |dt| {
            let mut timeline = timeline.borrow_mut();
            let running = timeline.update(dt.max(0.0));
            set_if_changed(&progress, timeline.progress());
            set_if_changed(&complete, timeline.is_complete());
            set_if_changed(&state, timeline.state());
            running
        }
    });

    let handle = TimelineHandle {
        timeline,
        progress,
        complete,
        state,
        loop_handle,
    };
    if !*handle.complete {
        handle.loop_handle.kick();
    }

    handle
}

/// Control handle for a Yew state-backed keyframe track.
#[derive(Clone)]
pub struct KeyframeHandle<T: Animatable + PartialEq + 'static> {
    track: Rc<RefCell<KeyframeTrack<T>>>,
    value: UseStateHandle<T>,
    progress: UseStateHandle<f32>,
    complete: UseStateHandle<bool>,
    paused: Rc<Cell<bool>>,
    time_scale: Rc<Cell<f32>>,
    loop_handle: RafLoop,
}

impl<T: Animatable + PartialEq + 'static> fmt::Debug for KeyframeHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KeyframeHandle").finish_non_exhaustive()
    }
}

impl<T: Animatable + PartialEq + 'static> KeyframeHandle<T> {
    /// Start or resume playback.
    pub fn play(&self) {
        self.resume();
    }

    /// Pause playback.
    pub fn pause(&self) {
        self.paused.set(true);
        self.loop_handle.stop();
    }

    /// Resume playback.
    pub fn resume(&self) {
        self.paused.set(false);
        self.loop_handle.kick();
    }

    /// Reset to the beginning.
    pub fn reset(&self) {
        self.with_track(KeyframeTrack::reset);
        self.loop_handle.kick();
    }

    /// Change playback time scale.
    pub fn set_time_scale(&self, scale: f32) {
        self.time_scale.set(crate::finite_or(scale, 1.0).max(0.0));
        self.loop_handle.kick();
    }

    /// Current value state handle.
    pub fn value(&self) -> UseStateHandle<T> {
        self.value.clone()
    }

    /// Progress state handle.
    pub fn progress(&self) -> UseStateHandle<f32> {
        self.progress.clone()
    }

    /// Completion state handle.
    pub fn is_complete(&self) -> UseStateHandle<bool> {
        self.complete.clone()
    }

    /// Deterministically advance by `dt` seconds.
    pub fn tick(&self, dt: f32) -> bool {
        self.tick_inner(dt.max(0.0))
    }

    fn tick_inner(&self, dt: f32) -> bool {
        if self.paused.get() {
            return true;
        }

        let running = {
            let mut track = self.track.borrow_mut();
            let running = track.update(dt * self.time_scale.get());
            self.sync(&track);
            running
        };
        if !running {
            self.loop_handle.stop();
        }
        running
    }

    fn with_track(&self, f: impl FnOnce(&mut KeyframeTrack<T>)) {
        let mut track = self.track.borrow_mut();
        f(&mut track);
        self.sync(&track);
    }

    fn sync(&self, track: &KeyframeTrack<T>) {
        if let Some(value) = track.value() {
            set_if_changed(&self.value, value);
        }
        set_if_changed(&self.progress, track.progress());
        set_if_changed(&self.complete, track.is_complete());
    }
}

/// Create a Yew state-backed keyframe hook.
///
/// The builder must insert at least one keyframe. Empty tracks are ambiguous
/// because there is no fallback value for the returned state handle.
#[hook]
pub fn use_keyframes<T>(
    builder: impl FnOnce(KeyframeTrack<T>) -> KeyframeTrack<T>,
) -> (UseStateHandle<T>, KeyframeHandle<T>)
where
    T: Animatable + PartialEq + 'static,
{
    let track = Rc::new(RefCell::new(builder(KeyframeTrack::new())));
    let value = {
        let track = Rc::clone(&track);
        use_state_eq(move || {
            track
                .borrow()
                .value()
                .expect("use_keyframes requires at least one keyframe")
        })
    };
    let progress = {
        let track = Rc::clone(&track);
        use_state_eq(move || track.borrow().progress())
    };
    let complete = {
        let track = Rc::clone(&track);
        use_state_eq(move || track.borrow().is_complete())
    };
    let paused = Rc::new(Cell::new(false));
    let time_scale = Rc::new(Cell::new(1.0));

    let loop_handle = RafLoop::new({
        let track = Rc::clone(&track);
        let value = value.clone();
        let progress = progress.clone();
        let complete = complete.clone();
        let paused = Rc::clone(&paused);
        let time_scale = Rc::clone(&time_scale);
        move |dt| {
            if paused.get() {
                return true;
            }
            let mut track = track.borrow_mut();
            let running = track.update(dt.max(0.0) * time_scale.get());
            if let Some(next) = track.value() {
                set_if_changed(&value, next);
            }
            set_if_changed(&progress, track.progress());
            set_if_changed(&complete, track.is_complete());
            running
        }
    });

    let handle = KeyframeHandle {
        track,
        value: value.clone(),
        progress,
        complete,
        paused,
        time_scale,
        loop_handle,
    };
    if !*handle.complete {
        handle.loop_handle.kick();
    }

    (value, handle)
}

fn set_if_changed<T>(state: &UseStateHandle<T>, next: T)
where
    T: PartialEq + 'static,
{
    if **state != next {
        state.set(next);
    }
}
