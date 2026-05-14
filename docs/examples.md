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

## Compile Examples Without Running

```sh
cargo test -p animato --all-features --examples
```

## Related Docs

- [Getting Started](./getting-started.md)
- [Recipes](./recipes.md)
- [Testing](./testing.md)
