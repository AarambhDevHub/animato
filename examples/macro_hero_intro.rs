//! `macro_hero_intro` — The full hero example from the v1.7.0 roadmap.

use animato::Update;
use animato::prelude::*;

fn main() {
    let mut timeline = animato! {
        sequence {
            tween opacity: 0.0 => 1.0, duration: 0.35, easing: ease_out_cubic;

            parallel {
                tween y: 24.0 => 0.0, duration: 0.55, easing: ease_out_back;
                spring scale: 0.92 => 1.0, preset: snappy;
            }

            stagger pattern: grid(cols: 3, rows: 2, origin: center), delay: 0.06 {
                tween opacity: 0.0 => 1.0, duration: 0.25;
                tween opacity: 0.0 => 1.0, duration: 0.25;
                tween opacity: 0.0 => 1.0, duration: 0.25;
                tween opacity: 0.0 => 1.0, duration: 0.25;
                tween opacity: 0.0 => 1.0, duration: 0.25;
                tween opacity: 0.0 => 1.0, duration: 0.25;
            }
        }
    };

    println!("Hero intro timeline (v1.7.0 roadmap example):");
    timeline.play();
    let dt = 0.1;
    let mut elapsed = 0.0;
    while timeline.update(dt) {
        elapsed += dt;
        println!(
            "  t={:.2}s  progress={:.2}%  entries={}",
            elapsed,
            timeline.progress() * 100.0,
            timeline.entry_count()
        );
    }
    println!("Hero intro complete at t={:.2}s", elapsed);
}
