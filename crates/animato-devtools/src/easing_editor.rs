//! Easing curve editor state.

use animato_core::Easing;

/// Interactive easing curve editor.
#[derive(Clone, Debug, PartialEq)]
pub struct EasingCurveEditor {
    /// Current easing curve.
    pub current: Easing,
    /// Optional comparison curve.
    pub compare: Option<Easing>,
    /// Number of sample points used for rendering.
    pub sample_count: usize,
}

impl EasingCurveEditor {
    /// Create an editor for an easing curve.
    pub fn new(easing: Easing) -> Self {
        Self {
            current: easing,
            compare: None,
            sample_count: 100,
        }
    }

    /// Set the primary easing.
    pub fn set_easing(&mut self, easing: Easing) {
        self.current = easing;
    }

    /// Set an optional comparison easing.
    pub fn set_compare(&mut self, easing: Option<Easing>) {
        self.compare = easing;
    }

    /// Set curve sampling resolution. Values below two are clamped.
    pub fn set_sample_count(&mut self, sample_count: usize) {
        self.sample_count = sample_count.max(2);
    }

    /// Return sample points for the current easing.
    pub fn samples(&self) -> Vec<[f32; 2]> {
        let mut out = Vec::with_capacity(self.sample_count.max(2));
        self.samples_into(&mut out);
        out
    }

    /// Write sample points for the current easing into a reusable buffer.
    pub fn samples_into(&self, out: &mut Vec<[f32; 2]>) {
        sample_easing(&self.current, self.sample_count, out);
    }

    /// Return sample points for the comparison easing.
    pub fn compare_samples(&self) -> Option<Vec<[f32; 2]>> {
        self.compare.as_ref().map(|easing| {
            let mut out = Vec::with_capacity(self.sample_count.max(2));
            sample_easing(easing, self.sample_count, &mut out);
            out
        })
    }

    /// Update cubic-bezier control points and make them the current easing.
    pub fn set_control_points(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        self.current = Easing::CubicBezier(x1, y1, x2, y2);
    }

    /// Return Rust code for the current easing.
    pub fn copy_code(&self) -> String {
        match self.current {
            Easing::CubicBezier(x1, y1, x2, y2) => {
                format!("Easing::CubicBezier({x1:.3}, {y1:.3}, {x2:.3}, {y2:.3})")
            }
            Easing::Steps(steps) => format!("Easing::Steps({steps})"),
            _ => format!("Easing::{:?}", self.current),
        }
    }
}

impl Default for EasingCurveEditor {
    fn default() -> Self {
        Self::new(Easing::Linear)
    }
}

fn sample_easing(easing: &Easing, sample_count: usize, out: &mut Vec<[f32; 2]>) {
    out.clear();
    let count = sample_count.max(2);
    for index in 0..count {
        let t = index as f32 / (count - 1) as f32;
        out.push([t, easing.apply(t)]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples_include_endpoints() {
        let mut editor = EasingCurveEditor::new(Easing::EaseOutCubic);
        editor.set_sample_count(8);
        let samples = editor.samples();
        assert_eq!(samples.len(), 8);
        assert_eq!(samples[0], [0.0, 0.0]);
        assert_eq!(samples[7], [1.0, 1.0]);
    }

    #[test]
    fn compare_and_cubic_updates_work() {
        let mut editor = EasingCurveEditor::default();
        editor.set_compare(Some(Easing::EaseInSine));
        assert!(editor.compare_samples().is_some());
        editor.set_control_points(0.1, 0.2, 0.3, 0.4);
        assert_eq!(editor.current, Easing::CubicBezier(0.1, 0.2, 0.3, 0.4));
        assert!(editor.copy_code().contains("CubicBezier"));
    }
}
