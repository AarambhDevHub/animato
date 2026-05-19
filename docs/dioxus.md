# Dioxus

Animato v1.2.0 adds `animato-dioxus`, a Dioxus 0.7.9 integration crate with
signal-backed animation hooks, motion handles, scroll helpers, presence and page
transitions, list helpers, gesture state, and portable native window handles.

## Install

Choose the Animato facade feature that matches the renderer in your app:

```toml
[dependencies]
animato = { version = "1.2", features = ["dioxus-web"] }
dioxus = { version = "0.7.9", default-features = false, features = ["web", "launch"] }
```

Desktop apps use the desktop renderer and can opt into native handles:

```toml
[dependencies]
animato = { version = "1.2", features = ["dioxus-desktop", "dioxus-native"] }
dioxus = { version = "0.7.9", default-features = false, features = ["desktop", "launch"] }
```

Router-driven page transitions add `dioxus-router`:

```toml
[dependencies]
animato = { version = "1.2", features = ["dioxus-router"] }
dioxus = { version = "0.7.9", default-features = false, features = ["web", "router"] }
dioxus-router = { version = "0.7.9", default-features = false }
```

## Core Hooks

```rust
use animato::{Easing, use_tween};
use dioxus::prelude::*;

#[component]
fn App() -> Element {
    let (x, controls) = use_tween(0.0_f32, 240.0, |builder| {
        builder.duration(0.8).easing(Easing::EaseOutCubic)
    });
    let style = format!("transform: translateX({:.1}px);", *x.read());

    rsx! {
        div { style: "{style}", "Animated" }
        button { onclick: move |_| controls.reverse(), "Reverse" }
    }
}
```

`use_tween`, `use_spring`, `use_timeline`, and `use_keyframes` return Dioxus
`Signal<T>` values plus deterministic handles. Handles include methods such as
`tick`, `seek`, `reset`, `snap_to`, `play`, `pause`, and `reverse`, so tests can
advance animations without depending on real frame timing.

## Motion

`use_motion(initial)` provides a single handle for common UI animation flows:

```rust
use animato::{Easing, MotionConfig, use_motion};

let motion = use_motion(0.0_f32);
motion.animate_to(
    1.0,
    MotionConfig::Tween {
        duration: 0.25,
        easing: Easing::EaseOutCubic,
        delay: 0.0,
    },
);
```

Use `MotionConfig::Spring(config)` or `motion.spring_to(target, config)` for
spring-driven transitions, and `motion.keyframes(track)` for keyframe tracks.

## Components And Helpers

`AnimatedStyle`, `css_tween`, and `css_spring` convert numeric animation state to
CSS strings. `AnimatePresence`, `PageTransition`, `TransitionMode`, and
`AnimatedFor` provide RSX helpers for common UI patterns.

Scroll and DOM-dependent hooks are web-gated. On non-web targets,
`use_scroll_progress`, `use_scroll_trigger`, and `use_scroll_velocity` return
stable no-op signals so cross-platform code can compile unchanged.

Gesture helpers expose deterministic state handles:

```rust
use animato::use_drag;

let drag = use_drag(());
drag.snap_to([40.0, 20.0]);
```

## Platform And Native

`PlatformAdapter::detect()` reports the selected backend:

| Backend | Target |
|---------|--------|
| `AnimationBackend::WebRaf` | `wasm32` with Dioxus web |
| `AnimationBackend::NativeClock` | desktop or mobile |
| `AnimationBackend::TerminalPoll` | terminal-style builds |

Native helpers track portable window state first. Direct OS window mutation is
renderer-specific, so `WindowAnimationHandle` and `WindowSpringHandle` expose
deterministic Animato state that apps can bridge into their window layer.

## Examples

```sh
cargo check --manifest-path examples/dioxus_web_tween/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/dioxus_desktop_spring/Cargo.toml
cargo check --manifest-path examples/dioxus_cross_platform/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/dioxus_cross_platform/Cargo.toml --no-default-features --features desktop
cargo check --manifest-path examples/dioxus_tui_progress/Cargo.toml
```

## Related Docs

- [Installation](./installation.md)
- [Feature Flags](./feature-flags.md)
- [API Full](./api-full.md)
- [Testing](./testing.md)
