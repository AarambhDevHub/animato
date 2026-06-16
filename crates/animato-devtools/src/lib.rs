//! Runtime DevTools for Animato.
//!
//! This crate provides renderer-agnostic inspection and tuning state for
//! animation debugging. Optional panel modules adapt the shared state to web,
//! desktop, and terminal environments.

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

#[cfg(feature = "easing-editor")]
pub mod easing_editor;
#[cfg(feature = "egui-panel")]
pub mod egui_panel;
#[cfg(feature = "inspector")]
pub mod inspector;
#[cfg(feature = "perf-monitor")]
pub mod perf_monitor;
#[cfg(feature = "recorder")]
pub mod recorder_controls;
#[cfg(feature = "spring-viz")]
pub mod spring_viz;
pub mod state;
#[cfg(feature = "tui-panel")]
pub mod tui_panel;
#[cfg(feature = "web-panel")]
pub mod web_panel;

#[cfg(feature = "easing-editor")]
pub use easing_editor::EasingCurveEditor;
#[cfg(feature = "egui-panel")]
pub use egui_panel::DevToolsEguiPanel;
#[cfg(feature = "inspector")]
pub use inspector::{AnimationSnapshot, TimelineInspector};
#[cfg(feature = "perf-monitor")]
pub use perf_monitor::{AnimationCostRecord, PerformanceMonitor};
#[cfg(feature = "recorder")]
pub use recorder_controls::RecorderControls;
#[cfg(feature = "spring-viz")]
pub use spring_viz::{SpringFrame, SpringVisualizer};
pub use state::DevToolsState;
#[cfg(feature = "tui-panel")]
pub use tui_panel::DevToolsTuiPanel;
#[cfg(feature = "web-panel")]
pub use web_panel::DevToolsWebPanel;
