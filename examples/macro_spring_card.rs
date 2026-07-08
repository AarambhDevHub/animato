//! `macro_spring_card` — Demonstrates `spring!{}` with a preset and velocity.

use animato::Update;
use animato::prelude::*;

fn main() {
    // A card that springs into place with initial velocity.
    let mut spring = spring! {
        scale: 0.8 => 1.0,
        preset: snappy,
        velocity: 200.0,
    };

    println!("Macro-generated spring with velocity:");
    let dt = 1.0 / 60.0;
    let mut steps = 0;
    while spring.update(dt) {
        steps += 1;
        if steps % 10 == 0 {
            println!(
                "  step={}  position={:.4}  velocity={:.4}  settled={}",
                steps,
                spring.position(),
                spring.velocity(),
                spring.is_settled()
            );
        }
    }
    println!(
        "Spring settled at position={:.4} after {} steps",
        spring.position(),
        steps
    );
}
