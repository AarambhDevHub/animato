# Color Interpolation

Feature: `color`.

```toml
[dependencies]
animato = { version = "1.0", features = ["color"] }
```

Animato wraps `palette` color types so tweens can interpolate in perceptual or
linear-light spaces.

## Lab

```rust
use animato::{palette::Srgb, InLab, Tween, Update};

let mut tween = Tween::new(
    InLab::new(Srgb::new(1.0, 0.0, 0.0)),
    InLab::new(Srgb::new(0.0, 0.0, 1.0)),
)
.duration(1.0)
.build();

tween.update(0.5);
let color = tween.value().into_inner();
assert!(color.red >= 0.0 || color.blue >= 0.0);
```

## Spaces

| Wrapper | Use when |
|---------|----------|
| `InLab<C>` | You want perceptual midpoints with broad compatibility. |
| `InOklch<C>` | You want modern hue-aware perceptual interpolation. |
| `InLinear<C>` | You want gamma-correct sRGB blending. |

## Related Docs

- [Tween](./tween.md)
- [Recipes](./recipes.md)
