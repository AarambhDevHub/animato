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

#[cfg(test)]
mod tests {
    use super::Decompose;

    #[test]
    fn f32_round_trips_components() {
        let mut out = [0.0; 1];

        3.5_f32.write_components(&mut out);

        assert_eq!(<f32 as Decompose>::component_count(), 1);
        assert_eq!(out, [3.5]);
        assert_eq!(<f32 as Decompose>::from_components(&out), 3.5);
    }

    #[test]
    fn vec2_round_trips_components() {
        let mut out = [0.0; 2];

        [1.0_f32, 2.0].write_components(&mut out);

        assert_eq!(<[f32; 2] as Decompose>::component_count(), 2);
        assert_eq!(out, [1.0, 2.0]);
        assert_eq!(<[f32; 2] as Decompose>::from_components(&out), [1.0, 2.0]);
    }

    #[test]
    fn vec3_round_trips_components() {
        let mut out = [0.0; 3];

        [1.0_f32, 2.0, 3.0].write_components(&mut out);

        assert_eq!(<[f32; 3] as Decompose>::component_count(), 3);
        assert_eq!(out, [1.0, 2.0, 3.0]);
        assert_eq!(
            <[f32; 3] as Decompose>::from_components(&out),
            [1.0, 2.0, 3.0]
        );
    }

    #[test]
    fn vec4_round_trips_components() {
        let mut out = [0.0; 4];

        [1.0_f32, 2.0, 3.0, 4.0].write_components(&mut out);

        assert_eq!(<[f32; 4] as Decompose>::component_count(), 4);
        assert_eq!(out, [1.0, 2.0, 3.0, 4.0]);
        assert_eq!(
            <[f32; 4] as Decompose>::from_components(&out),
            [1.0, 2.0, 3.0, 4.0]
        );
    }
}
