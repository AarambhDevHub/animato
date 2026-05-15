# Tweens And Keyframes

Feature: `tween`.

```toml
[dependencies]
animato = "1.0"
```

## Tween

`Tween<T>` animates one value from `start` to `end`.

```rust
use animato::{Easing, Tween, Update};

let mut tween = Tween::new(0.0_f32, 100.0)
    .duration(1.0)
    .delay(0.1)
    .easing(Easing::EaseOutCubic)
    .build();

tween.update(0.6);
assert!(tween.value() > 0.0);
```

## Control

```rust
use animato::{Tween, Update};

let mut tween = Tween::new(0.0_f32, 1.0).duration(1.0).build();
tween.seek(0.5);
assert_eq!(tween.value(), 0.5);
tween.pause();
assert!(!tween.update(1.0));
tween.resume();
```

## Looping

```rust
use animato::{Loop, Tween, Update};

let mut tween = Tween::new(0.0_f32, 1.0)
    .duration(1.0)
    .looping(Loop::Times(2))
    .build();

tween.update(1.5);
assert!(!tween.is_complete());
```

## Keyframe Track

```rust
use animato::{Easing, KeyframeTrack, Update};

let mut track = KeyframeTrack::new()
    .push_eased(0.0, 0.0_f32, Easing::EaseOutCubic)
    .push(1.0, 100.0);

track.update(0.5);
assert!(track.value().unwrap() > 50.0);
```

## Notes

- `duration <= 0.0` completes immediately.
- Negative `dt` is clamped to zero.
- `value()` does not allocate.
- `KeyframeTrack` uses allocation and is not stack-only.

## Related Docs

- [Timeline](./timeline.md)
- [Performance](./performance.md)
- [API Full](./api-full.md)
