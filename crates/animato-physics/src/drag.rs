//! Pointer drag tracking with constraints and velocity estimation.

#[cfg(any(feature = "std", feature = "alloc"))]
use crate::inertia::{InertiaBounds, InertiaConfig, InertiaN};

/// Pointer sample used by drag and gesture systems.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PointerData {
    /// Pointer x coordinate.
    pub x: f32,
    /// Pointer y coordinate.
    pub y: f32,
    /// Pointer pressure, usually in `[0.0, 1.0]`.
    pub pressure: f32,
    /// Stable pointer identifier supplied by the input backend.
    pub pointer_id: u64,
}

impl PointerData {
    /// Create a pointer sample with default pressure `1.0`.
    pub fn new(x: f32, y: f32, pointer_id: u64) -> Self {
        Self {
            x,
            y,
            pressure: 1.0,
            pointer_id,
        }
    }

    /// Return the pointer position as `[x, y]`.
    pub fn position(&self) -> [f32; 2] {
        [self.x, self.y]
    }
}

/// Axis filter for drag movement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DragAxis {
    /// Drag on both x and y axes.
    Both,
    /// Drag only on the x axis.
    X,
    /// Drag only on the y axis.
    Y,
}

/// Constraints applied to drag position.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DragConstraints {
    /// Optional minimum x position.
    pub min_x: Option<f32>,
    /// Optional maximum x position.
    pub max_x: Option<f32>,
    /// Optional minimum y position.
    pub min_y: Option<f32>,
    /// Optional maximum y position.
    pub max_y: Option<f32>,
    /// Optional grid size used to snap both axes.
    pub grid_snap: Option<f32>,
}

impl Default for DragConstraints {
    fn default() -> Self {
        Self::unbounded()
    }
}

impl DragConstraints {
    /// Create unconstrained drag bounds.
    pub fn unbounded() -> Self {
        Self {
            min_x: None,
            max_x: None,
            min_y: None,
            max_y: None,
            grid_snap: None,
        }
    }

    /// Create rectangular drag bounds.
    pub fn bounded(min_x: f32, max_x: f32, min_y: f32, max_y: f32) -> Self {
        Self {
            min_x: Some(min_x),
            max_x: Some(max_x),
            min_y: Some(min_y),
            max_y: Some(max_y),
            grid_snap: None,
        }
    }

    /// Add grid snapping.
    pub fn with_grid_snap(mut self, grid: f32) -> Self {
        self.grid_snap = if grid.is_finite() && grid > 0.0 {
            Some(grid)
        } else {
            None
        };
        self
    }

    /// Apply axis filtering, bounds, and grid snapping.
    pub fn constrain(
        &self,
        position: [f32; 2],
        axis: DragAxis,
        locked_origin: [f32; 2],
    ) -> [f32; 2] {
        let mut x = finite_or_zero(position[0]);
        let mut y = finite_or_zero(position[1]);

        match axis {
            DragAxis::Both => {}
            DragAxis::X => y = locked_origin[1],
            DragAxis::Y => x = locked_origin[0],
        }

        x = clamp_optional(x, self.min_x, self.max_x);
        y = clamp_optional(y, self.min_y, self.max_y);

        if let Some(grid) = self.grid_snap {
            x = snap(x, grid);
            y = snap(y, grid);
            x = clamp_optional(x, self.min_x, self.max_x);
            y = clamp_optional(y, self.min_y, self.max_y);
        }

        [x, y]
    }
}

