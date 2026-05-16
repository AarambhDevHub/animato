# Full API Map

This file lists the stable v1.1.0 API surface by crate. For signatures,
generic bounds, and exhaustive docs, use `cargo doc --workspace --all-features`
or [docs.rs/animato](https://docs.rs/animato).

Install the facade:

```toml
[dependencies]
animato = "1.1"
```

## animato-core

Purpose: foundational traits, interpolation, easing, and portable math.

Feature requirements: no facade feature required for the core re-exports.

Stable public items:

| Item | Purpose |
|------|---------|
| `Interpolate` | Implement `lerp(&self, other, t)` for custom values. |
| `Animatable` | Blanket marker for `Interpolate + Clone + 'static`. |
| `Update` | Advance state by `dt` seconds. |
| `Playable` | Object-safe animation abstraction for composition. |
| `Easing` | 38 named and parameterized easing variants plus `Custom`. |
| `easing::*` | Free easing functions such as `ease_out_cubic`. |

Example:

```rust
use animato::{Interpolate, Tween, Update};

#[derive(Clone)]
struct Size(f32);

impl Interpolate for Size {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        Self(self.0 + (other.0 - self.0) * t)
    }
}

let mut tween = Tween::new(Size(10.0), Size(20.0)).duration(1.0).build();
tween.update(0.5);
assert_eq!(tween.value().0, 15.0);
```

## animato-tween

Feature: `tween`.

Stable public items:

| Item | Purpose |
|------|---------|
| `Tween<T>` | Single-value animation. |
| `TweenBuilder<T>` | Consuming builder returned by `Tween::new`. |
| `TweenState` | `Idle`, `Running`, `Paused`, `Completed`. |
| `TweenSnapshot` | Batch-friendly read-only state view. |
| `Loop` | `Once`, `Times`, `Forever`, `PingPong`. |
| `Keyframe<T>` | A timed value with easing to the next frame. |
| `KeyframeTrack<T>` | Multi-stop animation track. |
| `snap_to` | Snap a numeric value to a grid. |
| `round_to` | Round a numeric value to decimal places. |

Example:

```rust
use animato::{Easing, Loop, Tween, Update};

let mut tween = Tween::new(0.0_f32, 100.0)
    .duration(1.0)
    .easing(Easing::EaseInOutSine)
    .looping(Loop::PingPong)
    .build();

tween.update(0.5);
assert!(tween.value() > 0.0);
```

## animato-timeline

Feature: `timeline`.

Stable public items:

| Item | Purpose |
|------|---------|
| `Timeline` | Concurrent animation composition. |
| `TimelineState` | Playback state. |
| `At` | Entry placement: start, end, label, offset, absolute. |
| `Sequence` | Sequential timeline builder. |
| `stagger` | Build offset starts for a collection of animations. |

Example:

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

## animato-spring

Feature: `spring`.

Stable public items:

| Item | Purpose |
|------|---------|
| `Spring` | One-dimensional damped spring. |
| `SpringN<T>` | Component spring for decomposable values. |
| `SpringConfig` | Stiffness, damping, mass, epsilon presets. |
| `Integrator` | Semi-implicit Euler or RK4. |
| `Decompose` | Sealed component decomposition trait. |

Example:

```rust
use animato::{Spring, SpringConfig, Update};

let mut spring = Spring::new(SpringConfig::snappy());
spring.set_target(1.0);
while spring.update(1.0 / 60.0) {}
assert!(spring.is_settled());
```

## animato-path

Feature: `path`.

Stable public items:

| Item | Purpose |
|------|---------|
| `PathEvaluate` | Position, tangent, rotation, length. |
| `QuadBezier`, `CubicBezierCurve` | Bezier path primitives. |
| `CatmullRomSpline`, `PolyPath` | Smooth point paths. |
| `LineSegment`, `EllipticalArc`, `PathSegment` | Compound path segments. |
| `PathCommand`, `SvgPathParser`, `SvgPathError` | SVG path parser. |
| `MotionPath`, `MotionPathTween`, `MotionPathTweenBuilder` | Path animation. |
| `MorphPath`, `resample` | Shape morphing. |
| `DrawSvg`, `DrawValues` | Stroke draw animation values. |

Example:

```rust
use animato::{CubicBezierCurve, MotionPathTween, Update};

let curve = CubicBezierCurve::new([0.0, 0.0], [50.0, 100.0], [150.0, -100.0], [200.0, 0.0]);
let mut motion = MotionPathTween::new(curve).duration(1.0).build();
motion.update(0.5);
let [x, _y] = motion.value();
assert!(x > 0.0);
```

## animato-physics

Feature: `physics`.

Stable public items:

| Item | Purpose |
|------|---------|
| `Inertia`, `InertiaN<T>` | Friction deceleration after input. |
| `InertiaConfig`, `InertiaBounds` | Inertia tuning and bounds. |
| `PointerData` | Pointer sample. |
| `DragAxis`, `DragConstraints`, `DragState` | Drag tracking and release velocity. |
| `GestureConfig`, `GestureRecognizer`, `Gesture`, `SwipeDirection` | Gesture recognition. |

Example:

```rust
use animato::{Inertia, InertiaConfig, Update};

let mut inertia = Inertia::new(InertiaConfig::smooth());
inertia.kick(400.0);
inertia.update(1.0 / 60.0);
assert!(inertia.position() > 0.0);
```

## animato-color

Feature: `color`.

Stable public items:

| Item | Purpose |
|------|---------|
| `InLab<C>` | Interpolate through CIE Lab. |
| `InOklch<C>` | Interpolate through Oklch. |
| `InLinear<C>` | Interpolate in linear light. |
| `palette` | Re-export of the `palette` crate. |

Example:

```rust
use animato::{palette::Srgb, InLab, Tween, Update};

let mut tween = Tween::new(
    InLab::new(Srgb::new(1.0, 0.0, 0.0)),
    InLab::new(Srgb::new(0.0, 0.0, 1.0)),
)
.duration(1.0)
.build();
tween.update(0.5);
let midpoint = tween.value().into_inner();
assert!(midpoint.red > 0.0 || midpoint.blue > 0.0);
```

## animato-driver

Feature: `driver`.

Stable public items:

| Item | Purpose |
|------|---------|
| `AnimationDriver`, `AnimationId` | Own and tick many `Update` values. |
| `Clock` | Abstract source of frame delta. |
| `WallClock`, `ManualClock`, `MockClock` | Hosted, manual, and test clocks. |
| `ScrollDriver`, `ScrollClock` | Drive animations from scroll position. |

Example:

```rust
use animato::{AnimationDriver, Easing, Tween};

let mut driver = AnimationDriver::new();
let id = driver.add(Tween::new(0.0_f32, 1.0).duration(1.0).easing(Easing::Linear).build());
driver.tick(1.0);
assert!(!driver.is_active(id));
```

## animato-gpu

Feature: `gpu`.

Stable public items:

| Item | Purpose |
|------|---------|
| `GpuAnimationBatch` | Batch `Tween<f32>` values. |
| `GpuBackend` | Active backend, GPU or CPU fallback. |
| `GpuBatchError` | GPU initialization and dispatch errors. |

Example:

```rust
use animato::{GpuAnimationBatch, Tween};

let mut batch = GpuAnimationBatch::new_cpu();
batch.push(Tween::new(0.0_f32, 1.0).duration(1.0).build());
batch.tick(0.5);
assert_eq!(batch.read_back().len(), 1);
```

## animato-bevy

Feature: `bevy`.

Stable public items include `AnimatoPlugin`, `AnimatoTween<T>`,
`AnimatoSpring<T>`, `AnimationChannel`, `AnimationLabel`, `TweenCompleted`,
`SpringSettled`, `AnimatoTweenPlugin<T>`, `AnimatoSpringPlugin<T>`, and
`AnimatoSet`.

Use [bevy.md](./bevy.md) for complete setup because examples require Bevy's app
runtime and transform components.

## animato-wasm

Features: `wasm`, optional `wasm-dom`.

Stable public items include `RafDriver`, `ScrollSmoother`, and with `wasm-dom`:
`FlipState`, `FlipAnimation`, `LayoutAnimator`, `SharedElementTransition`,
`SplitText`, `SplitMode`, `Draggable`, `Observer`, and `ObserverEvent`.

Use [wasm.md](./wasm.md) for browser setup.

## animato-leptos

Features: `leptos`, plus one app mode feature such as `leptos-csr`,
`leptos-hydrate`, or `leptos-ssr`.

Stable public items include `use_tween`, `use_spring`, `use_timeline`,
`use_keyframes`, `TweenHandle`, `SpringHandle`, `TimelineHandle`,
`KeyframeHandle`, `use_scroll_progress`, `use_scroll_trigger`,
`use_scroll_velocity`, `SmoothScroll`, `AnimatePresence`, `PresenceAnimation`,
`PageTransition`, `TransitionMode`, `AnimatedFor`, `use_drag`, `use_gesture`,
`use_pinch`, `use_swipe`, `AnimatedStyle`, `css_tween`, `css_spring`,
`is_hydrating`, `use_client_only`, and `SsrFallback`.

Use [leptos.md](./leptos.md) for app setup and examples.

## Related Docs

- [Feature Flags](./feature-flags.md)
- [Getting Started](./getting-started.md)
- [Examples](./examples.md)
- [Testing](./testing.md)
