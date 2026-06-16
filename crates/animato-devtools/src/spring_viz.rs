//! Spring visualizer state.

use animato_core::Update;
use animato_spring::{Spring, SpringConfig};

/// One sampled spring frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpringFrame {
    /// Time in seconds.
    pub time: f32,
    /// Spring position.
    pub position: f32,
    /// Spring velocity.
    pub velocity: f32,
}

/// Real-time spring parameter visualizer.
#[derive(Clone, Debug)]
pub struct SpringVisualizer {
    /// Spring configuration.
    pub config: SpringConfig,
    /// Recorded position/velocity history.
    pub history: Vec<SpringFrame>,
    /// Maximum retained frames.
    pub max_frames: usize,
    target: f32,
    start: f32,
}

impl SpringVisualizer {
    /// Create a spring visualizer.
    pub fn new(config: SpringConfig) -> Self {
        Self {
            config,
            history: Vec::new(),
            max_frames: 600,
            target: 1.0,
            start: 0.0,
        }
    }

    /// Simulate a spring from zero to `target`.
    pub fn simulate(&mut self, target: f32, dt: f32, steps: usize) {
        self.history.clear();
        self.target = target;
        self.start = 0.0;
        let dt = dt.max(0.0);
        let mut spring = Spring::new(self.config.clone());
        spring.snap_to(self.start);
        spring.set_target(target);

        let steps = steps.min(self.max_frames);
        for index in 0..steps {
            let time = index as f32 * dt;
            self.history.push(SpringFrame {
                time,
                position: spring.position(),
                velocity: spring.velocity(),
            });
            if !spring.update(dt) && index + 1 < steps {
                self.history.push(SpringFrame {
                    time: time + dt,
                    position: spring.position(),
                    velocity: spring.velocity(),
                });
                break;
            }
        }
    }

    /// Set stiffness.
    pub fn set_stiffness(&mut self, stiffness: f32) {
        self.config.stiffness = stiffness.max(0.0);
    }

    /// Set damping.
    pub fn set_damping(&mut self, damping: f32) {
        self.config.damping = damping.max(0.0);
    }

    /// Set mass.
    pub fn set_mass(&mut self, mass: f32) {
        self.config.mass = mass.max(f32::EPSILON);
    }

    /// Apply a named preset.
    pub fn set_preset(&mut self, name: &str) {
        self.config = match normalize(name).as_str() {
            "gentle" => SpringConfig::gentle(),
            "wobbly" => SpringConfig::wobbly(),
            "stiff" => SpringConfig::stiff(),
            "slow" => SpringConfig::slow(),
            "snappy" => SpringConfig::snappy(),
            _ => self.config.clone(),
        };
    }

    /// First sampled time where the spring is settled.
    pub fn settle_time(&self) -> f32 {
        self.history
            .iter()
            .find(|frame| {
                (frame.position - self.target).abs() < self.config.epsilon
                    && frame.velocity.abs() < self.config.epsilon
            })
            .map_or_else(
                || self.history.last().map_or(0.0, |frame| frame.time),
                |frame| frame.time,
            )
    }

    /// Largest overshoot beyond target as a percentage of the travel distance.
    pub fn overshoot_pct(&self) -> f32 {
        let travel = (self.target - self.start).abs().max(f32::EPSILON);
        let direction = (self.target - self.start).signum();
        let overshoot = self
            .history
            .iter()
            .map(|frame| (frame.position - self.target) * direction)
            .fold(0.0_f32, f32::max);
        (overshoot / travel) * 100.0
    }

    /// Count target crossings in the recorded history.
    pub fn oscillation_count(&self) -> u32 {
        let mut count = 0_u32;
        let mut previous = None;
        for frame in &self.history {
            let displacement = frame.position - self.target;
            if displacement.abs() <= self.config.epsilon {
                continue;
            }
            if let Some(prev) = previous
                && prev * displacement < 0.0
            {
                count = count.saturating_add(1);
            }
            previous = Some(displacement);
        }
        count
    }
}

impl Default for SpringVisualizer {
    fn default() -> Self {
        Self::new(SpringConfig::default())
    }
}

fn normalize(value: &str) -> String {
    value
        .chars()
        .filter(|ch| !ch.is_whitespace() && *ch != '-' && *ch != '_')
        .flat_map(char::to_lowercase)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simulate_records_bounded_history() {
        let mut visualizer = SpringVisualizer::new(SpringConfig::snappy());
        visualizer.max_frames = 120;
        visualizer.simulate(1.0, 1.0 / 60.0, 240);
        assert!(!visualizer.history.is_empty());
        assert!(visualizer.history.len() <= 120);
        assert!(visualizer.settle_time() >= 0.0);
    }

    #[test]
    fn preset_and_metrics_work() {
        let mut visualizer = SpringVisualizer::default();
        visualizer.set_preset("wobbly");
        visualizer.simulate(1.0, 1.0 / 60.0, 180);
        assert!(visualizer.overshoot_pct() >= 0.0);
        assert!(visualizer.oscillation_count() > 0);
    }
}
