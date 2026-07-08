// Compile-fail test: an unknown easing name.
use animato::tween;

fn main() {
    let _ = tween! { x: 0.0 => 1.0, duration: 0.3, easing: ease_out_magic };
}
