# Leptos Integration

Animato v1.1.0 adds `animato-leptos`, a Leptos 0.8 integration crate with
signal-backed animation hooks and browser-safe helpers.

## Install

```toml
[dependencies]
animato = { version = "1.7.2", features = ["leptos-csr"] }
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

## Animated Lists

`AnimatedFor` applies enter animations to the initial render and newly
inserted rows, retains removed rows while their exit animation runs, and then
uses FLIP transforms to move the surviving keyed rows into place. Version 1.7.2
implements the complete enter–exit–move lifecycle while preserving the layout
and stacking controls introduced in v1.7.1.

```rust,ignore
<AnimatedFor
    each=items.into()
    key=|item: &Item| item.id
    children=|item: Item| view! { <article>{item.label}</article> }
    enter=PresenceAnimation::slide_up()
    exit=PresenceAnimation::slide_down()
    move_duration=0.35
    move_easing=Easing::EaseOutCubic
    move_delay=0.20
    stagger_delay=0.04
    gap=12.0
    item_class="relative z-0 hover:z-50"
/>
```

| Prop | Behavior |
|------|----------|
| `enter` | Presence animation used for the first render and newly inserted rows. |
| `exit` | Presence animation used while removed rows remain mounted; defaults to `enter.reversed()`. |
| `move_duration` | Duration, in seconds, of FLIP movement for existing rows. |
| `move_easing` | Easing used by FLIP movement. |
| `move_delay` | Additional delay before surviving rows start their FLIP movement after an exiting row is removed. |
| `stagger_delay` | Per-row delay added by index. |
| `gap` | Gap between generated row wrappers in pixels; defaults to `0.0`. |
| `item_class` | CSS class applied to every generated row wrapper, including stacking-context utilities. |

Removed rows are kept in the rendered list through the exit duration plus
their stagger delay. Pointer events are disabled during exit, and a generation
check prevents an old removal timer from deleting a key that was reinserted.
After removal, surviving rows are measured and animated to their new positions.

The list no longer inserts an implicit 10px gap. Applications that relied on
that spacing should set `gap=10.0` explicitly or provide spacing in their own
layout CSS.


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
