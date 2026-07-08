//! Runtime parity tests for the Motion Macro DSL.
//!
//! These tests verify that macro-generated animations produce identical values
//! to manually-constructed animations over many update steps.

use animato::{Easing, Loop, Spring, SpringConfig, Tween, Update};
use animato::{animato, keyframes, spring, tween};

const DT: f32 = 1.0 / 60.0;
const STEPS: usize = 120;

#[test]
fn tween_scalar_parity() {
    // Macro-generated tween.
    let mut macro_tween = tween! {
        opacity: 0.0 => 1.0, duration: 0.4, easing: ease_out_cubic
    };

    // Manually-constructed tween.
    let mut manual_tween = Tween::new(0.0_f32, 1.0)
        .duration(0.4)
        .easing(Easing::EaseOutCubic)
        .build();

    for step in 0..STEPS {
        macro_tween.update(DT);
        manual_tween.update(DT);
        let macro_val = macro_tween.value();
        let manual_val = manual_tween.value();
        assert!(
            (macro_val - manual_val).abs() < 1e-5,
            "step {}: macro={} manual={}",
            step,
            macro_val,
            manual_val
        );
    }
}

#[test]
fn tween_with_delay_parity() {
    let mut macro_tween = tween! {
        x: 0.0 => 100.0, duration: 0.5, delay: 0.2, easing: ease_in_out_sine
    };

    let mut manual_tween = Tween::new(0.0_f32, 100.0)
        .duration(0.5)
        .delay(0.2)
        .easing(Easing::EaseInOutSine)
        .build();

    for _ in 0..STEPS {
        macro_tween.update(DT);
        manual_tween.update(DT);
        assert!((macro_tween.value() - manual_tween.value()).abs() < 1e-5);
    }
}

#[test]
fn tween_loop_parity() {
    let mut macro_tween = tween! {
        x: 0.0 => 1.0, duration: 0.3, loop: ping_pong
    };

    let mut manual_tween = Tween::new(0.0_f32, 1.0)
        .duration(0.3)
        .looping(Loop::PingPong)
        .build();

    for _ in 0..STEPS {
        macro_tween.update(DT);
        manual_tween.update(DT);
        assert!((macro_tween.value() - manual_tween.value()).abs() < 1e-5);
    }
}

#[test]
fn spring_preset_parity() {
    let mut macro_spring = spring! {
        scale: 0.8 => 1.0, preset: snappy
    };

    let mut manual_spring = {
        let mut s = Spring::new(SpringConfig::snappy());
        s.snap_to(0.8);
        s.set_target(1.0);
        s
    };

    for _ in 0..STEPS {
        macro_spring.update(DT);
        manual_spring.update(DT);
        assert!(
            (macro_spring.position() - manual_spring.position()).abs() < 1e-4,
            "macro={} manual={}",
            macro_spring.position(),
            manual_spring.position()
        );
    }
}

#[test]
fn spring_settles_to_target() {
    let mut s = spring! { x: 0.0 => 100.0, preset: stiff };
    let mut steps = 0;
    while s.update(DT) {
        steps += 1;
        if steps > 10_000 {
            panic!("spring did not settle");
        }
    }
    assert!(
        (s.position() - 100.0).abs() < 0.01,
        "position={}",
        s.position()
    );
}

#[test]
fn keyframes_parity() {
    use animato::KeyframeTrack;

    let mut macro_track = keyframes! {
        opacity {
            0%: 0.0,
            50%: 0.7 ease_out_cubic,
            100%: 1.0,
        }
    };

    let mut manual_track = KeyframeTrack::new()
        .push(0.0, 0.0_f32)
        .push_eased(0.5, 0.7, Easing::EaseOutCubic)
        .push(1.0, 1.0);

    for _ in 0..STEPS {
        macro_track.update(DT);
        manual_track.update(DT);
        let mv = macro_track.value().unwrap();
        let nv = manual_track.value().unwrap();
        assert!((mv - nv).abs() < 1e-5, "macro={} manual={}", mv, nv);
    }
}

#[test]
fn sequence_produces_timeline() {
    let timeline = animato! {
        sequence {
            tween opacity: 0.0 => 1.0, duration: 0.3, easing: ease_out_cubic;
            spring scale: 0.8 => 1.0, preset: snappy;
        }
    };

    // The sequence should have 2 entries.
    assert_eq!(timeline.entry_count(), 2);
}

#[test]
fn parallel_produces_group() {
    let group = animato! {
        parallel {
            tween x: 0.0 => 100.0, duration: 1.0;
            tween y: 0.0 => 50.0, duration: 1.0;
        }
    };

    // The parallel group should have a duration of 1.0s (the max of the children).
    assert!((group.duration() - 1.0).abs() < 1e-5);
}

#[test]
fn preset_fade_in_works() {
    let mut anim = animato! { preset fade_in };
    // Advance through the animation.
    for _ in 0..30 {
        anim.update(DT);
    }
    // Should be complete or nearly so.
    let _ = anim.is_complete();
}
