// Compile-fail test: mismatched tween from/to value types.
use animato::tween;

fn main() {
    let _ = tween! { x: 0.0 => [1.0, 2.0], duration: 0.3 };
}
