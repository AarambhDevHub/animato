//! `macro_basic_tween` — Demonstrates the `tween!{}` macro for a simple scalar tween.
//!
//! Run with: `cargo run --example macro_basic_tween --features macro`

use animato::Update;
use animato::prelude::*;

fn main() {
    // A simple opacity fade-in using the macro DSL.
    let mut tween = tween! { opacity: 0.0 => 1.0, duration: 0.4, easing: ease_out_cubic };

    println!("Macro-generated basic tween:");
    let dt = 1.0 / 60.0;
    for step in 0..=24 {
        if step > 0 {
            tween.update(dt);
        }
        println!("  t={:.2}s  opacity={:.4}", step as f32 * dt, tween.value());
    }

    assert!(tween.is_complete());
    println!("Tween complete at opacity={:.4}", tween.value());
}
