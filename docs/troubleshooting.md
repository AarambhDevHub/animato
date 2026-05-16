# Troubleshooting

## Missing Type From The Facade

Enable the matching feature:

```toml
animato = { version = "1.1", features = ["path"] }
```

Examples:

- `MotionPathTween` needs `path`.
- `Inertia` needs `physics`.
- `GpuAnimationBatch` needs `gpu`.
- `RafDriver` needs `wasm`.
- DOM helpers need `wasm-dom` and a `wasm32` target.

## no_std Build Fails

Use focused crates and disable default features:

```toml
animato-tween = { version = "1.1", default-features = false }
```

Do not use `AnimationDriver`, `Timeline`, Bevy, WASM DOM helpers, or wall-clock
types on bare-metal targets.

## Bevy Example Does Not Compile

The example in docs uses the full `bevy` crate for user convenience. The
workspace integration crate depends on modular Bevy crates. In an app, add Bevy
as usual and enable `animato`'s `bevy` feature.

## WASM Build Fails

Install `wasm-pack`, add the target, and build from the example directory:

```sh
rustup target add wasm32-unknown-unknown
cd examples/wasm_counter
wasm-pack build --target web
```

## Coverage Or Fuzz Command Missing

Install tools:

```sh
cargo install cargo-llvm-cov --locked
cargo install cargo-fuzz --locked
```

## Related Docs

- [Feature Flags](./feature-flags.md)
- [Testing](./testing.md)
