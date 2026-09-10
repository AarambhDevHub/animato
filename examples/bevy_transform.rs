//! Example: drive a Bevy `Transform` with Animato.
//!
//! Run with:
//! ```sh
//! cargo run --example bevy_transform --features bevy
//! ```
//! or
//! ```sh
//! cargo run --example bevy_transform --features bevy --no-default-features
//! ```
#![cfg_attr(not(feature = "std"), no_std)]

use animato::{AnimatoPlugin, AnimatoTween, Easing, Tween};
use bevy_app::App;
use bevy_transform::components::Transform;
use core::time::Duration;

fn main() {
    #[cfg(feature = "std")]
    println!("Animato v1.0.0 - bevy_transform example\n");

    let mut app = App::new();
    app.insert_resource(bevy_time::Time::<()>::default());
    app.add_plugins(AnimatoPlugin);

    let entity = app
        .world_mut()
        .spawn((
            Transform::default(),
            AnimatoTween::translation(
                Tween::new([0.0_f32, 0.0, 0.0], [240.0, 80.0, 0.0])
                    .duration(1.0)
                    .easing(Easing::EaseOutCubic)
                    .build(),
            ),
        ))
        .id();

    for _frame in 0..=10 {
        app.world_mut()
            .resource_mut::<bevy_time::Time>()
            .advance_by(Duration::from_secs_f32(0.1));
        app.update();

        let _transform = app.world().get::<Transform>(entity).unwrap();
        #[cfg(feature = "std")]
        println!(
            "frame {_frame:02}: translation=({:7.2}, {:7.2}, {:7.2})",
            _transform.translation.x, _transform.translation.y, _transform.translation.z
        );
    }
}
