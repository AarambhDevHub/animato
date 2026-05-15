//! Friction inertia for post-drag motion.

#[cfg(any(feature = "std", feature = "alloc"))]
use crate::decompose::Decompose;
#[cfg(any(feature = "std", feature = "alloc"))]
use alloc::vec::Vec;
use animato_core::Update;
#[cfg(any(feature = "std", feature = "alloc"))]
use core::marker::PhantomData;

/// Inclusive bounds for inertia position.
///
/// For 1D inertia use `InertiaBounds<f32>`. For multi-dimensional inertia,
/// use the same component shape as the animated value, such as
/// `InertiaBounds<[f32; 2]>`.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InertiaBounds<T = f32> {
    /// Minimum allowed position.
    pub min: T,
    /// Maximum allowed position.
    pub max: T,
}

impl<T> InertiaBounds<T> {
    /// Create bounds from a minimum and maximum value.
    pub fn new(min: T, max: T) -> Self {
        Self { min, max }
    }
}

/// Configuration for friction inertia.
///
/// `friction` is a constant deceleration in units per second squared.
/// `min_velocity` is the absolute velocity threshold below which inertia is
/// considered settled.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InertiaConfig<T = f32> {
    /// Constant deceleration in units per second squared.
    pub friction: f32,
    /// Velocity threshold below which the inertia settles.
    pub min_velocity: f32,
    /// Optional inclusive bounds for the position.
    pub bounds: Option<InertiaBounds<T>>,
}

impl<T> InertiaConfig<T> {
    /// Create an inertia configuration.
    pub fn new(friction: f32, min_velocity: f32) -> Self {
        Self {
            friction,
            min_velocity,
            bounds: None,
        }
    }

    /// Attach inclusive position bounds.
    pub fn with_bounds(mut self, bounds: InertiaBounds<T>) -> Self {
        self.bounds = Some(bounds);
        self
    }

    #[inline]
    fn friction(&self) -> f32 {
        if self.friction.is_finite() {
            self.friction.max(0.0)
        } else {
            0.0
        }
    }

    #[inline]
    fn min_velocity(&self) -> f32 {
        if self.min_velocity.is_finite() {
            self.min_velocity.max(0.0)
        } else {
            0.0
        }
    }
}

impl Default for InertiaConfig<f32> {
    fn default() -> Self {
        Self::smooth()
    }
}

impl InertiaConfig<f32> {
    /// Smooth, long-running inertia for scroll and carousel-like movement.
    pub fn smooth() -> Self {
        Self {
            friction: 1400.0,
            min_velocity: 2.0,
            bounds: None,
        }
    }

    /// Short, responsive inertia for direct-manipulation UI.
    pub fn snappy() -> Self {
        Self {
            friction: 3600.0,
            min_velocity: 4.0,
            bounds: None,
        }
    }

    /// Heavy inertia with slower deceleration for large panels and canvases.
    pub fn heavy() -> Self {
        Self {
            friction: 800.0,
            min_velocity: 1.0,
            bounds: None,
        }
    }
}

/// One-dimensional friction inertia.
///
/// `Inertia` starts at a position, receives an initial velocity through
/// [`kick`](Self::kick), and decelerates until velocity falls below
/// `InertiaConfig::min_velocity` or a bound is reached.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Inertia {
    /// Runtime configuration.
    pub config: InertiaConfig<f32>,
    position: f32,
    velocity: f32,
}

impl Inertia {
    /// Create inertia at position `0.0`.
    pub fn new(config: InertiaConfig<f32>) -> Self {
        Self::with_position(config, 0.0)
    }

    /// Create inertia at a specific position.
    pub fn with_position(config: InertiaConfig<f32>, position: f32) -> Self {
        let mut this = Self {
            config,
            position: finite_or_zero(position),
            velocity: 0.0,
        };
        this.apply_bounds();
        this
    }

    /// Start inertia from an initial velocity.
    pub fn kick(&mut self, velocity: f32) {
        let velocity = finite_or_zero(velocity);
        self.velocity = if velocity.abs() <= self.config.min_velocity() {
            0.0
        } else {
            velocity
        };
    }

    /// Current position.
    pub fn position(&self) -> f32 {
        self.position
    }

    /// Current velocity.
    pub fn velocity(&self) -> f32 {
        self.velocity
    }

    /// Teleport to `position` and clear velocity.
    pub fn snap_to(&mut self, position: f32) {
        self.position = finite_or_zero(position);
        self.velocity = 0.0;
        self.apply_bounds();
    }

    /// `true` when velocity is below the configured threshold.
    pub fn is_settled(&self) -> bool {
        self.velocity.abs() <= self.config.min_velocity()
    }

