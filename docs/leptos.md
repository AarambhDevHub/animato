# Leptos Integration

Animato v1.1.0 adds `animato-leptos`, a Leptos 0.8 integration crate with
signal-backed animation hooks and browser-safe helpers.

## Install

```toml
[dependencies]
animato = { version = "1.1", features = ["leptos-csr"] }
leptos = { version = "0.8.19", features = ["csr"] }
```

Use `leptos-hydrate` or `leptos-ssr` for hydrated or server-rendered apps.
The plain `leptos` facade feature exposes the API without forcing a Leptos app
mode.

## Tween Hook

```rust
use animato::{Easing, use_tween};
use leptos::prelude::*;

#[component]
fn Box() -> impl IntoView {
    let (x, handle) = use_tween(0.0_f32, 240.0, |b| {
        b.duration(0.8).easing(Easing::EaseOutCubic)
    });

    view! {
        <div style=move || format!("transform:translateX({:.1}px);", x.get()) />
        <button on:click=move |_| handle.reverse()>"Reverse"</button>
    }
}
```

## Available APIs

| Area | API |
|------|-----|
| Hooks | `use_tween`, `use_spring`, `use_timeline`, `use_keyframes` |
| Scroll | `use_scroll_progress`, `use_scroll_trigger`, `use_scroll_velocity`, `SmoothScroll` |
| Presence | `AnimatePresence`, `PresenceAnimation` presets |
| Routes | `PageTransition`, `TransitionMode` |
| Lists | `AnimatedFor` |
| Gestures | `use_drag`, `use_gesture`, `use_pinch`, `use_swipe` |
| CSS | `AnimatedStyle`, `css_tween`, `css_spring` |
| SSR | `is_hydrating`, `use_client_only`, `SsrFallback` |

## SSR Behavior

On server and non-browser targets, hooks skip browser rAF work and expose static
values. This keeps SSR deterministic and prevents browser API access before
hydration.

## Examples

```sh
cargo check --manifest-path examples/leptos_basic_tween/Cargo.toml
cargo check --manifest-path examples/leptos_scroll_trigger/Cargo.toml
cargo check --manifest-path examples/leptos_page_transition/Cargo.toml
cargo check --manifest-path examples/leptos_animated_list/Cargo.toml
cargo check --manifest-path examples/leptos_drag_gesture/Cargo.toml
```
