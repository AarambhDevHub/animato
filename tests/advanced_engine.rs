use animato::{
    Angle, AnimationDriver, AnimationGroup, AnimationRecorder, At, GridOrigin, Interpolate, Mat4,
    Quaternion, Spring, SpringConfig, StaggerPattern, Timeline, Tween, Update, Waveform,
};
use std::sync::{Arc, Mutex};

#[test]
fn facade_exports_advanced_value_types() {
    let angle = Angle::from_degrees(359.0).lerp(&Angle::from_degrees(1.0), 0.5);
    assert_eq!(angle.normalized().degrees(), 0.0);

    let rotation = Quaternion::IDENTITY.lerp(
        &Quaternion::from_axis_angle([0.0, 1.0, 0.0], Angle(180.0)),
        0.5,
    );
    assert!((rotation.length() - 1.0).abs() < 0.0001);

    let matrix = Mat4::IDENTITY.lerp(
        &Mat4::from_translation_rotation_scale(
            [2.0, 4.0, 6.0],
            Quaternion::IDENTITY,
            [3.0, 5.0, 7.0],
        ),
        0.5,
    );
    let (translation, _, scale) = matrix.decompose();
    assert_eq!(translation, [1.0, 2.0, 3.0]);
    assert_eq!(scale, [2.0, 3.0, 4.0]);
}

#[test]
fn spring_from_velocity_dissipates_energy() {
    let mut spring = Spring::from_velocity(0.0, 500.0, 100.0, SpringConfig::stiff());
    let initial_energy = spring.energy();
    for _ in 0..600 {
        if !spring.update(1.0 / 60.0) {
            break;
        }
    }
    assert!(spring.is_settled());
    assert!(spring.energy() < initial_energy);
}

#[test]
fn waveform_and_stagger_are_facade_available() {
    let waveform = Waveform::Sine {
        frequency: 1.0,
        amplitude: 2.0,
        phase: 0.0,
    };
    assert!((waveform.sample(0.25) - 2.0).abs() < 0.0001);

    let track = waveform.to_keyframe_track(1.0, 8.0);
    assert_eq!(track.duration(), 1.0);

    let pattern = StaggerPattern::Grid {
        cols: 3,
        rows: 3,
        origin: GridOrigin::Center,
        step: 0.1,
    };
    assert_eq!(pattern.delay(4, 9), 0.0);
    assert_eq!(pattern.delay(0, 9), pattern.delay(8, 9));
}

#[test]
fn animation_group_parallel_sequence_and_nested_timeline() {
    let mut parallel = AnimationGroup::parallel(vec![
        Tween::new(0.0_f32, 1.0).duration(0.5).build(),
        Tween::new(0.0_f32, 1.0).duration(1.0).build(),
    ]);
    parallel.play();
    assert!(parallel.update(0.75));
    assert!(!parallel.update(0.25));

    let sub = Timeline::new().add(
        "x",
        Tween::new(0.0_f32, 10.0).duration(1.0).build(),
        At::Start,
    );
    let mut parent = Timeline::new().add_timeline("sub", sub, At::Start);
    parent.play();
    parent.seek(0.5);
    let child = parent.get::<Timeline>("sub").expect("nested timeline");
    assert_eq!(
        child.get::<Tween<f32>>("x").expect("nested tween").value(),
        5.0
    );
}

#[test]
fn animation_recorder_round_trip_and_driver_sampling() {
    #[derive(Clone)]
    struct SharedValue {
        value: Arc<Mutex<f64>>,
    }

    impl Update for SharedValue {
        fn update(&mut self, dt: f32) -> bool {
            let mut value = self.value.lock().expect("value lock");
            *value += dt as f64;
            *value < 1.0
        }
    }

    let value = Arc::new(Mutex::new(0.0));
    let mut driver = AnimationDriver::new();
    driver.add_recorded(
        "value",
        SharedValue {
            value: Arc::clone(&value),
        },
        {
            let value = Arc::clone(&value);
            move || *value.lock().expect("value lock")
        },
    );

    let mut recorder = AnimationRecorder::new();
    recorder.start();
    driver.tick_recorded(0.25, 0.25, &mut recorder);
    driver.tick_recorded(0.25, 0.50, &mut recorder);

    assert_eq!(recorder.replay("value", 0.25), Some(0.25));
    assert_eq!(recorder.replay("value", 0.50), Some(0.50));

    let json = recorder.export_json();
    let from_json = AnimationRecorder::import_json(&json).expect("json import");
    assert_eq!(from_json.replay("value", 0.375), Some(0.375));

    let binary = recorder.export_binary();
    let from_binary = AnimationRecorder::import_binary(&binary).expect("binary import");
    assert_eq!(from_binary.replay("value", 0.375), Some(0.375));
}
