// Compile-pass test: a preset call.
use animato::{animato, Update};

#[allow(dead_code)]
fn preset_call() {
    let mut a = animato! { preset fade_in };
    a.update(0.1);
}

fn main() {}