    #[inline]
    fn apply_bounds(&mut self) -> bool {
        if let Some(bounds) = &self.config.bounds {
            let min = bounds.min.min(bounds.max);
            let max = bounds.min.max(bounds.max);
            if self.position < min {
                self.position = min;
                self.velocity = 0.0;
                return true;
            }
            if self.position > max {
                self.position = max;
                self.velocity = 0.0;
                return true;
            }
        }
        false
    }
}

impl Update for Inertia {
    /// Advance inertia by `dt` seconds.
    ///
    /// Negative `dt` is treated as `0.0`. Bounds clamp and stop the simulated
    /// axis immediately.
    fn update(&mut self, dt: f32) -> bool {
        let dt = dt.max(0.0);
        if dt == 0.0 || self.is_settled() {
            if self.is_settled() {
                self.velocity = 0.0;
            }
            return !self.is_settled();
        }

        let friction = self.config.friction();
        if friction <= 0.0 {
            self.velocity = 0.0;
            return false;
        }

        let sign = self.velocity.signum();
        let speed = self.velocity.abs();
        let stop_time = speed / friction;
        let step = dt.min(stop_time);

        self.position += self.velocity * step - 0.5 * sign * friction * step * step;

        let next_speed = speed - friction * step;
        self.velocity = if step >= stop_time || next_speed <= self.config.min_velocity() {
            0.0
        } else {
            sign * next_speed
        };

        if self.apply_bounds() {
            return false;
        }

        !self.is_settled()
    }
}

