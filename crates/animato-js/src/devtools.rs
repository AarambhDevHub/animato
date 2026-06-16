//! DevTools JavaScript bindings.

use crate::driver::{RafDriver, RafSnapshot};
use crate::easing::parse_easing;
use crate::types::f32_array;
use animato_core::Easing;
use animato_devtools::{
    DevToolsState as CoreDevToolsState, EasingCurveEditor as CoreEasingCurveEditor,
    PerformanceMonitor as CorePerformanceMonitor, SpringVisualizer as CoreSpringVisualizer,
};
use animato_spring::SpringConfig;
use js_sys::Float32Array;
use wasm_bindgen::prelude::*;

/// Timeline inspector for JavaScript rAF drivers.
#[wasm_bindgen(js_name = TimelineInspector)]
#[derive(Clone, Debug, Default)]
pub struct TimelineInspector {
    snapshots: Vec<RafSnapshot>,
    completed_count: usize,
}

#[wasm_bindgen(js_class = TimelineInspector)]
impl TimelineInspector {
    /// Create an empty timeline inspector.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Capture snapshots from a JavaScript rAF driver.
    #[wasm_bindgen(js_name = captureRafDriver)]
    pub fn capture_raf_driver(&mut self, driver: &RafDriver) {
        self.snapshots = driver.snapshots();
        self.completed_count = driver.completed_count();
    }

    /// Number of captured snapshots.
    #[wasm_bindgen(js_name = snapshotCount)]
    pub fn snapshot_count(&self) -> usize {
        self.snapshots.len()
    }

    /// Number of active captured animations.
    #[wasm_bindgen(js_name = activeCount)]
    pub fn active_count(&self) -> usize {
        self.snapshots
            .iter()
            .filter(|snapshot| snapshot.state != "complete")
            .count()
    }

    /// Number of animations completed by the captured driver.
    #[wasm_bindgen(js_name = completedCount)]
    pub fn completed_count(&self) -> usize {
        self.completed_count
    }

    /// Snapshot id by index.
    #[wasm_bindgen(js_name = snapshotId)]
    pub fn snapshot_id(&self, index: usize) -> u32 {
        self.snapshots.get(index).map_or(0, |snapshot| snapshot.id)
    }

    /// Snapshot label by index.
    #[wasm_bindgen(js_name = snapshotLabel)]
    pub fn snapshot_label(&self, index: usize) -> String {
        self.snapshots
            .get(index)
            .and_then(|snapshot| snapshot.label.as_ref())
            .cloned()
            .unwrap_or_default()
    }

    /// Snapshot kind by index.
    #[wasm_bindgen(js_name = snapshotKind)]
    pub fn snapshot_kind(&self, index: usize) -> String {
        self.snapshots
            .get(index)
            .map_or_else(String::new, |snapshot| snapshot.kind.to_owned())
    }

    /// Snapshot normalized progress by index.
    #[wasm_bindgen(js_name = snapshotProgress)]
    pub fn snapshot_progress(&self, index: usize) -> f32 {
        self.snapshots
            .get(index)
            .map_or(0.0, |snapshot| snapshot.progress)
    }

    /// Snapshot playback state by index.
    #[wasm_bindgen(js_name = snapshotState)]
    pub fn snapshot_state(&self, index: usize) -> String {
        self.snapshots
            .get(index)
            .map_or_else(String::new, |snapshot| snapshot.state.clone())
    }
}

/// Easing curve editor for graph rendering.
#[wasm_bindgen(js_name = EasingCurveEditor)]
#[derive(Clone, Debug)]
pub struct EasingCurveEditor {
    inner: CoreEasingCurveEditor,
}

#[wasm_bindgen(js_class = EasingCurveEditor)]
impl EasingCurveEditor {
    /// Create an editor with an easing name.
    #[wasm_bindgen(constructor)]
    pub fn new(easing: &str) -> Result<Self, JsValue> {
        Ok(Self {
            inner: CoreEasingCurveEditor::new(parse_easing(easing)?),
        })
    }

    /// Set the primary easing.
    #[wasm_bindgen(js_name = setEasing)]
    pub fn set_easing(&mut self, easing: &str) -> Result<(), JsValue> {
        self.inner.set_easing(parse_easing(easing)?);
        Ok(())
    }

    /// Set the optional comparison easing.
    #[wasm_bindgen(js_name = setCompare)]
    pub fn set_compare(&mut self, easing: Option<String>) -> Result<(), JsValue> {
        self.inner
            .set_compare(easing.as_deref().map(parse_easing).transpose()?);
        Ok(())
    }

    /// Set sample count.
    #[wasm_bindgen(js_name = setSampleCount)]
    pub fn set_sample_count(&mut self, sample_count: usize) {
        self.inner.set_sample_count(sample_count);
    }

    /// Update cubic-bezier control points.
    #[wasm_bindgen(js_name = setControlPoints)]
    pub fn set_control_points(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        self.inner.set_control_points(x1, y1, x2, y2);
    }

    /// Flattened `[x, y, ...]` sample points.
    #[wasm_bindgen(js_name = samplePoints)]
    pub fn sample_points(&self) -> Float32Array {
        flatten_points(&self.inner.samples())
    }

    /// Flattened comparison sample points, or an empty array.
    #[wasm_bindgen(js_name = compareSamplePoints)]
    pub fn compare_sample_points(&self) -> Float32Array {
        self.inner
            .compare_samples()
            .map_or_else(|| f32_array(&[]), |samples| flatten_points(&samples))
    }

