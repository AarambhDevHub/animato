// Compile-pass test: a stagger grid.
use animato::{animato, Update};

#[allow(dead_code)]
fn stagger_grid() {
    let mut tl = animato! {
        stagger pattern: grid(cols: 3, rows: 2, origin: center), delay: 0.06 {
            tween x: 0.0 => 1.0, duration: 0.25;
        }
    };
    tl.update(0.1);
}

fn main() {}
