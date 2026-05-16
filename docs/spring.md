# Springs

Feature: `spring`.

```toml
[dependencies]
animato = "1.1"
```

Springs are physics-based animations that approach a target over time.

## One-Dimensional Spring

```rust
use animato::{Spring, SpringConfig, Update};

let mut spring = Spring::new(SpringConfig::wobbly());
spring.set_target(200.0);

while spring.update(1.0 / 60.0) {}
assert!(spring.is_settled());
```

## Presets

| Preset | Feel |
|--------|------|
| `gentle()` | Soft and controlled. |
| `wobbly()` | More overshoot. |
| `stiff()` | Fast but damped. |
| `slow()` | Heavy and slow. |
| `snappy()` | Fast UI response. |

## Multi-Dimensional Spring

```rust
use animato::{SpringConfig, SpringN, Update};

let mut spring = SpringN::new(SpringConfig::stiff(), [0.0_f32, 0.0, 0.0]);
spring.set_target([100.0, 50.0, 0.0]);
spring.update(1.0 / 60.0);
let position = spring.position();
assert!(position[0] > 0.0);
```

## Notes

- `Spring` is stack allocated.
- `SpringN<T>` uses one scalar spring per component and requires allocation.
- `snap_to` teleports without animation.
- Use RK4 only when accuracy is more important than per-step cost.

## Related Docs

- [Performance](./performance.md)
- [no_std](./no-std.md)