/// Multi-dimensional friction inertia backed by one [`Inertia`] per component.
///
/// Requires the `alloc` or `std` feature.
#[cfg(any(feature = "std", feature = "alloc"))]
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InertiaN<T: Decompose> {
    components: Vec<Inertia>,
    _marker: PhantomData<T>,
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T: Decompose> InertiaN<T> {
    /// Create multi-dimensional inertia at `initial` position.
    pub fn new(config: InertiaConfig<T>, initial: T) -> Self {
        let count = T::component_count();
        let mut initial_components = alloc::vec![0.0; count];
        initial.write_components(&mut initial_components);

        let mut min_components = alloc::vec![0.0; count];
        let mut max_components = alloc::vec![0.0; count];
        let has_bounds = if let Some(bounds) = &config.bounds {
            bounds.min.write_components(&mut min_components);
            bounds.max.write_components(&mut max_components);
            true
        } else {
            false
        };

        let mut components = Vec::with_capacity(count);
        for index in 0..count {
            let mut component_config = InertiaConfig::new(config.friction, config.min_velocity);
            if has_bounds {
                component_config = component_config.with_bounds(InertiaBounds::new(
                    min_components[index],
                    max_components[index],
                ));
            }
            components.push(Inertia::with_position(
                component_config,
                initial_components[index],
            ));
        }

        Self {
            components,
            _marker: PhantomData,
        }
    }

    /// Start inertia from a multi-dimensional velocity.
    #[allow(clippy::useless_conversion)]
    pub fn kick(&mut self, velocity: T) {
        let count = T::component_count();
        let mut velocity_components = alloc::vec![0.0; count];
        velocity.write_components(&mut velocity_components);
        for (component, velocity) in self
            .components
            .iter_mut()
            .zip(velocity_components.into_iter())
        {
            component.kick(velocity);
        }
    }

    /// Current position.
    pub fn position(&self) -> T {
        let values: Vec<f32> = self
            .components
            .iter()
            .map(|component| component.position())
            .collect();
        T::from_components(&values)
    }

    /// Current velocity.
    pub fn velocity(&self) -> T {
        let values: Vec<f32> = self
            .components
            .iter()
            .map(|component| component.velocity())
            .collect();
        T::from_components(&values)
    }

    /// Teleport to `position` and clear all component velocities.
    #[allow(clippy::useless_conversion)]
    pub fn snap_to(&mut self, position: T) {
        let count = T::component_count();
        let mut position_components = alloc::vec![0.0; count];
        position.write_components(&mut position_components);
        for (component, position) in self
            .components
            .iter_mut()
            .zip(position_components.into_iter())
        {
            component.snap_to(position);
        }
    }

    /// `true` when every component has settled.
    pub fn is_settled(&self) -> bool {
        self.components
            .iter()
            .all(|component| component.is_settled())
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<T: Decompose> Update for InertiaN<T> {
    fn update(&mut self, dt: f32) -> bool {
        if self.is_settled() {
            return false;
        }
        for component in self.components.iter_mut() {
            component.update(dt);
        }
        !self.is_settled()
    }
}

#[inline]
fn finite_or_zero(value: f32) -> f32 {
    if value.is_finite() { value } else { 0.0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DT: f32 = 1.0 / 60.0;

    #[test]
    fn inertia_settles_from_kick() {
        let mut inertia = Inertia::new(InertiaConfig::smooth());
        inertia.kick(600.0);
        for _ in 0..10_000 {
            if !inertia.update(DT) {
                break;
            }
        }
        assert!(inertia.is_settled());
        assert_eq!(inertia.velocity(), 0.0);
        assert!(inertia.position() > 0.0);
    }

    #[test]
    fn negative_dt_is_noop() {
        let mut inertia = Inertia::new(InertiaConfig::smooth());
        inertia.kick(100.0);
        inertia.update(-1.0);
        assert_eq!(inertia.position(), 0.0);
        assert_eq!(inertia.velocity(), 100.0);
    }

    #[test]
    fn bounds_clamp_and_stop() {
        let config = InertiaConfig::smooth().with_bounds(InertiaBounds::new(0.0, 10.0));
        let mut inertia = Inertia::with_position(config, 5.0);
        inertia.kick(1000.0);
        for _ in 0..60 {
            if !inertia.update(DT) {
                break;
            }
        }
        assert_eq!(inertia.position(), 10.0);
        assert_eq!(inertia.velocity(), 0.0);
        assert!(inertia.is_settled());
    }

    #[test]
    fn snap_to_respects_bounds() {
        let config = InertiaConfig::smooth().with_bounds(InertiaBounds::new(-5.0, 5.0));
        let mut inertia = Inertia::new(config);
        inertia.snap_to(20.0);
        assert_eq!(inertia.position(), 5.0);
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn inertia_n_updates_independent_axes() {
        let config = InertiaConfig::new(1000.0, 1.0)
            .with_bounds(InertiaBounds::new([-100.0, -100.0], [100.0, 100.0]));
        let mut inertia: InertiaN<[f32; 2]> = InertiaN::new(config, [0.0, 0.0]);
        inertia.kick([400.0, -200.0]);
        inertia.update(DT);
        let position = inertia.position();
        assert!(position[0] > 0.0);
        assert!(position[1] < 0.0);
    }

    #[test]
    fn presets_and_bounds_are_constructible() {
        let bounds = InertiaBounds::new(-10.0, 10.0);
        let config = InertiaConfig::snappy().with_bounds(bounds.clone());

        assert_eq!(bounds.min, -10.0);
        assert_eq!(bounds.max, 10.0);
        assert_eq!(config.bounds, Some(bounds));
        assert!(InertiaConfig::heavy().friction < InertiaConfig::snappy().friction);
        assert_eq!(InertiaConfig::<f32>::default(), InertiaConfig::smooth());
    }

    #[test]
    fn invalid_config_values_settle_immediately() {
        let mut inertia = Inertia::with_position(InertiaConfig::new(f32::NAN, f32::NAN), f32::NAN);

        inertia.kick(f32::INFINITY);

        assert_eq!(inertia.position(), 0.0);
        assert_eq!(inertia.velocity(), 0.0);
        assert!(!inertia.update(DT));
    }

    #[test]
    fn zero_friction_stops_on_first_update() {
        let mut inertia = Inertia::new(InertiaConfig::new(0.0, 0.0));

        inertia.kick(100.0);

        assert!(!inertia.update(DT));
        assert_eq!(inertia.velocity(), 0.0);
    }

    #[test]
    fn reversed_bounds_are_normalized_when_applied() {
        let config = InertiaConfig::new(1000.0, 1.0).with_bounds(InertiaBounds::new(10.0, -10.0));
        let mut inertia = Inertia::with_position(config, 100.0);

        assert_eq!(inertia.position(), 10.0);
        inertia.snap_to(-100.0);
        assert_eq!(inertia.position(), -10.0);
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn inertia_n_vec4_bounds_velocity_and_snap() {
        let config = InertiaConfig::new(1000.0, 0.5).with_bounds(InertiaBounds::new(
            [-10.0, -10.0, -10.0, -10.0],
            [10.0, 10.0, 10.0, 10.0],
        ));
        let mut inertia: InertiaN<[f32; 4]> = InertiaN::new(config, [0.0, 0.0, 0.0, 0.0]);

        inertia.kick([100.0, -50.0, 25.0, -10.0]);
        assert_eq!(inertia.velocity(), [100.0, -50.0, 25.0, -10.0]);
        assert!(inertia.update(DT));
        inertia.snap_to([20.0, -20.0, 5.0, -5.0]);

        assert_eq!(inertia.position(), [10.0, -10.0, 5.0, -5.0]);
        assert_eq!(inertia.velocity(), [0.0, 0.0, 0.0, 0.0]);
    }
}
