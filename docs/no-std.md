# no_std

Animato's crates support `no_std` so animation math can run on embedded
and constrained targets. The `animato` facade itself is `no_std`-capable:
with default features disabled, `tween`, `spring`, `physics`, and `color`
compile for bare metal. `timeline` and `path` still require `std`.

## Facade (default-features = false)

```toml
[dependencies]
animato = { version = "1.7.3", default-features = false, features = ["tween", "spring"] }
```

## Focused Crates

```toml
[dependencies]
animato-core = { version = "1.7.3", default-features = false }
animato-tween = { version = "1.7.3", default-features = false }
animato-spring = { version = "1.7.3", default-features = false }
animato-path = { version = "1.7.3", default-features = false }
animato-physics = { version = "1.7.3", default-features = false }
animato-color = { version = "1.7.3", default-features = false }
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
animato-spring = { version = "1.7.3", default-features = false, features = ["alloc"] }
animato-path = { version = "1.7.3", default-features = false, features = ["alloc"] }
```

## Bare-Metal Check

```sh
rustup target add thumbv7m-none-eabi
cargo check -p animato --no-default-features --features tween,spring --target thumbv7m-none-eabi
cargo build -p animato-core --target thumbv7m-none-eabi --no-default-features
cargo build -p animato-tween --target thumbv7m-none-eabi --no-default-features
cargo build -p animato-spring --target thumbv7m-none-eabi --no-default-features
```

The facade and the sub-crates both compile for `thumbv7m-none-eabi` in the
CI `no_std cross-compile` job and in the publish workflow's
"Build no_std crates for bare metal" step.

## Related Docs

- [Feature Flags](./feature-flags.md)
- [Testing](./testing.md)
