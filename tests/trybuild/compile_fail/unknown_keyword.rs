// Compile-fail test: an unknown DSL keyword.
use animato::animato;

fn main() {
    let _ = animato! { banana opacity: 0.0 => 1.0, duration: 0.3 };
}
