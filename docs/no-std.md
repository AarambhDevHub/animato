# no_std

Animato's low-level crates support `no_std` so animation math can run on
embedded and constrained targets.

## Dependencies

```toml
[dependencies]
animato-core = { version = "1.0", default-features = false }
animato-tween = { version = "1.0", default-features = false }
animato-spring = { version = "1.0", default-features = false }
animato-path = { version = "1.0", default-features = false }
animato-physics = { version = "1.0", default-features = false }
animato-color = { version = "1.0", default-features = false }
```

## Available Without Allocation

- `Interpolate`, `Animatable`, `Update`
- `Easing` and free easing functions
- `Tween<T>`
- `Spring`
- fixed Bezier primitives
- `Inertia`
- core gesture recognition types
- color wrappers backed by `palette` with `libm`

## Requires alloc

- `KeyframeTrack<T>`
- `SpringN<T>`
- `MotionPath`, SVG parser, morphing, compound paths
- `InertiaN<T>` and drag helpers

Enable crate-specific `alloc` features where needed:

```toml
animato-spring = { version = "1.0", default-features = false, features = ["alloc"] }
animato-path = { version = "1.0", default-features = false, features = ["alloc"] }
```

## Bare-Metal Check

```sh
rustup target add thumbv7m-none-eabi
cargo build -p animato-core --target thumbv7m-none-eabi --no-default-features
cargo build -p animato-tween --target thumbv7m-none-eabi --no-default-features
cargo build -p animato-spring --target thumbv7m-none-eabi --no-default-features
```

## Related Docs

- [Feature Flags](./feature-flags.md)
- [Testing](./testing.md)
