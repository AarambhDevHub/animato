# JavaScript

Animato v1.4.0 publishes the WASM package as `@animato/core`. It exposes the
renderer-agnostic animation engines to JavaScript and TypeScript while keeping
framework adapters in app code.

## Install

```sh
npm install @animato/core
```

For local development inside this repository, build the package first:

```sh
bash scripts/build-js-package.sh
```

Then use the generated package from `crates/animato-js/pkg`.

## Quick Start

```js
import init, { Tween, RafDriver, ease, availableEasings } from "@animato/core";

await init();

const tween = new Tween(0, 320, 0.8);
tween.setEasing("easeOutCubic");

const driver = new RafDriver();
driver.addTween(tween);

function frame(now) {
  driver.tick(now);
  element.style.transform = `translateX(${tween.value()}px)`;
  if (driver.activeCount() > 0) requestAnimationFrame(frame);
}

requestAnimationFrame(frame);

console.log(ease("spring", 0.5));
console.log(availableEasings());
```

## Exported API

Core animation:

| Export | Purpose |
|--------|---------|
| `Tween`, `Tween2D`, `Tween3D`, `Tween4D` | Scalar and vector tweens. |
| `KeyframeTrack`, `KeyframeTrack2D`, `KeyframeTrack3D`, `KeyframeTrack4D` | Timed keyframe tracks. |
| `Spring`, `Spring2D`, `Spring3D`, `Spring4D` | Damped springs with presets and custom config. |
| `Timeline` | Compose tweens, keyframes, and motion paths. |
| `RafDriver`, `ScrollDriver` | Drive many animations from rAF timestamps or scroll progress. |
| `TweenBatch` | Batch scalar tween evaluation. |

Motion, input, color, and DOM helpers:

| Export | Purpose |
|--------|---------|
| `MotionPath`, `MorphPath` | SVG/path following, draw values, and shape morphing. |
| `Inertia`, `Inertia2D`, `DragState`, `GestureRecognizer` | Momentum, drag tracking, and gesture recognition. |
| `ColorTween`, `interpolateColor` | Hex/rgb color interpolation in RGB, linear, Lab, or Oklab/Oklch spaces. |
| `ScrollSmoother`, `FlipAnimation`, `LayoutAnimator`, `SplitText`, `Draggable`, `Observer` | Browser DOM helpers. |

Utilities:

| Export | Purpose |
|--------|---------|
| `version()` | Return the package version. |
| `initAnimato()` | Install optional browser runtime hooks. |
| `availableEasings()` | Return easing names for picker UIs. |
| `ease(name, t)` | Evaluate one easing by name. |
| `parseEasing(name)` | Validate and normalize an easing name. |
| `snapTo(value, grid)` | Snap a scalar value. |
| `roundTo(value, decimals)` | Round a scalar value. |

## Easing Names

The parser accepts camelCase, kebab-case, and lowercase names such as
`linear`, `easeOutCubic`, `ease-in-out-back`, `steps(5)`, and
`cubicBezier(0.4, 0, 0.2, 1)`. Invalid names or malformed arguments throw a
JavaScript error with a short message.

## Examples

The v1.4.0 repository includes source examples for the common JS entry points:

```sh
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
```

Each example imports `@animato/core` directly. Dedicated framework adapter
packages such as `@animato/react` remain future scope.

## Package Checks

```sh
cargo check -p animato-js --target wasm32-unknown-unknown --all-features
bash scripts/wasm-pack-test-js.sh
bash scripts/build-js-package.sh
```

The package build rewrites the generated `wasm-pack` metadata to publish as
`@animato/core` and checks the gzipped WASM size budget.

## Related Docs

- [Installation](./installation.md)
- [Examples](./examples.md)
- [Testing](./testing.md)
- [Release](./release.md)
