//! `macro_loading_wave` — Demonstrates `waveform sine ...` for a loading animation.

use animato::Update;
use animato::prelude::*;

fn main() {
    // A sine waveform driving a loading-pulse animation.
    let mut track = animato! {
        waveform sine frequency: 1.5, amplitude: 0.3, duration: 2.0;
    };

    println!("Macro-generated loading-wave keyframe track:");
    let dt = 0.1;
    for step in 0..=20 {
        if step > 0 {
            track.update(dt);
        }
        let value = track.value().unwrap_or(0.0);
        let bar_len = ((value + 0.3) / 0.6 * 40.0).clamp(0.0, 40.0) as usize;
        let bar: String = "=".repeat(bar_len);
        println!(
            "  t={:.2}s  value={:+.4}  [{}]",
            step as f32 * dt,
            value,
            bar
        );
    }
    println!("Loading wave complete.");
}
