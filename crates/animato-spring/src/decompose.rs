//! Private `Decompose` trait — splits an `Animatable` into f32 components and back.
//!
//! Uses the sealed-trait pattern: `Decompose` is `pub` (required for `SpringN<T>` bounds)
//! but `Sealed` is private, preventing external implementations.

use animato_core::Animatable;

mod private {
    pub trait Sealed {}
}

/// A type that can be decomposed into f32 components and recomposed.
///
/// This trait is sealed — it cannot be implemented outside `animato-spring`.
/// It is implemented for `f32`, `[f32; 2]`, `[f32; 3]`, `[f32; 4]`.
pub trait Decompose: Animatable + private::Sealed {
    /// Number of f32 components in this type.
    fn component_count() -> usize;
    /// Write all components into `out`. `out.len()` must equal `component_count()`.
    fn write_components(&self, out: &mut [f32]);
    /// Reconstruct from a slice of components.
    fn from_components(components: &[f32]) -> Self;
}

impl private::Sealed for f32 {}
impl Decompose for f32 {
    fn component_count() -> usize {
        1
    }
    fn write_components(&self, out: &mut [f32]) {
        out[0] = *self;
    }
    fn from_components(c: &[f32]) -> Self {
        c[0]
    }
}

impl private::Sealed for [f32; 2] {}
impl Decompose for [f32; 2] {
    fn component_count() -> usize {
        2
    }
    fn write_components(&self, out: &mut [f32]) {
        out[0] = self[0];
        out[1] = self[1];
    }
    fn from_components(c: &[f32]) -> Self {
        [c[0], c[1]]
    }
}

impl private::Sealed for [f32; 3] {}
impl Decompose for [f32; 3] {
    fn component_count() -> usize {
        3
    }
    fn write_components(&self, out: &mut [f32]) {
        out[0] = self[0];
        out[1] = self[1];
        out[2] = self[2];
    }
    fn from_components(c: &[f32]) -> Self {
        [c[0], c[1], c[2]]
    }
}

impl private::Sealed for [f32; 4] {}
impl Decompose for [f32; 4] {
    fn component_count() -> usize {
        4
    }
    fn write_components(&self, out: &mut [f32]) {
        out[0] = self[0];
        out[1] = self[1];
        out[2] = self[2];
        out[3] = self[3];
    }
    fn from_components(c: &[f32]) -> Self {
        [c[0], c[1], c[2], c[3]]
    }
}
