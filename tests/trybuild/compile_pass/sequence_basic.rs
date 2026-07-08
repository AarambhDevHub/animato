// Compile-pass test: a sequence block.
use animato::{animato, Update};

#[allow(dead_code)]
fn sequence_block() {
    let mut tl = animato! {
        sequence {
            tween opacity: 0.0 => 1.0, duration: 0.3, easing: ease_out_cubic;
            spring scale: 0.8 => 1.0, preset: snappy;
        }
    };
    tl.update(0.1);
}

fn main() {}
