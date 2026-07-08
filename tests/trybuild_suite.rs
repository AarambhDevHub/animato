//! trybuild compile-pass and compile-fail tests for the Motion Macro DSL.
//!
//! Run with: `cargo test -p animato --features macro --test trybuild_suite`

fn main() {
    let t = trybuild::TestCases::new();
    t.pass("tests/trybuild/compile_pass/*.rs");
    t.compile_fail("tests/trybuild/compile_fail/*.rs");
}
