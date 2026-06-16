//! Performance monitoring state.

use animato_driver::{AnimationId, DriverFrameProfile};
use std::collections::VecDeque;

/// Per-animation update cost retained by the performance monitor.
#[derive(Clone, Debug, PartialEq)]
pub struct AnimationCostRecord {
    /// Stable animation id.
    pub id: AnimationId,
    /// Optional label.
    pub label: Option<String>,
    /// Update cost in milliseconds.
    pub update_time_ms: f32,
}

/// Rolling performance monitor for animation workloads.
#[derive(Clone, Debug, PartialEq)]
pub struct PerformanceMonitor {
    frame_times: VecDeque<f32>,
    window_size: usize,
    animation_costs: Vec<AnimationCostRecord>,
    active_animation_count: usize,
}

impl PerformanceMonitor {
    /// Create a monitor with a rolling window size.
    pub fn new(window_size: usize) -> Self {
        Self {
            frame_times: VecDeque::with_capacity(window_size.max(1)),
            window_size: window_size.max(1),
            animation_costs: Vec::new(),
            active_animation_count: 0,
        }
    }

    /// Record a frame delta in seconds.
    pub fn record_frame(&mut self, dt: f32) {
        if self.frame_times.len() == self.window_size {
            self.frame_times.pop_front();
        }
        self.frame_times.push_back(dt.max(0.0));
    }

    /// Record driver profile data.
    pub fn record_profile(&mut self, profile: &DriverFrameProfile) {
        self.record_frame(profile.dt);
        self.active_animation_count = profile.animation_costs.len();
        self.animation_costs.clear();
        self.animation_costs
            .extend(
                profile
                    .animation_costs
                    .iter()
                    .map(|cost| AnimationCostRecord {
                        id: cost.id,
                        label: cost.label.clone(),
                        update_time_ms: cost.update_time_ms,
                    }),
            );
    }

    /// Rolling frames per second.
    pub fn fps(&self) -> f32 {
        let avg = self.avg_frame_time_ms();
        if avg <= f32::EPSILON {
            0.0
        } else {
            1000.0 / avg
        }
    }

    /// Average frame time in milliseconds.
    pub fn avg_frame_time_ms(&self) -> f32 {
        if self.frame_times.is_empty() {
            return 0.0;
        }
        self.frame_times.iter().sum::<f32>() * 1000.0 / self.frame_times.len() as f32
    }

    /// Maximum frame time in milliseconds.
    pub fn max_frame_time_ms(&self) -> f32 {
        self.frame_times.iter().copied().fold(0.0_f32, f32::max) * 1000.0
    }

    /// Frame budget usage where `1.0` means 100% of budget.
    pub fn frame_budget_usage(&self, target_fps: f32) -> f32 {
        if target_fps <= 0.0 {
            return 0.0;
        }
        self.avg_frame_time_ms() / (1000.0 / target_fps)
    }

    /// Whether the rolling average exceeds the target frame budget.
    pub fn exceeds_budget(&self, target_fps: f32) -> bool {
        self.frame_budget_usage(target_fps) > 1.0
    }

    /// Latest per-animation costs.
    pub fn animation_costs(&self) -> &[AnimationCostRecord] {
        &self.animation_costs
    }

    /// Latest active animation count.
    pub fn active_animation_count(&self) -> usize {
        self.active_animation_count
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new(120)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_fps_and_budget() {
        let mut monitor = PerformanceMonitor::new(2);
        monitor.record_frame(1.0 / 60.0);
        monitor.record_frame(1.0 / 30.0);
        assert!(monitor.fps() > 0.0);
        assert!(monitor.avg_frame_time_ms() > 0.0);
        assert!(monitor.max_frame_time_ms() >= monitor.avg_frame_time_ms());
        assert!(monitor.frame_budget_usage(60.0) > 0.0);
    }
}
