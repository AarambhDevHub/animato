# Motus

> *Latin: mōtus — motion, movement, impulse.*

[![Crates.io](https://img.shields.io/crates/v/motus.svg)](https://crates.io/crates/motus)
[![Docs.rs](https://docs.rs/motus/badge.svg)](https://docs.rs/motus)
[![CI](https://github.com/AarambhDevHub/motus/actions/workflows/ci.yml/badge.svg)](https://github.com/AarambhDevHub/motus/actions)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

A professional-grade, renderer-agnostic animation library for Rust.

Zero mandatory dependencies. `no_std`-ready. Built as a clean Cargo workspace — use only the crates you need.

Works everywhere: TUIs, Web (WASM), Bevy games, embedded targets, CLI tools, and native desktop apps.

---

## Why Motus?

Most Rust animation crates are either too minimal (just easing functions) or too coupled to a specific renderer or framework. Motus sits in between:

- **Computes values, never renders.** You own the render loop; Motus just tells you what the value is at each frame.
- **Workspace architecture.** Each concern lives in its own crate. You don't download `wgpu` just to animate a progress bar.
- **`no_std` core.** The trait system, all easing functions, `Tween<T>`, and `Spring` compile without `std` or heap allocation.
- **Builder pattern everywhere.** No positional argument confusion; every optional field has a sensible default.
- **Generic over your types.** Implement `Interpolate` once and animate any value — `f32`, `[f32; 3]`, your custom color type, anything.

---

## Crates

| Crate | Description | `no_std` |
|-------|-------------|----------|
| [`motus-core`](./crates/motus-core) | Traits (`Interpolate`, `Animatable`, `Update`) + 43 easing functions | ✅ |
| [`motus-tween`](./crates/motus-tween) | `Tween<T>`, `KeyframeTrack<T>`, `Loop` | ✅ |
| [`motus-spring`](./crates/motus-spring) | `Spring`, `SpringN<T>`, `SpringConfig` presets | ✅ |
| [`motus-timeline`](./crates/motus-timeline) | `Timeline`, `Sequence`, `stagger`, `At` positioning | — |
| [`motus-path`](./crates/motus-path) | Bezier, CatmullRom, SVG path parser, shape morph | — |
| [`motus-physics`](./crates/motus-physics) | Inertia, `DragState`, `GestureRecognizer` | — |
| [`motus-color`](./crates/motus-color) | Perceptual color interpolation (Lab, Oklch, Linear) | — |
| [`motus-driver`](./crates/motus-driver) | `AnimationDriver`, `Clock` variants, `ScrollDriver` | — |
| [`motus-gpu`](./crates/motus-gpu) | `GpuAnimationBatch` — 10K+ tweens per frame on GPU | — |
| [`motus-bevy`](./crates/motus-bevy) | `MotusPlugin` for Bevy | — |
| [`motus-wasm`](./crates/motus-wasm) | `RafDriver`, FLIP, SplitText, ScrollSmoother | — |
| [`motus`](./crates/motus) | Facade crate — re-exports everything | — |

Most users only need the facade:

```toml
[dependencies]
motus = "0.1"
```

---

## Quick Start

### Animate a single value

```rust
use motus::{Tween, Easing, Update};

let mut tween = Tween::new(0.0_f32, 100.0)
    .duration(1.0)
    .easing(Easing::EaseOutCubic)
    .build();

// In your update loop — pass dt (seconds since last frame):
tween.update(0.016); // ~60fps tick
println!("{}", tween.value()); // current interpolated value
```

### Spring physics

```rust
use motus::{Spring, SpringConfig, Update};

let mut spring = Spring::new(SpringConfig::wobbly());
spring.set_target(200.0);

// Animate until settled:
while !spring.is_settled() {
    spring.update(1.0 / 60.0);
}
println!("settled at {}", spring.position());
```

### Keyframe animation with looping

```rust
use motus::{KeyframeTrack, Loop, Easing, Update};

let mut track = KeyframeTrack::new()
    .push(0.0, 0.0_f32)
    .push_eased(0.5, 1.0, Easing::EaseOutQuad)
    .push_eased(1.0, 0.0, Easing::EaseInQuad)
    .looping(Loop::Forever);

for _ in 0..600 {
    track.update(1.0 / 60.0);
}
println!("{}", track.value()); // oscillates between 0.0 and 1.0
```

### Multi-step timeline

```rust
use motus::{Tween, Easing, Sequence, Update};

let mut seq = Sequence::new()
    .then("fade_in",  Tween::new(0.0_f32, 1.0).duration(0.4).easing(Easing::EaseOutCubic).build(), 0.4)
    .gap(0.1)
    .then("slide_up", Tween::new(0.0_f32, 100.0).duration(0.6).easing(Easing::EaseOutBack).build(), 0.6)
    .build();

seq.play();
// In loop: seq.update(dt);
```

### Animate along a motion path

```rust
use motus::{MotionPath, MotionPathTween, Easing, Update};

let path = MotionPath::new()
    .cubic([0.0, 0.0], [50.0, 100.0], [100.0, 100.0], [150.0, 0.0]);

let mut tween = MotionPathTween::new(path)
    .duration(2.0)
    .easing(Easing::EaseInOutCubic)
    .auto_rotate(true);

tween.update(1.0);
let [x, y] = tween.value();
let heading = tween.rotation_deg();
```

### Perceptual color interpolation

```rust
use motus::{Tween, Easing, Update};
use motus::color::InLab;
use palette::Srgba;

let mut tween = Tween::new(
    InLab(Srgba::new(1.0_f32, 0.0, 0.0, 1.0)),  // red
    InLab(Srgba::new(0.0_f32, 0.0, 1.0, 1.0)),  // blue
)
    .duration(1.0)
    .easing(Easing::Linear)
    .build();

tween.update(0.5);
let color: Srgba = tween.value().0; // perceptually correct midpoint
```

### AnimationDriver — manage many animations

```rust
use motus::{Tween, Easing, AnimationDriver, WallClock};

let mut driver = AnimationDriver::new();
let mut clock  = WallClock::new();

let id = driver.add(
    Tween::new(0.0_f32, 1.0).duration(2.0).easing(Easing::EaseInOutSine).build()
);

loop {
    let dt = clock.delta();
    driver.tick(dt);

    if !driver.is_active(id) { break; }
}
```

---

## Feature Flags

```toml
[dependencies]
motus = { version = "0.1", features = ["path", "color", "serde"] }
```

| Feature | What it adds |
|---------|-------------|
| `default` | `std` + `tween` + `timeline` + `spring` + `driver` |
| `std` | Wall clock, callbacks, heap allocation |
| `tween` | `Tween<T>`, `KeyframeTrack<T>` |
| `timeline` | `Timeline`, `Sequence`, `stagger` |
| `spring` | `Spring`, `SpringN<T>`, all presets |
| `path` | Bezier, MotionPath, SVG parser, shape morph |
| `physics` | Inertia, DragState, GestureRecognizer |
| `color` | Perceptual color interpolation via `palette` |
| `driver` | `AnimationDriver`, Clock variants, ScrollDriver |
| `gpu` | `GpuAnimationBatch` via `wgpu` |
| `bevy` | `MotusPlugin` for Bevy |
| `wasm` | `RafDriver` + WASM bindings |
| `serde` | `Serialize`/`Deserialize` on all public types |
| `tokio` | `.wait().await` on `Timeline` completion |

### `no_std` usage

```toml
[dependencies]
motus-core   = { version = "0.1", default-features = false }
motus-tween  = { version = "0.1", default-features = false }
motus-spring = { version = "0.1", default-features = false }
```

Available in `no_std`: `Easing`, `Tween<T>`, `Spring`, `SpringN<T>`, all `Interpolate` blanket impls.

---

## Easing Functions

43 easing functions available as `Easing::EaseOutCubic` (enum) or `ease_out_cubic(t)` (free function):

| Group | Variants |
|-------|----------|
| Linear | `Linear` |
| Polynomial | `EaseIn/Out/InOut` × Quad, Cubic, Quart, Quint (12 total) |
| Sinusoidal | `EaseIn/Out/InOutSine` |
| Exponential | `EaseIn/Out/InOutExpo` |
| Circular | `EaseIn/Out/InOutCirc` |
| Back (overshoot) | `EaseIn/Out/InOutBack` |
| Elastic | `EaseIn/Out/InOutElastic` |
| Bounce | `EaseIn/Out/InOutBounce` |
| CSS-compatible | `CubicBezier(x1, y1, x2, y2)`, `Steps(n)` |
| Advanced | `RoughEase`, `SlowMo`, `Wiggle`, `CustomBounce`, `ExpoScale` |
| Escape hatch | `Custom(fn(f32) -> f32)` |

---

## Bevy Integration

```rust
use bevy::prelude::*;
use motus_bevy::{MotusPlugin, TweenCompleted};
use motus::Tween;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MotusPlugin)
        .add_systems(Startup, spawn_animated)
        .add_systems(Update, on_tween_done)
        .run();
}

fn spawn_animated(mut commands: Commands) {
    commands.spawn((
        SpriteBundle::default(),
        Tween::new([0.0_f32, 0.0], [300.0, 0.0])
            .duration(1.0)
            .easing(Easing::EaseOutBack)
            .build(),
    ));
}

fn on_tween_done(mut events: EventReader<TweenCompleted>) {
    for ev in events.read() {
        println!("Entity {:?} finished animating", ev.entity);
    }
}
```

---

## WASM Integration

```toml
[dependencies]
motus = { version = "0.1", features = ["wasm"] }
```

```rust
use wasm_bindgen::prelude::*;
use motus::{Tween, Easing};
use motus::wasm::RafDriver;

#[wasm_bindgen]
pub struct App {
    tween:  Tween<f32>,
    driver: RafDriver,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            tween:  Tween::new(0.0_f32, 500.0).duration(1.5).easing(Easing::EaseOutBounce).build(),
            driver: RafDriver::new(),
        }
    }

    // Call from JavaScript requestAnimationFrame callback:
    pub fn tick(&mut self, timestamp_ms: f64) { self.driver.tick(timestamp_ms); }
    pub fn value(&self) -> f32 { self.tween.value() }
}
```

Build: `wasm-pack build --target web --features wasm`

---

## Architecture

Motus is a Cargo workspace of 12 focused crates. See [ARCHITECTURE.md](./ARCHITECTURE.md) for the full design document — crate boundaries, module specifications, type system design, data flow diagrams, and performance guidelines.

---

## Examples

```
cargo run --example basic_tween
cargo run --example spring_demo
cargo run --example timeline_sequence
cargo run --example motion_path
cargo run --example color_animation
cargo run --example tui_progress
cargo run --example tui_spinner
```

---

## Running Tests

```sh
# All tests, all features:
cargo test --workspace --all-features

# no_std check:
cargo test --workspace --no-default-features

# Benchmarks:
cargo bench

# Docs:
cargo doc --workspace --all-features --open
```

---

## Roadmap

See [ROADMAP.md](./ROADMAP.md) for the full versioned plan from `v0.1.0` to `v1.0.0`.

Current milestone: **v0.1.0 — Foundation**

---

## Contributing

Contributions are welcome — bug reports, feature suggestions, documentation improvements, and pull requests.

See [CONTRIBUTING.md](./CONTRIBUTING.md) for how to set up the workspace, run tests, and submit a PR.

---

## Support

If Motus is useful to you, consider supporting ongoing development:

- ⭐ Star the repo on [GitHub](https://github.com/AarambhDevHub/motus)
- ☕ [Buy Me a Coffee](https://buymeacoffee.com/aarambhdevhub)
- 💖 [GitHub Sponsors](https://github.com/sponsors/aarambh-darshan)

---

## License

Licensed under either of:

- [MIT License](./LICENSE-MIT)
- [Apache License, Version 2.0](./LICENSE-APACHE)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project shall be dual-licensed as above, without any additional terms or conditions.
