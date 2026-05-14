//! Example: perceptual color animation in Lab space.
//!
//! Run with:
//! ```sh
//! cargo run --example color_animation --features color
//! ```

use animato::{Easing, InLab, Tween, Update, palette::Srgb};

fn main() {
    println!("Animato v1.0.0 - color_animation example\n");

    let start = InLab::new(Srgb::new(1.0, 0.1, 0.0));
    let end = InLab::new(Srgb::new(0.0, 0.2, 1.0));

    let mut tween = Tween::new(start, end)
        .duration(1.0)
        .easing(Easing::EaseInOutSine)
        .build();

    for frame in 0..=12 {
        let color = tween.value().into_inner();
        println!(
            "frame {frame:02}: rgb({:6.3}, {:6.3}, {:6.3})",
            color.red, color.green, color.blue
        );

        tween.update(1.0 / 12.0);
    }
}
