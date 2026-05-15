# Concepts

Animato computes animated values. Rendering, layout, entity ownership, and DOM
updates stay in your application.

## Interpolate

`Interpolate` is the only trait a custom value needs:

```rust
use animato::Interpolate;

#[derive(Clone)]
struct Point {
    x: f32,
    y: f32,
}

impl Interpolate for Point {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
        }
    }
}
```

## Animatable

`Animatable` is implemented automatically for `Interpolate + Clone + 'static`.
Do not implement it manually.

## Update

`Update` advances state by seconds:

```rust
use animato::{Tween, Update};

let mut tween = Tween::new(0.0_f32, 1.0).duration(1.0).build();
assert!(tween.update(0.5));
```

`dt` values below zero are treated as zero by animation types.

## Easing

`Easing` transforms linear progress before interpolation. Use enum variants when
the easing must be stored, or free functions from `animato::easing` when a direct
function call is preferred.

## Clocks

Animato accepts `dt`; it does not require one clock implementation. Use:

- `WallClock` in native hosted loops.
- `MockClock` in deterministic tests.
- `ManualClock` when an external scheduler owns time.
- `RafDriver` in browsers.
- Bevy's `Time` through `AnimatoPlugin`.

## Composition

Use `Timeline` for concurrent animations and `Sequence` for back-to-back steps.
Each entry is a `Playable`, so timelines can own tweens, keyframe tracks, and
other playable animation types.

## Related Docs

- [Tween](./tween.md)
- [Timeline](./timeline.md)
- [Driver](./driver.md)
- [API Full](./api-full.md)
