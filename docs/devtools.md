# DevTools

Animato v1.6.0 adds `animato-devtools` for runtime inspection, tuning,
recording, and performance monitoring.

## Install

```toml
[dependencies]
animato = { version = "1.6.0", features = ["devtools"] }
```

Panel adapters are opt in:

```toml
animato = { version = "1.6.0", features = ["devtools-web-panel"] }
animato = { version = "1.6.0", features = ["devtools-egui-panel"] }
animato = { version = "1.6.0", features = ["devtools-tui-panel"] }
```

## Timeline Inspector

Register inspectable animations with `AnimationDriver::add_inspectable`:

```rust
use animato::{AnimationDriver, TimelineInspector, Tween};

let mut driver = AnimationDriver::new();
driver.add_inspectable("opacity", Tween::new(0.0_f32, 1.0).duration(1.0).build());
driver.tick(0.5);

let mut inspector = TimelineInspector::new();
inspector.capture(&driver);

for snapshot in inspector.snapshots() {
    println!(
        "{} {:.0}%",
        snapshot.label.as_deref().unwrap_or("animation"),
        snapshot.progress * 100.0
    );
}
```

## Easing Editor

```rust
use animato::{Easing, EasingCurveEditor};

let mut editor = EasingCurveEditor::new(Easing::EaseOutCubic);
editor.set_sample_count(64);
let points = editor.samples();

editor.set_control_points(0.4, 0.0, 0.2, 1.0);
assert!(editor.copy_code().contains("CubicBezier"));
```

## Spring Visualizer

```rust
use animato::{SpringConfig, SpringVisualizer};

let mut visualizer = SpringVisualizer::new(SpringConfig::wobbly());
visualizer.simulate(1.0, 1.0 / 60.0, 180);

println!("settle={}s overshoot={}%", visualizer.settle_time(), visualizer.overshoot_pct());
```

## Recorder Controls

`RecorderControls` wraps the existing `animato-driver` recorder so Rust and JS
use one JSON/binary recording format.

```rust
use animato::RecorderControls;

let mut recorder = RecorderControls::new();
recorder.start();
recorder.record("x", 0.0, 0.0);
recorder.record("x", 1.0, 100.0);

let json = recorder.export_json();
let replay = recorder.replay("x", 0.5);
assert_eq!(replay, Some(50.0));
```

## Performance Monitor

```rust
use animato::{AnimationDriver, PerformanceMonitor, Tween};

let mut driver = AnimationDriver::new();
driver.add_inspectable("fade", Tween::new(0.0_f32, 1.0).duration(1.0).build());

let profile = driver.tick_profiled(1.0 / 60.0);
let mut perf = PerformanceMonitor::new(120);
perf.record_profile(&profile);

println!("fps={} budget={}", perf.fps(), perf.frame_budget_usage(60.0));
```

## JavaScript

The NPM package exports DevTools classes:

```js
import init, {
  EasingCurveEditor,
  RafDriver,
  TimelineInspector,
  Tween,
} from "@aarambhdevhub/animato-core";

await init();

const tween = new Tween(0, 1, 1);
const driver = new RafDriver();
driver.addTween(tween);
driver.tick(1000);
driver.tick(1250);

const inspector = new TimelineInspector();
inspector.captureRafDriver(driver);

const editor = new EasingCurveEditor("easeOutCubic");
const points = editor.samplePoints();
```

## Examples

```sh
cargo check --manifest-path examples/devtools_web_overlay/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/devtools_bevy_egui/Cargo.toml
cargo run --manifest-path examples/devtools_tui/Cargo.toml

bash scripts/build-js-package.sh
npm ci --prefix examples/js_devtools
npm run build --prefix examples/js_devtools
```

## Verification

```sh
cargo test -p animato-devtools --all-features
cargo test -p animato --features devtools --test devtools_facade
cargo check -p animato-devtools --target wasm32-unknown-unknown --features web-panel
cargo check -p animato-js --target wasm32-unknown-unknown --all-features
```
