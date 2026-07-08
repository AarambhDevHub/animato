// Compile-pass test: a spring with a named preset.
use animato::{spring, Update};

#[allow(dead_code)]
fn spring_preset() {
    let mut s = spring! { scale: 0.8 => 1.0, preset: snappy };
    s.update(1.0 / 60.0);
    let _ = s.position();
}

fn main() {}
