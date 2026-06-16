//! Shared DevTools state model.

/// Shared DevTools state consumed by optional rendering backends.
#[derive(Clone, Debug, Default)]
pub struct DevToolsState {
    /// Timeline inspector state.
    #[cfg(feature = "inspector")]
    pub inspector: crate::TimelineInspector,
    /// Easing curve editor state.
    #[cfg(feature = "easing-editor")]
    pub easing_editor: crate::EasingCurveEditor,
    /// Spring visualizer state.
    #[cfg(feature = "spring-viz")]
    pub spring_visualizer: crate::SpringVisualizer,
    /// Recorder controls.
    #[cfg(feature = "recorder")]
    pub recorder: crate::RecorderControls,
    /// Performance monitor state.
    #[cfg(feature = "perf-monitor")]
    pub performance: crate::PerformanceMonitor,
    open: bool,
}

impl DevToolsState {
    /// Create default DevTools state.
    pub fn new() -> Self {
        Self {
            open: true,
            ..Self::default()
        }
    }

    /// Whether a panel should be visible.
    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Set panel visibility.
    pub fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    /// Toggle panel visibility.
    pub fn toggle(&mut self) {
        self.open = !self.open;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggles_visibility() {
        let mut state = DevToolsState::new();
        assert!(state.is_open());
        state.toggle();
        assert!(!state.is_open());
    }
}
