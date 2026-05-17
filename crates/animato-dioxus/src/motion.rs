//! Unified Dioxus motion hook.

use animato_core::{Easing, Update};
use animato_spring::{Decompose, SpringConfig, SpringN};
use animato_tween::{KeyframeTrack, Tween};
use dioxus::prelude::{Signal, use_signal};
use std::fmt;
use std::sync::{Arc, Mutex};

/// Motion transition configuration.
#[derive(Clone, Debug)]
pub enum MotionConfig {
    /// Tween to a target using duration, easing, and delay.
    Tween {
        /// Duration in seconds.
        duration: f32,
        /// Easing curve.
        easing: Easing,
        /// Start delay in seconds.
        delay: f32,
    },
    /// Spring to a target using a spring configuration.
    Spring(SpringConfig),
}

enum ActiveMotion<T: Decompose + Send + Sync + Clone + 'static> {
    Idle,
    Tween(Tween<T>),
    Spring(SpringN<T>),
    Keyframes(KeyframeTrack<T>),
}

/// All-in-one motion handle.
#[derive(Clone)]
pub struct MotionHandle<T: Decompose + Send + Sync + Clone + 'static> {
    value: Signal<T>,
    active: Arc<Mutex<ActiveMotion<T>>>,
}

impl<T: Decompose + Send + Sync + Clone + 'static> fmt::Debug for MotionHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MotionHandle").finish_non_exhaustive()
    }
}

impl<T: Decompose + Send + Sync + Clone + 'static> MotionHandle<T> {
    /// Current reactive value signal.
    pub fn signal(&self) -> Signal<T> {
        self.value
    }

    /// Current value snapshot.
    pub fn value(&self) -> T {
        crate::read_signal(self.value)
    }

    /// Animate to a target with tween or spring configuration.
    pub fn animate_to(&self, target: T, config: MotionConfig) {
        match config {
            MotionConfig::Tween {
                duration,
                easing,
                delay,
            } => {
                let tween = Tween::new(self.value(), target)
                    .duration(duration.max(0.0))
                    .delay(delay.max(0.0))
                    .easing(easing)
                    .build();
                crate::with_lock(&self.active, |active| *active = ActiveMotion::Tween(tween));
            }
            MotionConfig::Spring(config) => self.spring_to(target, config),
        }
    }

    /// Spring to a target.
    pub fn spring_to(&self, target: T, config: SpringConfig) {
        let mut spring = SpringN::new(config, self.value());
        spring.set_target(target);
        crate::with_lock(&self.active, |active| {
            *active = ActiveMotion::Spring(spring)
        });
    }

    /// Play a keyframe track.
    pub fn keyframes(&self, track: KeyframeTrack<T>) {
        crate::with_lock(&self.active, |active| {
            *active = ActiveMotion::Keyframes(track)
        });
    }

    /// Stop the active animation without changing the current value.
    pub fn stop(&self) {
        crate::with_lock(&self.active, |active| *active = ActiveMotion::Idle);
    }

    /// Snap instantly to a value and stop animation.
    pub fn snap_to(&self, value: T) {
        crate::set_signal(self.value, value);
        self.stop();
    }

    /// Returns `true` while an animation is active.
    pub fn is_animating(&self) -> bool {
        crate::with_lock(&self.active, |active| !matches!(active, ActiveMotion::Idle))
    }

    /// Deterministically advance the active animation by `dt` seconds.
    pub fn tick(&self, dt: f32) -> bool {
        crate::with_lock(&self.active, |active| match active {
            ActiveMotion::Idle => false,
            ActiveMotion::Tween(tween) => {
                let running = tween.update(dt.max(0.0));
                crate::set_signal(self.value, tween.value());
                if !running {
                    *active = ActiveMotion::Idle;
                }
                running
            }
            ActiveMotion::Spring(spring) => {
                let running = spring.update(dt.max(0.0));
                crate::set_signal(self.value, spring.position());
                if !running {
                    *active = ActiveMotion::Idle;
                }
                running
            }
            ActiveMotion::Keyframes(track) => {
                let running = track.update(dt.max(0.0));
                if let Some(value) = track.value() {
                    crate::set_signal(self.value, value);
                }
                if !running {
                    *active = ActiveMotion::Idle;
                }
                running
            }
        })
    }
}

/// Create an all-in-one motion hook.
pub fn use_motion<T>(initial: T) -> MotionHandle<T>
where
    T: Decompose + Send + Sync + Clone + 'static,
{
    let value = use_signal(move || initial);
    let handle = MotionHandle {
        value,
        active: Arc::new(Mutex::new(ActiveMotion::Idle)),
    };

    let loop_handle = handle.clone();
    crate::spawn_animation_loop(move |dt| {
        loop_handle.tick(dt);
        true
    });

    handle
}
