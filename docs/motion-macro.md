# Motion Macro — Declarative Animation DSL

> Introduced in **v1.7.0 — Motion Macro**.

The Motion Macro is a compile-time authoring layer that lets you describe
complex animations with readable Rust syntax. It does **not** introduce a new
runtime — every macro expansion generates normal Animato primitives
(`Tween`, `Spring`, `Timeline`, `AnimationGroup`, `KeyframeTrack`,
`MotionPathTween`).

---

## Why a Macro DSL?

Before v1.7.0, Animato was powerful but builder-heavy. Every animation
required manual `TweenBuilder`, `SpringConfig`, and `Timeline::add(...)` calls:

```rust
let mut timeline = Timeline::new();
timeline = timeline.add(
    "opacity",
    Tween::new(0.0, 1.0).duration(0.35).easing(Easing::EaseOutCubic).build(),
    At::Start,
);
```

The macro DSL lets you write the same thing declaratively:

```rust,ignore
let timeline = animato! {
    sequence {
        tween opacity: 0.0 => 1.0, duration: 0.35, easing: ease_out_cubic;
    }
};
```

Both produce identical runtime behavior. The macro is purely an ergonomic
layer.

---

## Installation

Enable the `macro` feature on the `animato` facade:

```toml
[dependencies]
animato = { version = "1.7.0", features = ["macro"] }
```

Then import the prelude for ergonomic access:

```rust,ignore
use animato::prelude::*;
```

---

## The Seven Public Macros

| Macro | Purpose |
|-------|---------|
| `animato! { ... }` | Primary declarative DSL — composes tweens, springs, keyframes, timelines, stagger, paths, colors, waveforms, presets |
| `motion! { ... }` | Alias for `animato!` focused on UI-style motion |
| `tween! { ... }` | Standalone `Tween<T>` generator |
| `spring! { ... }` | Standalone `Spring` / `SpringN<T>` generator |
| `timeline! { ... }` | Standalone `Timeline` generator |
| `keyframes! { ... }` | Standalone `KeyframeTrack<T>` generator |
| `preset! { ... }` | User-defined reusable preset generator |

---

## First Animation

### A simple tween

```rust,ignore
use animato::prelude::*;
use animato::Update;

let mut t = tween! {
    opacity: 0.0 => 1.0,
    duration: 0.4,
    easing: ease_out_cubic,
};

t.update(0.2);
println!("opacity = {}", t.value());
```

### A composed sequence

```rust,ignore
use animato::prelude::*;
use animato::Update;

let mut timeline = animato! {
    sequence {
        tween opacity: 0.0 => 1.0, duration: 0.35, easing: ease_out_cubic;
        parallel {
            tween y: 24.0 => 0.0, duration: 0.55, easing: ease_out_back;
            spring scale: 0.92 => 1.0, preset: snappy;
        }
    }
};

timeline.play();
while timeline.update(1.0 / 60.0) {}
```

### Using a built-in preset

```rust,ignore
use animato::prelude::*;
use animato::Update;

let mut enter = animato! { preset modal_enter };
enter.play();
while enter.update(1.0 / 60.0) {}
```

---

## DSL Categories

The macro supports 12 animation categories:

1. **Tween** — scalar, vector, with easing, delay, loop, snap, round, reverse
2. **Spring** — presets, explicit config, velocity, damping modes, RK4
3. **Keyframes** — percentage/seconds times, per-frame easing, loop modes
4. **Composition** — `sequence`, `parallel`, `group`, nesting, labels, `at` offsets
5. **Stagger** — linear, `grid`, `random`, `center_out`, `edges_in`
6. **Path** — SVG `d` attribute motion, `auto_rotate`, start/end offsets
7. **Morph** — shape morphing between two SVG paths with resampling
8. **Draw** — SVG draw-on animation
9. **Color** — hex/named colors, `linear`/`lab`/`oklch` spaces
10. **Waveform** — sine, sawtooth, square, triangle, noise → `KeyframeTrack<f32>`
11. **Preset** — 23 built-in presets + user-defined `preset!{}`
12. **Framework helpers** — `leptos_motion!`, `dioxus_motion!`, `yew_motion!`, `bevy_motion!`, `wasm_motion!`

---

## When to Use the Macro vs the Builder API

| Use the macro when... | Use builders when... |
|---|---|
| You want readable, intent-driven animation declarations | You need dynamic/runtime-constructed animations |
| The animation structure is known at compile time | You're integrating with a data-driven config system |
| You want compile-time validation of easing names and presets | You need maximum flexibility and minimal compile-time overhead |
| You're authoring UI transitions, intros, hero animations | You're building procedural particle systems or generative art |

The macro is **additive** — it never replaces the builder API. You can freely
mix both in the same project.

---

## Next Steps

- [Macro Reference](./macro-reference.md) — full DSL grammar and keyword list
- [Macro Recipes](./macro-recipes.md) — 20+ copy-paste recipes
- [Macro Frameworks](./macro-frameworks.md) — Leptos, Dioxus, Yew, Bevy, WASM helpers
- [Macro Expansion](./macro-expansion.md) — what code the macro generates
