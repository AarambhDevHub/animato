//! Integration test: full tween lifecycle using MockClock.

use animato_core::{Easing, Update};
use animato_driver::{Clock, MockClock};
use animato_tween::{Loop, Tween, TweenState};

const DT: f32 = 1.0 / 60.0;

fn tick_n(tween: &mut Tween<f32>, n: usize) {
    let mut clk = MockClock::new(DT);
    for _ in 0..n {
        tween.update(clk.delta());
    }
}

// ── State transitions ─────────────────────────────────────────────────────────

#[test]
fn starts_running_without_delay() {
    let t = Tween::new(0.0_f32, 1.0).duration(1.0).build();
    assert_eq!(t.state(), &TweenState::Running);
}

#[test]
fn starts_idle_with_delay() {
    let t = Tween::new(0.0_f32, 1.0).duration(1.0).delay(0.5).build();
    assert_eq!(t.state(), &TweenState::Idle);
}

#[test]
fn transitions_idle_to_running_after_delay() {
    let mut t = Tween::new(0.0_f32, 1.0).duration(1.0).delay(0.5).build();
    tick_n(&mut t, 31); // 31 × (1/60) ≈ 0.516 > 0.5
    assert_eq!(t.state(), &TweenState::Running);
}

#[test]
fn completes_after_full_duration() {
    let mut t = Tween::new(0.0_f32, 1.0).duration(1.0).build();
    tick_n(&mut t, 61); // 61 frames > 1.0s
    assert!(t.is_complete());
}

// ── Delay correctness ─────────────────────────────────────────────────────────

#[test]
fn value_is_start_during_delay() {
    let mut t = Tween::new(10.0_f32, 90.0).duration(1.0).delay(0.5).build();
    tick_n(&mut t, 20); // 20 × (1/60) ≈ 0.33 < 0.5
    assert_eq!(t.value(), 10.0, "value must equal start during delay");
}

#[test]
fn value_is_end_after_completion() {
    let mut t = Tween::new(0.0_f32, 100.0)
        .duration(1.0)
        .easing(Easing::Linear)
        .build();
    tick_n(&mut t, 120); // well past 1.0s
    assert!((t.value() - 100.0).abs() < 0.001);
}

// ── Loop modes ────────────────────────────────────────────────────────────────

#[test]
fn loop_once_completes() {
    let mut t = Tween::new(0.0_f32, 1.0)
        .duration(1.0)
        .looping(Loop::Once)
        .build();
    tick_n(&mut t, 120);
    assert!(t.is_complete());
}

#[test]
fn loop_forever_never_completes() {
    let mut t = Tween::new(0.0_f32, 1.0)
        .duration(1.0)
        .looping(Loop::Forever)
        .build();
    tick_n(&mut t, 600); // 10 seconds worth
    assert!(!t.is_complete());
    // Value stays in [0, 1]
    assert!(t.value() >= 0.0 && t.value() <= 1.0);
}

#[test]
fn loop_times_3_completes_after_3_cycles() {
    let mut t = Tween::new(0.0_f32, 1.0)
        .duration(1.0)
        .looping(Loop::Times(3))
        .build();
    // 3 full cycles = 180 frames; add a small buffer
    tick_n(&mut t, 185);
    assert!(t.is_complete());
}

#[test]
fn loop_times_3_not_complete_before_3_cycles() {
    let mut t = Tween::new(0.0_f32, 1.0)
        .duration(1.0)
        .looping(Loop::Times(3))
        .build();
    tick_n(&mut t, 120); // only ~2 cycles
    assert!(!t.is_complete());
}

#[test]
fn ping_pong_value_goes_up_then_down() {
    let mut t = Tween::new(0.0_f32, 100.0)
        .duration(1.0)
        .easing(Easing::Linear)
        .looping(Loop::PingPong)
        .build();

    // After 0.5s forward: value should be ~50
    tick_n(&mut t, 30);
    let mid = t.value();
    assert!(mid > 30.0 && mid < 70.0, "forward mid = {}", mid);

    // After 1.0s: should be at or near 100
    tick_n(&mut t, 30);
    let peak = t.value();
    assert!(peak > 90.0, "peak = {}", peak);

    // After 1.5s: should be descending ~50
    tick_n(&mut t, 30);
    let descend = t.value();
    assert!(descend > 30.0 && descend < 70.0, "descend = {}", descend);
}

// ── Seek + Reverse ────────────────────────────────────────────────────────────

#[test]
fn seek_to_midpoint_gives_half_value() {
    let mut t = Tween::new(0.0_f32, 100.0)
        .duration(1.0)
        .easing(Easing::Linear)
        .build();
    t.seek(0.5);
    assert!((t.value() - 50.0).abs() < 0.01);
}

#[test]
fn seek_to_end_marks_running_not_complete() {
    let mut t = Tween::new(0.0_f32, 100.0).duration(1.0).build();
    t.seek(1.0);
    // seek doesn't flip to Completed — only update() does
    assert!(!t.is_complete());
}

#[test]
fn reverse_plays_backward() {
    let mut t = Tween::new(0.0_f32, 100.0)
        .duration(1.0)
        .easing(Easing::Linear)
        .build();
    // Advance 30% forward: value ≈ 30
    t.update(0.3);
    assert!(
        (t.value() - 30.0).abs() < 1.0,
        "before reverse: value={}",
        t.value()
    );
    // Reverse mid-animation: elapsed mirrors to 70% of backward journey
    // Visual position is preserved — still ≈ 30
    t.reverse();
    assert!(
        (t.value() - 30.0).abs() < 1.0,
        "after reverse: visual position should be preserved, got {}",
        t.value()
    );
    // Continue updating — value should now DECREASE toward 0 (new end)
    t.update(0.2);
    assert!(
        t.value() < 30.0,
        "after update post-reverse: value should decrease, got {}",
        t.value()
    );
}

// ── Pause / Resume ────────────────────────────────────────────────────────────

#[test]
fn paused_tween_does_not_advance() {
    let mut t = Tween::new(0.0_f32, 100.0).duration(1.0).build();
    tick_n(&mut t, 30);
    let v_before = t.value();
    t.pause();
    tick_n(&mut t, 30);
    assert_eq!(t.value(), v_before, "paused tween must not advance");
}

#[test]
fn resumed_tween_continues() {
    let mut t = Tween::new(0.0_f32, 100.0)
        .duration(1.0)
        .easing(Easing::Linear)
        .build();
    tick_n(&mut t, 30);
    let v_paused = t.value();
    t.pause();
    tick_n(&mut t, 30);
    t.resume();
    tick_n(&mut t, 30);
    assert!(
        t.value() > v_paused,
        "resumed tween must advance past pause point"
    );
}

// ── Large dt safety ───────────────────────────────────────────────────────────

#[test]
fn huge_dt_completes_without_panic() {
    let mut t = Tween::new(0.0_f32, 1.0).duration(1.0).build();
    t.update(1_000_000.0);
    assert!(t.is_complete());
    assert_eq!(t.value(), 1.0);
}

#[test]
fn negative_dt_never_moves_value() {
    let mut t = Tween::new(0.0_f32, 1.0).duration(1.0).build();
    let v0 = t.value();
    t.update(-100.0);
    assert_eq!(t.value(), v0);
}
