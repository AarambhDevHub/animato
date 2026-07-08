//! `macro_color_theme` — Demonstrates `color ... => ..., space: oklch` for perceptual color tween.

use animato::Update;
use animato::prelude::*;

fn main() {
    // Tween from red to blue in Oklch perceptual space.
    let mut color = animato! {
        color background: "#ff0000" => "#0000ff", duration: 0.8, space: oklch, easing: ease_in_out_sine;
    };

    println!("Macro-generated color tween (red -> blue in oklch):");
    let dt = 0.1;
    for step in 0..=8 {
        if step > 0 {
            color.update(dt);
        }
        println!(
            "  t={:.2}s  channel_value={:.4}  progress={:.2}%",
            step as f32 * dt,
            color.value(),
            color.progress() * 100.0
        );
    }
    println!("Color tween complete.");
}
