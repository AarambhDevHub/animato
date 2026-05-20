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

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use dioxus::prelude::*;
    use std::cell::RefCell;

    thread_local! {
        static MOTION_CAPTURE: RefCell<Option<MotionHandle<f32>>> = const { RefCell::new(None) };
    }

    #[allow(non_snake_case)]
    fn MotionHookApp() -> Element {
        let handle = use_motion(0.0_f32);
        MOTION_CAPTURE.with(|slot| *slot.borrow_mut() = Some(handle));

        rsx! { div {} }
    }

    fn mount_motion() -> (VirtualDom, MotionHandle<f32>) {
        MOTION_CAPTURE.with(|slot| *slot.borrow_mut() = None);
        let mut dom = VirtualDom::new(MotionHookApp);
        dom.rebuild_in_place();
        let handle = MOTION_CAPTURE.with(|slot| {
            slot.borrow()
                .as_ref()
                .cloned()
                .expect("motion hook captured")
        });
        (dom, handle)
    }

    #[test]
    fn motion_tween_stop_and_snap_are_deterministic() {
        let (_dom, handle) = mount_motion();

        assert_relative_eq!(handle.value(), 0.0);
        assert!(!handle.is_animating());
        assert!(!handle.tick(0.1));

        handle.animate_to(
            10.0,
            MotionConfig::Tween {
                duration: 1.0,
                easing: Easing::Linear,
                delay: 0.0,
            },
        );
        assert!(handle.is_animating());
        assert!(handle.tick(0.25));
        assert_relative_eq!(handle.value(), 2.5, epsilon = 0.001);
        assert_relative_eq!(crate::read_signal(handle.signal()), 2.5, epsilon = 0.001);

        handle.stop();
        assert!(!handle.is_animating());
        assert!(!handle.tick(0.25));
        assert_relative_eq!(handle.value(), 2.5, epsilon = 0.001);

        handle.snap_to(7.0);
        assert_relative_eq!(handle.value(), 7.0, epsilon = 0.001);
        assert!(!handle.is_animating());
    }

    #[test]
    fn motion_tween_delay_spring_and_keyframes_update_value() {
        let (_dom, handle) = mount_motion();

        handle.animate_to(
            10.0,
            MotionConfig::Tween {
                duration: 1.0,
                easing: Easing::Linear,
                delay: 0.25,
            },
        );
        assert!(handle.tick(0.1));
        assert_relative_eq!(handle.value(), 0.0, epsilon = 0.001);
        assert!(handle.tick(0.15));
        assert_relative_eq!(handle.value(), 0.0, epsilon = 0.001);
        assert!(handle.tick(0.25));
        assert_relative_eq!(handle.value(), 2.5, epsilon = 0.001);

        handle.animate_to(1.0, MotionConfig::Spring(SpringConfig::snappy()));
        assert!(handle.is_animating());
        assert!(handle.tick(1.0 / 60.0));
        assert!(handle.value() < 2.5);

        handle.keyframes(
            KeyframeTrack::new()
                .push(0.0, 4.0_f32)
                .push(0.5, 8.0)
                .push(1.0, 12.0),
        );
        assert!(handle.tick(0.5));
        assert_relative_eq!(handle.value(), 8.0, epsilon = 0.001);
        assert!(!handle.tick(0.5));
        assert_relative_eq!(handle.value(), 12.0, epsilon = 0.001);
        assert!(!handle.is_animating());
    }
}
