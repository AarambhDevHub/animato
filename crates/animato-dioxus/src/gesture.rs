//! Pointer, drag, pinch, and swipe helpers.

use animato_core::Update;
use animato_physics::{DragState, InertiaConfig, InertiaN, PointerData};
use dioxus::prelude::{Signal, use_signal};
use std::fmt;
use std::sync::{Arc, Mutex};

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
    state: Arc<Mutex<DragState>>,
    inertia: Arc<Mutex<Option<InertiaN<[f32; 2]>>>>,
    position: Signal<[f32; 2]>,
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
        crate::with_lock(&self.inertia, |inertia| *inertia = None);
        crate::with_lock(&self.state, |state| {
            state.on_pointer_down(PointerData::new(x, y, pointer_id));
            crate::set_signal(self.position, state.position());
        });
    }

    /// Feed a pointer-move sample into the drag tracker.
    pub fn pointer_move(&self, x: f32, y: f32, pointer_id: u64, dt: f32) {
        crate::with_lock(&self.state, |state| {
            state.on_pointer_move(PointerData::new(x, y, pointer_id), dt.max(0.0));
            crate::set_signal(self.position, state.position());
        });
    }

    /// Feed a pointer-up sample into the drag tracker.
    pub fn pointer_up(&self, x: f32, y: f32, pointer_id: u64) {
        let inertia = crate::with_lock(&self.state, |state| {
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
            crate::set_signal(self.position, state.position());
            inertia
        });
        crate::with_lock(&self.inertia, |slot| *slot = inertia);
    }

    /// Replace the active drag constraints and clamp the current position.
    pub fn set_constraints(&self, constraints: Option<DragConstraints>) {
        crate::with_lock(&self.state, |state| {
            state.set_constraints(constraints.unwrap_or_else(DragConstraints::unbounded));
            crate::set_signal(self.position, state.position());
        });
    }

    /// Move instantly to a position, applying the current constraints.
    pub fn snap_to(&self, position: [f32; 2]) {
        crate::with_lock(&self.inertia, |inertia| *inertia = None);
        crate::with_lock(&self.state, |state| {
            state.snap_to(position);
            crate::set_signal(self.position, state.position());
        });
    }

    /// Position signal.
    pub fn position(&self) -> Signal<[f32; 2]> {
        self.position
    }

    /// Advance any post-release inertia by `dt` seconds.
    pub fn tick(&self, dt: f32) -> bool {
        crate::with_lock(&self.inertia, |inertia| {
            if let Some(active) = inertia.as_mut() {
                let running = active.update(dt.max(0.0));
                crate::set_signal(self.position, active.position());
                if !running {
                    *inertia = None;
                }
                running
            } else {
                false
            }
        })
    }
}

/// Create a draggable element hook.
pub fn use_drag<T: 'static>(target: T, config: DragConfig) -> (Signal<[f32; 2]>, DragHandle) {
    let _ = target;
    let initial = [0.0, 0.0];
    let mut state = DragState::new(initial).axis(config.axis);
    if let Some(constraints) = config.constraints {
        state = state.constraints(constraints);
    }
    state = state.inertia_config(config.inertia_config.clone());

    let position = use_signal(|| initial);
    let handle = DragHandle {
        state: Arc::new(Mutex::new(state)),
        inertia: Arc::new(Mutex::new(None)),
        position,
        config,
    };

    let loop_handle = handle.clone();
    crate::spawn_animation_loop(move |dt| {
        loop_handle.tick(dt);
        true
    });

    (position, handle)
}

/// Listen for recognized pointer gestures on a target.
pub fn use_gesture<T: 'static>(target: T, config: GestureConfig) -> Signal<Option<Gesture>> {
    let _ = (target, config);
    use_signal(|| None)
}

/// Handle returned by [`use_pinch`].
#[derive(Clone)]
pub struct PinchHandle {
    scale: Signal<f32>,
}

impl fmt::Debug for PinchHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PinchHandle").finish_non_exhaustive()
    }
}

impl PinchHandle {
    /// Set the pinch scale.
    pub fn set_scale(&self, scale: f32) {
        crate::set_signal(self.scale, crate::finite_or(scale, 1.0).max(0.0));
    }

    /// Reset the pinch scale to `1.0`.
    pub fn reset(&self) {
        crate::set_signal(self.scale, 1.0);
    }

