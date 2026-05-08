//! Private component decomposition for multi-dimensional inertia.

use animato_core::Animatable;

mod private {
    pub trait Sealed {}
}

/// A type that can be split into independent `f32` components.
///
/// This trait is sealed and only implemented for `f32`, `[f32; 2]`,
/// `[f32; 3]`, and `[f32; 4]`.
pub trait Decompose: Animatable + private::Sealed {
    /// Number of f32 components in this type.
    fn component_count() -> usize;
    /// Write all components into `out`.
    fn write_components(&self, out: &mut [f32]);
    /// Reconstruct a value from component values.
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

    fn from_components(components: &[f32]) -> Self {
        components[0]
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

    fn from_components(components: &[f32]) -> Self {
        [components[0], components[1]]
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

    fn from_components(components: &[f32]) -> Self {
        [components[0], components[1], components[2]]
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

    fn from_components(components: &[f32]) -> Self {
        [components[0], components[1], components[2], components[3]]
    }
}
