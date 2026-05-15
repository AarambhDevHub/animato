# Installation

Animato is published as focused crates plus the `animato` facade.

## Facade

```toml
[dependencies]
animato = "1.0"
```

Default features include `std`, `tween`, `timeline`, `spring`, and `driver`.

## Optional Capabilities

```toml
[dependencies]
animato = { version = "1.0", features = ["path", "physics", "color", "serde"] }
```

Common combinations:

| Use case | Dependency |
|----------|------------|
| TUI or CLI app | `animato = "1.0"` |
| Bevy game | `animato = { version = "1.0", features = ["bevy"] }` |
| Browser rAF loop | `animato = { version = "1.0", features = ["wasm"] }` |
| Browser DOM helpers | `animato = { version = "1.0", features = ["wasm-dom"] }` |
| SVG paths and morphing | `animato = { version = "1.0", features = ["path"] }` |
| Drag and gestures | `animato = { version = "1.0", features = ["physics"] }` |
| GPU batches | `animato = { version = "1.0", features = ["gpu"] }` |

## Focused Crates

Use focused crates when dependency size matters:

```toml
[dependencies]
animato-core = "1.0"
animato-tween = "1.0"
```

## no_std

```toml
[dependencies]
animato-core = { version = "1.0", default-features = false }
animato-tween = { version = "1.0", default-features = false }
animato-spring = { version = "1.0", default-features = false }
```

See [no-std.md](./no-std.md) for details.

## WASM

Install `wasm-pack` and build the example:

```sh
cd examples/wasm_counter
wasm-pack build --target web
```

See [wasm.md](./wasm.md).

## Verification

After adding Animato, run:

```sh
cargo check
cargo test
```
