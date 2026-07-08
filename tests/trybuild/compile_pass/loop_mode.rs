// Compile-pass test: a loop mode.
use animato::{tween, Update};

#[allow(dead_code)]
fn loop_mode() {
    let mut t = tween! {
        x: 0.0 => 1.0,
        duration: 0.3,
        easing: ease_out_cubic,
        loop: ping_pong,
    };
    t.update(0.1);
}

fn main() {}
