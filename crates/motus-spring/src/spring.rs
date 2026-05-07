//! 1D [`Spring`] — damped harmonic oscillator.

use crate::config::SpringConfig;
use motus_core::Update;

/// Integration method for the spring ODE.
#[derive(Clone, Debug, PartialEq)]
pub enum Integrator {
    /// Semi-implicit Euler — fast, stable, default choice for animation.
    SemiImplicitEuler,
    /// 4th-order Runge-Kutta — more accurate for high-stiffness springs.
    RungeKutta4,
}

/// A 1D damped harmonic oscillator spring.
///
/// Stack-allocated and `no_std`-compatible. Use [`SpringN<T>`](crate::SpringN)
/// for multi-dimensional animation.
///
/// # Example
///
/// ```rust
/// use motus_spring::{Spring, SpringConfig};
/// use motus_core::Update;
///
/// let mut s = Spring::new(SpringConfig::stiff());
/// s.set_target(100.0);
/// for _ in 0..300 {
///     s.update(1.0 / 60.0);
/// }
/// assert!((s.position() - 100.0).abs() < 0.01);
/// ```
#[derive(Clone, Debug)]
pub struct Spring {
    /// The spring configuration (stiffness, damping, mass, epsilon).
    pub config: SpringConfig,
    position: f32,
    velocity: f32,
    target: f32,
    integrator: Integrator,
}

impl Spring {
    /// Create a new spring at position `0.0` with target `0.0`.
    pub fn new(config: SpringConfig) -> Self {
        Self {
            config,
            position: 0.0,
            velocity: 0.0,
            target: 0.0,
            integrator: Integrator::SemiImplicitEuler,
        }
    }

    /// Set the target position the spring moves toward.
    pub fn set_target(&mut self, target: f32) {
        self.target = target;
    }

    /// Current position of the spring.
    pub fn position(&self) -> f32 {
        self.position
    }

    /// Current velocity of the spring.
    pub fn velocity(&self) -> f32 {
        self.velocity
    }

    /// `true` when the spring has come to rest within `epsilon` of the target.
    pub fn is_settled(&self) -> bool {
        let eps = self.config.epsilon;
        (self.position - self.target).abs() < eps && self.velocity.abs() < eps
    }

    /// Teleport to `pos` instantly — no animation, velocity zeroed.
    pub fn snap_to(&mut self, pos: f32) {
        self.position = pos;
        self.velocity = 0.0;
    }

    /// Switch to RK4 integration (more accurate for high-stiffness springs).
    pub fn use_rk4(mut self, yes: bool) -> Self {
        self.integrator = if yes {
            Integrator::RungeKutta4
        } else {
            Integrator::SemiImplicitEuler
        };
        self
    }

    // ── Integration ──────────────────────────────────────────────────────────

    #[inline]
    fn acceleration(&self, position: f32, velocity: f32) -> f32 {
        let displacement = position - self.target;
        let spring_force = -self.config.stiffness * displacement;
        let damping_force = -self.config.damping * velocity;
        (spring_force + damping_force) / self.config.mass
    }

    fn step_euler(&mut self, dt: f32) {
        let acc = self.acceleration(self.position, self.velocity);
        self.velocity += acc * dt;
        self.position += self.velocity * dt;
    }

    fn step_rk4(&mut self, dt: f32) {
        // Classic RK4 for the coupled ODEs:
        //   dx/dt = v
        //   dv/dt = acceleration(x, v)
        let p0 = self.position;
        let v0 = self.velocity;

        let k1v = self.acceleration(p0, v0);
        let k1p = v0;

        let k2v = self.acceleration(p0 + k1p * dt / 2.0, v0 + k1v * dt / 2.0);
        let k2p = v0 + k1v * dt / 2.0;

        let k3v = self.acceleration(p0 + k2p * dt / 2.0, v0 + k2v * dt / 2.0);
        let k3p = v0 + k2v * dt / 2.0;

        let k4v = self.acceleration(p0 + k3p * dt, v0 + k3v * dt);
        let k4p = v0 + k3v * dt;

        self.position += (dt / 6.0) * (k1p + 2.0 * k2p + 2.0 * k3p + k4p);
        self.velocity += (dt / 6.0) * (k1v + 2.0 * k2v + 2.0 * k3v + k4v);
    }
}

