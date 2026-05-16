# Paths, SVG, And Morphing

Feature: `path`.

```toml
[dependencies]
animato = { version = "1.1", features = ["path"] }
```

## Motion Path

```rust
use animato::{CubicBezierCurve, Easing, MotionPathTween, Update};

let curve = CubicBezierCurve::new(
    [0.0, 0.0],
    [40.0, 90.0],
    [140.0, -90.0],
    [200.0, 0.0],
);

let mut motion = MotionPathTween::new(curve)
    .duration(1.0)
    .easing(Easing::EaseInOutSine)
    .auto_rotate(true)
    .build();

motion.update(0.5);
let position = motion.value();
let rotation = motion.rotation_deg();
assert!(position[0] > 0.0 || rotation.is_finite());
```

## SVG Parser

```rust
use animato::{CompoundPath, PathEvaluate};

let path = CompoundPath::try_from_svg("M0 0 L100 0 L100 100").unwrap();
assert!(path.arc_length() > 100.0);
```

## Morphing

```rust
use animato::MorphPath;

let square = vec![[0.0_f32, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
let line = vec![[0.0_f32, 0.5], [1.0, 0.5]];
let morph = MorphPath::with_resolution(square, line, 8);
let midpoint = morph.evaluate(0.5);
assert_eq!(midpoint.len(), 8);
```

## Draw SVG

`DrawSvg` computes stroke dash values for path reveal animations.

```rust
use animato::{CubicBezierCurve, DrawSvg};

let path = CubicBezierCurve::new([0.0, 0.0], [1.0, 2.0], [3.0, 2.0], [4.0, 0.0]);
let values = path.draw_on(0.5);
assert!(values.dash_array > 0.0);
```

## Related Docs

- [Recipes](./recipes.md)
- [Performance](./performance.md)
- [Fuzzing in Testing](./testing.md)
