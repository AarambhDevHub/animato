//! Pointer, drag, pinch, and swipe helpers.

use animato_core::Update;
use animato_physics::{DragState, InertiaConfig, InertiaN, PointerData};
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use yew::prelude::{NodeRef, UseStateHandle, hook, use_state_eq};

pub use animato_physics::{DragAxis, DragConstraints, Gesture, GestureConfig, SwipeDirection};

/// Draggable element configuration.
#[derive(Clone, Debug, PartialEq)]
pub struct DragConfig {
    /// Drag axis.
    pub axis: DragAxis,
    /// Optional drag constraints.
    pub constraints: Option<DragConstraints>,
    /// Enable inertia after pointer release.
    pub inertia: bool,
    /// Inertia configuration.
    pub inertia_config: InertiaConfig<[f32; 2]>,
    /// Snap-to points after release.
    pub snap_points: Vec<[f32; 2]>,
    /// Allow elastic edge behavior at constraints.
    pub elastic_edges: bool,
}

impl Default for DragConfig {
    fn default() -> Self {
        Self {
            axis: DragAxis::Both,
            constraints: None,
            inertia: true,
            inertia_config: InertiaConfig::new(1400.0, 2.0),
            snap_points: Vec::new(),
            elastic_edges: false,
        }
    }
}

/// Handle returned by [`use_drag`].
#[derive(Clone)]
pub struct DragHandle {
    state: Rc<RefCell<DragState>>,
    inertia: Rc<RefCell<Option<InertiaN<[f32; 2]>>>>,
    position: UseStateHandle<[f32; 2]>,
    config: DragConfig,
}

impl fmt::Debug for DragHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DragHandle")
            .field("config", &self.config)
            .finish_non_exhaustive()
    }
}

impl DragHandle {
    /// Feed a pointer-down sample into the drag tracker.
    pub fn pointer_down(&self, x: f32, y: f32, pointer_id: u64) {
        *self.inertia.borrow_mut() = None;
        let mut state = self.state.borrow_mut();
        state.on_pointer_down(PointerData::new(x, y, pointer_id));
        set_if_changed(&self.position, state.position());
    }

    /// Feed a pointer-move sample into the drag tracker.
    pub fn pointer_move(&self, x: f32, y: f32, pointer_id: u64, dt: f32) {
        let mut state = self.state.borrow_mut();
        state.on_pointer_move(PointerData::new(x, y, pointer_id), dt);
        set_if_changed(&self.position, state.position());
    }

    /// Feed a pointer-up sample into the drag tracker.
    pub fn pointer_up(&self, x: f32, y: f32, pointer_id: u64) {
        let inertia = {
            let mut state = self.state.borrow_mut();
            let inertia = if self.config.inertia {
                state.on_pointer_up(PointerData::new(x, y, pointer_id))
            } else {
                let _ = state.on_pointer_up(PointerData::new(x, y, pointer_id));
                None
            };
            if inertia.is_none()
                && let Some(snapped) = nearest_snap(state.position(), &self.config.snap_points)
            {
                state.snap_to(snapped);
            }
            set_if_changed(&self.position, state.position());
            inertia
        };
        *self.inertia.borrow_mut() = inertia;
    }

    /// Replace the active drag constraints and clamp the current position.
    pub fn set_constraints(&self, constraints: Option<DragConstraints>) {
        let mut state = self.state.borrow_mut();
        state.set_constraints(constraints.unwrap_or_else(DragConstraints::unbounded));
        set_if_changed(&self.position, state.position());
    }

    /// Move instantly to a position, applying the current constraints.
    pub fn snap_to(&self, position: [f32; 2]) {
        *self.inertia.borrow_mut() = None;
        let mut state = self.state.borrow_mut();
        state.snap_to(position);
        set_if_changed(&self.position, state.position());
    }

    /// Current drag position state handle.
    pub fn position(&self) -> UseStateHandle<[f32; 2]> {
        self.position.clone()
    }

    /// Advance any post-release inertia by `dt` seconds.
    pub fn tick(&self, dt: f32) -> bool {
        let mut inertia = self.inertia.borrow_mut();
        if let Some(active) = inertia.as_mut() {
            let running = active.update(dt.max(0.0));
            set_if_changed(&self.position, active.position());
            if !running {
                *inertia = None;
            }
            running
        } else {
            false
        }
    }
}

/// Create a draggable element hook.
#[hook]
pub fn use_drag(target: NodeRef, config: DragConfig) -> (UseStateHandle<[f32; 2]>, DragHandle) {
    let initial = [0.0, 0.0];
    let mut state = DragState::new(initial).axis(config.axis);
    if let Some(constraints) = config.constraints {
        state = state.constraints(constraints);
    }
    state = state.inertia_config(config.inertia_config.clone());

    let position = use_state_eq(move || initial);
    let handle = DragHandle {
        state: Rc::new(RefCell::new(state)),
        inertia: Rc::new(RefCell::new(None)),
        position: position.clone(),
        config,
    };

    #[cfg(target_arch = "wasm32")]
    install_drag_listeners(target.clone(), handle.clone());

    #[cfg(not(target_arch = "wasm32"))]
    let _ = target;

    (position, handle)
}

