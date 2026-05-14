//! Example: 10,000 particle tween values through `GpuAnimationBatch`.
//!
//! Run with:
//! ```sh
//! cargo run --example gpu_particles --features gpu
//! ```

use animato::{Easing, GpuAnimationBatch, Tween};

fn main() {
    println!("Animato v0.9.0 - gpu_particles example");
    println!("  Evaluating 10,000 particle tween values\n");

    let mut batch = GpuAnimationBatch::new_auto();
    for i in 0..10_000 {
        batch.push(
            Tween::new(0.0_f32, 1.0 + (i % 256) as f32)
                .duration(2.0)
                .easing(Easing::EaseOutCubic)
                .build(),
        );
    }

    for frame in 0..5 {
        batch.tick(1.0 / 60.0);
        let values = batch.read_back();
        let sample = [
            values[0],
            values[values.len() / 2],
            values[values.len() - 1],
        ];
        println!(
            "frame {frame:02} backend={:?} sample=[{:.3}, {:.3}, {:.3}]",
            batch.backend(),
            sample[0],
            sample[1],
            sample[2]
        );
    }

    println!("\n  Complete: {} tweens", batch.len());
}
