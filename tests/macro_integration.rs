//! Integration tests for the Motion Macro DSL.
//!
//! These tests verify that the macro compiles and produces working animations.

use animato::{Update, animato, keyframes, spring, tween};

#[test]
fn top_level_tween_compiles() {
    let mut t = tween! { x: 0.0 => 1.0, duration: 0.3, easing: ease_out_cubic };
    t.update(0.1);
    let _ = t.value();
}

#[test]
fn top_level_spring_compiles() {
    let mut s = spring! { x: 0.0 => 1.0, preset: snappy };
    s.update(1.0 / 60.0);
    let _ = s.position();
}

#[test]
fn top_level_keyframes_compiles() {
    let mut track = keyframes! {
        opacity {
            0%: 0.0,
            100%: 1.0,
        }
    };
    track.update(0.1);
    let _ = track.value();
}

#[test]
fn sequence_compiles() {
    let mut tl = animato! {
        sequence {
            tween x: 0.0 => 1.0, duration: 0.3;
        }
    };
    tl.update(0.1);
}

#[test]
fn parallel_compiles() {
    let mut g = animato! {
        parallel {
            tween x: 0.0 => 1.0, duration: 0.3;
        }
    };
    g.update(0.1);
}

#[test]
fn stagger_simple_compiles() {
    let mut tl = animato! {
        stagger delay: 0.05 {
            tween x: 0.0 => 1.0, duration: 0.25;
        }
    };
    tl.update(0.1);
}
