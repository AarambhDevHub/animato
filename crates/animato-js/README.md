# animato-js

Rust/WASM bindings for Animato, published to NPM as `@aarambhdevhub/animato-core`.

```sh
npm install @aarambhdevhub/animato-core
```

```js
import init, { Tween, RafDriver, ColorTween } from "@aarambhdevhub/animato-core";

await init();

const tween = new Tween(0, 240, 0.7);
tween.setEasing("easeOutCubic");

const driver = new RafDriver();
driver.addTween(tween);
driver.tick(16.67);

const color = new ColorTween("#ff3355", "#2f80ed", 0.6, "oklch");
color.update(0.3);
```

Exports include scalar/vector tweens, keyframes, timelines, springs, rAF and
scroll drivers, motion paths, morphing, draw values, inertia, drag, gestures,
color interpolation, DOM helpers, and batch tween evaluation.

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
