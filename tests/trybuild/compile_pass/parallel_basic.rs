// Compile-pass test: a parallel block.
use animato::{animato, Update};

#[allow(dead_code)]
fn parallel_block() {
    let mut g = animato! {
        parallel {
            tween x: 0.0 => 100.0, duration: 1.0;
            tween y: 0.0 => 50.0, duration: 1.0;
        }
    };
    g.update(0.1);
}

fn main() {}
