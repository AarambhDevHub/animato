// Compile-pass test: a basic scalar tween.
use animato::{tween, Easing, Update};

#[allow(dead_code)]
fn basic_tween() {
    let mut t = tween! { opacity: 0.0 => 1.0, duration: 0.4, easing: ease_out_cubic };
    t.update(0.1);
    let _ = t.value();
}

fn main() {}
