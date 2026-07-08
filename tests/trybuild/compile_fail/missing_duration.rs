// Compile-fail test: a tween without a `duration` field.
use animato::tween;

fn main() {
    let _ = tween! { opacity: 0.0 => 1.0, easing: ease_out_cubic };
}
