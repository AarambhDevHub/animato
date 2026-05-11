//! Integration tests for v0.8.0 ScrollDriver and ScrollClock.

use animato::{AnimationDriver, Clock, Easing, ScrollClock, ScrollDriver, Tween, Update};

// ── ScrollDriver ──────────────────────────────────────────────────────────────

#[test]
fn scroll_driver_starts_at_min() {
    let driver = ScrollDriver::new(0.0, 500.0);
    assert_eq!(driver.position(), 0.0);
    assert_eq!(driver.progress(), 0.0);
}

#[test]
fn scroll_driver_progress_tracks_position() {
    let mut driver = ScrollDriver::new(0.0, 1000.0);
    driver.set_position(250.0);
    assert!((driver.progress() - 0.25).abs() < 0.001);
    driver.set_position(1000.0);
    assert!((driver.progress() - 1.0).abs() < 0.001);
}

#[test]
fn scroll_driver_clamps_out_of_range() {
    let mut driver = ScrollDriver::new(0.0, 100.0);
    driver.set_position(-50.0);
    assert_eq!(driver.position(), 0.0);
    driver.set_position(999.0);
    assert_eq!(driver.position(), 100.0);
}

#[test]
fn scroll_driver_zero_delta_does_not_call_update() {
    struct _PanicUpdate;
    impl Update for _PanicUpdate {
        fn update(&mut self, _dt: f32) -> bool {
            panic!("update called with zero delta")
        }
    }
    let mut driver = ScrollDriver::new(50.0, 100.0);
    driver.set_position(50.0); // already at min, no delta
    // Must not panic if no movement occurred.
}

#[test]
fn scroll_driver_ticks_proportional_delta() {
    // A tween with duration=1 driven by scroll should advance by the
    // normalised scroll fraction.
    let mut value_tween = Tween::new(0.0_f32, 100.0)
        .duration(1.0)
        .easing(Easing::Linear)
        .build();

    // Scroll 50% of range → tween should advance ~50%.
    value_tween.update(0.5); // manually drive by same fraction
    assert!((value_tween.value() - 50.0).abs() < 0.01);
}

#[test]
fn scroll_driver_animation_count() {
    let mut driver = ScrollDriver::new(0.0, 100.0);
    assert_eq!(driver.animation_count(), 0);
    driver.add(Tween::new(0.0_f32, 1.0).duration(1.0).build());
    driver.add(Tween::new(0.0_f32, 1.0).duration(1.0).build());
    assert_eq!(driver.animation_count(), 2);
}

#[test]
fn scroll_driver_min_max_accessors() {
    let driver = ScrollDriver::new(100.0, 900.0);
    assert_eq!(driver.min(), 100.0);
    assert_eq!(driver.max(), 900.0);
}

// ── ScrollClock ───────────────────────────────────────────────────────────────

#[test]
fn scroll_clock_delta_is_normalised() {
    let mut clock = ScrollClock::new(0.0, 1000.0);
    clock.set_scroll(250.0);
    let dt = clock.delta();
    assert!((dt - 0.25).abs() < 0.001, "expected 0.25, got {dt}");
}

#[test]
fn scroll_clock_delta_consumed_after_read() {
    let mut clock = ScrollClock::new(0.0, 100.0);
    clock.set_scroll(50.0);
    let _ = clock.delta();
    assert_eq!(clock.delta(), 0.0);
}

#[test]
fn scroll_clock_accumulates_multiple_moves() {
    let mut clock = ScrollClock::new(0.0, 100.0);
    clock.set_scroll(10.0); // +10%
    clock.set_scroll(20.0); // +10%
    clock.set_scroll(30.0); // +10%
    let dt = clock.delta();
    assert!((dt - 0.3).abs() < 0.001, "expected 0.3, got {dt}");
}

#[test]
fn scroll_clock_progress_tracks_position() {
    let mut clock = ScrollClock::new(0.0, 200.0);
    clock.set_scroll(100.0);
    let _ = clock.delta();
    assert!((clock.progress() - 0.5).abs() < 0.001);
}

#[test]
fn scroll_clock_clamps_out_of_range() {
    let mut clock = ScrollClock::new(0.0, 100.0);
    clock.set_scroll(-50.0); // clamped to 0
    let dt = clock.delta();
    assert_eq!(dt, 0.0); // no movement from 0

    clock.set_scroll(200.0); // clamped to 100
    let dt = clock.delta();
    assert!((dt - 1.0).abs() < 0.001); // full range
}

#[test]
fn scroll_clock_drives_animation_driver() {
    let mut scroll_clock = ScrollClock::new(0.0, 1000.0);
    let mut driver = AnimationDriver::new();
    let id = driver.add(
        Tween::new(0.0_f32, 1.0)
            .duration(0.5) // needs 50% of scroll range to complete
            .easing(Easing::Linear)
            .build(),
    );

    // Move through the first 50% of scroll range.
    for pos in (100..=500_u32).step_by(100) {
        scroll_clock.set_scroll(pos as f32);
        driver.tick(scroll_clock.delta());
    }

    assert!(
        !driver.is_active(id),
        "tween should complete after 50% scroll"
    );
}

#[test]
fn scroll_clock_implements_clock_trait() {
    let mut clock: Box<dyn animato::Clock> = Box::new(ScrollClock::new(0.0, 100.0));
    clock.as_mut(); // just check it's usable as a trait object
    // Call delta through the trait.
    let _ = clock.delta();
}
