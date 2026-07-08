# Macro Framework Helpers

> Framework-specific helper macros for the `animato!{}` DSL (v1.7.0).

The `animato-macro` crate provides five feature-gated helper macros that
generate framework-integrated animation code. Each macro is only available
when the corresponding feature is enabled.

---

## Feature Flags

| Feature | Macro | Integration Crate |
|---------|-------|-------------------|
| `leptos` | `leptos_motion! { ... }` | `animato-leptos` |
| `dioxus` | `dioxus_motion! { ... }` | `animato-dioxus` |
| `yew` | `yew_motion! { ... }` | `animato-yew` |
| `bevy` | `bevy_motion! { ... }` | `animato-bevy` |
| `wasm` | `wasm_motion! { ... }` | `animato-wasm` |

Enable the feature on the facade:

```toml
[dependencies]
animato = { version = "1.7.0", features = ["macro", "leptos"] }
```

---

## Leptos

```rust,ignore
use animato::leptos_motion;

let animation = leptos_motion! {
    sequence {
        tween opacity: 0.0 => 1.0, duration: 0.35, easing: ease_out_cubic;
        spring scale: 0.92 => 1.0, preset: snappy;
    }
};
```

The generated code produces an animation value that you can drive with
`animato-leptos` hooks (`use_tween`, `use_spring`, `use_timeline`).

---

## Dioxus

```rust,ignore
use animato::dioxus_motion;

let animation = dioxus_motion! {
    parallel {
        tween x: 0.0 => 100.0, duration: 1.0;
        spring y: 0.0 => 50.0, preset: snappy;
    }
};
```

Pair with `animato-dioxus` hooks (`use_tween`, `use_spring`, `use_motion`).

---

## Yew

```rust,ignore
use animato::yew_motion;

let animation = yew_motion! {
    sequence {
        tween opacity: 0.0 => 1.0, duration: 0.3;
        spring scale: 0.8 => 1.0, preset: snappy;
    }
};
```

Pair with `animato-yew` hooks (`use_tween`, `use_spring`, `use_timeline`).

---

## Bevy

```rust,ignore
use animato::bevy_motion;

let animation = bevy_motion! {
    sequence {
        tween x: 0.0 => 100.0, duration: 1.0, easing: ease_out_cubic;
    }
};
```

The generated code produces an animation value that can be wrapped in
`AnimatoTween` / `AnimatoSpring` components for Bevy ECS.

---

## WASM

```rust,ignore
use animato::wasm_motion;

let animation = wasm_motion! {
    sequence {
        tween opacity: 0.0 => 1.0, duration: 0.3;
    }
};
```

Pair with `animato-wasm`'s `RafDriver` for browser `requestAnimationFrame`
loops.

---

## SSR Safety

Framework helper macros are SSR-safe where applicable. When targeting SSR
(Leptos, Yew), the macro-generated animation value resolves to its target
state immediately — no rAF loop is started until hydration completes.

---

## Combining Macros

You can mix framework helpers with the core `animato!{}` macro:

```rust,ignore
use animato::{animato, leptos_motion};

// Define a reusable animation with the core macro
let intro = animato! {
    sequence {
        tween opacity: 0.0 => 1.0, duration: 0.35, easing: ease_out_cubic;
        spring scale: 0.92 => 1.0, preset: snappy;
    }
};

// Or use the framework-specific helper for direct integration
let leptos_intro = leptos_motion! {
    sequence {
        tween opacity: 0.0 => 1.0, duration: 0.35, easing: ease_out_cubic;
        spring scale: 0.92 => 1.0, preset: snappy;
    }
};
```