/// Stateful drag tracker.
///
/// Requires the `alloc` or `std` feature because releasing a drag can create
/// an [`InertiaN<[f32; 2]>`].
#[cfg(any(feature = "std", feature = "alloc"))]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DragState {
    position: [f32; 2],
    start_position: [f32; 2],
    start_pointer: [f32; 2],
    last_position: [f32; 2],
    velocity: [f32; 2],
    active_pointer_id: Option<u64>,
    axis: DragAxis,
    constraints: DragConstraints,
    inertia_config: InertiaConfig<[f32; 2]>,
    velocity_smoothing: f32,
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl DragState {
    /// Create a drag tracker at an initial position.
    pub fn new(position: [f32; 2]) -> Self {
        Self {
            position,
            start_position: position,
            start_pointer: [0.0, 0.0],
            last_position: position,
            velocity: [0.0, 0.0],
            active_pointer_id: None,
            axis: DragAxis::Both,
            constraints: DragConstraints::unbounded(),
            inertia_config: InertiaConfig::new(1400.0, 2.0),
            velocity_smoothing: 0.35,
        }
    }

    /// Set the drag axis filter.
    pub fn axis(mut self, axis: DragAxis) -> Self {
        self.axis = axis;
        self
    }

    /// Set drag constraints.
    pub fn constraints(mut self, constraints: DragConstraints) -> Self {
        self.constraints = constraints;
        self
    }

    /// Replace drag constraints and clamp the current position into them.
    pub fn set_constraints(&mut self, constraints: DragConstraints) {
        self.constraints = constraints;
        self.position = self
            .constraints
            .constrain(self.position, self.axis, self.position);
        self.last_position = self.position;
    }

    /// Set the inertia configuration used on pointer release.
    pub fn inertia_config(mut self, config: InertiaConfig<[f32; 2]>) -> Self {
        self.inertia_config = config;
        self
    }

    /// Set velocity EMA smoothing in `[0.0, 1.0]`.
    pub fn velocity_smoothing(mut self, smoothing: f32) -> Self {
        self.velocity_smoothing = smoothing.clamp(0.0, 1.0);
        self
    }

    /// Current constrained drag position.
    pub fn position(&self) -> [f32; 2] {
        self.position
    }

    /// Current estimated drag velocity.
    pub fn velocity(&self) -> [f32; 2] {
        self.velocity
    }

    /// Move instantly to a position, applying axis filters and constraints.
    pub fn snap_to(&mut self, position: [f32; 2]) {
        self.active_pointer_id = None;
        self.velocity = [0.0, 0.0];
        self.position = self
            .constraints
            .constrain(position, self.axis, self.position);
        self.start_position = self.position;
        self.last_position = self.position;
    }

    /// `true` while a pointer is captured.
    pub fn is_dragging(&self) -> bool {
        self.active_pointer_id.is_some()
    }

    /// Captured pointer id, if any.
    pub fn active_pointer_id(&self) -> Option<u64> {
        self.active_pointer_id
    }

    /// Capture a pointer and start tracking movement.
    pub fn on_pointer_down(&mut self, data: PointerData) {
        self.active_pointer_id = Some(data.pointer_id);
        self.start_pointer = data.position();
        self.start_position = self.position;
        self.last_position = self.position;
        self.velocity = [0.0, 0.0];
    }

    /// Update position and velocity from a pointer move.
    pub fn on_pointer_move(&mut self, data: PointerData, dt: f32) {
        if self.active_pointer_id != Some(data.pointer_id) {
            return;
        }

        let delta = [
            data.x - self.start_pointer[0],
            data.y - self.start_pointer[1],
        ];
        let raw_position = [
            self.start_position[0] + delta[0],
            self.start_position[1] + delta[1],
        ];
        let constrained = self
            .constraints
            .constrain(raw_position, self.axis, self.start_position);

        let dt = dt.max(0.0);
        if dt > 0.0 {
            let instant = [
                (constrained[0] - self.last_position[0]) / dt,
                (constrained[1] - self.last_position[1]) / dt,
            ];
            let alpha = self.velocity_smoothing;
            self.velocity = [
                self.velocity[0] * (1.0 - alpha) + instant[0] * alpha,
                self.velocity[1] * (1.0 - alpha) + instant[1] * alpha,
            ];
        }

        self.position = constrained;
        self.last_position = constrained;
    }

    /// Release the captured pointer and create inertia if velocity is high enough.
    pub fn on_pointer_up(&mut self, data: PointerData) -> Option<InertiaN<[f32; 2]>> {
        if self.active_pointer_id != Some(data.pointer_id) {
            return None;
        }

        self.active_pointer_id = None;
        let velocity = match self.axis {
            DragAxis::Both => self.velocity,
            DragAxis::X => [self.velocity[0], 0.0],
            DragAxis::Y => [0.0, self.velocity[1]],
        };

        if velocity[0].abs() <= self.inertia_config.min_velocity
            && velocity[1].abs() <= self.inertia_config.min_velocity
        {
            self.velocity = [0.0, 0.0];
            return None;
        }

        let mut config = self.inertia_config.clone();
        config.bounds = self.bounds_for_inertia();
        let mut inertia = InertiaN::new(config, self.position);
        inertia.kick(velocity);
        Some(inertia)
    }

    fn bounds_for_inertia(&self) -> Option<InertiaBounds<[f32; 2]>> {
        match (
            self.constraints.min_x,
            self.constraints.max_x,
            self.constraints.min_y,
            self.constraints.max_y,
        ) {
            (Some(min_x), Some(max_x), Some(min_y), Some(max_y)) => {
                Some(InertiaBounds::new([min_x, min_y], [max_x, max_y]))
            }
            _ => self.inertia_config.bounds.clone(),
        }
    }
}

