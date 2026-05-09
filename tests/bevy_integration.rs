//! Integration tests for v0.7.0 Bevy APIs.

use animato::{
    AnimationChannel, AnimationLabel, AnimatoPlugin, AnimatoSpring, AnimatoTween, Easing,
    SpringConfig, SpringN, Tween, TweenCompleted,
};
use bevy_app::App;
use bevy_ecs::message::Messages;
use bevy_transform::components::Transform;
use std::time::Duration;

#[test]
fn tween_component_ticks_and_emits_completion_message() {
    let mut app = App::new();
    app.insert_resource(bevy_time::Time::<()>::default());
    app.add_plugins(AnimatoPlugin);

    let entity = app
        .world_mut()
        .spawn((
            AnimationLabel::new("fade"),
            AnimatoTween::new(
                Tween::new(0.0_f32, 1.0)
                    .duration(0.5)
                    .easing(Easing::Linear)
                    .build(),
            ),
        ))
        .id();

    app.world_mut()
        .resource_mut::<bevy_time::Time>()
        .advance_by(Duration::from_secs_f32(0.5));
    app.update();

    let messages = app.world().resource::<Messages<TweenCompleted>>();
    assert_eq!(messages.len(), 1);
    let mut cursor = messages.get_cursor();
    let completed = cursor.read(messages).next().unwrap();
    assert_eq!(completed.entity, entity);
    assert_eq!(completed.channel, AnimationChannel::Value);
    assert_eq!(completed.label.as_ref().unwrap().as_str(), "fade");
}

#[test]
fn translation_tween_applies_to_transform() {
    let mut app = App::new();
    app.insert_resource(bevy_time::Time::<()>::default());
    app.add_plugins(AnimatoPlugin);

    let entity = app
        .world_mut()
        .spawn((
            Transform::default(),
            AnimatoTween::translation(
                Tween::new([0.0_f32, 0.0, 0.0], [10.0, 20.0, 30.0])
                    .duration(1.0)
                    .easing(Easing::Linear)
                    .build(),
            ),
        ))
        .id();

    app.world_mut()
        .resource_mut::<bevy_time::Time>()
        .advance_by(Duration::from_secs_f32(0.5));
    app.update();

    let transform = app.world().get::<Transform>(entity).unwrap();
    assert!((transform.translation.x - 5.0).abs() < 0.001);
    assert!((transform.translation.y - 10.0).abs() < 0.001);
    assert!((transform.translation.z - 15.0).abs() < 0.001);
}

#[test]
fn spring_component_settles() {
    let mut app = App::new();
    app.insert_resource(bevy_time::Time::<()>::default());
    app.add_plugins(AnimatoPlugin);

    let mut spring = SpringN::new(SpringConfig::stiff(), [0.0_f32, 0.0, 0.0]);
    spring.set_target([1.0, 1.0, 1.0]);
    let entity = app
        .world_mut()
        .spawn((Transform::default(), AnimatoSpring::translation(spring)))
        .id();

    for _ in 0..240 {
        app.world_mut()
            .resource_mut::<bevy_time::Time>()
            .advance_by(Duration::from_secs_f32(1.0 / 60.0));
        app.update();
    }

    let transform = app.world().get::<Transform>(entity).unwrap();
    assert!((transform.translation.x - 1.0).abs() < 0.05);
}
