// Compile-pass test: a motion path.
use animato::{animato, Update};

#[allow(dead_code)]
fn motion_path() {
    let mut m = animato! {
        path position along "M 0 0 L 100 100", duration: 1.0, easing: ease_in_out_sine;
    };
    m.update(0.1);
}

fn main() {}
