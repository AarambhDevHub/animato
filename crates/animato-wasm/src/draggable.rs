//! DOM pointer binding for [`animato_physics::DragState`].

use animato_physics::{DragState, InertiaN, PointerData};
use core::fmt;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Element, EventTarget, PointerEvent};

/// DOM element drag binding backed by [`DragState`].
pub struct Draggable {
    element: Element,
    state: Rc<RefCell<DragState>>,
    inertia: Rc<RefCell<Option<InertiaN<[f32; 2]>>>>,
    _listeners: Vec<EventListener>,
}

impl fmt::Debug for Draggable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Draggable")
            .field("position", &self.position())
            .field("is_dragging", &self.is_dragging())
            .finish_non_exhaustive()
    }
}

impl Draggable {
    /// Attach pointer listeners to a DOM element.
    pub fn attach(element: Element, initial_position: [f32; 2]) -> Result<Self, JsValue> {
        let state = Rc::new(RefCell::new(DragState::new(initial_position)));
        let inertia = Rc::new(RefCell::new(None));
        let last_timestamp = Rc::new(RefCell::new(None::<f64>));
        let target: EventTarget = element.clone().dyn_into()?;

        let down_state = Rc::clone(&state);
        let down_last = Rc::clone(&last_timestamp);
        let down = EventListener::pointer(&target, "pointerdown", move |event| {
            event.prevent_default();
            *down_last.borrow_mut() = Some(event.time_stamp());
            down_state
                .borrow_mut()
                .on_pointer_down(pointer_data(&event));
        })?;

        let move_state = Rc::clone(&state);
        let move_last = Rc::clone(&last_timestamp);
        let move_listener = EventListener::pointer(&target, "pointermove", move |event| {
            event.prevent_default();
            let timestamp = event.time_stamp();
            let dt = match move_last.borrow_mut().replace(timestamp) {
                Some(last) => ((timestamp - last) / 1000.0).max(0.0) as f32,
                None => 0.0,
            };
            move_state
                .borrow_mut()
                .on_pointer_move(pointer_data(&event), dt);
        })?;

        let up_state = Rc::clone(&state);
        let up_inertia = Rc::clone(&inertia);
        let up_last = Rc::clone(&last_timestamp);
        let up = EventListener::pointer(&target, "pointerup", move |event| {
            event.prevent_default();
            *up_last.borrow_mut() = None;
            *up_inertia.borrow_mut() = up_state.borrow_mut().on_pointer_up(pointer_data(&event));
        })?;

        Ok(Self {
            element,
            state,
            inertia,
            _listeners: vec![down, move_listener, up],
        })
    }

    /// Borrow the DOM element this draggable is bound to.
    pub fn element(&self) -> &Element {
        &self.element
    }

    /// Current drag position.
    pub fn position(&self) -> [f32; 2] {
        self.state.borrow().position()
    }

    /// Returns `true` while a pointer is captured.
    pub fn is_dragging(&self) -> bool {
        self.state.borrow().is_dragging()
    }

    /// Take inertia produced by the most recent pointer release, if any.
    pub fn take_inertia(&self) -> Option<InertiaN<[f32; 2]>> {
        self.inertia.borrow_mut().take()
    }
}

struct EventListener {
    target: EventTarget,
    kind: &'static str,
    closure: Closure<dyn FnMut(PointerEvent)>,
}

impl fmt::Debug for EventListener {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EventListener")
            .field("kind", &self.kind)
            .finish_non_exhaustive()
    }
}

impl EventListener {
    fn pointer(
        target: &EventTarget,
        kind: &'static str,
        mut callback: impl FnMut(PointerEvent) + 'static,
    ) -> Result<Self, JsValue> {
        let closure = Closure::wrap(Box::new(move |event: PointerEvent| {
            callback(event);
        }) as Box<dyn FnMut(PointerEvent)>);
        target.add_event_listener_with_callback(kind, closure.as_ref().unchecked_ref())?;
        Ok(Self {
            target: target.clone(),
            kind,
            closure,
        })
    }
}

impl Drop for EventListener {
    fn drop(&mut self) {
        let _ = self
            .target
            .remove_event_listener_with_callback(self.kind, self.closure.as_ref().unchecked_ref());
    }
}

fn pointer_data(event: &PointerEvent) -> PointerData {
    let pointer_id = event.pointer_id().max(0) as u64;
    PointerData {
        x: event.client_x() as f32,
        y: event.client_y() as f32,
        pressure: event.pressure(),
        pointer_id,
    }
}
