# Testing

This is the v1.5.0 release verification set.

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
cargo check -p animato-js --target wasm32-unknown-unknown --all-features
bash scripts/build-js-package.sh
cargo bench --workspace --no-run
```

For a local CI-style run, use:

```sh
bash scripts/ci-local.sh
```

## Coverage

Install:

```sh
cargo install cargo-llvm-cov --locked
```

Run:

```sh
bash scripts/coverage-core.sh
```

The coverage gate targets the core engine crates. Browser/UI adapter crates are
validated by their compile, browser, and example gates because native llvm
coverage does not measure their renderer paths accurately.

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
cargo check -p animato-js --target wasm32-unknown-unknown --all-features
cargo check --manifest-path examples/yew_basic_tween/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/yew_scroll_trigger/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/yew_animated_list/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/yew_drag_gesture/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/yew_page_transition/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/yew_agent_coordination/Cargo.toml --target wasm32-unknown-unknown
cd examples/wasm_counter
wasm-pack build --target web
```

## JavaScript Package

```sh
cargo test -p animato-js
bash scripts/wasm-pack-test-js.sh
bash scripts/build-js-package.sh
node scripts/check-js-size.mjs
npm run typecheck --prefix examples/js_vanilla_timeline
npm run build --prefix examples/js_vanilla_timeline
npm run typecheck --prefix examples/js_react_tween
npm run build --prefix examples/js_react_tween
npm run typecheck --prefix examples/js_svelte_spring
npm run build --prefix examples/js_svelte_spring
npm run typecheck --prefix examples/js_vue_motion
npm run build --prefix examples/js_vue_motion
npm run typecheck --prefix examples/js_angular_color
npm run build --prefix examples/js_angular_color
npm run typecheck --prefix examples/js_advanced_engine
npm run build --prefix examples/js_advanced_engine
```

The v1.5.0 package budget is 140 KiB gzipped WASM for the full JavaScript
surface.

## Related Docs

- [Release](./release.md)
- [JavaScript](./javascript.md)
- [Benchmarks](./benchmarks.md)
- [Troubleshooting](./troubleshooting.md)
