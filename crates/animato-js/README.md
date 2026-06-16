# animato-js

Rust/WASM bindings for Animato, published to NPM as `@aarambhdevhub/animato-core`.

```sh
npm install @aarambhdevhub/animato-core
```

```js
import init, { Tween, RafDriver, TimelineInspector, ColorTween, Quaternion, QuaternionTween, Waveform } from "@aarambhdevhub/animato-core";

await init();

const tween = new Tween(0, 240, 0.7);
tween.setEasing("easeOutCubic");

const driver = new RafDriver();
driver.addTween(tween);
driver.tick(16.67);

const inspector = new TimelineInspector();
inspector.captureRafDriver(driver);

const color = new ColorTween("#ff3355", "#2f80ed", 0.6, "oklch");
color.update(0.3);

const rotation = new QuaternionTween(
  Quaternion.identity(),
  Quaternion.fromAxisAngle(0, 1, 0, 180),
  1.0,
);
rotation.update(0.5);

const wave = Waveform.sine(1, 24, 0);
console.log(wave.sample(0.25));
```

Exports include scalar/vector tweens, keyframes, timelines, springs, rAF and
scroll drivers, motion paths, morphing, draw values, inertia, drag, gestures,
color interpolation, DOM helpers, batch tween evaluation, velocity springs,
waveforms, stagger patterns, quaternion/matrix tweens, animation groups, and
recording, plus DevTools timeline inspection, easing curve samples, spring
visualization, and performance monitoring.

Build the local package:

```sh
bash scripts/build-js-package.sh
```

Run package tests:

```sh
cargo test -p animato-js
cargo check -p animato-js --target wasm32-unknown-unknown --all-features
bash scripts/wasm-pack-test-js.sh
```

See [`docs/javascript.md`](../../docs/javascript.md) for the full JavaScript
guide.
