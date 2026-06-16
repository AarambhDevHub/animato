//! Terminal panel adapter.

use crate::DevToolsState;

/// Ratatui-oriented DevTools panel state.
#[derive(Clone, Debug, PartialEq)]
pub struct DevToolsTuiPanel {
    title: String,
}

impl DevToolsTuiPanel {
    /// Create a TUI panel.
    pub fn new() -> Self {
        Self {
            title: "Animato DevTools".to_owned(),
        }
    }

    /// Render a compact text summary for terminal widgets.
    pub fn render_summary(&self, state: &DevToolsState) -> String {
        format!("{} open={}", self.title, state.is_open())
    }

    /// Panel title.
    pub fn title(&self) -> &str {
        &self.title
    }
}

impl Default for DevToolsTuiPanel {
    fn default() -> Self {
        Self::new()
    }
}
