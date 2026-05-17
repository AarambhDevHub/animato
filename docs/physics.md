# Input Physics

Feature: `physics`.

```toml
[dependencies]
animato = { version = "1.1", features = ["physics"] }
```

## Inertia

```rust
use animato::{Inertia, InertiaConfig, Update};

let mut inertia = Inertia::new(InertiaConfig::smooth());
inertia.kick(600.0);
inertia.update(1.0 / 60.0);
assert!(inertia.position() > 0.0);
```

## Drag

```rust
use animato::{DragConstraints, DragState, PointerData};

let mut drag = DragState::new([0.0, 0.0])
    .constraints(DragConstraints::bounded(0.0, 300.0, 0.0, 200.0));

drag.on_pointer_down(PointerData::new(0.0, 0.0, 1));
drag.on_pointer_move(PointerData::new(100.0, 25.0, 1), 1.0 / 60.0);
assert!(drag.position()[0] > 0.0);
let release = drag.on_pointer_up(PointerData::new(100.0, 25.0, 1));
assert!(release.is_some());
```

## Gestures

```rust
use animato::{Gesture, GestureRecognizer, PointerData};

let mut gestures = GestureRecognizer::default();
gestures.on_pointer_down(PointerData::new(0.0, 0.0, 1), 0.0);
let gesture = gestures.on_pointer_up(PointerData::new(80.0, 0.0, 1), 0.2);
assert!(matches!(gesture, Some(Gesture::Swipe { .. })));
```

## Notes

- `PointerData::pointer_id` keeps concurrent input streams isolated.
- Bounds clamp position and zero velocity on that axis.
- `DragState` uses velocity smoothing to avoid noisy release inertia.

## Related Docs

- [WASM](./wasm.md)
- [Recipes](./recipes.md)
