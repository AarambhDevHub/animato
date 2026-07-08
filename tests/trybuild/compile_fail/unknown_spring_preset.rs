// Compile-fail test: an unknown spring preset.
use animato::spring;

fn main() {
    let _ = spring! { x: 0.0 => 1.0, preset: bouncy };
}
