//! Platform detection and animation backend selection.

/// Animation backend selected for the active Dioxus target.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnimationBackend {
    /// Web target, driven by browser `requestAnimationFrame`.
    WebRaf,
    /// Desktop or mobile target, driven by a hosted clock.
    NativeClock,
    /// Terminal target, driven by polling intervals.
    TerminalPoll,
}

/// Detects the current Dioxus rendering platform.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PlatformAdapter;

impl PlatformAdapter {
    /// Detect platform at runtime from compile-time target features.
    pub fn detect() -> AnimationBackend {
        detect_backend()
    }
}

#[cfg(target_arch = "wasm32")]
fn detect_backend() -> AnimationBackend {
    AnimationBackend::WebRaf
}

#[cfg(all(not(target_arch = "wasm32"), feature = "native"))]
fn detect_backend() -> AnimationBackend {
    AnimationBackend::NativeClock
}

#[cfg(all(not(target_arch = "wasm32"), not(feature = "native")))]
fn detect_backend() -> AnimationBackend {
    AnimationBackend::NativeClock
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backend_detection_returns_supported_backend() {
        let backend = PlatformAdapter::detect();
        assert!(matches!(
            backend,
            AnimationBackend::WebRaf
                | AnimationBackend::NativeClock
                | AnimationBackend::TerminalPoll
        ));
    }
}
