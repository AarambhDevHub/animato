//! Color-space interpolation wrappers.

use animato_core::Interpolate;
use palette::{FromColor, IntoColor, Lab, LinSrgb, Mix, Oklch};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Interpolates a color through CIE L*a*b* space.
///
/// Lab interpolation is useful for perceptually smoother color transitions
/// than direct gamma-encoded RGB interpolation.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InLab<C>(pub C);

/// Interpolates a color through Oklch space.
///
/// Oklch is a modern perceptual cylindrical color space with lightness,
/// chroma, and hue components.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InOklch<C>(pub C);

/// Interpolates a color through linear-light sRGB.
///
/// Linear interpolation avoids blending directly in gamma-encoded sRGB.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InLinear<C>(pub C);

impl<C> InLab<C> {
    /// Wrap a color for Lab interpolation.
    #[inline]
    pub const fn new(color: C) -> Self {
        Self(color)
    }

    /// Return a shared reference to the wrapped color.
    #[inline]
    pub const fn as_inner(&self) -> &C {
        &self.0
    }

    /// Consume the wrapper and return the wrapped color.
    #[inline]
    pub fn into_inner(self) -> C {
        self.0
    }
}

impl<C> InOklch<C> {
    /// Wrap a color for Oklch interpolation.
    #[inline]
    pub const fn new(color: C) -> Self {
        Self(color)
    }

    /// Return a shared reference to the wrapped color.
    #[inline]
    pub const fn as_inner(&self) -> &C {
        &self.0
    }

    /// Consume the wrapper and return the wrapped color.
    #[inline]
    pub fn into_inner(self) -> C {
        self.0
    }
}

impl<C> InLinear<C> {
    /// Wrap a color for linear-light sRGB interpolation.
    #[inline]
    pub const fn new(color: C) -> Self {
        Self(color)
    }

    /// Return a shared reference to the wrapped color.
    #[inline]
    pub const fn as_inner(&self) -> &C {
        &self.0
    }

    /// Consume the wrapper and return the wrapped color.
    #[inline]
    pub fn into_inner(self) -> C {
        self.0
    }
}

impl<C> Interpolate for InLab<C>
where
    C: Clone + IntoColor<Lab> + FromColor<Lab> + 'static,
{
    #[inline]
    fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = clamp01(t);
        let start: Lab = self.0.clone().into_color();
        let end: Lab = other.0.clone().into_color();
        Self(C::from_color(start.mix(end, t)))
    }
}

impl<C> Interpolate for InOklch<C>
where
    C: Clone + IntoColor<Oklch> + FromColor<Oklch> + 'static,
{
    #[inline]
    fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = clamp01(t);
        let start: Oklch = self.0.clone().into_color();
        let end: Oklch = other.0.clone().into_color();
        Self(C::from_color(start.mix(end, t)))
    }
}

impl<C> Interpolate for InLinear<C>
where
    C: Clone + IntoColor<LinSrgb> + FromColor<LinSrgb> + 'static,
{
    #[inline]
    fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = clamp01(t);
        let start: LinSrgb = self.0.clone().into_color();
        let end: LinSrgb = other.0.clone().into_color();
        Self(C::from_color(start.mix(end, t)))
    }
}

#[inline]
fn clamp01(t: f32) -> f32 {
    t.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use palette::Srgb;

    fn approx(a: f32, b: f32) -> bool {
        (a - b).abs() <= 0.0001
    }

    fn assert_rgb_close(a: Srgb, b: Srgb) {
        assert!(approx(a.red, b.red), "red: {} != {}", a.red, b.red);
        assert!(
            approx(a.green, b.green),
            "green: {} != {}",
            a.green,
            b.green
        );
        assert!(approx(a.blue, b.blue), "blue: {} != {}", a.blue, b.blue);
    }

    #[test]
    fn lab_interpolation_keeps_endpoints() {
        let red = Srgb::new(1.0, 0.0, 0.0);
        let blue = Srgb::new(0.0, 0.0, 1.0);

        assert_rgb_close(
            InLab::new(red).lerp(&InLab::new(blue), 0.0).into_inner(),
            red,
        );
        assert_rgb_close(
            InLab::new(red).lerp(&InLab::new(blue), 1.0).into_inner(),
            blue,
        );
    }

    #[test]
    fn lab_midpoint_between_red_and_blue_is_not_muddy_brown() {
        let red = InLab::new(Srgb::new(1.0, 0.0, 0.0));
        let blue = InLab::new(Srgb::new(0.0, 0.0, 1.0));
        let midpoint = red.lerp(&blue, 0.5).into_inner();

        assert!(midpoint.red > 0.45);
        assert!(midpoint.blue > 0.45);
        assert!(midpoint.green < 0.35);
    }

    #[test]
    fn linear_and_lab_midpoints_differ() {
        let red = Srgb::new(1.0, 0.0, 0.0);
        let blue = Srgb::new(0.0, 0.0, 1.0);

        let lab = InLab::new(red).lerp(&InLab::new(blue), 0.5).into_inner();
        let linear = InLinear::new(red)
            .lerp(&InLinear::new(blue), 0.5)
            .into_inner();

        assert!((lab.red - linear.red).abs() > 0.01 || (lab.blue - linear.blue).abs() > 0.01);
    }

    #[test]
    fn oklch_midpoint_is_finite() {
        let red = InOklch::new(Srgb::new(1.0, 0.0, 0.0));
        let blue = InOklch::new(Srgb::new(0.0, 0.0, 1.0));
        let midpoint = red.lerp(&blue, 0.5).into_inner();

        assert!(midpoint.red.is_finite());
        assert!(midpoint.green.is_finite());
        assert!(midpoint.blue.is_finite());
        assert!((0.0..=1.0).contains(&midpoint.red));
        assert!((0.0..=1.0).contains(&midpoint.green));
        assert!((0.0..=1.0).contains(&midpoint.blue));
    }

    #[test]
    fn interpolation_factor_is_clamped() {
        let red = Srgb::new(1.0, 0.0, 0.0);
        let blue = Srgb::new(0.0, 0.0, 1.0);

        let before = InLinear::new(red)
            .lerp(&InLinear::new(blue), -1.0)
            .into_inner();
        let after = InLinear::new(red)
            .lerp(&InLinear::new(blue), 2.0)
            .into_inner();

        assert_rgb_close(before, red);
        assert_rgb_close(after, blue);
    }
}
