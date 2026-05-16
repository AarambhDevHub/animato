//! Pointer, drag, pinch, and swipe helpers.

use animato_core::Update;
use animato_physics::{
    DragAxis, DragConstraints, DragState, Gesture, GestureConfig, InertiaConfig, InertiaN,
    PointerData, SwipeDirection,
};
use leptos::html;
use leptos::prelude::*;
#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
use std::cell::Cell;
use std::fmt;
#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
use std::rc::Rc;
use std::sync::{Arc, Mutex};
#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
use wasm_bindgen::JsCast;

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
    position: WriteSignal<[f32; 2]>,
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
            self.position.set(state.position());
        });
    }

    /// Feed a pointer-move sample into the drag tracker.
    pub fn pointer_move(&self, x: f32, y: f32, pointer_id: u64, dt: f32) {
        crate::with_lock(&self.state, |state| {
            state.on_pointer_move(PointerData::new(x, y, pointer_id), dt);
            self.position.set(state.position());
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
                state.on_pointer_down(PointerData::new(x, y, pointer_id));
                state.on_pointer_move(PointerData::new(snapped[0], snapped[1], pointer_id), 1.0);
                let _ = state.on_pointer_up(PointerData::new(snapped[0], snapped[1], pointer_id));
            }
            self.position.set(state.position());
            inertia
        });
        crate::with_lock(&self.inertia, |slot| *slot = inertia);
    }

    /// Advance any post-release inertia by `dt` seconds.
    pub fn tick(&self, dt: f32) -> bool {
        crate::with_lock(&self.inertia, |inertia| {
            if let Some(active) = inertia.as_mut() {
                let running = active.update(dt);
                self.position.set(active.position());
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
pub fn use_drag(
    target: NodeRef<html::Div>,
    config: DragConfig,
) -> (ReadSignal<[f32; 2]>, DragHandle) {
    let initial = [0.0, 0.0];
    let mut state = DragState::new(initial).axis(config.axis);
    if let Some(constraints) = config.constraints {
        state = state.constraints(constraints);
    }
    state = state.inertia_config(config.inertia_config.clone());

    let (position, set_position) = signal(initial);
    let handle = DragHandle {
        state: Arc::new(Mutex::new(state)),
        inertia: Arc::new(Mutex::new(None)),
        position: set_position,
        config,
    };

    let loop_handle = handle.clone();
    crate::spawn_raf_loop(move |dt| {
        loop_handle.tick(dt);
        true
    });

    #[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
    if !crate::ssr::is_hydrating() {
        install_drag_listeners(target, handle.clone());
    }

    #[cfg(not(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate"))))]
    let _ = target;

    (position, handle)
}

/// Listen for recognized pointer gestures on a target.
pub fn use_gesture(
    target: NodeRef<html::Div>,
    config: GestureConfig,
) -> ReadSignal<Option<Gesture>> {
    let (gesture, set_gesture) = signal(None);

    #[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
    if !crate::ssr::is_hydrating() {
        install_gesture_listeners(target, config, set_gesture);
    }

    #[cfg(not(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate"))))]
    let _ = (target, config, set_gesture);

    gesture
}

/// Handle returned by [`use_pinch`].
#[derive(Clone)]
pub struct PinchHandle {
    scale: WriteSignal<f32>,
}

impl fmt::Debug for PinchHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PinchHandle").finish_non_exhaustive()
    }
}

impl PinchHandle {
    /// Set the pinch scale.
    pub fn set_scale(&self, scale: f32) {
        self.scale.set(crate::finite_or(scale, 1.0).max(0.0));
    }

    /// Reset the pinch scale to `1.0`.
    pub fn reset(&self) {
        self.scale.set(1.0);
    }
}

/// Create a pinch-zoom hook.
pub fn use_pinch(target: NodeRef<html::Div>) -> (ReadSignal<f32>, PinchHandle) {
    let (scale, set_scale) = signal(1.0);

    #[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
    if !crate::ssr::is_hydrating() {
        install_pinch_listeners(target, set_scale);
    }

    #[cfg(not(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate"))))]
    let _ = target;

    (scale, PinchHandle { scale: set_scale })
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
pub fn use_swipe(
    target: NodeRef<html::Div>,
    config: SwipeConfig,
) -> ReadSignal<Option<SwipeEvent>> {
    let (swipe, set_swipe) = signal(None);

    #[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
    if !crate::ssr::is_hydrating() {
        install_swipe_listeners(target, config, set_swipe);
    }

    #[cfg(not(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate"))))]
    let _ = (target, config, set_swipe);

    swipe
}

#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
fn install_drag_listeners(target: NodeRef<html::Div>, handle: DragHandle) {
    use leptos::ev;
    use leptos::leptos_dom::helpers::window_event_listener;

    let active_pointer = Rc::new(Cell::new(None::<u64>));
    let last_time = Rc::new(Cell::new(now_seconds()));

    let down_handle = {
        let active_pointer = Rc::clone(&active_pointer);
        let last_time = Rc::clone(&last_time);
        let handle = handle.clone();
        window_event_listener(ev::pointerdown, move |event| {
            if !event_in_target(target, &event) {
                return;
            }

            let pointer_id = pointer_id(&event);
            active_pointer.set(Some(pointer_id));
            last_time.set(now_seconds());
            handle.pointer_down(event.client_x() as f32, event.client_y() as f32, pointer_id);
        })
    };

    let move_handle = {
        let active_pointer = Rc::clone(&active_pointer);
        let last_time = Rc::clone(&last_time);
        let handle = handle.clone();
        window_event_listener(ev::pointermove, move |event| {
            let pointer_id = pointer_id(&event);
            if active_pointer.get() != Some(pointer_id) {
                return;
            }

            let now = now_seconds();
            let dt = (now - last_time.replace(now)).max(0.0);
            handle.pointer_move(
                event.client_x() as f32,
                event.client_y() as f32,
                pointer_id,
                dt,
            );
        })
    };

    let up_handle = {
        let active_pointer = Rc::clone(&active_pointer);
        let handle = handle.clone();
        window_event_listener(ev::pointerup, move |event| {
            let pointer_id = pointer_id(&event);
            if active_pointer.get() != Some(pointer_id) {
                return;
            }

            handle.pointer_up(event.client_x() as f32, event.client_y() as f32, pointer_id);
            active_pointer.set(None);
        })
    };

    let cancel_handle = {
        let active_pointer = Rc::clone(&active_pointer);
        window_event_listener(ev::pointercancel, move |event| {
            if active_pointer.get() == Some(pointer_id(&event)) {
                active_pointer.set(None);
            }
        })
    };

    on_cleanup(move || {
        down_handle.remove();
        move_handle.remove();
        up_handle.remove();
        cancel_handle.remove();
    });
}

#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
fn install_gesture_listeners(
    target: NodeRef<html::Div>,
    config: GestureConfig,
    set_gesture: WriteSignal<Option<Gesture>>,
) {
    use animato_physics::GestureRecognizer;
    use leptos::ev;
    use leptos::leptos_dom::helpers::window_event_listener;

    let recognizer = Rc::new(std::cell::RefCell::new(GestureRecognizer::new(config)));

    let down_handle = {
        let recognizer = Rc::clone(&recognizer);
        window_event_listener(ev::pointerdown, move |event| {
            if event_in_target(target, &event) {
                recognizer
                    .borrow_mut()
                    .on_pointer_down(pointer_data(&event), now_seconds());
            }
        })
    };

    let move_handle = {
        let recognizer = Rc::clone(&recognizer);
        window_event_listener(ev::pointermove, move |event| {
            recognizer
                .borrow_mut()
                .on_pointer_move(pointer_data(&event), now_seconds());
        })
    };

    let up_handle = {
        let recognizer = Rc::clone(&recognizer);
        window_event_listener(ev::pointerup, move |event| {
            if let Some(gesture) = recognizer
                .borrow_mut()
                .on_pointer_up(pointer_data(&event), now_seconds())
            {
                set_gesture.set(Some(gesture));
            }
        })
    };

    let cancel_handle = window_event_listener(ev::pointercancel, move |_| {});

    on_cleanup(move || {
        down_handle.remove();
        move_handle.remove();
        up_handle.remove();
        cancel_handle.remove();
    });
}

#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
fn install_pinch_listeners(target: NodeRef<html::Div>, set_scale: WriteSignal<f32>) {
    use animato_physics::{Gesture, GestureRecognizer};
    use leptos::ev;
    use leptos::leptos_dom::helpers::window_event_listener;

    let recognizer = Rc::new(std::cell::RefCell::new(GestureRecognizer::default()));

    let down_handle = {
        let recognizer = Rc::clone(&recognizer);
        window_event_listener(ev::pointerdown, move |event| {
            if event_in_target(target, &event) {
                recognizer
                    .borrow_mut()
                    .on_pointer_down(pointer_data(&event), now_seconds());
            }
        })
    };

    let move_handle = {
        let recognizer = Rc::clone(&recognizer);
        window_event_listener(ev::pointermove, move |event| {
            recognizer
                .borrow_mut()
                .on_pointer_move(pointer_data(&event), now_seconds());
        })
    };

    let up_handle = {
        let recognizer = Rc::clone(&recognizer);
        window_event_listener(ev::pointerup, move |event| {
            if let Some(Gesture::Pinch { scale, .. }) = recognizer
                .borrow_mut()
                .on_pointer_up(pointer_data(&event), now_seconds())
            {
                set_scale.set(crate::finite_or(scale, 1.0).max(0.0));
            }
        })
    };

    let cancel_handle = window_event_listener(ev::pointercancel, move |_| {});

    on_cleanup(move || {
        down_handle.remove();
        move_handle.remove();
        up_handle.remove();
        cancel_handle.remove();
    });
}

#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
fn install_swipe_listeners(
    target: NodeRef<html::Div>,
    config: SwipeConfig,
    set_swipe: WriteSignal<Option<SwipeEvent>>,
) {
    use animato_physics::{Gesture, GestureRecognizer};
    use leptos::ev;
    use leptos::leptos_dom::helpers::window_event_listener;

    let gesture_config = GestureConfig {
        swipe_min_distance: config.min_distance,
        ..GestureConfig::default()
    };
    let recognizer = Rc::new(std::cell::RefCell::new(GestureRecognizer::new(
        gesture_config,
    )));

    let down_handle = {
        let recognizer = Rc::clone(&recognizer);
        window_event_listener(ev::pointerdown, move |event| {
            if event_in_target(target, &event) {
                recognizer
                    .borrow_mut()
                    .on_pointer_down(pointer_data(&event), now_seconds());
            }
        })
    };

    let move_handle = {
        let recognizer = Rc::clone(&recognizer);
        window_event_listener(ev::pointermove, move |event| {
            recognizer
                .borrow_mut()
                .on_pointer_move(pointer_data(&event), now_seconds());
        })
    };

    let up_handle = {
        let recognizer = Rc::clone(&recognizer);
        window_event_listener(ev::pointerup, move |event| {
            if let Some(Gesture::Swipe {
                direction,
                velocity,
                distance,
            }) = recognizer
                .borrow_mut()
                .on_pointer_up(pointer_data(&event), now_seconds())
                && velocity >= config.min_velocity
            {
                set_swipe.set(Some(SwipeEvent {
                    direction,
                    velocity,
                    distance,
                }));
            }
        })
    };

    let cancel_handle = window_event_listener(ev::pointercancel, move |_| {});

    on_cleanup(move || {
        down_handle.remove();
        move_handle.remove();
        up_handle.remove();
        cancel_handle.remove();
    });
}

#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
fn pointer_data(event: &web_sys::PointerEvent) -> PointerData {
    PointerData::new(
        event.client_x() as f32,
        event.client_y() as f32,
        pointer_id(event),
    )
}

#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
fn pointer_id(event: &web_sys::PointerEvent) -> u64 {
    u64::try_from(event.pointer_id()).unwrap_or(0)
}

#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
fn now_seconds() -> f32 {
    (crate::now_ms() / 1000.0) as f32
}

#[cfg(all(target_arch = "wasm32", any(feature = "csr", feature = "hydrate")))]
fn event_in_target(target: NodeRef<html::Div>, event: &web_sys::PointerEvent) -> bool {
    let Some(element) = target.get_untracked() else {
        return false;
    };
    let Some(event_target) = event.target() else {
        return false;
    };
    let Some(event_node) = event_target.dyn_ref::<web_sys::Node>() else {
        return false;
    };

    let element_node: &web_sys::Node = element.as_ref();
    element_node.contains(Some(event_node))
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
    use animato_physics::GestureRecognizer;
    use leptos::prelude::{Get, Owner};

    #[test]
    fn drag_handle_updates_position() {
        Owner::new().with(|| {
            let (position, handle) = use_drag(NodeRef::new(), DragConfig::default());
            handle.pointer_down(0.0, 0.0, 1);
            handle.pointer_move(12.0, 6.0, 1, 0.016);
            assert_eq!(position.get(), [12.0, 6.0]);
        });
    }

    #[test]
    fn gesture_recognizer_detects_swipe() {
        let mut recognizer = GestureRecognizer::new(GestureConfig::default());
        recognizer.on_pointer_down(PointerData::new(0.0, 0.0, 1), 0.0);
        recognizer.on_pointer_move(PointerData::new(100.0, 0.0, 1), 0.1);
        let gesture = recognizer.on_pointer_up(PointerData::new(100.0, 0.0, 1), 0.1);

        assert!(matches!(
            gesture,
            Some(Gesture::Swipe {
                direction: SwipeDirection::Right,
                ..
            })
        ));
    }
}
