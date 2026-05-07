//! Integration test: v0.3.0 control features.

use animato::{At, Easing, Loop, Spring, SpringConfig, Timeline, Tween, Update};

fn tween(end: f32, duration: f32) -> Tween<f32> {
    Tween::new(0.0_f32, end)
        .duration(duration)
        .easing(Easing::Linear)
        .build()
}

#[test]
fn timeline_time_scale_controls_progress() {
    let mut timeline = Timeline::new()
        .add("x", tween(100.0, 1.0), At::Start)
        .time_scale(0.5);

    timeline.play();
    timeline.update(0.5);

    assert_eq!(timeline.elapsed(), 0.25);
    assert_eq!(timeline.get::<Tween<f32>>("x").unwrap().value(), 25.0);
}

#[test]
fn easing_cubic_bezier_and_steps_are_usable_from_facade() {
    let bezier = Easing::CubicBezier(0.0, 0.0, 1.0, 1.0);
    assert!((bezier.apply(0.5) - 0.5).abs() < 1e-5);

    let steps = Easing::Steps(4);
    assert_eq!(steps.apply(0.01), 0.25);
}

#[test]
fn serde_feature_exports_traits_for_core_types() {
    fn assert_serde<T: animato::Serialize + for<'de> animato::Deserialize<'de>>() {}

    assert_serde::<Easing>();
    assert_serde::<Loop>();
    assert_serde::<Tween<f32>>();
    assert_serde::<SpringConfig>();
    assert_serde::<Spring>();
}
