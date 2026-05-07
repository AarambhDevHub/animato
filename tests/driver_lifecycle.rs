//! Integration test: AnimationDriver lifecycle.

use animato_core::Easing;
use animato_driver::{AnimationDriver, Clock, MockClock};
use animato_tween::Tween;

const DT: f32 = 1.0 / 60.0;

// ── Basic lifecycle ───────────────────────────────────────────────────────────

#[test]
fn empty_driver_active_count_is_zero() {
    let driver = AnimationDriver::new();
    assert_eq!(driver.active_count(), 0);
}

#[test]
fn add_increases_active_count() {
    let mut driver = AnimationDriver::new();
    driver.add(Tween::new(0.0_f32, 1.0).duration(1.0).build());
    driver.add(Tween::new(0.0_f32, 1.0).duration(1.0).build());
    assert_eq!(driver.active_count(), 2);
}

#[test]
fn completed_animation_auto_removed() {
    let mut driver = AnimationDriver::new();
    let id = driver.add(Tween::new(0.0_f32, 1.0).duration(0.5).build());
    assert!(driver.is_active(id));
    driver.tick(1.0); // past duration
    assert!(!driver.is_active(id));
    assert_eq!(driver.active_count(), 0);
}

#[test]
fn active_animation_not_removed_early() {
    let mut driver = AnimationDriver::new();
    let id = driver.add(Tween::new(0.0_f32, 1.0).duration(5.0).build());
    driver.tick(1.0);
    assert!(driver.is_active(id));
}

// ── MockClock integration ─────────────────────────────────────────────────────

#[test]
fn mock_clock_drives_completion() {
    let mut driver = AnimationDriver::new();
    let mut clock = MockClock::new(DT);
    let id = driver.add(Tween::new(0.0_f32, 1.0).duration(1.0).build());

    // 60 frames × (1/60)s = exactly 1.0s
    for _ in 0..60 {
        driver.tick(clock.delta());
    }
    // 61st frame pushes past and triggers removal
    driver.tick(clock.delta());
    assert!(!driver.is_active(id));
}

// ── Cancel ────────────────────────────────────────────────────────────────────

#[test]
fn cancel_removes_active_animation() {
    let mut driver = AnimationDriver::new();
    let id = driver.add(Tween::new(0.0_f32, 1.0).duration(10.0).build());
    driver.tick(DT);
    driver.cancel(id);
    assert!(!driver.is_active(id));
}

#[test]
fn cancel_stale_id_is_noop() {
    let mut driver = AnimationDriver::new();
    let id = driver.add(Tween::new(0.0_f32, 1.0).duration(0.1).build());
    driver.tick(1.0); // completes + auto-removed
    driver.cancel(id); // should not panic
    assert_eq!(driver.active_count(), 0);
}

#[test]
fn cancel_all_clears_driver() {
    let mut driver = AnimationDriver::new();
    for _ in 0..5 {
        driver.add(Tween::new(0.0_f32, 1.0).duration(10.0).build());
    }
    assert_eq!(driver.active_count(), 5);
    driver.cancel_all();
    assert_eq!(driver.active_count(), 0);
}

// ── Multiple concurrent ───────────────────────────────────────────────────────

#[test]
fn three_animations_complete_independently() {
    let mut driver = AnimationDriver::new();
    let a = driver.add(Tween::new(0.0_f32, 1.0).duration(0.5).build());
    let b = driver.add(Tween::new(0.0_f32, 1.0).duration(1.0).build());
    let c = driver.add(Tween::new(0.0_f32, 1.0).duration(2.0).build());

    driver.tick(0.6); // a done
    assert!(!driver.is_active(a));
    assert!(driver.is_active(b));
    assert!(driver.is_active(c));
    assert_eq!(driver.active_count(), 2);

    driver.tick(0.6); // b done
    assert!(!driver.is_active(b));
    assert!(driver.is_active(c));
    assert_eq!(driver.active_count(), 1);

    driver.tick(1.0); // c done
    assert!(!driver.is_active(c));
    assert_eq!(driver.active_count(), 0);
}

// ── ID uniqueness ─────────────────────────────────────────────────────────────

#[test]
fn all_ids_are_unique() {
    let mut driver = AnimationDriver::new();
    let ids: Vec<_> = (0..50)
        .map(|_| driver.add(Tween::new(0.0_f32, 1.0).duration(1.0).build()))
        .collect();
    let set: std::collections::HashSet<_> = ids.iter().collect();
    assert_eq!(set.len(), 50, "all IDs must be unique");
}

// ── Mixed easing ─────────────────────────────────────────────────────────────

#[test]
fn multiple_easings_all_complete() {
    let easings = [
        Easing::Linear,
        Easing::EaseInQuad,
        Easing::EaseOutCubic,
        Easing::EaseInOutSine,
        Easing::EaseOutBounce,
        Easing::EaseOutElastic,
    ];
    let mut driver = AnimationDriver::new();
    let ids: Vec<_> = easings
        .iter()
        .cloned()
        .map(|e| driver.add(Tween::new(0.0_f32, 100.0).duration(1.0).easing(e).build()))
        .collect();

    driver.tick(2.0); // all should be done
    for id in ids {
        assert!(!driver.is_active(id));
    }
    assert_eq!(driver.active_count(), 0);
}
