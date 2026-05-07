//! Integration test: spring settles for all presets.

use animato_core::Update;
use animato_spring::{Spring, SpringConfig, SpringN};

const DT: f32 = 1.0 / 60.0;
const MAX_STEPS: usize = 10_000;
const EPSILON: f32 = 0.01;

fn settle_steps(spring: &mut Spring) -> usize {
    for i in 0..MAX_STEPS {
        if !spring.update(DT) {
            return i;
        }
    }
    panic!("Spring did not settle within {} steps", MAX_STEPS);
}

// ── All presets settle ────────────────────────────────────────────────────────

#[test]
fn gentle_settles() {
    let mut s = Spring::new(SpringConfig::gentle());
    s.set_target(100.0);
    settle_steps(&mut s);
    assert!((s.position() - 100.0).abs() < EPSILON);
}

#[test]
fn wobbly_settles() {
    let mut s = Spring::new(SpringConfig::wobbly());
    s.set_target(100.0);
    settle_steps(&mut s);
    assert!((s.position() - 100.0).abs() < EPSILON);
}

#[test]
fn stiff_settles() {
    let mut s = Spring::new(SpringConfig::stiff());
    s.set_target(100.0);
    settle_steps(&mut s);
    assert!((s.position() - 100.0).abs() < EPSILON);
}

#[test]
fn slow_settles() {
    let mut s = Spring::new(SpringConfig::slow());
    s.set_target(100.0);
    settle_steps(&mut s);
    assert!((s.position() - 100.0).abs() < EPSILON);
}

#[test]
fn snappy_settles() {
    let mut s = Spring::new(SpringConfig::snappy());
    s.set_target(100.0);
    settle_steps(&mut s);
    assert!((s.position() - 100.0).abs() < EPSILON);
}

// ── Ordering: snappy < stiff < gentle < wobbly < slow ─────────────────────────

#[test]
fn snappy_settles_faster_than_slow() {
    let mut fast = Spring::new(SpringConfig::snappy());
    fast.set_target(100.0);
    let fast_n = settle_steps(&mut fast);

    let mut slow = Spring::new(SpringConfig::slow());
    slow.set_target(100.0);
    let slow_n = settle_steps(&mut slow);

    assert!(
        fast_n < slow_n,
        "snappy={} frames, slow={} frames",
        fast_n,
        slow_n
    );
}

#[test]
fn stiff_settles_faster_than_gentle() {
    let mut a = Spring::new(SpringConfig::stiff());
    a.set_target(100.0);
    let a_n = settle_steps(&mut a);

    let mut b = Spring::new(SpringConfig::gentle());
    b.set_target(100.0);
    let b_n = settle_steps(&mut b);

    assert!(a_n < b_n, "stiff={} frames, gentle={} frames", a_n, b_n);
}

// ── Negative target ───────────────────────────────────────────────────────────

#[test]
fn settles_to_negative_target() {
    let mut s = Spring::new(SpringConfig::stiff());
    s.set_target(-250.0);
    settle_steps(&mut s);
    assert!((s.position() - (-250.0)).abs() < EPSILON);
}

// ── RK4 integration also settles ─────────────────────────────────────────────

#[test]
fn rk4_settles() {
    let mut s = Spring::new(SpringConfig::wobbly()).use_rk4(true);
    s.set_target(100.0);
    settle_steps(&mut s);
    assert!((s.position() - 100.0).abs() < EPSILON);
}

// ── SpringN<[f32; 3]> ────────────────────────────────────────────────────────

#[test]
fn spring_n_vec3_settles() {
    let mut s: SpringN<[f32; 3]> = SpringN::new(SpringConfig::stiff(), [0.0; 3]);
    s.set_target([10.0, 20.0, 30.0]);
    for _ in 0..MAX_STEPS {
        if !s.update(DT) {
            break;
        }
    }
    assert!(s.is_settled(), "SpringN<[f32;3]> did not settle");
    let pos = s.position();
    assert!((pos[0] - 10.0).abs() < EPSILON);
    assert!((pos[1] - 20.0).abs() < EPSILON);
    assert!((pos[2] - 30.0).abs() < EPSILON);
}

#[test]
fn spring_n_independent_axes() {
    let mut s: SpringN<[f32; 3]> = SpringN::new(SpringConfig::snappy(), [0.0; 3]);
    s.set_target([1.0, -1.0, 0.5]);
    for _ in 0..MAX_STEPS {
        if !s.update(DT) {
            break;
        }
    }
    let pos = s.position();
    assert!((pos[0] - 1.0).abs() < EPSILON);
    assert!((pos[1] - (-1.0)).abs() < EPSILON);
    assert!((pos[2] - 0.5).abs() < EPSILON);
}

// ── At target from start ──────────────────────────────────────────────────────

#[test]
fn spring_at_target_is_settled() {
    let mut s = Spring::new(SpringConfig::default());
    // target = 0, position = 0 → already settled
    assert!(s.is_settled());
    assert!(!s.update(DT)); // returns false immediately
}
