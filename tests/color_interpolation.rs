//! Integration tests for v0.6.0 color interpolation APIs.

use animato::{InLab, InLinear, InOklch, Interpolate, Tween, Update, palette::Srgb};

#[test]
fn facade_exports_color_wrappers() {
    let red = InLab::new(Srgb::new(1.0, 0.0, 0.0));
    let blue = InLab::new(Srgb::new(0.0, 0.0, 1.0));
    let midpoint = red.lerp(&blue, 0.5).into_inner();

    assert!(midpoint.red > 0.45);
    assert!(midpoint.blue > 0.45);
    assert!(midpoint.green < 0.35);
}

#[test]
fn tween_can_drive_lab_color() {
    let mut tween = Tween::new(
        InLab::new(Srgb::new(1.0, 0.0, 0.0)),
        InLab::new(Srgb::new(0.0, 0.0, 1.0)),
    )
    .duration(1.0)
    .build();

    tween.update(0.5);
    let midpoint = tween.value().into_inner();

    assert!(midpoint.red.is_finite());
    assert!(midpoint.green.is_finite());
    assert!(midpoint.blue.is_finite());
}

#[test]
fn linear_and_lab_wrappers_choose_different_spaces() {
    let red = Srgb::new(1.0, 0.0, 0.0);
    let blue = Srgb::new(0.0, 0.0, 1.0);

    let lab = InLab::new(red).lerp(&InLab::new(blue), 0.5).into_inner();
    let linear = InLinear::new(red)
        .lerp(&InLinear::new(blue), 0.5)
        .into_inner();

    assert!((lab.red - linear.red).abs() > 0.01 || (lab.blue - linear.blue).abs() > 0.01);
}

#[test]
fn oklch_wrapper_produces_usable_color() {
    let red = InOklch::new(Srgb::new(1.0, 0.0, 0.0));
    let blue = InOklch::new(Srgb::new(0.0, 0.0, 1.0));
    let midpoint = red.lerp(&blue, 0.5).into_inner();

    assert!((0.0..=1.0).contains(&midpoint.red));
    assert!((0.0..=1.0).contains(&midpoint.green));
    assert!((0.0..=1.0).contains(&midpoint.blue));
}
