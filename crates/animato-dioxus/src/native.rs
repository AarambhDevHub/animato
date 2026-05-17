//! Portable native-window animation helpers.

use animato_core::Update;
use animato_spring::{SpringConfig, SpringN};
use animato_tween::{Tween, TweenBuilder};
use std::fmt;
use std::sync::{Arc, Mutex};

/// Portable native window state tracked by Animato.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NativeWindowState {
    /// Window position in physical pixels.
    pub position: [f32; 2],
    /// Window size in physical pixels.
    pub size: [f32; 2],
    /// Window opacity in `[0.0, 1.0]`.
    pub opacity: f32,
}

impl Default for NativeWindowState {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0],
            size: [800.0, 600.0],
            opacity: 1.0,
        }
    }
}

/// Handle for tween-driven native window animation.
#[derive(Clone)]
pub struct WindowAnimationHandle {
    state: Arc<Mutex<NativeWindowState>>,
    position: Arc<Mutex<Option<Tween<[f32; 2]>>>>,
    size: Arc<Mutex<Option<Tween<[f32; 2]>>>>,
    opacity: Arc<Mutex<Option<Tween<f32>>>>,
}

impl fmt::Debug for WindowAnimationHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WindowAnimationHandle")
            .field("state", &self.state())
            .finish_non_exhaustive()
    }
}

impl WindowAnimationHandle {
    /// Current portable window state.
    pub fn state(&self) -> NativeWindowState {
        *self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// Animate the tracked window position.
    pub fn move_to(&self, x: f32, y: f32) {
        let start = self.state().position;
        let tween = Tween::new(start, [x, y]).duration(0.25).build();
        crate::with_lock(&self.position, |slot| *slot = Some(tween));
    }

    /// Animate the tracked window size.
    pub fn resize_to(&self, w: f32, h: f32) {
        let start = self.state().size;
        let tween = Tween::new(start, [w.max(1.0), h.max(1.0)])
            .duration(0.25)
            .build();
        crate::with_lock(&self.size, |slot| *slot = Some(tween));
    }

    /// Animate the tracked window opacity.
    pub fn opacity_to(&self, opacity: f32) {
        let start = self.state().opacity;
        let tween = Tween::new(start, opacity.clamp(0.0, 1.0))
            .duration(0.25)
            .build();
        crate::with_lock(&self.opacity, |slot| *slot = Some(tween));
    }

    /// Snap to a complete portable window state.
    pub fn snap_to(&self, state: NativeWindowState) {
        crate::with_lock(&self.state, |current| *current = state);
        crate::with_lock(&self.position, |slot| *slot = None);
        crate::with_lock(&self.size, |slot| *slot = None);
        crate::with_lock(&self.opacity, |slot| *slot = None);
    }

    /// Deterministically advance all active window tweens.
    pub fn tick(&self, dt: f32) -> bool {
        let mut running = false;
        let dt = dt.max(0.0);

        crate::with_lock(&self.position, |slot| {
            if let Some(tween) = slot.as_mut() {
                running |= tween.update(dt);
                let value = tween.value();
                crate::with_lock(&self.state, |state| state.position = value);
                if tween.is_complete() {
                    *slot = None;
                }
            }
        });

        crate::with_lock(&self.size, |slot| {
            if let Some(tween) = slot.as_mut() {
                running |= tween.update(dt);
                let value = tween.value();
                crate::with_lock(&self.state, |state| state.size = value);
                if tween.is_complete() {
                    *slot = None;
                }
            }
        });

        crate::with_lock(&self.opacity, |slot| {
            if let Some(tween) = slot.as_mut() {
                running |= tween.update(dt);
                let value = tween.value();
                crate::with_lock(&self.state, |state| state.opacity = value);
                if tween.is_complete() {
                    *slot = None;
                }
            }
        });

        running
    }
}

/// Spring-driven native window handle.
#[derive(Clone)]
pub struct WindowSpringHandle {
    state: Arc<Mutex<NativeWindowState>>,
    position: Arc<Mutex<SpringN<[f32; 2]>>>,
}

impl fmt::Debug for WindowSpringHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WindowSpringHandle")
            .field("state", &self.state())
            .finish_non_exhaustive()
    }
}

impl WindowSpringHandle {
    /// Current portable window state.
    pub fn state(&self) -> NativeWindowState {
        *self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// Spring the tracked window position to a target.
    pub fn move_to(&self, x: f32, y: f32) {
        crate::with_lock(&self.position, |spring| spring.set_target([x, y]));
    }

    /// Snap instantly to a tracked window position.
    pub fn snap_to(&self, x: f32, y: f32) {
        crate::with_lock(&self.position, |spring| spring.snap_to([x, y]));
        crate::with_lock(&self.state, |state| state.position = [x, y]);
    }

    /// Deterministically advance the spring.
    pub fn tick(&self, dt: f32) -> bool {
        crate::with_lock(&self.position, |spring| {
            let running = spring.update(dt.max(0.0));
            let position = spring.position();
            crate::with_lock(&self.state, |state| state.position = position);
            running
        })
    }
}

/// Create a tween-driven native window animation handle.
pub fn use_window_animation(
    config: impl FnOnce(TweenBuilder<[f32; 2]>) -> TweenBuilder<[f32; 2]>,
) -> WindowAnimationHandle {
    let _ = config(Tween::new([0.0, 0.0], [0.0, 0.0]));
    WindowAnimationHandle {
        state: Arc::new(Mutex::new(NativeWindowState::default())),
        position: Arc::new(Mutex::new(None)),
        size: Arc::new(Mutex::new(None)),
        opacity: Arc::new(Mutex::new(None)),
    }
}

/// Create a spring-driven native window position handle.
pub fn use_window_spring(config: SpringConfig) -> WindowSpringHandle {
    WindowSpringHandle {
        state: Arc::new(Mutex::new(NativeWindowState::default())),
        position: Arc::new(Mutex::new(SpringN::new(config, [0.0, 0.0]))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_animation_tracks_state() {
        let handle = use_window_animation(|builder| builder.duration(0.1));
        handle.move_to(100.0, 50.0);
        assert!(handle.tick(0.05));
        assert!(handle.state().position[0] >= 0.0);
        handle.tick(0.1);
        assert_eq!(handle.state().position, [100.0, 50.0]);
    }

    #[test]
    fn window_spring_snap_updates_state() {
        let handle = use_window_spring(SpringConfig::snappy());
        handle.snap_to(12.0, 24.0);
        assert_eq!(handle.state().position, [12.0, 24.0]);
    }
}
