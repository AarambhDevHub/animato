//! `macro_motion_path` — Demonstrates `path ... along "M..."` for SVG motion.

use animato::Update;
use animato::prelude::*;

fn main() {
    let mut motion = animato! {
        path position along "M 0 0 C 50 100 150 100 200 0", duration: 1.2, easing: ease_in_out_sine, auto_rotate: true;
    };

    println!("Macro-generated motion-path tween:");
    let dt = 0.15;
    let mut elapsed = 0.0;
    while motion.update(dt) {
        elapsed += dt;
        let [x, y] = motion.value();
        println!("  t={:.2}s  position=({:.2}, {:.2})", elapsed, x, y);
    }
    let [x, y] = motion.value();
    println!("Motion path complete at ({:.2}, {:.2})", x, y);
}
