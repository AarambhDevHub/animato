//! Example: shape morphing from a square to a diamond.
//!
//! Run with:
//! ```sh
//! cargo run --example morph_path --features path
//! ```

use animato::{Easing, MorphPath, Tween, Update};
use core::f32::consts::TAU;

fn square(size: f32) -> Vec<[f32; 2]> {
    vec![
        [0.0, 0.0],
        [size, 0.0],
        [size, size],
        [0.0, size],
        [0.0, 0.0], // closed
    ]
}

fn circle_approx(cx: f32, cy: f32, radius: f32, points: usize) -> Vec<[f32; 2]> {
    (0..=points)
        .map(|i| {
            let angle = i as f32 * TAU / points as f32;
            [cx + radius * angle.cos(), cy + radius * angle.sin()]
        })
        .collect()
}

fn main() {
    println!("Animato v0.9.0 - morph_path example");
    println!("  Morphing square → circle over 1.0 s\n");

    let from = square(100.0);
    let to = circle_approx(50.0, 50.0, 50.0, 32);

    let morph = MorphPath::with_resolution(from, to, 32);

    let mut tween = Tween::new(0.0_f32, 1.0)
        .duration(1.0)
        .easing(Easing::EaseInOutCubic)
        .build();

    for frame in 0..=12 {
        let t = tween.value();
        let shape = morph.evaluate(t);
        let bounds = morph.bounds_at(t);
        println!(
            "frame {:02}  t={:.2}  bounds=[{:.1},{:.1},{:.1},{:.1}]  pts={}",
            frame,
            t,
            bounds[0],
            bounds[1],
            bounds[2],
            bounds[3],
            shape.len()
        );
        tween.update(1.0 / 12.0);
    }

    println!("\n  ✓ Morph complete!");
}
