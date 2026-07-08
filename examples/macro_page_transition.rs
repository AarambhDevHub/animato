//! `macro_page_transition` — Demonstrates `preset page_enter` / `page_exit` for route transitions.

use animato::Update;
use animato::prelude::*;

fn main() {
    println!("Page enter (fade + slide up):");
    let mut enter = animato! { preset page_enter };
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
    println!("Page enter complete at t={:.2}s\n", elapsed);

    println!("Page exit (fade + slide up):");
    let mut exit = animato! { preset page_exit };
    let mut elapsed = 0.0;
    while exit.update(dt) {
        elapsed += dt;
        println!(
            "  t={:.2}s  progress={:.2}%",
            elapsed,
            exit.progress() * 100.0
        );
    }
    println!("Page exit complete at t={:.2}s", elapsed);
}
