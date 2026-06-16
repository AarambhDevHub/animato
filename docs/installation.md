# Installation

Animato is published as focused crates plus the `animato` facade.

## Facade

```toml
[dependencies]
animato = "1.6.0"
```

Default features include `std`, `tween`, `timeline`, `spring`, and `driver`.

## Optional Capabilities

```toml
[dependencies]
animato = { version = "1.6.0", features = ["path", "physics", "color", "serde"] }
```

Common combinations:

| Use case | Dependency |
|----------|------------|
| TUI or CLI app | `animato = "1.6.0"` |
| Bevy game | `animato = { version = "1.6.0", features = ["bevy"] }` |
| Browser rAF loop | `animato = { version = "1.6.0", features = ["wasm"] }` |
| Browser DOM helpers | `animato = { version = "1.6.0", features = ["wasm-dom"] }` |
| Leptos CSR app | `animato = { version = "1.6.0", features = ["leptos-csr"] }` |
| Dioxus web app | `animato = { version = "1.6.0", features = ["dioxus-web"] }` |
| Dioxus desktop app | `animato = { version = "1.6.0", features = ["dioxus-desktop"] }` |
| Yew CSR app | `animato = { version = "1.6.0", features = ["yew-csr"] }` |
| Yew agent coordination | `animato = { version = "1.6.0", features = ["yew-csr", "yew-agent"] }` |
| JavaScript or TypeScript app | `npm install @aarambhdevhub/animato-core` |
| Runtime DevTools | `animato = { version = "1.6.0", features = ["devtools"] }` |
| SVG paths and morphing | `animato = { version = "1.6.0", features = ["path"] }` |
| Drag and gestures | `animato = { version = "1.6.0", features = ["physics"] }` |
| GPU batches | `animato = { version = "1.6.0", features = ["gpu"] }` |

## Focused Crates

Use focused crates when dependency size matters:

```toml
[dependencies]
animato-core = "1.6.0"
animato-tween = "1.4"
```

## no_std

```toml
[dependencies]
animato-core = { version = "1.6.0", default-features = false }
animato-tween = { version = "1.6.0", default-features = false }
animato-spring = { version = "1.6.0", default-features = false }
```

See [no-std.md](./no-std.md) for details.

## WASM

Install `wasm-pack` and build the example:

```sh
cd examples/wasm_counter
wasm-pack build --target web
```

See [wasm.md](./wasm.md).

## JavaScript

Use the NPM package for browser and framework apps:

```sh
npm install @aarambhdevhub/animato-core
```

```js
import init, { Tween, RafDriver } from "@aarambhdevhub/animato-core";

await init();
const tween = new Tween(0, 100, 0.5);
const driver = new RafDriver();
driver.addTween(tween);
```

See [javascript.md](./javascript.md).

## Verification

After adding Animato, run:

```sh
cargo check
cargo test
```
