# Testing

This is the v1.3.0 release verification set.

## Required Local Gates

```sh
cargo fmt --check
cargo clippy --workspace --all-features -- -D warnings
cargo test --workspace --all-features
cargo test --workspace --no-default-features
cargo test -p animato --all-features --examples
cargo doc --workspace --all-features --no-deps
cargo check -p animato-wasm --target wasm32-unknown-unknown --features wasm-dom
cargo check -p animato-leptos --target wasm32-unknown-unknown --features csr
cargo check -p animato-dioxus
cargo check -p animato-dioxus --target wasm32-unknown-unknown --features web
cargo check -p animato-yew --target wasm32-unknown-unknown --features csr
cargo bench --workspace --no-run
```

## Coverage

Install:

```sh
cargo install cargo-llvm-cov --locked
```

Run:

```sh
cargo llvm-cov --workspace --all-features --fail-under-lines 90
```

## Fuzzing

Install:

```sh
cargo install cargo-fuzz --locked
```

Run the SVG parser target:

```sh
cargo +nightly fuzz run svg_path_parser -- -max_total_time=60
```

## WASM

```sh
cargo check -p animato-wasm --target wasm32-unknown-unknown --features wasm-dom
cargo check -p animato-leptos --target wasm32-unknown-unknown --features csr
cargo check -p animato-dioxus --target wasm32-unknown-unknown --features web
cargo check -p animato-yew --target wasm32-unknown-unknown --features csr
cargo check --manifest-path examples/yew_basic_tween/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/yew_scroll_trigger/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/yew_animated_list/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/yew_drag_gesture/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/yew_page_transition/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/yew_agent_coordination/Cargo.toml --target wasm32-unknown-unknown
cd examples/wasm_counter
wasm-pack build --target web
```

## Related Docs

- [Release](./release.md)
- [Benchmarks](./benchmarks.md)
- [Troubleshooting](./troubleshooting.md)