    /// Scale signal.
    pub fn scale(&self) -> Signal<f32> {
        self.scale
    }
}

/// Create a pinch-zoom hook.
pub fn use_pinch<T: 'static>(target: T) -> (Signal<f32>, PinchHandle) {
    let _ = target;
    let scale = use_signal(|| 1.0);
    (scale, PinchHandle { scale })
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
pub fn use_swipe<T: 'static>(target: T, config: SwipeConfig) -> Signal<Option<SwipeEvent>> {
    let _ = (target, config);
    use_signal(|| None)
}

fn nearest_snap(position: [f32; 2], snap_points: &[[f32; 2]]) -> Option<[f32; 2]> {
    snap_points.iter().copied().min_by(|a, b| {
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

#[cfg(test)]
mod tests {
    use super::*;
    use animato_physics::{GestureRecognizer, PointerData as PhysicsPointerData};
    use dioxus::prelude::*;
    use std::cell::RefCell;

    thread_local! {
        static DRAG_CAPTURE: RefCell<Option<(Signal<[f32; 2]>, DragHandle)>> = const { RefCell::new(None) };
        static INERTIA_DRAG_CAPTURE: RefCell<Option<(Signal<[f32; 2]>, DragHandle)>> = const { RefCell::new(None) };
        static PINCH_CAPTURE: RefCell<Option<(Signal<f32>, PinchHandle)>> = const { RefCell::new(None) };
        static GESTURE_CAPTURE: RefCell<Option<Signal<Option<Gesture>>>> = const { RefCell::new(None) };
        static SWIPE_CAPTURE: RefCell<Option<Signal<Option<SwipeEvent>>>> = const { RefCell::new(None) };
    }

    #[allow(non_snake_case)]
    fn DragHookApp() -> Element {
        let pair = use_drag(
            "node",
            DragConfig {
                axis: DragAxis::X,
                constraints: Some(DragConstraints::bounded(0.0, 100.0, 0.0, 100.0)),
                inertia: false,
                snap_points: vec![[0.0, 0.0], [100.0, 0.0]],
                ..DragConfig::default()
            },
        );
        DRAG_CAPTURE.with(|slot| *slot.borrow_mut() = Some(pair));

        rsx! { div {} }
    }

    #[allow(non_snake_case)]
    fn InertiaDragHookApp() -> Element {
        let pair = use_drag(
            "node",
            DragConfig {
                constraints: Some(DragConstraints::bounded(-500.0, 500.0, -500.0, 500.0)),
                inertia: true,
                inertia_config: InertiaConfig::new(500.0, 1.0),
                ..DragConfig::default()
            },
        );
        INERTIA_DRAG_CAPTURE.with(|slot| *slot.borrow_mut() = Some(pair));

        rsx! { div {} }
    }

    #[allow(non_snake_case)]
    fn GestureHookApp() -> Element {
        let gesture = use_gesture("node", GestureConfig::default());
        let pinch = use_pinch("node");
        let swipe = use_swipe("node", SwipeConfig::default());
        GESTURE_CAPTURE.with(|slot| *slot.borrow_mut() = Some(gesture));
        PINCH_CAPTURE.with(|slot| *slot.borrow_mut() = Some(pinch));
        SWIPE_CAPTURE.with(|slot| *slot.borrow_mut() = Some(swipe));

        rsx! { div {} }
    }

    #[test]
    fn nearest_snap_selects_closest_point() {
        let points = [[0.0, 0.0], [10.0, 0.0], [20.0, 0.0]];
        assert_eq!(nearest_snap([12.0, 0.0], &points), Some([10.0, 0.0]));
    }

    #[test]
    fn swipe_config_has_useful_defaults() {
        let config = SwipeConfig::default();
        assert!(config.min_distance > 0.0);
        assert!(config.min_velocity > 0.0);
    }

    #[test]
    fn gesture_recognizer_detects_swipe() {
        let mut recognizer = GestureRecognizer::new(GestureConfig::default());
        recognizer.on_pointer_down(PhysicsPointerData::new(0.0, 0.0, 1), 0.0);
        recognizer.on_pointer_move(PhysicsPointerData::new(100.0, 0.0, 1), 0.1);
        let gesture = recognizer.on_pointer_up(PhysicsPointerData::new(100.0, 0.0, 1), 0.1);

        assert!(matches!(
            gesture,
            Some(Gesture::Swipe {
                direction: SwipeDirection::Right,
                ..
            })
        ));
    }

    #[test]
    fn drag_hook_updates_snaps_clamps_and_stops_without_inertia() {
        DRAG_CAPTURE.with(|slot| *slot.borrow_mut() = None);
        let mut dom = VirtualDom::new(DragHookApp);
        dom.rebuild_in_place();
        let (position, handle) =
            DRAG_CAPTURE.with(|slot| slot.borrow().as_ref().cloned().expect("drag hook captured"));

        assert_eq!(crate::read_signal(position), [0.0, 0.0]);
        handle.pointer_down(0.0, 0.0, 1);
        handle.pointer_move(80.0, 40.0, 99, 0.1);
        assert_eq!(crate::read_signal(position), [0.0, 0.0]);

        handle.pointer_move(80.0, 40.0, 1, 0.1);
        assert_eq!(crate::read_signal(handle.position()), [80.0, 0.0]);
        handle.pointer_up(80.0, 40.0, 1);
        assert_eq!(crate::read_signal(position), [100.0, 0.0]);
        assert!(!handle.tick(0.016));

        handle.set_constraints(Some(DragConstraints::bounded(-10.0, 40.0, -10.0, 40.0)));
        assert_eq!(crate::read_signal(position), [40.0, 0.0]);
        handle.snap_to([5.0, 20.0]);
        assert_eq!(crate::read_signal(position), [5.0, 0.0]);
        handle.set_constraints(None);
        handle.snap_to([f32::INFINITY, f32::NAN]);
        assert_eq!(crate::read_signal(position), [0.0, 0.0]);
    }

    #[test]
    fn drag_hook_runs_release_inertia_until_settled_or_cancelled() {
        INERTIA_DRAG_CAPTURE.with(|slot| *slot.borrow_mut() = None);
        let mut dom = VirtualDom::new(InertiaDragHookApp);
        dom.rebuild_in_place();
        let (position, handle) = INERTIA_DRAG_CAPTURE.with(|slot| {
            slot.borrow()
                .as_ref()
                .cloned()
                .expect("inertia drag hook captured")
        });

        handle.pointer_down(0.0, 0.0, 1);
        handle.pointer_move(100.0, 0.0, 1, 0.01);
        let release_position = crate::read_signal(position);
        handle.pointer_up(100.0, 0.0, 1);
        assert!(handle.tick(0.016));
        assert!(crate::read_signal(position)[0] >= release_position[0]);

        handle.snap_to([12.0, 24.0]);
        assert_eq!(crate::read_signal(position), [12.0, 24.0]);
        assert!(!handle.tick(0.016));
    }

    #[test]
    fn gesture_pinch_and_swipe_hooks_return_stable_signals() {
        GESTURE_CAPTURE.with(|slot| *slot.borrow_mut() = None);
        PINCH_CAPTURE.with(|slot| *slot.borrow_mut() = None);
        SWIPE_CAPTURE.with(|slot| *slot.borrow_mut() = None);
        let mut dom = VirtualDom::new(GestureHookApp);
        dom.rebuild_in_place();

        let gesture = GESTURE_CAPTURE.with(|slot| {
            slot.borrow()
                .as_ref()
                .copied()
                .expect("gesture signal captured")
        });
        let (scale, pinch) = PINCH_CAPTURE.with(|slot| {
            slot.borrow()
                .as_ref()
                .cloned()
                .expect("pinch hook captured")
        });
        let swipe = SWIPE_CAPTURE.with(|slot| {
            slot.borrow()
                .as_ref()
                .copied()
                .expect("swipe signal captured")
        });

        assert_eq!(crate::read_signal(gesture), None);
        assert_eq!(crate::read_signal(swipe), None);
        assert_eq!(crate::read_signal(scale), 1.0);

        pinch.set_scale(f32::NAN);
        assert_eq!(crate::read_signal(pinch.scale()), 1.0);
        pinch.set_scale(-2.0);
        assert_eq!(crate::read_signal(scale), 0.0);
        pinch.reset();
        assert_eq!(crate::read_signal(scale), 1.0);
    }
}
