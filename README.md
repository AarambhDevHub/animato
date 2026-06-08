<div align="center">
  <img src="./assets/animato_logo.svg" alt="Animato logo" width="500">
</div>

> *Italian: animato — animated, lively, with life and movement.*

[![Crates.io](https://img.shields.io/crates/v/animato.svg)](https://crates.io/crates/animato)
[![Docs.rs](https://docs.rs/animato/badge.svg)](https://docs.rs/animato)
[![CI](https://github.com/AarambhDevHub/animato/actions/workflows/ci.yml/badge.svg)](https://github.com/AarambhDevHub/animato/actions)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

# Animato

Animato is a stable, renderer-agnostic animation toolkit for Rust. It computes
animated values and leaves rendering to your app, engine, terminal UI, browser,
or embedded target.

The v1.5.0 API is stable. The current public crates cover easing, tweens,
timelines, springs, motion paths, input physics, perceptual color interpolation,
drivers, GPU batch evaluation, Bevy integration, WASM/browser helpers, and
first-class Leptos, Dioxus, Yew, JavaScript/WASM integration, and advanced
engine primitives such as velocity springs, waveforms, quaternion slerp,
animation groups, stagger patterns, and recording.

## Install

Most applications use the facade crate:

```toml
[dependencies]
animato = "1.5"
```

Enable only the integrations you need:

```toml
[dependencies]
animato = { version = "1.5", features = ["path", "physics", "color"] }
```

Leptos applications enable the facade feature plus the app rendering mode:

```toml
[dependencies]
animato = { version = "1.5", features = ["leptos-csr"] }
leptos = { version = "0.8.19", features = ["csr"] }
```

Dioxus applications enable the facade feature plus the renderer they ship:

```toml
[dependencies]
animato = { version = "1.5", features = ["dioxus-web"] }
dioxus = { version = "0.7.9", default-features = false, features = ["web", "launch"] }
```

Yew applications enable the facade feature plus the app rendering mode:

```toml
[dependencies]
animato = { version = "1.5", features = ["yew-csr"] }
yew = { version = "0.23", features = ["csr"] }
```

JavaScript applications install the NPM package built from `animato-js`:

```sh
npm install @aarambhdevhub/animato-core
```

```js
import init, { Tween } from "@aarambhdevhub/animato-core";

await init();
const tween = new Tween(0, 320, 0.9);
tween.setEasing("easeOutCubic");
```

For `no_std`, depend on the focused crates directly:

```toml
[dependencies]
animato-core    = { version = "1.5", default-features = false }
animato-tween   = { version = "1.5", default-features = false }
animato-spring  = { version = "1.5", default-features = false }
animato-path    = { version = "1.5", default-features = false }
animato-physics = { version = "1.5", default-features = false }
animato-color   = { version = "1.5", default-features = false }
```

## Quick Start

```rust
use animato::{Easing, Tween, Update};

let mut tween = Tween::new(0.0_f32, 100.0)
    .duration(1.0)
    .easing(Easing::EaseOutCubic)
    .build();

tween.update(0.5);
assert!(tween.value() > 50.0);

tween.update(0.5);
assert_eq!(tween.value(), 100.0);
assert!(tween.is_complete());
```

## Core Concepts

Animato is built around four small traits:

| Trait | Purpose |
|-------|---------|
| `Interpolate` | Defines how a value lerps between two values. |
| `Animatable` | Blanket marker for values that can be animated. |
| `Update` | Advances animation state by a `dt` in seconds. |
| `Playable` | Object-safe animation interface used by timelines. |

The hot path is intentionally simple:

```rust
animation.update(dt);
let value = animation.value();
```

Animato never renders. Your application reads the computed value and applies it
to a UI widget, game transform, DOM style, terminal cell, shader input, or any
other target.

## Crates

| Crate | Purpose | `no_std` |
|-------|---------|----------|
| [`animato-core`](./crates/animato-core) | Traits, interpolation, easing functions | yes |
| [`animato-tween`](./crates/animato-tween) | `Tween<T>`, keyframes, looping, modifiers | yes |
| [`animato-timeline`](./crates/animato-timeline) | Timelines, sequences, staggered starts | std |
| [`animato-spring`](./crates/animato-spring) | 1D and component springs | yes |
| [`animato-path`](./crates/animato-path) | Bezier, SVG paths, motion paths, morphing | yes/core |
| [`animato-physics`](./crates/animato-physics) | Inertia, drag tracking, gestures | yes/core |
| [`animato-color`](./crates/animato-color) | Lab, Oklch, and linear color interpolation | yes |
| [`animato-driver`](./crates/animato-driver) | Animation driver, clocks, scroll driver | std |
| [`animato-gpu`](./crates/animato-gpu) | Batched `Tween<f32>` evaluation with CPU fallback | std |
| [`animato-bevy`](./crates/animato-bevy) | Bevy ECS components, systems, completion messages | std |
| [`animato-wasm`](./crates/animato-wasm) | rAF driver and optional DOM helpers | wasm/std |
| [`animato-leptos`](./crates/animato-leptos) | Leptos signal hooks, scroll, presence, lists, gestures, CSS, SSR | wasm/std |
| [`animato-dioxus`](./crates/animato-dioxus) | Dioxus signals, motion hooks, scroll, presence, lists, gestures, native helpers | wasm/std |
| [`animato-yew`](./crates/animato-yew) | Yew hooks, CSS, scroll, presence, lists, gestures, transitions, agents | wasm/std |
| [`animato-js`](./crates/animato-js) | WASM-to-NPM bindings for JavaScript and framework examples | wasm |
| [`animato`](./crates/animato) | Facade crate re-exporting stable APIs | feature gated |

## Feature Flags

| Feature | What it adds |
|---------|--------------|
| `default` | `std`, `tween`, `timeline`, `spring`, `driver` |
| `std` | Hosted functionality, wall clock, heap-backed helpers |
| `tween` | `Tween<T>`, `KeyframeTrack<T>`, `Loop`, `TweenState` |
| `timeline` | `Timeline`, `Sequence`, `stagger`, timeline callbacks |
| `spring` | `Spring`, `SpringN<T>`, `SpringConfig` presets |
| `path` | Bezier curves, SVG parser, motion paths, morphing, draw helpers |
| `physics` | `Inertia`, `DragState`, `GestureRecognizer` |
| `color` | `InLab<C>`, `InOklch<C>`, `InLinear<C>`, `palette` re-export |
| `driver` | `AnimationDriver`, clocks, `ScrollDriver`, `ScrollClock` |
| `gpu` | `GpuAnimationBatch` through `wgpu` with CPU fallback |
| `bevy` | `AnimatoPlugin`, tween/spring components, transform helpers |
| `wasm` | `RafDriver`, `ScrollSmoother` |
| `wasm-dom` | FLIP, split text, drag, observer, shared-element helpers |
| `leptos` | Leptos hooks/components without forcing CSR/hydrate/SSR mode |
| `leptos-csr` | `leptos` plus Leptos CSR mode |
| `leptos-hydrate` | `leptos` plus Leptos hydration mode |
| `leptos-ssr` | `leptos` plus Leptos SSR mode |
| `dioxus` | Dioxus hooks/components without forcing a renderer |
| `dioxus-web` | `dioxus` plus Dioxus web renderer support |
| `dioxus-desktop` | `dioxus` plus Dioxus desktop renderer support |
| `dioxus-router` | `dioxus` plus route transition helpers |
| `dioxus-native` | `dioxus` plus portable native window animation handles |
| `yew` | Yew hooks/components without forcing an app mode |
| `yew-csr` | `yew` plus Yew CSR mode |
| `yew-hydration` | `yew` plus Yew hydration mode |
| `yew-ssr` | `yew` plus Yew SSR mode |
| `yew-agent` | `yew` plus serializable `f32` animation agent coordination |
| `js` | JavaScript/WASM bindings namespace for package builds |
| `serde` | Serialization for supported public types |
| `tokio` | `Timeline::wait()` async completion helper |

## Examples

```sh
cargo run --example basic_tween
cargo run --example spring_demo
cargo run --example spring_fling --features spring
cargo run --example keyframe_track
cargo run --example waveform_demo
cargo run --example timeline_sequence
cargo run --example quaternion_rotation
cargo run --example stagger_grid
cargo run --example animation_groups
cargo run --example motion_path --features path
cargo run --example morph_path --features path
cargo run --example scroll_linked --features driver
cargo run --example physics_drag --features physics
cargo run --example color_animation --features color
cargo run --example gpu_particles --features gpu
cargo run --example bevy_transform --features bevy
cargo run --example tui_progress
cargo run --example tui_spinner
```

WASM example:

```sh
cd examples/wasm_counter
wasm-pack build --target web
```

Leptos examples:

```sh
cargo check --manifest-path examples/leptos_basic_tween/Cargo.toml
cargo check --manifest-path examples/leptos_scroll_trigger/Cargo.toml
cargo check --manifest-path examples/leptos_page_transition/Cargo.toml
cargo check --manifest-path examples/leptos_animated_list/Cargo.toml
cargo check --manifest-path examples/leptos_drag_gesture/Cargo.toml
```

Dioxus examples:

```sh
cargo check --manifest-path examples/dioxus_web_tween/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/dioxus_desktop_spring/Cargo.toml
cargo check --manifest-path examples/dioxus_cross_platform/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/dioxus_cross_platform/Cargo.toml --no-default-features --features desktop
cargo check --manifest-path examples/dioxus_tui_progress/Cargo.toml
```

Yew examples:

```sh
cargo check --manifest-path examples/yew_basic_tween/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/yew_scroll_trigger/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/yew_animated_list/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/yew_drag_gesture/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/yew_page_transition/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/yew_agent_coordination/Cargo.toml --target wasm32-unknown-unknown
```

JavaScript package and examples:

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
npm ci --prefix examples/js_advanced_engine
npm run build --prefix examples/js_advanced_engine
```

## Documentation

The v1.5 documentation lives in [`docs/`](./docs/):

| Start here | Details |
|------------|---------|
| [Getting Started](./docs/getting-started.md) | First animation and update loop. |
| [API Full](./docs/api-full.md) | Complete stable API map by crate. |
| [Feature Flags](./docs/feature-flags.md) | Exact feature requirements. |
| [Concepts](./docs/concepts.md) | `Interpolate`, `Update`, clocks, composition. |
| [Recipes](./docs/recipes.md) | Practical patterns for UI, games, paths, and input. |
| [Leptos](./docs/leptos.md) | Signal-backed hooks and Leptos integration examples. |
| [Dioxus](./docs/dioxus.md) | Cross-platform Dioxus hooks and native helpers. |
| [Yew](./docs/yew.md) | Yew hooks, components, gestures, and agent coordination. |
| [JavaScript](./docs/javascript.md) | NPM package API and JavaScript framework examples. |
| [Advanced Engine](./docs/advanced-engine.md) | Velocity springs, waveforms, slerp, groups, staggers, recorder. |
| [Testing](./docs/testing.md) | Local and CI verification commands. |
| [Release](./docs/release.md) | v1.5 publishing checklist. |

The generated Rust API docs are available on
[docs.rs/animato](https://docs.rs/animato).

## Testing

The v1.5 release gate is:

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

The local CI mirror is:

```sh
bash scripts/ci-local.sh
```

Coverage and fuzzing are part of the stable release workflow when the tools are
installed:

```sh
bash scripts/coverage-core.sh
cargo +nightly fuzz run svg_path_parser -- -max_total_time=60
```

## Stability

Animato follows Semantic Versioning. Starting at `1.0.0`, public APIs are
treated as stable. Breaking changes require a future major release and migration
notes. The focused sub-crates can still be used independently when a project
needs tighter dependency control than the facade provides.

## Project Docs

- [Architecture](./ARCHITECTURE.md)
- [Roadmap](./ROADMAP.md)
- [Changelog](./CHANGELOG.md)
- [Contributing](./CONTRIBUTING.md)
- [Benchmarks](./docs/benchmarks.md)

## Support

- Star the repo on [GitHub](https://github.com/AarambhDevHub/animato)
- Open issues for reproducible bugs and clear feature requests
- Support development through [Buy Me a Coffee](https://buymeacoffee.com/aarambhdevhub)

## License

Licensed under either of:

- [MIT License](./LICENSE-MIT)
- [Apache License, Version 2.0](./LICENSE-APACHE)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project is dual-licensed as above, without additional
terms or conditions.
