//! `macro_stagger_grid` — Demonstrates `stagger pattern: grid(...)` for staggered children.

use animato::Update;
use animato::prelude::*;

fn main() {
    let mut timeline = animato! {
        stagger pattern: grid(cols: 3, rows: 2, origin: center), delay: 0.06 {
            tween x: 0.0 => 1.0, duration: 0.25, easing: ease_out_cubic;
            tween x: 0.0 => 1.0, duration: 0.25, easing: ease_out_cubic;
            tween x: 0.0 => 1.0, duration: 0.25, easing: ease_out_cubic;
            tween x: 0.0 => 1.0, duration: 0.25, easing: ease_out_cubic;
            tween x: 0.0 => 1.0, duration: 0.25, easing: ease_out_cubic;
            tween x: 0.0 => 1.0, duration: 0.25, easing: ease_out_cubic;
        }
    };

    println!("Macro-generated stagger-grid timeline:");
    timeline.play();
    let dt = 0.1;
    let mut elapsed = 0.0;
    while timeline.update(dt) {
        elapsed += dt;
        println!(
            "  t={:.2}s  progress={:.2}%  active_entries={}",
            elapsed,
            timeline.progress() * 100.0,
            timeline.entry_count()
        );
    }
    println!("Stagger grid complete at t={:.2}s", elapsed);
}
