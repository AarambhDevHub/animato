//! Desktop panel adapter.

use crate::DevToolsState;

/// Egui-compatible DevTools panel state.
///
/// This lightweight type keeps `animato-devtools` usable without forcing a
/// specific egui version. Applications can feed the summary text into their
/// own egui window.
#[derive(Clone, Debug, PartialEq)]
pub struct DevToolsEguiPanel {
    title: String,
}

impl DevToolsEguiPanel {
    /// Create a panel titled `Animato DevTools`.
    pub fn new() -> Self {
        Self {
            title: "Animato DevTools".to_owned(),
        }
    }

    /// Set the panel title.
    pub fn with_title(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
        }
    }

    /// Panel title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Render a compact text summary.
    pub fn render_summary(&self, state: &DevToolsState) -> String {
        format!("{} open={}", self.title, state.is_open())
    }
}

impl Default for DevToolsEguiPanel {
    fn default() -> Self {
        Self::new()
    }
}
