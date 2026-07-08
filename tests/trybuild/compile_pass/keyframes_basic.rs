// Compile-pass test: keyframes with per-frame easing.
use animato::{keyframes, Update};

#[allow(dead_code)]
fn keyframes_basic() {
    let mut track = keyframes! {
        opacity {
            0%: 0.0,
            50%: 0.7 ease_out_cubic,
            100%: 1.0,
        }
    };
    track.update(0.1);
    let _ = track.value();
}

fn main() {}
