//! `macro_parallel` — Demonstrates `parallel { ... }` producing an `AnimationGroup`.

use animato::Update;
use animato::prelude::*;

fn main() {
    let mut group = animato! {
        parallel {
            tween x: 0.0 => 100.0, duration: 1.0, easing: ease_out_cubic;
            tween y: 0.0 => 50.0, duration: 1.0, easing: ease_in_out_sine;
        }
    };

    println!("Macro-generated parallel AnimationGroup:");
    let dt = 1.0 / 10.0;
    let mut elapsed = 0.0;
    while group.update(dt) {
        elapsed += dt;
        println!(
            "  t={:.2}s  progress={:.2}%  complete={}",
            elapsed,
            group.progress() * 100.0,
            group.is_complete()
        );
    }
    println!("Parallel group complete at t={:.2}s", elapsed);
}
