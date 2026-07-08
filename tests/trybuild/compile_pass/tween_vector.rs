// Compile-pass test: a vector tween with array values.
use animato::{tween, Update};

#[allow(dead_code)]
fn vector_tween() {
    let mut t = tween! { position: [0.0, 20.0] => [10.0, 30.0], duration: 0.6, easing: ease_in_out_sine };
    t.update(0.1);
    let v = t.value();
    let _ = (v[0], v[1]);
}

fn main() {}
