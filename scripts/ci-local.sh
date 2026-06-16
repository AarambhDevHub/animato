#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

cargo fmt --check
cargo clippy --workspace --all-features -- -D warnings
cargo test --workspace --all-features
cargo test --workspace --no-default-features
cargo test -p animato --all-features --examples
cargo test -p animato-js
cargo doc --workspace --all-features --no-deps
cargo check -p animato-wasm --target wasm32-unknown-unknown --features wasm-dom
cargo check -p animato-leptos --target wasm32-unknown-unknown --features csr
cargo check -p animato-dioxus --target wasm32-unknown-unknown --features web
cargo check -p animato-yew --target wasm32-unknown-unknown --features csr
cargo check -p animato-js --target wasm32-unknown-unknown --all-features
cargo check -p animato-devtools --all-features
cargo test -p animato-devtools --all-features
cargo check -p animato-devtools --target wasm32-unknown-unknown --features web-panel
cargo check --manifest-path examples/devtools_web_overlay/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/devtools_bevy_egui/Cargo.toml
cargo check --manifest-path examples/devtools_tui/Cargo.toml
bash scripts/build-js-package.sh
bash scripts/wasm-pack-test-js.sh

for example in \
  examples/js_vanilla_timeline \
  examples/js_react_tween \
  examples/js_svelte_spring \
  examples/js_vue_motion \
  examples/js_angular_color \
  examples/js_advanced_engine \
  examples/js_devtools
do
  npm ci --prefix "$example"
  npm run typecheck --prefix "$example"
  npm run build --prefix "$example"
done

cargo bench --workspace --no-run
