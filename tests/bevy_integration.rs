//! Integration tests for v0.7.0 Bevy APIs.

use animato::{
    AnimationChannel, AnimationLabel, AnimatoPlugin, AnimatoSpring, AnimatoTween, Easing,
    SpringConfig, SpringN, SpringSettled, Tween, TweenCompleted,
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

#[test]
fn component_accessors_reset_reporting_and_scale_rotation_channels() {
    let mut tween = AnimatoTween::scale(
        Tween::new([1.0_f32, 1.0, 1.0], [2.0, 3.0, 4.0])
            .duration(1.0)
            .build(),
    );
    assert_eq!(tween.channel(), AnimationChannel::Scale);
    assert_eq!(tween.value(), [1.0, 1.0, 1.0]);
    tween.tween_mut().seek(0.5);
    assert_eq!(tween.tween().value(), [1.5, 2.0, 2.5]);
    tween.pause();
    tween.resume();
    tween.reset();
    assert_eq!(tween.value(), [1.0, 1.0, 1.0]);

    let mut spring = AnimatoSpring::rotation_z(SpringN::new(SpringConfig::gentle(), 0.0_f32));
    assert_eq!(spring.channel(), AnimationChannel::RotationZ);
    spring.set_target(1.0);
    spring.spring_mut().snap_to(0.5);
    assert_eq!(spring.spring().position(), 0.5);
    assert_eq!(spring.position(), 0.5);
}

#[test]
fn transform_scale_and_rotation_channels_are_applied() {
    let mut app = App::new();
    app.insert_resource(bevy_time::Time::<()>::default());
    app.add_plugins(AnimatoPlugin);

    let scale_entity = app
        .world_mut()
        .spawn((
            Transform::default(),
            AnimatoTween::scale(
                Tween::new([1.0_f32, 1.0, 1.0], [3.0, 5.0, 7.0])
                    .duration(1.0)
                    .easing(Easing::Linear)
                    .build(),
            ),
        ))
        .id();
    let rotation_entity = app
        .world_mut()
        .spawn((
            Transform::default(),
            AnimatoTween::rotation_z(
                Tween::new(0.0_f32, core::f32::consts::FRAC_PI_2)
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

    let scale = app.world().get::<Transform>(scale_entity).unwrap().scale;
    let rotation = app
        .world()
        .get::<Transform>(rotation_entity)
        .unwrap()
        .rotation;

    assert!((scale.x - 2.0).abs() < 0.001);
    assert!((scale.y - 3.0).abs() < 0.001);
    assert!((scale.z - 4.0).abs() < 0.001);
    assert!((rotation.to_scaled_axis().z - core::f32::consts::FRAC_PI_4).abs() < 0.001);
}

#[test]
fn spring_scale_rotation_and_settled_message_are_applied() {
    let mut app = App::new();
    app.insert_resource(bevy_time::Time::<()>::default());
    app.add_plugins(AnimatoPlugin);

    let config = SpringConfig {
        epsilon: 0.01,
        ..SpringConfig::snappy()
    };
    let mut scale_spring = SpringN::new(config.clone(), [1.0_f32, 1.0, 1.0]);
    scale_spring.set_target([2.0, 3.0, 4.0]);
    let scale_entity = app
        .world_mut()
        .spawn((Transform::default(), AnimatoSpring::scale(scale_spring)))
        .id();

    let mut rotation_spring = SpringN::new(config, 0.0_f32);
    rotation_spring.set_target(core::f32::consts::FRAC_PI_2);
    let rotation_entity = app
        .world_mut()
        .spawn((
            AnimationLabel::new("spin"),
            Transform::default(),
            AnimatoSpring::rotation_z(rotation_spring),
        ))
        .id();

    let mut saw_settled_message = false;
    for _ in 0..600 {
        app.world_mut()
            .resource_mut::<bevy_time::Time>()
            .advance_by(Duration::from_secs_f32(1.0 / 60.0));
        app.update();
        if app.world().resource::<Messages<SpringSettled>>().len() >= 1 {
            saw_settled_message = true;
            break;
        }
    }

    let scale = app.world().get::<Transform>(scale_entity).unwrap().scale;
    let rotation = app
        .world()
        .get::<Transform>(rotation_entity)
        .unwrap()
        .rotation;

    assert!((scale.x - 2.0).abs() < 0.05);
    assert!((scale.y - 3.0).abs() < 0.05);
    assert!((rotation.to_scaled_axis().z - core::f32::consts::FRAC_PI_2).abs() < 0.05);
    assert!(saw_settled_message);
}
