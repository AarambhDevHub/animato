// Compile-pass test: a spring with explicit config.
use animato::{spring, Update};

#[allow(dead_code)]
fn spring_explicit() {
    let mut s = spring! {
        x: 0.0 => 100.0,
        stiffness: 350.0,
        damping: 28.0,
        mass: 1.0,
    };
    s.update(1.0 / 60.0);
    let _ = s.position();
}

fn main() {}
