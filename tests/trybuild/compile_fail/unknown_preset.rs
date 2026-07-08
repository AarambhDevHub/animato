// Compile-fail test: an unknown built-in preset name.
use animato::animato;

fn main() {
    let _ = animato! { preset not_a_real_preset };
}
