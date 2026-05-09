//! Unified DOM input event conversion.

use web_sys::{PointerEvent, WheelEvent};

/// Normalized browser input event.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ObserverEvent {
    /// Pointer button/touch contact started.
    PointerDown {
        /// X coordinate in CSS pixels.
        x: f32,
        /// Y coordinate in CSS pixels.
        y: f32,
        /// Browser pointer id.
        pointer_id: i32,
    },
    /// Pointer moved.
    PointerMove {
        /// X coordinate in CSS pixels.
        x: f32,
        /// Y coordinate in CSS pixels.
        y: f32,
        /// Browser pointer id.
        pointer_id: i32,
    },
    /// Pointer button/touch contact ended.
    PointerUp {
        /// X coordinate in CSS pixels.
        x: f32,
        /// Y coordinate in CSS pixels.
        y: f32,
        /// Browser pointer id.
        pointer_id: i32,
    },
    /// Wheel or trackpad delta.
    Wheel {
        /// Horizontal delta.
        delta_x: f32,
        /// Vertical delta.
        delta_y: f32,
    },
}

/// Stateless converter from browser events into [`ObserverEvent`].
#[derive(Clone, Copy, Debug, Default)]
pub struct Observer;

impl Observer {
    /// Convert a `pointerdown` event.
    pub fn pointer_down(event: &PointerEvent) -> ObserverEvent {
        ObserverEvent::PointerDown {
            x: event.client_x() as f32,
            y: event.client_y() as f32,
            pointer_id: event.pointer_id(),
        }
    }

    /// Convert a `pointermove` event.
    pub fn pointer_move(event: &PointerEvent) -> ObserverEvent {
        ObserverEvent::PointerMove {
            x: event.client_x() as f32,
            y: event.client_y() as f32,
            pointer_id: event.pointer_id(),
        }
    }

    /// Convert a `pointerup` event.
    pub fn pointer_up(event: &PointerEvent) -> ObserverEvent {
        ObserverEvent::PointerUp {
            x: event.client_x() as f32,
            y: event.client_y() as f32,
            pointer_id: event.pointer_id(),
        }
    }

    /// Convert a `wheel` event.
    pub fn wheel(event: &WheelEvent) -> ObserverEvent {
        ObserverEvent::Wheel {
            delta_x: event.delta_x() as f32,
            delta_y: event.delta_y() as f32,
        }
    }
}