impl Update for Spring {
    /// Advance the spring by `dt` seconds.
    ///
    /// Returns `false` when settled, `true` while still moving.
    /// Negative `dt` is treated as `0.0`.
    fn update(&mut self, dt: f32) -> bool {
        let dt = dt.max(0.0);
        if dt == 0.0 || self.is_settled() {
            return !self.is_settled();
        }
        // Guard against degenerate config
        if self.config.stiffness <= 0.0 {
            self.position = self.target;
            self.velocity = 0.0;
            return false;
        }
        match self.integrator {
            Integrator::SemiImplicitEuler => self.step_euler(dt),
            Integrator::RungeKutta4 => self.step_rk4(dt),
        }
        !self.is_settled()
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Tests
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const DT: f32 = 1.0 / 60.0;
    const MAX_STEPS: usize = 10_000;

    fn run_to_settle(spring: &mut Spring) -> usize {
        let mut steps = 0;
        while !spring.is_settled() {
            spring.update(DT);
            steps += 1;
            assert!(
                steps < MAX_STEPS,
                "Spring did not settle within {} steps",
                MAX_STEPS
            );
        }
        steps
    }

    #[test]
    fn gentle_settles_to_target() {
        let mut s = Spring::new(SpringConfig::gentle());
        s.set_target(100.0);
        run_to_settle(&mut s);
        assert!((s.position() - 100.0).abs() < 0.01);
    }

    #[test]
    fn wobbly_settles_to_target() {
        let mut s = Spring::new(SpringConfig::wobbly());
        s.set_target(50.0);
        run_to_settle(&mut s);
        assert!((s.position() - 50.0).abs() < 0.01);
    }

    #[test]
    fn stiff_settles_to_target() {
        let mut s = Spring::new(SpringConfig::stiff());
        s.set_target(-30.0);
        run_to_settle(&mut s);
        assert!((s.position() - (-30.0)).abs() < 0.01);
    }

    #[test]
    fn slow_settles_to_target() {
        let mut s = Spring::new(SpringConfig::slow());
        s.set_target(1.0);
        run_to_settle(&mut s);
        assert!((s.position() - 1.0).abs() < 0.01);
    }

    #[test]
    fn snappy_settles_to_target() {
        let mut s = Spring::new(SpringConfig::snappy());
        s.set_target(200.0);
        run_to_settle(&mut s);
        assert!((s.position() - 200.0).abs() < 0.01);
    }

    #[test]
    fn snappy_settles_faster_than_slow() {
        let mut fast = Spring::new(SpringConfig::snappy());
        fast.set_target(100.0);
        let fast_steps = run_to_settle(&mut fast);

        let mut slow = Spring::new(SpringConfig::slow());
        slow.set_target(100.0);
        let slow_steps = run_to_settle(&mut slow);

        assert!(
            fast_steps < slow_steps,
            "snappy={} slow={}",
            fast_steps,
            slow_steps
        );
    }

    #[test]
    fn zero_damping_oscillates() {
        let cfg = SpringConfig {
            stiffness: 100.0,
            damping: 0.0,
            mass: 1.0,
            epsilon: 0.001,
        };
        let mut s = Spring::new(cfg);
        s.set_target(1.0);
        // Run 10,000 steps — should never settle with zero damping
        for _ in 0..10_000 {
            s.update(DT);
        }
        assert!(!s.is_settled());
    }

    #[test]
    fn snap_to_teleports() {
        let mut s = Spring::new(SpringConfig::default());
        s.set_target(100.0);
        s.snap_to(100.0);
        assert_eq!(s.position(), 100.0);
        assert_eq!(s.velocity(), 0.0);
        assert!(s.is_settled());
    }

    #[test]
    fn rk4_also_settles() {
        let mut s = Spring::new(SpringConfig::wobbly()).use_rk4(true);
        s.set_target(100.0);
        run_to_settle(&mut s);
        assert!((s.position() - 100.0).abs() < 0.01);
    }

    #[test]
    fn zero_stiffness_snaps_immediately() {
        let cfg = SpringConfig {
            stiffness: 0.0,
            ..SpringConfig::default()
        };
        let mut s = Spring::new(cfg);
        s.set_target(42.0);
        s.update(DT);
        assert_eq!(s.position(), 42.0);
        assert!(s.is_settled());
    }

    #[test]
    fn negative_dt_is_noop() {
        let mut s = Spring::new(SpringConfig::default());
        s.set_target(100.0);
        let pos_before = s.position();
        s.update(-1.0);
        assert_eq!(s.position(), pos_before);
    }
}
