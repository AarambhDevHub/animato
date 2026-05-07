//! Example: Compose tweens with Timeline and Sequence.
//!
//! Run with:
//! ```sh
//! cargo run --example timeline_sequence
//! ```

use animato::{At, Easing, Sequence, Timeline, Tween, Update};

fn tween(end: f32, duration: f32, easing: Easing) -> Tween<f32> {
    Tween::new(0.0_f32, end)
        .duration(duration)
        .easing(easing)
        .build()
}

fn main() {
    println!("Animato v0.2.0 - timeline_sequence example\n");

    let fade = tween(1.0, 0.8, Easing::EaseOutCubic);
    let slide = tween(120.0, 1.0, Easing::EaseInOutSine);
    let scale = tween(1.0, 0.4, Easing::EaseOutBack);

    let mut timeline = Timeline::new()
        .add("fade", fade, At::Start)
        .add("slide", slide, At::Label("fade"))
        .add("scale", scale, At::Offset(0.1));

    timeline.play();
    while !timeline.is_complete() {
        timeline.update(1.0 / 30.0);
        let opacity = timeline.get::<Tween<f32>>("fade").unwrap().value();
        let x = timeline.get::<Tween<f32>>("slide").unwrap().value();
        let scale = timeline.get::<Tween<f32>>("scale").unwrap().value();
        println!("  timeline opacity={opacity:.2} x={x:6.2} scale={scale:.2}");
    }

    let mut sequence = Sequence::new()
        .then("intro", tween(100.0, 0.5, Easing::EaseOutCubic))
        .gap(0.2)
        .then("outro", tween(1.0, 0.5, Easing::EaseInOutSine))
        .build();

    sequence.play();
    while !sequence.is_complete() {
        sequence.update(1.0 / 30.0);
        let intro = sequence.get::<Tween<f32>>("intro").unwrap().value();
        let outro = sequence.get::<Tween<f32>>("outro").unwrap().value();
        println!("  sequence intro={intro:6.2} outro={outro:.2}");
    }
}
