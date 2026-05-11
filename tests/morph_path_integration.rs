//! Integration tests for v0.8.0 MorphPath and DrawSvg.

use animato::{CubicBezierCurve, DrawSvg, Easing, LineSegment, MorphPath, Tween, Update, resample};

// ── resample ──────────────────────────────────────────────────────────────────

#[test]
fn resample_empty_is_empty() {
    assert!(resample(&[], 5).is_empty());
}

#[test]
fn resample_exact_length() {
    let pts: Vec<[f32; 2]> = (0..5).map(|i| [i as f32 * 25.0, 0.0]).collect();
    let r = resample(&pts, 9);
    assert_eq!(r.len(), 9);
}

#[test]
fn resample_preserves_total_length_roughly() {
    let pts = vec![[0.0_f32, 0.0], [100.0, 0.0], [100.0, 100.0]];
    let r = resample(&pts, 17);
    // Resampled points should still span the same approximate bounding box.
    let max_x = r.iter().map(|p| p[0]).fold(0.0_f32, f32::max);
    let max_y = r.iter().map(|p| p[1]).fold(0.0_f32, f32::max);
    assert!(max_x > 90.0, "max_x={max_x}");
    assert!(max_y > 90.0, "max_y={max_y}");
}

#[test]
fn resample_single_input_point_fills() {
    let r = resample(&[[7.0, 3.0]], 5);
    assert_eq!(r.len(), 5);
    assert!(r.iter().all(|&p| p == [7.0, 3.0]));
}

// ── MorphPath ────────────────────────────────────────────────────────────────

#[test]
fn morph_at_zero_equals_from() {
    let from = vec![[0.0_f32, 0.0], [100.0, 0.0]];
    let to = vec![[0.0_f32, 50.0], [100.0, 50.0]];
    let morph = MorphPath::new(from.clone(), to);
    let result = morph.evaluate(0.0);
    let expected = resample(&from, morph.point_count());
    for (r, e) in result.iter().zip(expected.iter()) {
        assert!((r[0] - e[0]).abs() < 0.01);
        assert!((r[1] - e[1]).abs() < 0.01);
    }
}

#[test]
fn morph_at_one_equals_to() {
    let from = vec![[0.0_f32, 0.0], [100.0, 0.0]];
    let to = vec![[0.0_f32, 50.0], [100.0, 50.0]];
    let morph = MorphPath::new(from, to.clone());
    let result = morph.evaluate(1.0);
    let expected = resample(&to, morph.point_count());
    for (r, e) in result.iter().zip(expected.iter()) {
        assert!((r[0] - e[0]).abs() < 0.01);
        assert!((r[1] - e[1]).abs() < 0.01);
    }
}

#[test]
fn morph_midpoint_is_halfway() {
    let from = vec![[0.0_f32, 0.0], [0.0, 0.0]];
    let to = vec![[100.0_f32, 0.0], [100.0, 0.0]];
    let morph = MorphPath::with_resolution(from, to, 2);
    let mid = morph.evaluate(0.5);
    assert!((mid[0][0] - 50.0).abs() < 0.01);
}

#[test]
fn morph_unequal_point_counts_auto_resampled() {
    let from: Vec<[f32; 2]> = (0..4).map(|i| [i as f32 * 10.0, 0.0]).collect();
    let to: Vec<[f32; 2]> = (0..7).map(|i| [i as f32 * 5.0, 20.0]).collect();
    let morph = MorphPath::new(from, to);
    // Both resampled to max(4, 7) = 7
    assert_eq!(morph.point_count(), 7);
}

#[test]
fn morph_bounds_at_zero() {
    let from = vec![[0.0_f32, 0.0], [100.0, 0.0], [100.0, 100.0], [0.0, 100.0]];
    let to = from.clone();
    let morph = MorphPath::new(from, to);
    let bounds = morph.bounds_at(0.0);
    assert!(bounds[0] >= -0.01 && bounds[2] <= 100.01);
    assert!(bounds[1] >= -0.01 && bounds[3] <= 100.01);
}

#[test]
fn morph_driven_by_tween() {
    let from = vec![[0.0_f32, 0.0], [100.0, 0.0]];
    let to = vec![[0.0_f32, 100.0], [100.0, 100.0]];
    let morph = MorphPath::with_resolution(from, to, 2);

    let mut tween = Tween::new(0.0_f32, 1.0)
        .duration(1.0)
        .easing(Easing::Linear)
        .build();

    tween.update(0.5);
    let shape = morph.evaluate(tween.value());
    // At t=0.5, y-values should be ~50
    assert!((shape[0][1] - 50.0).abs() < 0.01);
    assert!((shape[1][1] - 50.0).abs() < 0.01);
}

// ── DrawSvg ──────────────────────────────────────────────────────────────────

#[test]
fn draw_svg_on_zero_fully_hidden() {
    let line = LineSegment::new([0.0, 0.0], [100.0, 0.0]);
    let v = line.draw_on(0.0);
    assert_eq!(v.dash_array, 100.0);
    assert_eq!(v.dash_offset, 100.0);
}

#[test]
fn draw_svg_on_one_fully_visible() {
    let line = LineSegment::new([0.0, 0.0], [100.0, 0.0]);
    let v = line.draw_on(1.0);
    assert_eq!(v.dash_offset, 0.0);
}

#[test]
fn draw_svg_on_half_progress() {
    let line = LineSegment::new([0.0, 0.0], [200.0, 0.0]);
    let v = line.draw_on(0.5);
    assert!((v.dash_offset - 100.0).abs() < 0.01);
    assert!((v.progress() - 0.5).abs() < 0.01);
}

#[test]
fn draw_svg_reverse_erases_from_end() {
    let line = LineSegment::new([0.0, 0.0], [100.0, 0.0]);
    let fwd = line.draw_on(0.3);
    let rev = line.draw_on_reverse(0.7);
    // Both show 30% drawn — same dashoffset.
    assert!((fwd.dash_offset - rev.dash_offset).abs() < 0.001);
}

#[test]
fn cubic_bezier_has_draw_svg_via_blanket() {
    let curve = CubicBezierCurve::new([0.0, 0.0], [50.0, 100.0], [150.0, -100.0], [200.0, 0.0]);
    let v = curve.draw_on(0.5);
    assert!(v.dash_array > 100.0);
    assert!(v.dash_offset > 0.0);
    assert!((v.progress() - 0.5).abs() < 0.01);
}

#[test]
fn draw_values_progress_round_trip() {
    for i in 0..=10 {
        let p = i as f32 / 10.0;
        let line = LineSegment::new([0.0, 0.0], [100.0, 0.0]);
        let v = line.draw_on(p);
        assert!(
            (v.progress() - p).abs() < 0.001,
            "progress round-trip failed at p={p}"
        );
    }
}

#[test]
fn draw_svg_to_css() {
    let line = LineSegment::new([0.0, 0.0], [314.159, 0.0]);
    let v = line.draw_on(0.5);
    let css = v.to_css();
    assert!(css.contains("stroke-dasharray"));
    assert!(css.contains("stroke-dashoffset"));
}
