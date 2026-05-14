# Timelines

Feature: `timeline`.

```toml
[dependencies]
animato = "1.0"
```

`Timeline` composes animations by absolute or relative start time. `Sequence`
builds a timeline by placing entries back-to-back.

## Concurrent Timeline

```rust
use animato::{At, Timeline, Tween, Update};

let fade = Tween::new(0.0_f32, 1.0).duration(1.0).build();
let slide = Tween::new(0.0_f32, 100.0).duration(1.0).build();

let mut timeline = Timeline::new()
    .add("fade", fade, At::Start)
    .add("slide", slide, At::Label("fade"));

timeline.play();
timeline.update(0.5);
assert_eq!(timeline.get::<Tween<f32>>("fade").unwrap().value(), 0.5);
assert_eq!(timeline.get::<Tween<f32>>("slide").unwrap().value(), 50.0);
```

## Sequence

```rust
use animato::{Sequence, Tween, Update};

let mut timeline = Sequence::new()
    .then("intro", Tween::new(0.0_f32, 1.0).duration(0.5).build())
    .gap(0.25)
    .then("outro", Tween::new(1.0_f32, 0.0).duration(0.5).build())
    .build();

timeline.play();
timeline.update(0.5);
assert_eq!(timeline.get::<Tween<f32>>("intro").unwrap().value(), 1.0);
```

## Controls

- `play`, `pause`, `resume`, `reset`
- `seek(progress)` for normalized time
- `seek_abs(seconds)` for absolute time
- `time_scale` and `set_time_scale`
- `on_entry_complete` and `on_complete` with `std`
- `wait().await` with the `tokio` feature

## Related Docs

- [Tween](./tween.md)
- [Driver](./driver.md)
- [Recipes](./recipes.md)
