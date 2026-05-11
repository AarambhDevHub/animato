//! Integration tests for v0.8.0 advanced easing variants.

use animato::Easing;
use animato::easing::{custom_bounce, ease_out_bounce, expo_scale, rough_ease, slow_mo, wiggle};

const EPSILON: f32 = 1e-5;

fn approx(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

// ── Endpoint invariants ───────────────────────────────────────────────────────

#[test]
fn all_advanced_variants_satisfy_zero_to_one() {
    let variants = [
        Easing::RoughEase {
            strength: 0.5,
            points: 8,
        },
        Easing::SlowMo {
            linear_ratio: 0.5,
            power: 0.7,
        },
        Easing::Wiggle { wiggles: 5 },
        Easing::CustomBounce { strength: 0.7 },
        Easing::ExpoScale {
            start: 0.5,
            end: 2.0,
        },
    ];
    for v in &variants {
        assert!(approx(v.apply(0.0), 0.0), "{:?}.apply(0.0) ≠ 0.0", v);
        assert!(approx(v.apply(1.0), 1.0), "{:?}.apply(1.0) ≠ 1.0", v);
    }
}

#[test]
fn all_advanced_variants_no_panic_out_of_range() {
    let variants = [
        Easing::RoughEase {
            strength: 0.5,
            points: 8,
        },
        Easing::SlowMo {
            linear_ratio: 0.5,
            power: 0.7,
        },
        Easing::Wiggle { wiggles: 5 },
        Easing::CustomBounce { strength: 0.7 },
        Easing::ExpoScale {
            start: 0.5,
            end: 2.0,
        },
    ];
    for v in &variants {
        let _ = v.apply(-1.0);
        let _ = v.apply(2.0);
        let _ = v.apply(f32::INFINITY);
        let _ = v.apply(f32::NEG_INFINITY);
    }
}

// ── RoughEase ────────────────────────────────────────────────────────────────

#[test]
fn rough_ease_is_finite_across_range() {
    for i in 0..=100 {
        let t = i as f32 / 100.0;
        let v = rough_ease(t, 0.5, 8);
        assert!(v.is_finite(), "rough_ease at t={t} is non-finite: {v}");
    }
}

#[test]
fn rough_ease_zero_strength_is_approximately_linear() {
    for i in 1..10 {
        let t = i as f32 / 10.0;
        assert!(
            approx(rough_ease(t, 0.0, 8), t),
            "rough_ease(t={t}, strength=0) should be linear"
        );
    }
}

// ── SlowMo ───────────────────────────────────────────────────────────────────

#[test]
fn slow_mo_linear_ratio_one_is_linear() {
    for i in 1..10 {
        let t = i as f32 / 10.0;
        assert!(
            approx(slow_mo(t, 1.0, 1.0), t),
            "SlowMo lr=1.0 should be linear at t={t}"
        );
    }
}

#[test]
fn slow_mo_middle_velocity_less_than_edges() {
    let dt = 0.01_f32;
    let mid = (slow_mo(0.5 + dt, 0.5, 1.0) - slow_mo(0.5, 0.5, 1.0)) / dt;
    let edge = (slow_mo(0.05 + dt, 0.5, 1.0) - slow_mo(0.05, 0.5, 1.0)) / dt;
    assert!(
        mid < edge,
        "middle ({mid:.4}) should be slower than edges ({edge:.4})"
    );
}

#[test]
fn slow_mo_degenerate_zero_lr() {
    assert!(approx(slow_mo(0.0, 0.0, 1.0), 0.0));
    assert!(approx(slow_mo(1.0, 0.0, 1.0), 1.0));
}

// ── Wiggle ────────────────────────────────────────────────────────────────────

#[test]
fn wiggle_is_finite_everywhere() {
    for i in 0..=100 {
        let t = i as f32 / 100.0;
        assert!(wiggle(t, 5).is_finite());
    }
}

#[test]
fn wiggle_zero_wiggles_treated_as_one() {
    // wiggles=0 is clamped to 1 — should not panic.
    let v = wiggle(0.5, 0);
    assert!(v.is_finite());
}

// ── CustomBounce ──────────────────────────────────────────────────────────────

#[test]
fn custom_bounce_zero_is_linear() {
    for i in 1..10 {
        let t = i as f32 / 10.0;
        assert!(
            approx(custom_bounce(t, 0.0), t),
            "strength=0 should be linear at t={t}"
        );
    }
}

#[test]
fn custom_bounce_one_equals_ease_out_bounce() {
    for i in 1..10 {
        let t = i as f32 / 10.0;
        let cb = custom_bounce(t, 1.0);
        let eb = ease_out_bounce(t);
        assert!(
            approx(cb, eb),
            "custom_bounce(t={t}, 1.0) = {cb} ≠ ease_out_bounce = {eb}"
        );
    }
}

// ── ExpoScale ────────────────────────────────────────────────────────────────

#[test]
fn expo_scale_equal_start_end_is_linear() {
    for i in 1..10 {
        let t = i as f32 / 10.0;
        assert!(
            approx(expo_scale(t, 1.0, 1.0), t),
            "expo_scale start==end should be linear at t={t}"
        );
    }
}

#[test]
fn expo_scale_is_monotonically_increasing() {
    let mut prev = 0.0_f32;
    for i in 1..=20 {
        let t = i as f32 / 20.0;
        let v = expo_scale(t, 0.5, 2.0);
        assert!(
            v >= prev - 1e-4,
            "expo_scale should be monotone at t={t}: {v} < {prev}"
        );
        prev = v;
    }
}

#[test]
fn expo_scale_small_start_curves_up() {
    // With start < 1.0 and end > 1.0, early values grow slowly.
    let early = expo_scale(0.1, 0.1, 10.0);
    let late = expo_scale(0.9, 0.1, 10.0);
    assert!(
        late > 0.5,
        "late value should be well above 0.5, got {late}"
    );
    assert!(early < 0.2, "early value should be small, got {early}");
}

// ── all_named count ───────────────────────────────────────────────────────────

#[test]
fn all_named_returns_38_variants() {
    assert_eq!(Easing::all_named().len(), 38);
}

// ── Tween integration ─────────────────────────────────────────────────────────

#[test]
fn tween_can_use_advanced_easings() {
    use animato::{Tween, Update};

    let variants = [
        Easing::RoughEase {
            strength: 0.3,
            points: 6,
        },
        Easing::SlowMo {
            linear_ratio: 0.4,
            power: 0.5,
        },
        Easing::Wiggle { wiggles: 3 },
        Easing::CustomBounce { strength: 0.5 },
        Easing::ExpoScale {
            start: 0.3,
            end: 3.0,
        },
    ];

    for easing in &variants {
        let mut tween = Tween::new(0.0_f32, 100.0)
            .duration(1.0)
            .easing(easing.clone())
            .build();
        tween.update(0.5);
        assert!(
            tween.value().is_finite(),
            "{:?} produced non-finite value",
            easing
        );
        tween.update(0.5);
        assert!(tween.is_complete(), "{:?} tween did not complete", easing);
        let final_val = tween.value();
        assert!(
            (final_val - 100.0).abs() < 1.0,
            "{:?} final value {final_val} ≠ 100",
            easing
        );
    }
}
