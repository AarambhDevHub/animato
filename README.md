<div align="center">
  <img src="./assets/animato_logo.svg" alt="Animato logo" width="500">
</div>

> *Italian: animato — animated, lively, with life and movement.*

[![Crates.io](https://img.shields.io/crates/v/animato.svg)](https://crates.io/crates/animato)
[![Docs.rs](https://docs.rs/animato/badge.svg)](https://docs.rs/animato)
[![CI](https://github.com/AarambhDevHub/animato/actions/workflows/ci.yml/badge.svg)](https://github.com/AarambhDevHub/animato/actions)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![Version](https://img.shields.io/crates/v/animato.svg)](https://crates.io/crates/animato)

A professional-grade, renderer-agnostic animation library for Rust.

Zero mandatory dependencies. `no_std`-ready. Built as a clean Cargo workspace — use only the crates you need.

Works everywhere: TUIs, Web (WASM), Bevy games, embedded targets, CLI tools, and native desktop apps.

---

## Why Animato?

Most Rust animation crates are either too minimal (just easing functions) or too coupled to a specific renderer or framework. Animato sits in between:

- **Computes values, never renders.** You own the render loop; Animato just tells you what the value is at each frame.
- **Workspace architecture.** Each concern lives in its own crate. You don't download `wgpu` just to animate a progress bar.
- **`no_std` core.** The trait system, all easing functions, `Tween<T>`, and `Spring` compile without `std` or heap allocation.
- **Builder pattern everywhere.** No positional argument confusion; every optional field has a sensible default.
- **Generic over your types.** Implement `Interpolate` once and animate any value — `f32`, `[f32; 3]`, your custom color type, anything.

---

## Crates

**Shipped in v0.4.0:**

| Crate | Description | `no_std` |
|-------|-------------|----------|
| [`animato-core`](./crates/animato-core) | Traits (`Interpolate`, `Animatable`, `Update`, `Playable`) + 33 easing variants | yes |
| [`animato-tween`](./crates/animato-tween) | `Tween<T>`, `KeyframeTrack<T>`, `Loop`, `TweenState`, `TweenBuilder` | alloc |
| [`animato-timeline`](./crates/animato-timeline) | `Timeline`, `Sequence`, `stagger`, callbacks, time scale, async wait | alloc |
| [`animato-spring`](./crates/animato-spring) | `Spring`, `SpringN<T>`, `SpringConfig` presets | ✅ |
| [`animato-path`](./crates/animato-path) | Bezier curves, CatmullRom splines, motion paths, SVG path parsing | core no_std |
| [`animato-driver`](./crates/animato-driver) | `AnimationDriver`, `Clock`, `WallClock`, `MockClock` | — |
| [`animato`](./crates/animato) | Facade crate — re-exports all of the above | — |

**Planned in future versions (see [ROADMAP.md](./ROADMAP.md)):**

| Crate | Version | Description |
|-------|---------|-------------|
| `animato-physics` | v0.5.0 | Inertia, `DragState`, `GestureRecognizer` |
| `animato-color` | v0.6.0 | Perceptual color interpolation (Lab, Oklch, Linear) |
| `animato-bevy` | v0.7.0 | `AnimatoPlugin` for Bevy |
| `animato-wasm` | v0.7.0 | `RafDriver`, FLIP, SplitText, ScrollSmoother |
| `animato-gpu` | v0.9.0 | `GpuAnimationBatch` — 10K+ tweens per frame on GPU |

Most users only need the facade:

```toml
[dependencies]
animato = "0.4"
```

---

## Quick Start

### Animate a single value

```rust
use animato::{Tween, Easing, Update};

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
use animato::{Spring, SpringConfig, Update};

let mut spring = Spring::new(SpringConfig::wobbly());
spring.set_target(200.0);

// Animate until settled:
while !spring.is_settled() {
    spring.update(1.0 / 60.0);
}
println!("settled at {}", spring.position());
```

### Loop modes

```rust
use animato::{Tween, Loop, Easing, Update};

let mut tween = Tween::new(0.0_f32, 100.0)
    .duration(1.0)
    .easing(Easing::EaseInOutSine)
    .looping(Loop::PingPong)
    .build();

for _ in 0..600 {
    tween.update(1.0 / 60.0);
}
println!("{}", tween.value()); // oscillates 0 ↔ 100
```

### AnimationDriver — manage many animations

```rust
use animato::{Tween, Easing, AnimationDriver, WallClock, Clock};

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

### Multi-dimensional spring

```rust
use animato::{SpringN, SpringConfig, Update};

let mut spring: SpringN<[f32; 3]> = SpringN::new(SpringConfig::stiff(), [0.0; 3]);
spring.set_target([100.0, 200.0, 300.0]);

while !spring.is_settled() {
    spring.update(1.0 / 60.0);
}
let [x, y, z] = spring.position();
```

### Keyframe tracks

```rust
use animato::{Easing, KeyframeTrack, Update};

let mut track = KeyframeTrack::new()
    .push_eased(0.0, 0.0_f32, Easing::EaseOutCubic)
    .push(1.0, 100.0);

track.update(0.5);
assert!(track.value().unwrap() > 50.0);
```

### Timeline composition

```rust
use animato::{At, Timeline, Tween, Update};

let fade = Tween::new(0.0_f32, 1.0).duration(1.0).build();
let slide = Tween::new(0.0_f32, 100.0).duration(1.0).build();

let mut timeline = Timeline::new()
    .add("fade", fade, At::Start)
    .add("slide", slide, At::Label("fade"));

timeline.play();
timeline.update(0.5);
assert_eq!(timeline.get::<Tween<f32>>("slide").unwrap().value(), 50.0);
```

### Timeline control

```rust
use animato::{At, Timeline, Tween, Update};

let fade = Tween::new(0.0_f32, 1.0).duration(1.0).build();

let mut timeline = Timeline::new()
    .add("fade", fade, At::Start)
    .time_scale(0.5)
    .on_entry_complete("fade", || println!("fade done"))
    .on_complete(|| println!("timeline done"));

timeline.play();
timeline.update(1.0);
assert_eq!(timeline.get::<Tween<f32>>("fade").unwrap().value(), 0.5);
```

With the `tokio` feature, `timeline.wait().await` resolves when another task or loop drives the timeline to completion.

### Motion paths

```toml
[dependencies]
animato = { version = "0.4", features = ["path"] }
```

```rust
use animato::{CubicBezierCurve, Easing, MotionPathTween, Update};

let curve = CubicBezierCurve::new(
    [0.0, 0.0],
    [40.0, 90.0],
    [140.0, -90.0],
    [200.0, 0.0],
);

let mut motion = MotionPathTween::new(curve)
    .duration(1.0)
    .easing(Easing::EaseInOutSine)
    .auto_rotate(true)
    .build();

motion.update(0.5);
let [x, y] = motion.value();
let rotation = motion.rotation_deg();
```

---

## Feature Flags

```toml
[dependencies]
animato = { version = "0.4", features = ["serde"] }
```

**v0.4.0 features:**

| Feature | What it adds |
|---------|--------------|
| `default` | `std` + `tween` + `timeline` + `spring` + `driver` |
| `std` | Wall clock, heap-backed composition types, timeline callbacks |
| `tween` | `Tween<T>`, `KeyframeTrack<T>`, `Loop`, `TweenState` |
| `timeline` | `Timeline`, `Sequence`, `stagger`, time scale |
| `spring` | `Spring`, `SpringN<T>`, all presets |
| `path` | `QuadBezier`, `CubicBezierCurve`, `CatmullRomSpline`, `MotionPathTween`, `SvgPathParser` |
| `driver` | `AnimationDriver`, `Clock` variants |
| `serde` | `Serialize`/`Deserialize` on supported concrete public types |
| `tokio` | `.wait().await` on `Timeline` completion |

**Features planned for future versions:**

| Feature | Version | What it adds |
|---------|---------|--------------|
| `physics` | v0.5.0 | Inertia, DragState, GestureRecognizer |
| `color` | v0.6.0 | Perceptual color interpolation via `palette` |
| `bevy` | v0.7.0 | `AnimatoPlugin` for Bevy |
| `wasm` | v0.7.0 | `RafDriver` + WASM bindings |
| `gpu` | v0.9.0 | `GpuAnimationBatch` via `wgpu` |

### `no_std` usage

```toml
[dependencies]
animato-core   = { version = "0.4", default-features = false }
animato-tween  = { version = "0.4", default-features = false }
animato-spring = { version = "0.4", default-features = false }
animato-path   = { version = "0.4", default-features = false }
```

Available in `no_std`: `Easing`, `Tween<T>`, `Spring`, fixed Bezier curves, all `Interpolate` blanket impls. `KeyframeTrack<T>`, `Timeline`, `SpringN<T>`, `MotionPath`, and SVG parsing require allocation.

---

## Easing Functions

33 easing variants are available as `Easing::EaseOutCubic` (enum) or `ease_out_cubic(t)` (free function where applicable):

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
| CSS-compatible | `CubicBezier(f32, f32, f32, f32)`, `Steps(u32)` |
| Escape hatch | `Custom(fn(f32) -> f32)` |

Additional advanced variants (`RoughEase`, `SlowMo`, `Wiggle`, `CustomBounce`, `ExpoScale`) remain planned for v0.8.0.

---

## Future Bevy Integration

Planned for v0.7.0.

```rust
use bevy::prelude::*;
use animato_bevy::{AnimatoPlugin, TweenCompleted};
use animato::Tween;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AnimatoPlugin)
        .add_systems(Startup, spawn_animated)
        .add_systems(Update, on_tween_done)
        .run();
}

fn spawn_animated(mut commands: Commands) {
    commands.spawn((
        Sprite::default(),
        Transform::default(),
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

## Future WASM Integration

Planned for v0.7.0.

```toml
[dependencies]
animato = { version = "0.7", features = ["wasm"] }
```

```rust
use wasm_bindgen::prelude::*;
use animato::{Tween, Easing};
use animato::wasm::RafDriver;

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

Animato is a focused Cargo workspace with additional planned crates through v1.0. See [ARCHITECTURE.md](./ARCHITECTURE.md) for the full design document — crate boundaries, module specifications, type system design, data flow diagrams, and performance guidelines.

---

## Examples

```sh
cargo run --example basic_tween
cargo run --example spring_demo
cargo run --example keyframe_track
cargo run --example timeline_sequence
cargo run --example motion_path --features path

# Coming in future versions:
# cargo run --example color_animation     # v0.6.0
# cargo run --example tui_progress        # v0.7.0
```

---

## Running Tests

```sh
# All tests, all features:
cargo test --workspace --all-features

# no_std check:
cargo test -p animato-core -p animato-tween -p animato-spring -p animato-path --no-default-features

# Benchmarks:
cargo bench

# Docs:
cargo doc --workspace --all-features --open
```

---

## Roadmap

See [ROADMAP.md](./ROADMAP.md) for the full versioned plan from `v0.1.0` to `v1.0.0`.

**Current status: `v0.4.0 — Paths` shipped**

| Next | Milestone |
|------|-----------|
| `v0.5.0` | Physics — inertia, drag, gesture recognition |

---

## Contributing

Contributions are welcome — bug reports, feature suggestions, documentation improvements, and pull requests.

See [CONTRIBUTING.md](./CONTRIBUTING.md) for how to set up the workspace, run tests, and submit a PR.

---

## Support

If Animato is useful to you, consider supporting ongoing development:

- ⭐ Star the repo on [GitHub](https://github.com/AarambhDevHub/animato)
- ☕ [Buy Me a Coffee](https://buymeacoffee.com/aarambhdevhub)
- 💖 [GitHub Sponsors](https://github.com/sponsors/aarambh-darshan)

---

## License

Licensed under either of:

- [MIT License](./LICENSE-MIT)
- [Apache License, Version 2.0](./LICENSE-APACHE)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project shall be dual-licensed as above, without any additional terms or conditions.
