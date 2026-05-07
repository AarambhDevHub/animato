//! Integration test: timeline, sequence, and stagger composition.

use animato::{At, Easing, Loop, Sequence, Timeline, Tween, Update, stagger};

fn tween(end: f32, duration: f32) -> Tween<f32> {
    Tween::new(0.0_f32, end)
        .duration(duration)
        .easing(Easing::Linear)
        .build()
}

#[test]
fn timeline_plays_concurrent_entries() {
    let mut timeline = Timeline::new().add("x", tween(100.0, 1.0), At::Start).add(
        "opacity",
        tween(1.0, 1.0),
        At::Label("x"),
    );

    timeline.play();
    timeline.update(0.5);

    assert_eq!(timeline.get::<Tween<f32>>("x").unwrap().value(), 50.0);
    assert_eq!(timeline.get::<Tween<f32>>("opacity").unwrap().value(), 0.5);
}

#[test]
fn sequence_plays_steps_after_previous_steps() {
    let mut timeline = Sequence::new()
        .then("first", tween(10.0, 1.0))
        .gap(0.25)
        .then("second", tween(20.0, 1.0))
        .build();

    timeline.play();
    timeline.update(1.125);

    assert_eq!(timeline.get::<Tween<f32>>("first").unwrap().value(), 10.0);
    assert_eq!(timeline.get::<Tween<f32>>("second").unwrap().value(), 0.0);

    timeline.update(0.375);
    assert_eq!(timeline.get::<Tween<f32>>("second").unwrap().value(), 5.0);
}

#[test]
fn seek_abs_synchronizes_all_children() {
    let mut timeline = Timeline::new().add("a", tween(100.0, 2.0), At::Start).add(
        "b",
        tween(100.0, 1.0),
        At::Absolute(1.0),
    );

    timeline.seek_abs(1.5);

    assert_eq!(timeline.get::<Tween<f32>>("a").unwrap().value(), 75.0);
    assert_eq!(timeline.get::<Tween<f32>>("b").unwrap().value(), 50.0);
}

#[test]
fn pause_and_resume_control_timeline() {
    let mut timeline = Timeline::new().add("a", tween(100.0, 1.0), At::Start);
    timeline.play();
    timeline.update(0.25);
    timeline.pause();
    timeline.update(0.5);
    assert_eq!(timeline.get::<Tween<f32>>("a").unwrap().value(), 25.0);

    timeline.resume();
    timeline.update(0.25);
    assert_eq!(timeline.get::<Tween<f32>>("a").unwrap().value(), 50.0);
}

#[test]
fn timeline_times_loop_repeats_then_completes() {
    let mut timeline = Timeline::new()
        .add("a", tween(100.0, 1.0), At::Start)
        .looping(Loop::Times(2));
    timeline.play();

    timeline.update(1.25);
    assert_eq!(timeline.get::<Tween<f32>>("a").unwrap().value(), 25.0);

    assert!(!timeline.update(0.75));
    assert!(timeline.is_complete());
    assert_eq!(timeline.get::<Tween<f32>>("a").unwrap().value(), 100.0);
}

#[test]
fn timeline_ping_pong_reflects_time() {
    let mut timeline = Timeline::new()
        .add("a", tween(100.0, 1.0), At::Start)
        .looping(Loop::PingPong);
    timeline.play();

    timeline.update(1.25);
    assert_eq!(timeline.get::<Tween<f32>>("a").unwrap().value(), 75.0);
    assert!(!timeline.is_complete());
}

#[test]
fn stagger_offsets_start_times() {
    let animations = vec![tween(10.0, 1.0), tween(10.0, 1.0), tween(10.0, 1.0)];
    let mut timeline = stagger(animations, 0.25);

    timeline.play();
    timeline.update(0.5);

    assert_eq!(timeline.get::<Tween<f32>>("item_0").unwrap().value(), 5.0);
    assert_eq!(timeline.get::<Tween<f32>>("item_1").unwrap().value(), 2.5);
    assert_eq!(timeline.get::<Tween<f32>>("item_2").unwrap().value(), 0.0);
}
