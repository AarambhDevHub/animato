# Examples

Examples are registered on the facade crate.

## Run All Host Examples

```sh
cargo run --example basic_tween
cargo run --example spring_demo
cargo run --example keyframe_track
cargo run --example timeline_sequence
cargo run --example scroll_linked --features driver
cargo run --example tui_progress
cargo run --example tui_spinner
```

## Feature Examples

```sh
cargo run --example motion_path --features path
cargo run --example morph_path --features path
cargo run --example physics_drag --features physics
cargo run --example color_animation --features color
cargo run --example gpu_particles --features gpu
cargo run --example bevy_transform --features bevy
```

## WASM Example

```sh
cd examples/wasm_counter
wasm-pack build --target web
```

## Leptos Examples

```sh
cargo check --manifest-path examples/leptos_basic_tween/Cargo.toml
cargo check --manifest-path examples/leptos_scroll_trigger/Cargo.toml
cargo check --manifest-path examples/leptos_page_transition/Cargo.toml
cargo check --manifest-path examples/leptos_animated_list/Cargo.toml
cargo check --manifest-path examples/leptos_drag_gesture/Cargo.toml
```

## Dioxus Examples

Run the Dioxus examples from the workspace root. Web examples use Dioxus CLI;
desktop examples can be launched directly with Cargo.

```sh
# Install once if `dx` is not already available.
cargo install dioxus-cli --version 0.7.9 --locked

# Web tween UI, served by Dioxus CLI.
cd examples/dioxus_web_tween
dx serve --web
cd ../..

# Desktop spring/system UI.
cargo run --manifest-path examples/dioxus_desktop_spring/Cargo.toml

# Cross-platform UI as a web app.
cd examples/dioxus_cross_platform
dx serve --web
cd ../..

# Cross-platform UI as a desktop app.
cargo run --manifest-path examples/dioxus_cross_platform/Cargo.toml --no-default-features --features desktop

# Terminal-styled Dioxus progress UI.
cargo run --manifest-path examples/dioxus_tui_progress/Cargo.toml
```

Compile all Dioxus examples without launching a renderer:

```sh
# Web target check.
cargo check --manifest-path examples/dioxus_web_tween/Cargo.toml --target wasm32-unknown-unknown

# Desktop target check.
cargo check --manifest-path examples/dioxus_desktop_spring/Cargo.toml

# Cross-platform checks.
cargo check --manifest-path examples/dioxus_cross_platform/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/dioxus_cross_platform/Cargo.toml --no-default-features --features desktop

# Terminal-styled desktop check.
cargo check --manifest-path examples/dioxus_tui_progress/Cargo.toml
```

## Yew Examples

Run the Yew examples from the workspace root. These are CSR apps and are checked
for `wasm32-unknown-unknown`.

```sh
cargo check --manifest-path examples/yew_basic_tween/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/yew_scroll_trigger/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/yew_animated_list/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/yew_drag_gesture/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/yew_page_transition/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/yew_agent_coordination/Cargo.toml --target wasm32-unknown-unknown
```

## JavaScript Examples

Build the local WASM package once, then run the JavaScript example checks:

```sh
bash scripts/build-js-package.sh

npm ci --prefix examples/js_vanilla_timeline
npm run build --prefix examples/js_vanilla_timeline

npm ci --prefix examples/js_react_tween
npm run build --prefix examples/js_react_tween

npm ci --prefix examples/js_svelte_spring
npm run build --prefix examples/js_svelte_spring

npm ci --prefix examples/js_vue_motion
npm run build --prefix examples/js_vue_motion

npm ci --prefix examples/js_angular_color
npm run build --prefix examples/js_angular_color
```

## Compile Examples Without Running

```sh
cargo test -p animato --all-features --examples
```

## Related Docs

- [Getting Started](./getting-started.md)
- [JavaScript](./javascript.md)
- [Recipes](./recipes.md)
- [Testing](./testing.md)
