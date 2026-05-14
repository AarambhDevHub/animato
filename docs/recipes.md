# Recipes

Practical patterns for common Animato use cases.

## Fade A UI Element

```rust
use animato::{Easing, Tween, Update};

let mut opacity = Tween::new(0.0_f32, 1.0)
    .duration(0.2)
    .easing(Easing::EaseOutCubic)
    .build();
opacity.update(0.1);
let css_opacity = opacity.value();
assert!(css_opacity > 0.0);
```

## Slide And Fade Together

```rust
use animato::{At, Timeline, Tween, Update};

let mut timeline = Timeline::new()
    .add("x", Tween::new(0.0_f32, 200.0).duration(0.4).build(), At::Start)
    .add("opacity", Tween::new(0.0_f32, 1.0).duration(0.4).build(), At::Start);
timeline.play();
timeline.update(0.2);
```

## Spring To Pointer

```rust
use animato::{SpringConfig, SpringN, Update};

let mut spring = SpringN::new(SpringConfig::snappy(), [0.0_f32, 0.0]);
spring.set_target([120.0, 80.0]);
spring.update(1.0 / 60.0);
```

## Scroll Linked Progress

```rust
use animato::{Clock, ScrollClock};

let mut clock = ScrollClock::new(0.0, 1000.0);
clock.set_scroll(500.0);
let progress_delta = clock.delta();
assert_eq!(progress_delta, 0.5);
```

## Morph An SVG Shape

```rust
use animato::MorphPath;

let a = vec![[0.0_f32, 0.0], [1.0, 0.0], [1.0, 1.0]];
let b = vec![[0.0_f32, 0.5], [1.0, 0.5]];
let morph = MorphPath::with_resolution(a, b, 16);
let points = morph.evaluate(0.25);
assert_eq!(points.len(), 16);
```

## Related Docs

- [Tween](./tween.md)
- [Timeline](./timeline.md)
- [Path](./path.md)
