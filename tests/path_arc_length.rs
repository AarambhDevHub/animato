//! Integration test: path arc-length evaluation.

use animato::{CatmullRomSpline, CubicBezierCurve, PathEvaluate, QuadBezier};

fn distance(a: [f32; 2], b: [f32; 2]) -> f32 {
    let dx = b[0] - a[0];
    let dy = b[1] - a[1];
    (dx * dx + dy * dy).sqrt()
}

#[test]
fn quadratic_bezier_endpoints_and_length() {
    let curve = QuadBezier::new([0.0, 0.0], [50.0, 100.0], [100.0, 0.0]);

    assert_eq!(curve.position(0.0), [0.0, 0.0]);
    assert_eq!(curve.position(1.0), [100.0, 0.0]);
    assert!(curve.arc_length() > 100.0);
}

#[test]
fn cubic_bezier_straight_line_uses_uniform_distance() {
    let curve = CubicBezierCurve::new([0.0, 0.0], [25.0, 0.0], [75.0, 0.0], [100.0, 0.0]);

    assert!((curve.arc_length() - 100.0).abs() < 0.01);
    assert!((curve.position(0.25)[0] - 25.0).abs() < 1.0);
    assert!((curve.position(0.75)[0] - 75.0).abs() < 1.0);
}

#[test]
fn arc_length_samples_are_monotonic() {
    let curve = CubicBezierCurve::new([0.0, 0.0], [0.0, 100.0], [100.0, 100.0], [100.0, 0.0]);

    let mut previous = curve.position(0.0);
    let mut traveled = 0.0_f32;
    for step in 1..=32 {
        let point = curve.position(step as f32 / 32.0);
        let segment = distance(previous, point);
        assert!(segment >= 0.0);
        traveled += segment;
        previous = point;
    }

    assert!(traveled <= curve.arc_length() * 1.05);
    assert!(traveled >= curve.arc_length() * 0.90);
}

#[test]
fn catmull_rom_spline_interpolates_first_and_last_points() {
    let spline = CatmullRomSpline::new(vec![[0.0, 0.0], [50.0, 100.0], [100.0, 0.0]]);

    assert_eq!(spline.position(0.0), [0.0, 0.0]);
    assert_eq!(spline.position(1.0), [100.0, 0.0]);
    assert!(spline.arc_length() > 100.0);
}
