# Drivers And Clocks

Feature: `driver`.

```toml
[dependencies]
animato = "1.0"
```

## AnimationDriver

```rust
use animato::{AnimationDriver, Tween};

let mut driver = AnimationDriver::new();
let id = driver.add(Tween::new(0.0_f32, 1.0).duration(1.0).build());
driver.tick(1.0);
assert!(!driver.is_active(id));
```

## Clocks

```rust
use animato::{Clock, ManualClock};

let mut clock = ManualClock::default();
clock.advance(0.016);
assert_eq!(clock.delta(), 0.016);
assert_eq!(clock.delta(), 0.0);
```

## Scroll Driver

```rust
use animato::{ScrollClock, Clock};

let mut clock = ScrollClock::new(0.0, 1000.0);
clock.set_scroll(250.0);
assert_eq!(clock.delta(), 0.25);
```

## Choosing A Driver

| Context | Recommended source |
|---------|--------------------|
| Native app | `WallClock` or engine delta time |
| Unit test | `MockClock` |
| External scheduler | `ManualClock` |
| Browser | `RafDriver` |
| Scroll-linked UI | `ScrollClock` or `ScrollDriver` |
| Bevy | `AnimatoPlugin` |

## Related Docs

- [WASM](./wasm.md)
- [Bevy](./bevy.md)
- [Testing](./testing.md)
