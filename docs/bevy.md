# Bevy Integration

Feature: `bevy`.

```toml
[dependencies]
animato = { version = "1.1", features = ["bevy"] }
```

Animato targets Bevy 0.18.1 through lightweight wrapper components and systems.

## Plugin Shape

```rust,ignore
use animato::{AnimatoPlugin, AnimatoTween, Easing, Tween};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AnimatoPlugin)
        .add_systems(Startup, spawn)
        .run();
}

fn spawn(mut commands: Commands) {
    commands.spawn((
        Transform::default(),
        AnimatoTween::translation(
            Tween::new([0.0_f32, 0.0, 0.0], [300.0, 0.0, 0.0])
                .duration(1.0)
                .easing(Easing::EaseOutBack)
                .build(),
        ),
    ));
}
```

The snippet is ignored in doctests because the workspace depends on modular
Bevy crates, while most user apps use the full `bevy` crate.

## Stable Types

- `AnimatoPlugin`
- `AnimatoTween<T>`
- `AnimatoSpring<T>`
- `TweenCompleted`
- `SpringSettled`
- `AnimationLabel`
- `AnimationChannel`
- `AnimatoSet`

## Testing

The repository includes integration tests that build a `bevy_app::App`, insert
components, advance `Time`, and assert completion messages and transform values.

## Related Docs

- [Examples](./examples.md)
- [Driver](./driver.md)
