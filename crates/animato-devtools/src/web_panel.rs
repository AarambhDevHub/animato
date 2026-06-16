//! Web overlay panel adapter.

use crate::DevToolsState;

/// Floating web overlay panel state.
#[derive(Clone, Debug, PartialEq)]
pub struct DevToolsWebPanel {
    shortcut: String,
}

impl DevToolsWebPanel {
    /// Create a web panel toggled by the backtick key.
    pub fn new() -> Self {
        Self {
            shortcut: "`".to_owned(),
        }
    }

    /// Create a web panel with a custom shortcut label.
    pub fn with_shortcut(shortcut: impl Into<String>) -> Self {
        Self {
            shortcut: shortcut.into(),
        }
    }

    /// Shortcut label.
    pub fn shortcut(&self) -> &str {
        &self.shortcut
    }

    /// Toggle the shared state.
    pub fn toggle(&self, state: &mut DevToolsState) {
        state.toggle();
    }

    /// Render a compact text summary suitable for browser overlays.
    pub fn render_summary(&self, state: &DevToolsState) -> String {
        format!(
            "Animato DevTools [{}] open={}",
            self.shortcut,
            state.is_open()
        )
    }
}

impl Default for DevToolsWebPanel {
    fn default() -> Self {
        Self::new()
    }
}
