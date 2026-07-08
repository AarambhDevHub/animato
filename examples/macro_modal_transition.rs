//! `macro_modal_transition` — Demonstrates `preset modal_enter` / `modal_exit`.

use animato::Update;
use animato::prelude::*;

fn main() {
    println!("Modal enter preset:");
    let mut enter = animato! { preset modal_enter };
    let dt = 1.0 / 20.0;
    let mut elapsed = 0.0;
    while enter.update(dt) {
        elapsed += dt;
        println!(
            "  t={:.2}s  progress={:.2}%",
            elapsed,
            enter.progress() * 100.0
        );
    }
    println!("Modal enter complete at t={:.2}s\n", elapsed);

    println!("Modal exit preset:");
    let mut exit = animato! { preset modal_exit };
    let mut elapsed = 0.0;
    while exit.update(dt) {
        elapsed += dt;
        println!(
            "  t={:.2}s  progress={:.2}%",
            elapsed,
            exit.progress() * 100.0
        );
    }
    println!("Modal exit complete at t={:.2}s", elapsed);
}
