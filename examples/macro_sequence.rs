//! `macro_sequence` — Demonstrates `animato!{ sequence { ... } }` with labels and offsets.

use animato::Update;
use animato::prelude::*;

fn main() {
    let mut timeline = animato! {
        sequence {
            tween opacity: 0.0 => 1.0, duration: 0.3, easing: ease_out_cubic;
            spring scale: 0.8 => 1.0, preset: snappy;
        }
    };

    println!("Macro-generated sequence timeline:");
    timeline.play();
    let dt = 1.0 / 30.0;
    let mut elapsed = 0.0;
    while timeline.update(dt) {
        elapsed += dt;
        let opacity = timeline
            .get::<Tween<f32>>("node_0")
            .map(|t| t.value())
            .unwrap_or(1.0);
        println!("  t={:.2}s  opacity={:.4}", elapsed, opacity);
    }
    println!("Sequence complete at t={:.2}s", elapsed + dt);
}
