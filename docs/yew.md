# Yew Integration

Animato v1.4.0 includes `animato-yew`, a Yew 0.23 integration crate for
function components. It uses Yew `UseStateHandle<T>` values for local
animation state and schedules `requestAnimationFrame` only while an animation is
active.

## Install

```toml
[dependencies]
animato = { version = "1.4", features = ["yew-csr"] }
yew = { version = "0.23", features = ["csr"] }
```

Router and agent examples use:

```toml
yew-router = "0.20"
animato = { version = "1.4", features = ["yew-csr", "yew-agent"] }
```

## Hooks

```rust
use animato::{Easing, use_tween};
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let (x, handle) = use_tween(0.0_f32, 320.0, |builder| {
        builder.duration(0.9).easing(Easing::EaseOutCubic)
    });

    html! {
        <button onclick={Callback::from(move |_| handle.reverse())}>
            {format!("{:.0}px", *x)}
        </button>
    }
}
```

Available hooks:

| Hook | Returns |
|------|---------|
| `use_tween` | `(UseStateHandle<T>, TweenHandle<T>)` |
| `use_spring` | `(UseStateHandle<T>, SpringHandle<T>)` |
| `use_timeline` | `TimelineHandle` |
| `use_keyframes` | `(UseStateHandle<T>, KeyframeHandle<T>)` |

Yew hook names must start with `use_`. The facade also re-exports
`use_css_tween`, `use_css_spring`, and `use_route_transition_key`; compatibility
aliases named `css_tween`, `css_spring`, and `route_transition_key` are exported
for API discovery, but Yew components should call the `use_*` names.

## UI Helpers

`AnimatedStyle` builds CSS strings for tween and spring hooks:

```rust
let style = use_css_tween(
    AnimatedStyle::new().opacity(0.0).translate(0.0, 20.0),
    AnimatedStyle::new().opacity(1.0).translate(0.0, 0.0),
    0.25,
    Easing::EaseOutCubic,
);
```

Components and hooks:

| Area | API |
|------|-----|
| Presence | `PresenceAnimation`, `AnimatePresence` |
| Transitions | `TransitionMode`, `PageTransition`, `use_route_transition_key` |
| Lists | `AnimatedFor`, `AnimatedForProps`, `stable_key` |
| Scroll | `use_scroll_progress`, `use_scroll_trigger`, `use_scroll_velocity`, `SmoothScroll` |
| Gestures | `use_drag`, `use_gesture`, `use_pinch`, `use_swipe` |
| Agent | `use_animation_agent`, `AgentInput`, `AgentOutput` |

## Examples

```sh
cargo check --manifest-path examples/yew_basic_tween/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/yew_scroll_trigger/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/yew_animated_list/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/yew_drag_gesture/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/yew_page_transition/Cargo.toml --target wasm32-unknown-unknown
cargo check --manifest-path examples/yew_agent_coordination/Cargo.toml --target wasm32-unknown-unknown
```
