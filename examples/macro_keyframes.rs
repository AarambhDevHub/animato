//! `macro_keyframes` — Demonstrates `keyframes!{}` with per-frame easing.

use animato::Update;
use animato::prelude::*;

fn main() {
    let mut track = keyframes! {
        scale {
            0%: 0.0,
            50%: 1.1 ease_out_cubic,
            100%: 1.0 ease_in_out_sine,
        }
    };

    println!("Macro-generated keyframe track:");
    let dt = 0.1;
    for step in 0..=10 {
        if step > 0 {
            track.update(dt);
        }
        let value = track.value().unwrap_or(0.0);
        println!("  t={:.2}s  scale={:.4}", step as f32 * dt, value);
    }
    println!("Keyframe track complete.");
}