    /// Rust easing code for copying.
    #[wasm_bindgen(js_name = copyCode)]
    pub fn copy_code(&self) -> String {
        self.inner.copy_code()
    }
}

impl Default for EasingCurveEditor {
    fn default() -> Self {
        Self {
            inner: CoreEasingCurveEditor::new(Easing::Linear),
        }
    }
}

/// Spring visualizer for graph rendering.
#[wasm_bindgen(js_name = SpringVisualizer)]
#[derive(Clone, Debug)]
pub struct SpringVisualizer {
    inner: CoreSpringVisualizer,
}

#[wasm_bindgen(js_class = SpringVisualizer)]
impl SpringVisualizer {
    /// Create a spring visualizer with default config.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: CoreSpringVisualizer::new(SpringConfig::default()),
        }
    }

    /// Apply a named preset.
    #[wasm_bindgen(js_name = setPreset)]
    pub fn set_preset(&mut self, preset: &str) {
        self.inner.set_preset(preset);
    }

    /// Set custom spring config.
    #[wasm_bindgen(js_name = setConfig)]
    pub fn set_config(&mut self, stiffness: f32, damping: f32, mass: f32) {
        self.inner.set_stiffness(stiffness);
        self.inner.set_damping(damping);
        self.inner.set_mass(mass);
    }

    /// Simulate the spring.
    pub fn simulate(&mut self, target: f32, dt: f32, steps: usize) {
        self.inner.simulate(target, dt, steps);
    }

    /// Number of recorded frames.
    #[wasm_bindgen(js_name = frameCount)]
    pub fn frame_count(&self) -> usize {
        self.inner.history.len()
    }

    /// Flattened `[time, position, velocity, ...]` frames.
    #[wasm_bindgen(js_name = frameData)]
    pub fn frame_data(&self) -> Float32Array {
        let mut flat = Vec::with_capacity(self.inner.history.len() * 3);
        for frame in &self.inner.history {
            flat.push(frame.time);
            flat.push(frame.position);
            flat.push(frame.velocity);
        }
        f32_array(&flat)
    }

    /// Settle time in seconds.
    #[wasm_bindgen(js_name = settleTime)]
    pub fn settle_time(&self) -> f32 {
        self.inner.settle_time()
    }

    /// Overshoot percentage.
    #[wasm_bindgen(js_name = overshootPct)]
    pub fn overshoot_pct(&self) -> f32 {
        self.inner.overshoot_pct()
    }

    /// Target crossing count.
    #[wasm_bindgen(js_name = oscillationCount)]
    pub fn oscillation_count(&self) -> u32 {
        self.inner.oscillation_count()
    }
}

impl Default for SpringVisualizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Rolling performance monitor.
#[wasm_bindgen(js_name = PerformanceMonitor)]
#[derive(Clone, Debug)]
pub struct PerformanceMonitor {
    inner: CorePerformanceMonitor,
}

#[wasm_bindgen(js_class = PerformanceMonitor)]
impl PerformanceMonitor {
    /// Create a performance monitor.
    #[wasm_bindgen(constructor)]
    pub fn new(window_size: usize) -> Self {
        Self {
            inner: CorePerformanceMonitor::new(window_size),
        }
    }

    /// Record frame delta in seconds.
    #[wasm_bindgen(js_name = recordFrame)]
    pub fn record_frame(&mut self, dt: f32) {
        self.inner.record_frame(dt);
    }

    /// Current FPS estimate.
    pub fn fps(&self) -> f32 {
        self.inner.fps()
    }

    /// Average frame time in milliseconds.
    #[wasm_bindgen(js_name = avgFrameTimeMs)]
    pub fn avg_frame_time_ms(&self) -> f32 {
        self.inner.avg_frame_time_ms()
    }

    /// Max frame time in milliseconds.
    #[wasm_bindgen(js_name = maxFrameTimeMs)]
    pub fn max_frame_time_ms(&self) -> f32 {
        self.inner.max_frame_time_ms()
    }

    /// Frame budget usage for a target FPS.
    #[wasm_bindgen(js_name = frameBudgetUsage)]
    pub fn frame_budget_usage(&self, target_fps: f32) -> f32 {
        self.inner.frame_budget_usage(target_fps)
    }
}

/// Shared DevTools visibility state.
#[wasm_bindgen(js_name = DevToolsState)]
#[derive(Clone, Debug)]
pub struct DevToolsState {
    inner: CoreDevToolsState,
}

#[wasm_bindgen(js_class = DevToolsState)]
impl DevToolsState {
    /// Create default DevTools state.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: CoreDevToolsState::new(),
        }
    }

    /// Whether the panel is open.
    #[wasm_bindgen(js_name = isOpen)]
    pub fn is_open(&self) -> bool {
        self.inner.is_open()
    }

    /// Set whether the panel is open.
    #[wasm_bindgen(js_name = setOpen)]
    pub fn set_open(&mut self, open: bool) {
        self.inner.set_open(open);
    }

    /// Toggle panel visibility.
    pub fn toggle(&mut self) {
        self.inner.toggle();
    }
}

impl Default for DevToolsState {
    fn default() -> Self {
        Self::new()
    }
}

fn flatten_points(points: &[[f32; 2]]) -> Float32Array {
    let mut flat = Vec::with_capacity(points.len() * 2);
    for point in points {
        flat.push(point[0]);
        flat.push(point[1]);
    }
    f32_array(&flat)
}