/// Listen for recognized pointer gestures on a target.
#[hook]
pub fn use_gesture(target: NodeRef, config: GestureConfig) -> UseStateHandle<Option<Gesture>> {
    let gesture = use_state_eq(|| None);

    #[cfg(target_arch = "wasm32")]
    install_gesture_listeners(target.clone(), config, gesture.clone());

    #[cfg(not(target_arch = "wasm32"))]
    let _ = (target, config);

    gesture
}

/// Handle returned by [`use_pinch`].
#[derive(Clone)]
pub struct PinchHandle {
    scale: UseStateHandle<f32>,
}

impl fmt::Debug for PinchHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PinchHandle").finish_non_exhaustive()
    }
}

impl PinchHandle {
    /// Set the pinch scale.
    pub fn set_scale(&self, scale: f32) {
        set_if_changed(&self.scale, crate::finite_or(scale, 1.0).max(0.0));
    }

    /// Reset the pinch scale to `1.0`.
    pub fn reset(&self) {
        set_if_changed(&self.scale, 1.0);
    }
}

/// Create a pinch-zoom hook.
#[hook]
pub fn use_pinch(target: NodeRef) -> (UseStateHandle<f32>, PinchHandle) {
    let scale = use_state_eq(|| 1.0);

    #[cfg(target_arch = "wasm32")]
    install_pinch_listeners(target.clone(), scale.clone());

    #[cfg(not(target_arch = "wasm32"))]
    let _ = target;

    (
        scale.clone(),
        PinchHandle {
            scale: scale.clone(),
        },
    )
}

/// Swipe detection configuration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SwipeConfig {
    /// Minimum distance required to emit a swipe.
    pub min_distance: f32,
    /// Minimum velocity required to emit a swipe.
    pub min_velocity: f32,
}

impl Default for SwipeConfig {
    fn default() -> Self {
        Self {
            min_distance: 40.0,
            min_velocity: 100.0,
        }
    }
}

/// Swipe event emitted by [`use_swipe`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SwipeEvent {
    /// Swipe direction.
    pub direction: SwipeDirection,
    /// Swipe velocity in pixels per second.
    pub velocity: f32,
    /// Swipe distance in pixels.
    pub distance: f32,
}

/// Listen for swipe gestures on a target.
#[hook]
pub fn use_swipe(target: NodeRef, config: SwipeConfig) -> UseStateHandle<Option<SwipeEvent>> {
    let swipe = use_state_eq(|| None);

    #[cfg(target_arch = "wasm32")]
    install_swipe_listeners(target.clone(), config, swipe.clone());

    #[cfg(not(target_arch = "wasm32"))]
    let _ = (target, config);

    swipe
}

fn nearest_snap(position: [f32; 2], points: &[[f32; 2]]) -> Option<[f32; 2]> {
    points.iter().copied().min_by(|a, b| {
        distance_sq(position, *a)
            .partial_cmp(&distance_sq(position, *b))
            .unwrap_or(std::cmp::Ordering::Equal)
    })
}

fn distance_sq(a: [f32; 2], b: [f32; 2]) -> f32 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    dx * dx + dy * dy
}

fn set_if_changed<T>(state: &UseStateHandle<T>, next: T)
where
    T: PartialEq + 'static,
{
    if **state != next {
        state.set(next);
    }
}

#[cfg(target_arch = "wasm32")]
fn install_drag_listeners(_target: NodeRef, _handle: DragHandle) {}

#[cfg(target_arch = "wasm32")]
fn install_gesture_listeners(
    _target: NodeRef,
    _config: GestureConfig,
    _gesture: UseStateHandle<Option<Gesture>>,
) {
}

#[cfg(target_arch = "wasm32")]
fn install_pinch_listeners(_target: NodeRef, _scale: UseStateHandle<f32>) {}

#[cfg(target_arch = "wasm32")]
fn install_swipe_listeners(
    _target: NodeRef,
    _config: SwipeConfig,
    _swipe: UseStateHandle<Option<SwipeEvent>>,
) {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nearest_snap_selects_closest_point() {
        let points = [[0.0, 0.0], [10.0, 0.0], [20.0, 0.0]];
        assert_eq!(nearest_snap([12.0, 0.0], &points), Some([10.0, 0.0]));
    }

    #[test]
    fn configs_are_constructible() {
        let drag = DragConfig::default();
        assert_eq!(drag.axis, DragAxis::Both);
        assert!(drag.inertia);

        let swipe = SwipeConfig::default();
        assert_eq!(swipe.min_distance, 40.0);
        assert_eq!(swipe.min_velocity, 100.0);
    }

    #[test]
    fn snap_distance_handles_empty_and_non_finite_comparisons() {
        assert_eq!(nearest_snap([0.0, 0.0], &[]), None);
        assert_eq!(
            nearest_snap([f32::NAN, 0.0], &[[1.0, 0.0], [2.0, 0.0]]),
            Some([1.0, 0.0])
        );
        assert_eq!(distance_sq([1.0, 2.0], [4.0, 6.0]), 25.0);

        let event = SwipeEvent {
            direction: SwipeDirection::Right,
            velocity: 120.0,
            distance: 80.0,
        };
        assert!(format!("{event:?}").contains("Right"));
    }
}
