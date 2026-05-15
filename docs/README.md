# Animato Documentation

This directory contains the stable v1.0.0 documentation for Animato. Use it with
the generated API docs on [docs.rs](https://docs.rs/animato).

## Start Here

| Guide | Use it for |
|-------|------------|
| [Getting Started](./getting-started.md) | First tween, update loop, and value read. |
| [Installation](./installation.md) | Cargo setup, features, no_std, WASM, Bevy. |
| [Concepts](./concepts.md) | Core traits and runtime model. |
| [API Full](./api-full.md) | Stable public API map by crate. |
| [Feature Flags](./feature-flags.md) | Facade and sub-crate feature requirements. |

## Feature Guides

| Area | Guide |
|------|-------|
| Tweens and keyframes | [tween.md](./tween.md) |
| Timelines and sequences | [timeline.md](./timeline.md) |
| Springs | [spring.md](./spring.md) |
| Paths, SVG, morphing | [path.md](./path.md) |
| Input physics | [physics.md](./physics.md) |
| Color interpolation | [color.md](./color.md) |
| Drivers and clocks | [driver.md](./driver.md) |
| GPU batching | [gpu.md](./gpu.md) |
| Bevy | [bevy.md](./bevy.md) |
| WASM and DOM | [wasm.md](./wasm.md) |

## Operations

| Guide | Use it for |
|-------|------------|
| [Examples](./examples.md) | Running every example target. |
| [Recipes](./recipes.md) | Practical app patterns. |
| [Performance](./performance.md) | Hot path and allocation guidance. |
| [Benchmarks](./benchmarks.md) | Benchmark coverage and release baseline. |
| [Testing](./testing.md) | Local and CI test commands. |
| [Release](./release.md) | Publishing checklist. |
| [Migration](./migration.md) | Moving to v1.0.0. |
| [Troubleshooting](./troubleshooting.md) | Common setup and feature issues. |
| [FAQ](./faq.md) | Short answers to common questions. |

## Minimal Install

```toml
[dependencies]
animato = "1.0"
```

## First Animation

```rust
use animato::{Easing, Tween, Update};

let mut tween = Tween::new(0.0_f32, 1.0)
    .duration(0.25)
    .easing(Easing::EaseOutCubic)
    .build();

tween.update(0.125);
let opacity = tween.value();
assert!(opacity > 0.0 && opacity < 1.0);
```

## Stable Contract

Animato v1.0.0 freezes the current public API. Public items are documented and
covered by tests, and breaking changes require a future major release.
