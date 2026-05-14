# Getting Started

This guide creates a first animation and shows how Animato fits into an app
loop. It uses only the default facade features.

## Install

```toml
[dependencies]
animato = "1.0"
```

## First Tween

```rust
use animato::{Easing, Tween, Update};

let mut opacity = Tween::new(0.0_f32, 1.0)
    .duration(0.3)
    .easing(Easing::EaseOutCubic)
    .build();

opacity.update(0.15);
let current_opacity = opacity.value();
assert!(current_opacity > 0.0 && current_opacity < 1.0);
```

## App Loop Shape

Animato does not own the loop:

```rust
use animato::{Easing, MockClock, Tween, Update, Clock};

let mut clock = MockClock::new(1.0 / 60.0);
let mut x = Tween::new(0.0_f32, 300.0)
    .duration(1.0)
    .easing(Easing::EaseInOutSine)
    .build();

while !x.is_complete() {
    x.update(clock.delta());
    let render_x = x.value();
    assert!(render_x >= 0.0);
}
```

In a real app, replace `MockClock` with `WallClock`, Bevy's `Time`, browser
`requestAnimationFrame`, or your own delta source.

## What To Read Next

- [Concepts](./concepts.md) for traits and data flow.
- [Feature Flags](./feature-flags.md) for optional integrations.
- [Tween Guide](./tween.md) for keyframes, looping, pause, seek, and reverse.
- [Examples](./examples.md) for runnable examples.
