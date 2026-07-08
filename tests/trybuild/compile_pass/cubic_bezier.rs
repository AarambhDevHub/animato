// Compile-pass test: a cubic-bezier easing.
use animato::{tween, Update};

#[allow(dead_code)]
fn cubic_bezier() {
    let mut t = tween! {
        x: 0.0 => 1.0,
        duration: 0.3,
        easing: cubic_bezier(0.22, 1.0, 0.36, 1.0),
    };
    t.update(0.1);
}

fn main() {}