#[inline]
fn finite_or_zero(value: f32) -> f32 {
    if value.is_finite() { value } else { 0.0 }
}

#[inline]
fn clamp_optional(value: f32, min: Option<f32>, max: Option<f32>) -> f32 {
    match (min, max) {
        (Some(a), Some(b)) => value.clamp(a.min(b), a.max(b)),
        (Some(min), None) => value.max(min),
        (None, Some(max)) => value.min(max),
        (None, None) => value,
    }
}

#[inline]
fn snap(value: f32, grid: f32) -> f32 {
    if grid > 0.0 {
        libm::roundf(value / grid) * grid
    } else {
        value
    }
}

#[cfg(all(test, any(feature = "std", feature = "alloc")))]
mod tests {
    use super::*;

    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn drag_respects_axis_and_constraints() {
        let mut drag = DragState::new([0.0, 5.0])
            .axis(DragAxis::X)
            .constraints(DragConstraints::bounded(-10.0, 10.0, -10.0, 10.0));

        drag.on_pointer_down(PointerData::new(0.0, 0.0, 1));
        drag.on_pointer_move(PointerData::new(30.0, 40.0, 1), 0.016);

        assert_eq!(drag.position(), [10.0, 5.0]);
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn drag_ignores_wrong_pointer_id() {
        let mut drag = DragState::new([0.0, 0.0]);
        drag.on_pointer_down(PointerData::new(0.0, 0.0, 7));
        drag.on_pointer_move(PointerData::new(20.0, 0.0, 8), 0.016);
        assert_eq!(drag.position(), [0.0, 0.0]);
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn drag_estimates_velocity_with_ema() {
        let mut drag = DragState::new([0.0, 0.0]).velocity_smoothing(1.0);
        drag.on_pointer_down(PointerData::new(0.0, 0.0, 1));
        drag.on_pointer_move(PointerData::new(16.0, 0.0, 1), 0.016);
        assert!((drag.velocity()[0] - 1000.0).abs() < 0.01);
        assert_eq!(drag.velocity()[1], 0.0);
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn grid_snap_applies_to_position() {
        let mut drag = DragState::new([0.0, 0.0])
            .constraints(DragConstraints::unbounded().with_grid_snap(10.0));
        drag.on_pointer_down(PointerData::new(0.0, 0.0, 1));
        drag.on_pointer_move(PointerData::new(16.0, 24.0, 1), 0.016);
        assert_eq!(drag.position(), [20.0, 20.0]);
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn snap_to_and_updated_constraints_clamp_position() {
        let mut drag = DragState::new([0.0, 0.0])
            .constraints(DragConstraints::bounded(-10.0, 10.0, -8.0, 8.0));

        drag.snap_to([40.0, -30.0]);
        assert_eq!(drag.position(), [10.0, -8.0]);

        drag.set_constraints(DragConstraints::bounded(-4.0, 4.0, -3.0, 3.0));
        assert_eq!(drag.position(), [4.0, -3.0]);
        assert!(!drag.is_dragging());
        assert_eq!(drag.velocity(), [0.0, 0.0]);
    }
}
