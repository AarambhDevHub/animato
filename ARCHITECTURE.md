# Animato — Full Project Architecture

> *Italian: animato — animated, lively, with life and movement.*
>
> A professional-grade, renderer-agnostic animation library for Rust.  
> Zero mandatory dependencies. `no_std`-ready. Built as a clean Cargo workspace.
> Designed for TUIs, Web (WASM), Bevy, embedded targets, and everything in between.

---

## Table of Contents

1. [Project Vision](#1-project-vision)
2. [Why a Workspace — Not a Single Crate](#2-why-a-workspace-not-a-single-crate)
3. [Workspace Layout](#3-workspace-layout)
4. [Crate-by-Crate Specification](#4-crate-by-crate-specification)
   - 4.1 [animato-core](#41-animato-core)
   - 4.2 [animato-tween](#42-animato-tween)
   - 4.3 [animato-timeline](#43-animato-timeline)
   - 4.4 [animato-spring](#44-animato-spring)
   - 4.5 [animato-path](#45-animato-path)
   - 4.6 [animato-physics](#46-animato-physics)
   - 4.7 [animato-color](#47-animato-color)
   - 4.8 [animato-driver](#48-animato-driver)
   - 4.9 [animato-gpu](#49-animato-gpu)
   - 4.10 [animato-bevy](#410-animato-bevy)
   - 4.11 [animato-wasm](#411-animato-wasm)
   - 4.12 [animato-leptos](#412-animato-leptos)
   - 4.13 [animato-dioxus](#413-animato-dioxus)
   - 4.14 [animato-yew](#414-animato-yew)
   - 4.15 [animato-js](#415-animato-js)
   - 4.16 [animato (facade)](#416-animato-facade)
5. [Data Flow & Runtime Loop](#5-data-flow--runtime-loop)
6. [Type System Design](#6-type-system-design)
7. [Feature Flag Strategy](#7-feature-flag-strategy)
8. [Error Handling Strategy](#8-error-handling-strategy)
9. [Testing Strategy](#9-testing-strategy)
10. [Performance Guidelines](#10-performance-guidelines)
11. [Integration Targets](#11-integration-targets)
12. [CI / CD Pipeline](#12-ci--cd-pipeline)
13. [Publishing Checklist](#13-publishing-checklist)
14. [Naming & Style Conventions](#14-naming--style-conventions)

---

## 1. Project Vision

Animato is built around one principle: **any value that can be linearly interpolated can be animated.**

Everything else — easing curves, keyframe tracks, timelines, spring physics, motion paths, GPU batching — is layered cleanly on top of that single primitive. Each layer lives in its own crate, carries its own `Cargo.toml`, and can be used standalone or composed with others.

### Design Goals

| Goal | Decision |
|------|----------|
| Zero mandatory dependencies | Core is pure Rust math with no external crates |
| `no_std` support | `animato-core`, `animato-tween`, `animato-spring` are fully `no_std` |
| Clean crate boundaries | Each concern lives in its own crate — not one giant `src/` |
| Composable, not monolithic | Use only the crates you need |
| Ergonomic public API | Builder pattern on every complex type |
| Type-safe animation targets | Generic over `T: Animatable` throughout |
| Testable without a real clock | `Clock` trait with `MockClock` for deterministic tests |
| Serializable state | Optional `serde` feature, never forced |
| Discoverable | One facade crate (`animato`) re-exports everything |

### Non-Goals

- Animato does **NOT** render anything. It computes values; the caller renders.
- Animato does **NOT** own a game loop. It accepts a `dt` tick; the caller drives it.
- Animato does **NOT** manage scene graphs or entity hierarchies (Bevy handles that).

---

## 2. Why a Workspace — Not a Single Crate

Spanda grew into a flat `src/` with 25+ files and no clear internal boundaries.
Animato solves this with a Cargo workspace from day one.

**Benefits:**

- **Compile-time isolation.** Changes to `animato-path` do not recompile `animato-core`.
- **Clear ownership.** Each crate has one job. A contributor opening `animato-spring` only needs to understand springs.
- **Granular dependencies.** A user who only needs tweening adds `animato-tween`. They never download wgpu or bevy.
- **Parallel compilation.** Cargo compiles independent crates in parallel across CPU cores.
- **Separate versioning.** `animato-gpu` can be `0.1.0` while `animato-core` reaches `1.0.0`.

---

## 3. Workspace Layout

```
animato/
├── Cargo.toml                          ← workspace root (no [lib] here)
├── README.md
├── ARCHITECTURE.md                     ← this file
├── ROADMAP.md
├── CHANGELOG.md
├── CONTRIBUTING.md
├── LICENSE-MIT
├── LICENSE-APACHE
│
├── .github/
│   ├── workflows/
│   │   ├── ci.yml                      ← lint, test, no_std check, WASM build
│   │   └── publish.yml                 ← cargo publish on version tag
│   └── ISSUE_TEMPLATE/
│       ├── bug_report.md
│       └── feature_request.md
│
├── crates/
│   ├── animato-core/                     ← traits, easing, interpolation (no_std)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── traits.rs               ← Interpolate, Animatable, Update
│   │       └── easing.rs               ← Easing enum + 38+ functions
│   │
│   ├── animato-tween/                    ← Tween<T>, KeyframeTrack<T>, Loop
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── tween.rs
│   │       ├── builder.rs              ← TweenBuilder<T>
│   │       └── keyframe.rs             ← KeyframeTrack<T>, Keyframe<T>
│   │
│   ├── animato-timeline/                 ← Timeline, Sequence, At, stagger
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── timeline.rs
│   │       ├── sequence.rs
│   │       └── stagger.rs
│   │
│   ├── animato-spring/                   ← Spring, SpringN<T>, SpringConfig (no_std)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── spring.rs
│   │       └── config.rs
│   │
│   ├── animato-path/                     ← motion paths, Bezier, SVG, morphing
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── bezier.rs               ← quadratic, cubic Bezier + CatmullRom
│   │       ├── motion.rs               ← MotionPath, MotionPathTween
│   │       ├── poly.rs                 ← PolyPath, CompoundPath (arc-length param)
│   │       ├── morph.rs                ← MorphPath + auto-resample
│   │       └── svg.rs                  ← SvgPathParser (d-attribute)
│   │
│   ├── animato-physics/                  ← Inertia, DragState, GestureRecognizer
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── inertia.rs
│   │       ├── drag.rs
│   │       └── gesture.rs
│   │
│   ├── animato-color/                    ← perceptual color interpolation
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       └── spaces.rs               ← InLab, InOklch, InLinear wrappers
│   │
│   ├── animato-driver/                   ← AnimationDriver, Clock, ScrollDriver
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── driver.rs
│   │       ├── clock.rs
│   │       └── scroll.rs
│   │
│   ├── animato-gpu/                      ← GpuAnimationBatch via wgpu
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── batch.rs
│   │       └── shaders/
│   │           └── tween.wgsl
│   │
│   ├── animato-bevy/                     ← SpandaPlugin → AnimatoPlugin for Bevy
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── plugin.rs
│   │       └── systems.rs
│   │
│   ├── animato-wasm/                     ← WASM + DOM integrations
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── raf.rs                  ← requestAnimationFrame driver
│   │       ├── flip.rs                 ← FLIP layout transitions
│   │       ├── split_text.rs
│   │       ├── scroll_smoother.rs
│   │       ├── draggable.rs
│   │       └── observer.rs
│   │
│   ├── animato-leptos/                   ← Leptos signal-based animation hooks (v1.1.0)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── hooks.rs               ← use_tween, use_spring, use_timeline, use_keyframes
│   │       ├── scroll.rs              ← use_scroll_progress, use_scroll_trigger, SmoothScroll
│   │       ├── presence.rs            ← AnimatePresence mount/unmount transitions
│   │       ├── transition.rs          ← PageTransition route-change wrapper
│   │       ├── list.rs                ← AnimatedFor FLIP list reordering
│   │       ├── gesture.rs             ← use_drag, use_gesture, use_pinch
│   │       ├── css.rs                 ← AnimatedStyle, CSS property helpers
│   │       └── ssr.rs                 ← SSR-aware guards, hydration helpers
│   │
│   ├── animato-dioxus/                   ← Dioxus cross-platform animation hooks (v1.2.0)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── hooks.rs               ← use_tween, use_spring, use_timeline, use_keyframes
│   │       ├── motion.rs              ← use_motion all-in-one hook
│   │       ├── scroll.rs              ← use_scroll_progress, use_scroll_trigger
│   │       ├── presence.rs            ← AnimatePresence mount/unmount transitions
│   │       ├── transition.rs          ← PageTransition route-change wrapper
│   │       ├── list.rs                ← AnimatedFor FLIP list reordering
│   │       ├── gesture.rs             ← use_drag, use_gesture, use_pinch
│   │       ├── platform.rs            ← platform-adaptive animation (web/desktop/mobile/TUI)
│   │       └── native.rs              ← native window animation helpers (desktop/mobile)
│   │
│   ├── animato-yew/                      ← Yew component-based animation hooks (v1.3.0)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── hooks.rs               ← use_tween, use_spring, use_timeline, use_keyframes
│   │       ├── scroll.rs              ← use_scroll_progress, use_scroll_trigger
│   │       ├── presence.rs            ← AnimatePresence mount/unmount transitions
│   │       ├── transition.rs          ← PageTransition route-change wrapper
│   │       ├── list.rs                ← AnimatedFor FLIP list reordering
│   │       ├── gesture.rs             ← use_drag, use_gesture, use_pinch
│   │       ├── agent.rs               ← AnimationAgent for message-based coordination
│   │       └── css.rs                 ← AnimatedStyle, CSS property helpers
│   │
│   ├── animato-js/                       ← WASM-to-NPM bindings for JS frameworks (v1.4.0)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── tween.rs               ← JsTween — wasm_bindgen wrapper
│   │       ├── spring.rs              ← JsSpring — wasm_bindgen wrapper
│   │       ├── timeline.rs            ← JsTimeline — wasm_bindgen wrapper
│   │       ├── keyframe.rs            ← JsKeyframeTrack — wasm_bindgen wrapper
│   │       ├── driver.rs              ← JsRafDriver — rAF-based animation loop
│   │       ├── easing.rs              ← easing name parser (string → Easing enum)
│   │       └── path.rs                ← JsMotionPath — wasm_bindgen wrapper
│   │
│   └── animato/                          ← facade crate — the one users add to Cargo.toml
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs                  ← pub use everything from every sub-crate
│
├── examples/
│   ├── basic_tween.rs
│   ├── spring_demo.rs
│   ├── keyframe_track.rs
│   ├── timeline_sequence.rs
│   ├── motion_path.rs
│   ├── physics_drag.rs
│   ├── color_animation.rs
│   ├── tui_progress.rs
│   ├── tui_spinner.rs
│   └── wasm_counter/                   ← wasm-pack example project
│       ├── src/lib.rs
│       └── www/index.html
│
├── benches/
│   ├── easing_bench.rs
│   ├── tween_update_bench.rs
│   ├── spring_bench.rs
│   ├── path_bench.rs
│   └── physics_bench.rs
│
└── tests/
    ├── tween_lifecycle.rs
    ├── spring_settles.rs
    ├── keyframe_looping.rs
    ├── timeline_sequence.rs
    └── physics_input.rs
```

### Root `Cargo.toml`

```toml
[workspace]
resolver = "2"
members = [
    "crates/animato-core",
    "crates/animato-tween",
    "crates/animato-timeline",
    "crates/animato-spring",
    "crates/animato-path",
    "crates/animato-physics",
    "crates/animato-color",
    "crates/animato-driver",
    "crates/animato-gpu",
    "crates/animato-bevy",
    "crates/animato-wasm",
    "crates/animato-leptos",
    "crates/animato-dioxus",
    "crates/animato-yew",
    "crates/animato-js",
    "crates/animato",
]

[workspace.package]
version      = "1.0.0"
edition      = "2024"
license      = "MIT OR Apache-2.0"
repository   = "https://github.com/AarambhDevHub/animato"
authors      = ["Aarambh Dev Hub"]
rust-version = "1.89"

[workspace.dependencies]
# internal crates — version pinned to workspace
animato-core     = { path = "crates/animato-core",     version = "1.0" }
animato-tween    = { path = "crates/animato-tween",    version = "1.0" }
animato-timeline = { path = "crates/animato-timeline", version = "1.0" }
animato-spring   = { path = "crates/animato-spring",   version = "1.0" }
animato-path     = { path = "crates/animato-path",     version = "1.0" }
animato-physics  = { path = "crates/animato-physics",  version = "1.0" }
animato-color    = { path = "crates/animato-color",    version = "1.0" }
animato-driver   = { path = "crates/animato-driver",   version = "1.0" }
animato-gpu      = { path = "crates/animato-gpu",      version = "1.0" }
animato-bevy     = { path = "crates/animato-bevy",     version = "1.0" }
animato-wasm     = { path = "crates/animato-wasm",     version = "1.0" }
animato-leptos   = { path = "crates/animato-leptos",   version = "1.1" }
animato-dioxus   = { path = "crates/animato-dioxus",   version = "1.2" }
animato-yew      = { path = "crates/animato-yew",      version = "1.3" }
animato-js       = { path = "crates/animato-js",       version = "1.4" }

# external crates — shared version pins
serde        = { version = "1",    features = ["derive"] }
palette      = { version = "0.7", default-features = false, features = ["libm"] }
wasm-bindgen = { version = "0.2" }
js-sys       = { version = "0.3" }
web-sys      = { version = "0.3" }
wgpu         = { version = "29.0.3" }
bytemuck     = { version = "1.25", features = ["derive"] }
pollster     = { version = "0.4" }
tokio        = { version = "1",    features = ["sync"] }
bevy_app     = { version = "0.18.1", default-features = false }
bevy_ecs     = { version = "0.18.1", default-features = false }
bevy_math    = { version = "0.18.1", default-features = false, features = ["libm"] }
bevy_time    = { version = "0.18.1", default-features = false }
bevy_transform = { version = "0.18.1", default-features = false, features = ["bevy-support", "libm"] }
approx       = { version = "0.5" }
criterion    = { version = "0.5",  features = ["html_reports"] }
leptos       = { version = "0.7" }
leptos_router = { version = "0.7" }
dioxus       = { version = "0.7" }
dioxus-router = { version = "0.7" }
yew          = { version = "0.21" }
yew-router   = { version = "0.18" }
```

---

## 4. Crate-by-Crate Specification

---

### 4.1 `animato-core`

**Responsibility:** Core traits and the easing system. This is the foundation every other crate builds on. Must compile in `no_std` environments with zero external dependencies.

**Dependency rule:** This crate depends on NOTHING except `libcore`.

#### `src/traits.rs`

```rust
// The only thing a user-defined type needs:
pub trait Interpolate: Sized {
    fn lerp(&self, other: &Self, t: f32) -> Self;
}

// Blanket impl — never implement this manually:
pub trait Animatable: Interpolate + Clone + 'static {}
impl<T: Interpolate + Clone + 'static> Animatable for T {}

// Implemented by Tween, Timeline, Spring, KeyframeTrack — the driver calls this:
pub trait Update {
    /// Advance the animation by `dt` seconds.
    /// Returns `true` while still running, `false` when complete.
    fn update(&mut self, dt: f32) -> bool;
}

// Used by Timeline and other composition containers:
pub trait Playable: Update + core::any::Any {
    fn duration(&self) -> f32;
    fn reset(&mut self);
    fn seek_to(&mut self, progress: f32);
    fn is_complete(&self) -> bool;
    fn as_any(&self) -> &dyn core::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn core::any::Any;
}
```

**Blanket `Interpolate` implementations shipped in `animato-core`:**

| Type | Behavior |
|------|----------|
| `f32` | Direct lerp |
| `f64` | Casts `t` to `f64`, full precision lerp |
| `[f32; 2]` | Per-component lerp |
| `[f32; 3]` | Per-component lerp |
| `[f32; 4]` | Per-component lerp |
| `i32` | Lerps as `f32`, rounds to nearest |
| `u8` | Lerps as `f32`, clamps to `[0, 255]` |

#### `src/easing.rs`

All 38 shipped easing variants are exposed as:
1. `Easing` enum with `.apply(t: f32) -> f32` — storable, passable, optionally serializable
2. Free `#[inline] pub fn ease_out_cubic(t: f32) -> f32` — zero-overhead direct calls

```rust
#[derive(Clone, Debug, PartialEq)]
pub enum Easing {
    // Linear
    Linear,

    // Polynomial (Quad, Cubic, Quart, Quint — 12 variants)
    EaseInQuad, EaseOutQuad, EaseInOutQuad,
    EaseInCubic, EaseOutCubic, EaseInOutCubic,
    EaseInQuart, EaseOutQuart, EaseInOutQuart,
    EaseInQuint, EaseOutQuint, EaseInOutQuint,

    // Sinusoidal (3 variants)
    EaseInSine, EaseOutSine, EaseInOutSine,

    // Exponential (3 variants)
    EaseInExpo, EaseOutExpo, EaseInOutExpo,

    // Circular (3 variants)
    EaseInCirc, EaseOutCirc, EaseInOutCirc,

    // Back — overshoot (3 variants)
    EaseInBack, EaseOutBack, EaseInOutBack,

    // Elastic — spring-like oscillation (3 variants)
    EaseInElastic, EaseOutElastic, EaseInOutElastic,

    // Bounce — ball bouncing to rest (3 variants)
    EaseInBounce, EaseOutBounce, EaseInOutBounce,

    // CSS-compatible
    CubicBezier(f32, f32, f32, f32),   // (x1, y1, x2, y2)
    Steps(u32),                        // CSS steps()

    // Advanced parameterized (v0.8.0)
    RoughEase { strength: f32, points: u32 },
    SlowMo { linear_ratio: f32, power: f32 },
    Wiggle { wiggles: u32 },
    CustomBounce { strength: f32 },

    // Escape hatch — function pointer (serde-skipped)
    Custom(fn(f32) -> f32),
}

impl Easing {
    pub fn apply(&self, t: f32) -> f32 { /* match dispatch */ }
    pub fn all_named() -> &'static [Easing] { /* for picker UIs / test sweeps */ }
}
```

**Invariants enforced in the test suite:**
- `apply(0.0) == 0.0` for all named variants
- `apply(1.0) == 1.0` for all named variants
- `apply(t)` with `t` outside `[0, 1]` does not panic — `t` is clamped internally

#### `Cargo.toml`

```toml
[package]
name        = "animato-core"
description = "Core traits and easing system for the Animato animation library."
# inherits workspace.package fields

[features]
default = []
std     = []
serde   = ["dep:serde"]

[dependencies]
serde = { workspace = true, optional = true }
```

---

### 4.2 `animato-tween`

**Responsibility:** `Tween<T>` (single-value animation) and `KeyframeTrack<T>` (multi-stop animation). The bread-and-butter of the library.

**Depends on:** `animato-core`

#### `src/tween.rs`

```rust
pub struct Tween<T: Animatable> {
    pub start:    T,
    pub end:      T,
    pub duration: f32,       // seconds
    pub easing:   Easing,
    pub delay:    f32,       // pre-animation hold in seconds
    pub time_scale: f32,     // 1.0 = normal, 2.0 = double speed, 0.5 = half
    pub looping:  Loop,
    elapsed:      f32,       // private — managed by Update::update()
    state:        TweenState,
    loop_count:   u32,       // tracks current loop iteration
    #[cfg(feature = "std")]
    callbacks:    TweenCallbacks<T>,
}

pub enum TweenState {
    Idle,        // not yet started (delay period)
    Running,
    Paused,
    Completed,
}

pub enum Loop {
    Once,
    Times(u32),
    Forever,
    PingPong,    // plays forward then backward, repeatedly
}
```

**Builder — the primary construction API:**

```rust
// Users never call Tween { .. } directly — always via TweenBuilder:
let tween = Tween::new(0.0_f32, 100.0)
    .duration(1.5)
    .easing(Easing::EaseOutCubic)
    .delay(0.2)
    .time_scale(1.0)
    .looping(Loop::PingPong)
    .build();
```

**Key methods:**

```rust
impl<T: Animatable> Tween<T> {
    pub fn value(&self) -> T;            // current interpolated value
    pub fn progress(&self) -> f32;       // 0.0..=1.0 raw progress (before easing)
    pub fn eased_progress(&self) -> f32; // 0.0..=1.0 after easing applied
    pub fn is_complete(&self) -> bool;
    pub fn reset(&mut self);
    pub fn seek(&mut self, t: f32);      // jump to normalized time t ∈ [0, 1]
    pub fn reverse(&mut self);           // swap start/end in place
    pub fn pause(&mut self);
    pub fn resume(&mut self);

    // std feature only:
    pub fn on_start(self, f: impl FnMut() + 'static) -> Self;
    pub fn on_update(self, f: impl FnMut(&T) + 'static) -> Self;
    pub fn on_complete(self, f: impl FnMut() + 'static) -> Self;
}
```

**`Update` implementation:**

```rust
impl<T: Animatable> Update for Tween<T> {
    fn update(&mut self, dt: f32) -> bool {
        // 1. Apply time_scale to dt
        // 2. Drain delay bucket if in Idle state
        // 3. Advance elapsed by scaled_dt
        // 4. Handle loop boundary — reset or reverse on overflow
        // 5. Clamp elapsed to duration for Once
        // 6. Transition to Completed when loop_count is exhausted
        // 7. Fire callbacks (std only)
        // 8. Return state != Completed
    }
}
```

**Value computation (hot path — keep simple):**

```rust
pub fn value(&self) -> T {
    let raw_t    = (self.elapsed / self.duration).clamp(0.0, 1.0);
    let curved_t = self.easing.apply(raw_t);
    self.start.lerp(&self.end, curved_t)
}
```

**Value modifiers (free functions, not methods, to keep Tween<T> small):**

```rust
pub fn snap_to<T: Animatable + Into<f32> + From<f32>>(value: T, grid: f32) -> T;
pub fn round_to<T: Animatable + Into<f32> + From<f32>>(value: T, decimals: u32) -> T;
```

#### `src/keyframe.rs`

```rust
pub struct Keyframe<T: Animatable> {
    pub time:   f32,      // seconds from track start
    pub value:  T,
    pub easing: Easing,   // easing used from THIS keyframe to the NEXT
}

pub struct KeyframeTrack<T: Animatable> {
    frames:      Vec<Keyframe<T>>,   // sorted by time — invariant maintained by push
    elapsed:     f32,
    pub looping: Loop,
    loop_count:  u32,
}

impl<T: Animatable> KeyframeTrack<T> {
    pub fn new() -> Self;
    pub fn push(self, time: f32, value: T) -> Self;
    pub fn push_eased(self, time: f32, value: T, easing: Easing) -> Self;
    pub fn looping(self, mode: Loop) -> Self;

    pub fn value_at(&self, t: f32) -> Option<T>; // None when there are no frames
    pub fn value(&self) -> Option<T>;      // current value based on elapsed
    pub fn duration(&self) -> f32;         // time of the last keyframe
    pub fn is_complete(&self) -> bool;
}
```

**Interpolation algorithm:**

```
1. Binary-search frames for the last frame where frame.time <= t
2. If t >= last frame time → return last frame value (clamped at end)
3. local_t = (t − frames[i].time) / (frames[i+1].time − frames[i].time)
4. curved_t = frames[i].easing.apply(local_t)
5. return frames[i].value.lerp(&frames[i+1].value, curved_t)
```

**PingPong loop:**

```
total   = duration()
cycle_t = elapsed % (2.0 * total)
t = if cycle_t <= total { cycle_t } else { 2.0 * total - cycle_t }
```

---

### 4.3 `animato-timeline`

**Responsibility:** Composing animations into concurrent or sequential groups. The `Timeline` is the mixer; `Sequence` is sugar for chaining.

**Depends on:** `animato-core`, `animato-tween`

#### `src/timeline.rs`

```rust
pub struct Timeline {
    entries:    Vec<TimelineEntry>,
    elapsed:    f32,
    state:      TimelineState,
    pub looping: Loop,
}

struct TimelineEntry {
    label:      String,
    animation:  Box<dyn Playable + Send>,
    start_at:   f32,           // absolute offset from timeline start in seconds
    duration:   f32,           // for progress computation
}

pub enum TimelineState {
    Idle,
    Playing,
    Paused,
    Completed,
}
```

**Relative positioning — the `At` enum:**

```rust
pub enum At {
    Absolute(f32),         // explicit time offset
    Start,                 // t = 0.0 (same as Absolute(0.0))
    End,                   // immediately after the last entry ends
    Label(&'static str),   // same start time as a named entry
    Offset(f32),           // relative to timeline's current end
}
```

**Builder API:**

```rust
let mut tl = Timeline::new()
    // start "fade" at t=0.0
    .add("fade", fade_tween, At::Absolute(0.0))
    // start "slide" right after "fade" ends
    .add("slide", slide_tween, At::End)
    // start "glow" at the same time "fade" started (concurrent)
    .add("glow", glow_tween, At::Label("fade"))
    // start "pop" 0.1s after the timeline's current end
    .add("pop", pop_tween, At::Offset(0.1))
    .looping(Loop::Once);

tl.play();
```

**Playback control:**

```rust
impl Timeline {
    pub fn play(&mut self);
    pub fn pause(&mut self);
    pub fn resume(&mut self);
    pub fn seek(&mut self, t: f32);     // jump to normalized time ∈ [0, 1]
    pub fn seek_abs(&mut self, secs: f32); // jump to absolute time in seconds
    pub fn reset(&mut self);

    pub fn duration(&self) -> f32;      // end time of the last-finishing entry
    pub fn progress(&self) -> f32;      // 0.0..=1.0
    pub fn is_complete(&self) -> bool;
    pub fn get<T: Playable + 'static>(&self, label: &str) -> Option<&T>;
    pub fn get_mut<T: Playable + 'static>(&mut self, label: &str) -> Option<&mut T>;
}
```

#### `src/sequence.rs`

`Sequence` is a builder that auto-calculates `start_at` by accumulating durations and gaps. It produces a `Timeline`.

```rust
pub struct Sequence { inner: Timeline, cursor: f32 }

impl Sequence {
    pub fn new() -> Self;
    pub fn then(self, label: &str, anim: impl Playable + Send + 'static) -> Self;
    pub fn then_for(self, label: &str, anim: impl Playable + Send + 'static, duration: f32) -> Self;
    pub fn gap(self, seconds: f32) -> Self;    // pause between steps
    pub fn build(self) -> Timeline;
}
```

#### `src/stagger.rs`

```rust
/// Create a timeline where N animations each start `delay` seconds
/// after the previous one.
pub fn stagger(
    animations: Vec<impl Playable + Send + 'static>,
    delay: f32,
) -> Timeline;
```

---

### 4.4 `animato-spring`

**Responsibility:** Physics-based animation using a damped harmonic oscillator. `no_std`-compatible — no heap allocation needed for `Spring` itself.

**Depends on:** `animato-core`

#### `src/spring.rs`

```rust
pub struct Spring {
    pub config:   SpringConfig,
    position:     f32,
    velocity:     f32,
    target:       f32,
    integrator:   Integrator,
}

pub enum Integrator {
    SemiImplicitEuler,   // default — fast, stable for animation
    RungeKutta4,         // optional — more accurate for high-stiffness springs
}

impl Spring {
    pub fn new(config: SpringConfig) -> Self;
    pub fn set_target(&mut self, target: f32);
    pub fn position(&self) -> f32;
    pub fn velocity(&self) -> f32;
    pub fn is_settled(&self) -> bool;
    pub fn snap_to(&mut self, pos: f32);    // teleport with no animation
    pub fn use_rk4(mut self, yes: bool) -> Self;
}

impl Update for Spring {
    fn update(&mut self, dt: f32) -> bool {
        if self.is_settled() { return false; }
        match self.integrator {
            Integrator::SemiImplicitEuler => self.step_euler(dt),
            Integrator::RungeKutta4 => self.step_rk4(dt),
        }
        !self.is_settled()
    }
}
```

**Semi-implicit Euler (default integration):**

```
displacement = position − target
acceleration = (−stiffness × displacement − damping × velocity) / mass
velocity    += acceleration × dt
position    += velocity × dt
```

**Settle detection:**

```
is_settled = |position − target| < epsilon && |velocity| < epsilon
```

#### `src/config.rs`

```rust
#[derive(Clone, Debug)]
pub struct SpringConfig {
    pub stiffness: f32,    // default: 100.0
    pub damping:   f32,    // default: 10.0
    pub mass:      f32,    // default: 1.0
    pub epsilon:   f32,    // settle threshold, default: 0.001
}

impl SpringConfig {
    pub fn gentle() -> Self   { /* stiffness: 60,  damping: 14, mass: 1.0 */ }
    pub fn wobbly() -> Self   { /* stiffness: 180, damping: 12, mass: 1.0 */ }
    pub fn stiff() -> Self    { /* stiffness: 210, damping: 20, mass: 1.0 */ }
    pub fn slow() -> Self     { /* stiffness: 37,  damping: 14, mass: 1.0 */ }
    pub fn snappy() -> Self   { /* stiffness: 300, damping: 30, mass: 1.0 */ }
}
```

**Multi-dimensional spring (`SpringN<T>`):**

Uses one `Spring` per component, reconstructed into `T` each frame.

```rust
pub struct SpringN<T: Animatable> {
    components: Vec<Spring>,      // length = number of lerp dimensions of T
    _marker:    PhantomData<T>,
}

impl<T: Animatable> SpringN<T> {
    pub fn new(config: SpringConfig, initial: T) -> Self;
    pub fn set_target(&mut self, target: T);
    pub fn position(&self) -> T;
    pub fn is_settled(&self) -> bool;
}
```

---

### 4.5 `animato-path`

**Responsibility:** All motion-path related types — Bezier curves, CatmullRom splines, arc-length parameterization, SVG path parsing, shape morphing, and the `MotionPathTween`.

**Depends on:** `animato-core`, `animato-tween`

#### Module breakdown

| File | Contents |
|------|----------|
| `bezier.rs` | `QuadBezier`, `CubicBezier`, `CatmullRomSpline`, `PathEvaluate` trait |
| `motion.rs` | `MotionPath`, `MotionPathTween`, auto-rotate, start/end offsets |
| `poly.rs` | `PolyPath`, `CompoundPath`, `PathCommand` — arc-length parameterized |
| `morph.rs` | `MorphPath` — point-by-point morph with auto-resampling |
| `svg.rs` | `SvgPathParser` — parses SVG `d` attribute into `PathCommand` list |

#### Key types

```rust
// bezier.rs
pub trait PathEvaluate {
    fn position(&self, t: f32) -> [f32; 2];
    fn tangent(&self, t: f32) -> [f32; 2];
    fn rotation_deg(&self, t: f32) -> f32;
    fn arc_length(&self) -> f32;
}

// motion.rs — the main motion path driver
pub struct MotionPathTween {
    path:       Box<dyn PathEvaluate>,
    tween:      Tween<f32>,        // drives t ∈ [0, 1] along the path
    auto_rotate: bool,
    start_offset: f32,
    end_offset:   f32,
}

impl MotionPathTween {
    pub fn value(&self) -> [f32; 2];      // current (x, y) position
    pub fn rotation_deg(&self) -> f32;    // auto-rotate heading
}
```

---

### 4.6 `animato-physics`

**Responsibility:** Input-driven physics — inertia (friction deceleration), drag tracking with velocity, and gesture recognition.

**Depends on:** `animato-core`

#### Module breakdown

| File | Contents |
|------|----------|
| `inertia.rs` | `Inertia`, `InertiaN<T>`, `InertiaConfig`, presets |
| `drag.rs` | `DragState`, `DragConstraints`, `DragAxis`, `PointerData` |
| `gesture.rs` | `GestureRecognizer`, `Gesture` enum, `GestureConfig` |

#### Key types

```rust
pub struct InertiaConfig<T = f32> {
    pub friction: f32,
    pub min_velocity: f32,
    pub bounds: Option<InertiaBounds<T>>,
}

pub struct InertiaBounds<T = f32> {
    pub min: T,
    pub max: T,
}

pub struct Inertia {
    pub config: InertiaConfig<f32>,
    position: f32,
    velocity: f32,
}

impl Inertia {
    pub fn new(config: InertiaConfig<f32>) -> Self;
    pub fn with_position(config: InertiaConfig<f32>, position: f32) -> Self;
    pub fn kick(&mut self, velocity: f32);
    pub fn position(&self) -> f32;
    pub fn velocity(&self) -> f32;
    pub fn snap_to(&mut self, position: f32);
    pub fn is_settled(&self) -> bool;
}

pub struct DragState;
impl DragState {
    pub fn new(position: [f32; 2]) -> Self;
    pub fn on_pointer_down(&mut self, data: PointerData);
    pub fn on_pointer_move(&mut self, data: PointerData, dt: f32);
    pub fn on_pointer_up(&mut self, data: PointerData) -> Option<InertiaN<[f32; 2]>>;
}

pub enum Gesture {
    Tap { position: [f32; 2] },
    DoubleTap { position: [f32; 2] },
    LongPress { position: [f32; 2], duration: f32 },
    Swipe { direction: SwipeDirection, velocity: f32, distance: f32 },
    Pinch { scale: f32, center: [f32; 2] },
    Rotation { angle_delta: f32, center: [f32; 2] },
}
```

`Inertia` uses constant friction deceleration and performs no heap allocation.
`InertiaN<T>` uses one 1D inertia per component and supports `f32`,
`[f32; 2]`, `[f32; 3]`, and `[f32; 4]` behind `alloc`.
When optional bounds are reached, position is clamped and velocity on that axis
is set to zero. `GestureRecognizer` supports single-pointer gestures plus
two-pointer pinch and rotation.

---

### 4.7 `animato-color`

**Responsibility:** Perceptual color interpolation by wrapping the `palette` crate. Enabled from the facade with `features = ["color"]`.

**Depends on:** `animato-core`, `palette`

```rust
// spaces.rs — wrapper types that impl Interpolate using the correct color space
pub struct InLab<C>(pub C);      // CIE L*a*b* — perceptually uniform
pub struct InOklch<C>(pub C);    // Oklch — modern perceptual space
pub struct InLinear<C>(pub C);   // linear light (gamma-correct sRGB lerp)

// Example: interpolating in Lab space
impl<C> Interpolate for InLab<C>
where
    C: palette::IntoColor<palette::Lab> + palette::FromColor<palette::Lab> + Clone + 'static,
{
    fn lerp(&self, other: &Self, t: f32) -> Self {
        // clamp t, convert both colors to Lab, mix, convert back
    }
}
```

---

### 4.8 `animato-driver`

**Responsibility:** The runtime — `AnimationDriver` manages many animations, `Clock` abstracts time, `ScrollDriver` links scroll position to animation progress.

**Depends on:** `animato-core` (+ `std` for `AnimationDriver`)

#### `src/driver.rs`

```rust
pub struct AnimationDriver {
    slots:   Vec<Slot>,
    next_id: u64,
}

struct Slot {
    id:        AnimationId,
    animation: Box<dyn Update + Send>,
    removed:   bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct AnimationId(u64);

impl AnimationDriver {
    pub fn new() -> Self;
    pub fn add<A: Update + Send + 'static>(&mut self, anim: A) -> AnimationId;
    pub fn tick(&mut self, dt: f32);       // tick all, auto-remove completed
    pub fn cancel(&mut self, id: AnimationId);
    pub fn cancel_all(&mut self);
    pub fn active_count(&self) -> usize;
    pub fn is_active(&self, id: AnimationId) -> bool;
}
```

#### `src/clock.rs`

```rust
pub trait Clock {
    fn delta(&mut self) -> f32;    // seconds since last call
}

// Requires "std" feature:
pub struct WallClock { last: std::time::Instant }

// Manual — caller calls .advance(dt) then .delta() returns it:
pub struct ManualClock { pending: f32 }
impl ManualClock {
    pub fn advance(&mut self, dt: f32);
}

// Fixed-step mock for deterministic tests:
pub struct MockClock { step: f32 }
impl MockClock {
    pub fn new(step_seconds: f32) -> Self;
}
```

#### `src/scroll.rs`

```rust
pub struct ScrollDriver {
    min_scroll: f32,
    max_scroll: f32,
    animations: Vec<Box<dyn Update + Send>>,
    position:   f32,
}

impl ScrollDriver {
    pub fn new(min: f32, max: f32) -> Self;
    pub fn add<A: Update + Send + 'static>(&mut self, anim: A);
    pub fn set_position(&mut self, pos: f32);  // drives all animations by normalized pos
}
```

---

### 4.9 `animato-gpu`

**Responsibility:** Batch-evaluate 10,000+ tweens per frame on the GPU using `wgpu` compute shaders. Falls back to CPU if GPU unavailable.

**Depends on:** `animato-core`, `animato-tween`, `wgpu`, `bytemuck`, `pollster`

```rust
pub struct GpuAnimationBatch {
    tweens:    Vec<Tween<f32>>,
    values:    Vec<f32>,
    resources: Option<GpuResources>,
    force_cpu: bool,
}

impl GpuAnimationBatch {
    pub fn new_cpu() -> Self;
    pub fn new(device: wgpu::Device, queue: wgpu::Queue) -> Result<Self, GpuBatchError>;
    pub fn try_new_auto() -> Result<Self, GpuBatchError>;
    pub fn new_auto() -> Self;             // tries GPU, falls back to CPU mode
    pub fn push(&mut self, tween: Tween<f32>) -> usize;
    pub fn tick(&mut self, dt: f32);
    pub fn read_back(&self) -> &[f32];
    pub fn backend(&self) -> GpuBackend;
}
```

**WGSL shader (`shaders/tween.wgsl`):**

The shader receives a buffer of tween state structs `{start, end, duration, elapsed, easing_id}` and writes the output float value for each. The v1.0.0 shader covers the 31 classic easing variants; unsupported CSS, advanced, or custom easing falls back to exact CPU evaluation.

---

### 4.10 `animato-bevy`

**Responsibility:** Bevy plugin integrating Animato into the Bevy ECS. Component wrappers, system scheduling, transform helpers, and completion messages.

**Depends on:** `animato-core`, `animato-tween`, `animato-spring`, `bevy_app`, `bevy_ecs`, `bevy_time`, `bevy_math`, `bevy_transform`

#### `src/lib.rs`

```rust
pub struct AnimatoPlugin;

impl bevy_app::Plugin for AnimatoPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app
            .add_message::<TweenCompleted>()
            .add_message::<SpringSettled>()
            .configure_sets(Update, (AnimatoSet::Tick, AnimatoSet::Apply).chain())
            .add_systems(Update, tick_tweens::<[f32; 3]>.in_set(AnimatoSet::Tick))
            .add_systems(Update, apply_transform_vec3_tweens.in_set(AnimatoSet::Apply));
    }
}

#[derive(Component)]
pub struct AnimatoTween<T> { tween: Tween<T>, channel: AnimationChannel }

#[derive(Message)]
pub struct TweenCompleted { pub entity: Entity, pub label: Option<AnimationLabel> }

#[derive(Message)]
pub struct SpringSettled { pub entity: Entity, pub label: Option<AnimationLabel> }
```

---

### 4.11 `animato-wasm`

**Responsibility:** Browser-specific integrations. `requestAnimationFrame` driver, FLIP layout transitions, SplitText, ScrollSmoother, Draggable, Observer.

**Depends on:** `animato-core`, `animato-driver`, optional `animato-tween`, optional `animato-physics`, optional `wasm-bindgen`, optional `web-sys`

#### `src/raf.rs`

```rust
pub struct RafDriver {
    driver:            AnimationDriver,
    last_timestamp_ms: Option<f64>,
    time_scale:        f32,
    max_dt:            f32,
    paused:            bool,
}

impl RafDriver {
    pub fn new() -> Self;
    pub fn tick(&mut self, timestamp_ms: f64) -> f32; // returns dt seconds
    pub fn pause(&mut self);
    pub fn resume(&mut self);
    pub fn set_time_scale(&mut self, scale: f32);
    pub fn set_max_dt(&mut self, max_dt: f32);
}
```

---

### 4.12 `animato-leptos`

**Responsibility:** First-class Leptos integration. Signal-backed animation hooks, scroll-driven animations, mount/unmount presence transitions, FLIP list reordering, page transitions, gesture bindings, CSS helpers, and SSR-aware hydration guards.

**Depends on:** `animato-core`, `animato-tween`, `animato-spring`, `animato-timeline`, `animato-driver`, `animato-path`, `animato-physics`, `animato-wasm`, `leptos`, `leptos_router`

**Version:** Starts at `1.1.0` — published independently from the core `1.0` crates.

#### Module breakdown

| File | Contents |
|------|----------|
| `hooks.rs` | `use_tween`, `use_spring`, `use_timeline`, `use_keyframes` — signal-backed animation hooks |
| `scroll.rs` | `use_scroll_progress`, `use_scroll_trigger`, `use_scroll_velocity`, `SmoothScroll` component |
| `presence.rs` | `AnimatePresence` — mount/unmount transitions with configurable enter/exit animations |
| `transition.rs` | `PageTransition` — route-change wrapper with fade, slide, zoom, morph presets |
| `list.rs` | `AnimatedFor` — FLIP-powered list reordering, insert, and remove animations |
| `gesture.rs` | `use_drag`, `use_gesture`, `use_pinch`, `use_swipe` — pointer event → animation bindings |
| `css.rs` | `AnimatedStyle`, `css_transform()`, `css_spring()` — CSS property animation helpers |
| `ssr.rs` | `is_hydrating()`, `use_client_only()`, `SsrFallback` — SSR-aware animation guards |

#### `src/hooks.rs`

```rust
/// Signal-backed tween. Returns a reactive ReadSignal<T> that updates every frame
/// and a TweenHandle for playback control.
pub fn use_tween<T: Animatable + Send + Sync + 'static>(
    from: T,
    to: T,
    config: impl FnOnce(TweenBuilder<T>) -> TweenBuilder<T>,
) -> (ReadSignal<T>, TweenHandle)

/// Signal-backed spring. The returned signal updates every frame until settled.
pub fn use_spring<T: Animatable + Send + Sync + 'static>(
    initial: T,
    config: SpringConfig,
) -> (ReadSignal<T>, SpringHandle)

/// Compose multiple animations with timeline scheduling.
pub fn use_timeline(
    builder: impl FnOnce(&mut Timeline),
) -> TimelineHandle

/// Multi-stop keyframe animation driven by a signal.
pub fn use_keyframes<T: Animatable + Send + Sync + 'static>(
    builder: impl FnOnce(KeyframeTrack<T>) -> KeyframeTrack<T>,
) -> (ReadSignal<T>, KeyframeHandle)
```

**Handle types:**

```rust
pub struct TweenHandle {
    pub fn play(&self);
    pub fn pause(&self);
    pub fn resume(&self);
    pub fn reset(&self);
    pub fn reverse(&self);
    pub fn seek(&self, t: f32);
    pub fn set_time_scale(&self, ts: f32);
    pub fn is_complete(&self) -> ReadSignal<bool>;
    pub fn progress(&self) -> ReadSignal<f32>;
}

pub struct SpringHandle {
    pub fn set_target(&self, target: T);
    pub fn snap_to(&self, value: T);
    pub fn is_settled(&self) -> ReadSignal<bool>;
}
```

#### `src/scroll.rs`

```rust
/// Returns a signal in [0.0, 1.0] tracking scroll progress of a target element.
pub fn use_scroll_progress(
    target: NodeRef<html::Div>,
    config: ScrollConfig,
) -> ReadSignal<f32>

/// Fires a callback when an element enters/exits the viewport.
pub fn use_scroll_trigger(
    target: NodeRef<html::Div>,
    config: ScrollTriggerConfig,
) -> ScrollTriggerHandle

/// Returns the current scroll velocity in px/sec for momentum-based effects.
pub fn use_scroll_velocity() -> ReadSignal<f32>

pub struct ScrollConfig {
    pub axis: ScrollAxis,          // Vertical (default), Horizontal, Both
    pub offset_start: f32,         // viewport offset to begin (default 0.0)
    pub offset_end: f32,           // viewport offset to end (default 1.0)
    pub smooth: bool,              // lerp smoothing (default true)
    pub smooth_factor: f32,        // smoothing speed (default 0.1)
}

pub struct ScrollTriggerConfig {
    pub threshold: f32,            // intersection ratio 0.0..=1.0
    pub once: bool,                // fire only on first enter
    pub start: &'static str,      // e.g. "top bottom" (element top hits viewport bottom)
    pub end: &'static str,        // e.g. "bottom top"
    pub scrub: bool,              // link animation progress to scroll position
    pub pin: bool,                // pin element during scroll range
}

/// Smooth scroll container with momentum and overscroll damping.
#[component]
pub fn SmoothScroll(children: Children) -> impl IntoView
```

#### `src/presence.rs`

```rust
/// Mount/unmount transition wrapper. Children animate in on mount
/// and animate out before unmount completes.
#[component]
pub fn AnimatePresence(
    /// Show or hide the children.
    show: ReadSignal<bool>,
    /// Enter animation config.
    #[prop(optional)] enter: Option<PresenceAnimation>,
    /// Exit animation config.
    #[prop(optional)] exit: Option<PresenceAnimation>,
    /// Delay unmount until exit animation completes (default true).
    #[prop(default = true)] wait_exit: bool,
    children: Children,
) -> impl IntoView

pub struct PresenceAnimation {
    pub duration: f32,
    pub easing: Easing,
    pub from: AnimatedStyle,   // CSS properties at animation start
    pub to: AnimatedStyle,     // CSS properties at animation end
}

impl PresenceAnimation {
    pub fn fade() -> Self;           // opacity 0 → 1
    pub fn slide_up() -> Self;       // translateY(20px) + opacity
    pub fn slide_down() -> Self;
    pub fn slide_left() -> Self;
    pub fn slide_right() -> Self;
    pub fn zoom_in() -> Self;        // scale(0.8) + opacity
    pub fn zoom_out() -> Self;
    pub fn flip_x() -> Self;         // rotateX(90deg) + opacity
    pub fn flip_y() -> Self;
    pub fn blur_in() -> Self;        // filter: blur(10px) + opacity
    pub fn spring(config: SpringConfig) -> Self;
}
```

#### `src/transition.rs`

```rust
/// Route-change transition wrapper. Animates the outgoing page out
/// and the incoming page in with configurable transition modes.
#[component]
pub fn PageTransition(
    #[prop(optional)] mode: TransitionMode,
    #[prop(optional)] enter: Option<PresenceAnimation>,
    #[prop(optional)] exit: Option<PresenceAnimation>,
    children: Children,
) -> impl IntoView

pub enum TransitionMode {
    Sequential,     // old exits, then new enters
    Parallel,       // old exits and new enters simultaneously
    CrossFade,      // both overlap with opposing opacity
    SlideOver,      // new slides on top of old
    MorphHero,      // shared-element morph between pages
}
```

#### `src/list.rs`

```rust
/// FLIP-animated list. Automatically animates item insertion, removal,
/// and reordering using layout-aware FLIP transitions.
#[component]
pub fn AnimatedFor<T, K, V>(
    each: Signal<Vec<T>>,
    key: impl Fn(&T) -> K + 'static,
    children: impl Fn(T) -> V + 'static,
    #[prop(optional)] enter: Option<PresenceAnimation>,
    #[prop(optional)] exit: Option<PresenceAnimation>,
    #[prop(optional)] move_duration: Option<f32>,
    #[prop(optional)] move_easing: Option<Easing>,
    #[prop(optional)] stagger_delay: Option<f32>,
) -> impl IntoView
```

#### `src/gesture.rs`

```rust
/// Draggable element hook. Returns position signal and drag handle.
pub fn use_drag(
    target: NodeRef<html::Div>,
    config: DragConfig,
) -> (ReadSignal<[f32; 2]>, DragHandle)

/// Gesture recognition hook. Emits Gesture signals from pointer events.
pub fn use_gesture(
    target: NodeRef<html::Div>,
    config: GestureConfig,
) -> ReadSignal<Option<Gesture>>

/// Pinch-zoom hook for touch interfaces.
pub fn use_pinch(
    target: NodeRef<html::Div>,
) -> (ReadSignal<f32>, PinchHandle)    // scale signal

/// Swipe detection hook with direction and velocity.
pub fn use_swipe(
    target: NodeRef<html::Div>,
    config: SwipeConfig,
) -> ReadSignal<Option<SwipeEvent>>

pub struct DragConfig {
    pub axis: DragAxis,
    pub constraints: Option<DragConstraints>,
    pub inertia: bool,             // enable inertia on release
    pub inertia_config: InertiaConfig,
    pub snap_points: Vec<f32>,     // snap-to positions after release
    pub elastic_edges: bool,       // rubber-band at constraints
}
```

#### `src/css.rs`

```rust
/// CSS property bag for animated styles.
pub struct AnimatedStyle {
    pub opacity: Option<f32>,
    pub transform: Option<String>,
    pub scale: Option<f32>,
    pub translate_x: Option<f32>,
    pub translate_y: Option<f32>,
    pub rotate: Option<f32>,
    pub skew_x: Option<f32>,
    pub skew_y: Option<f32>,
    pub blur: Option<f32>,
    pub background_color: Option<[f32; 4]>,
    pub border_radius: Option<f32>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub clip_path: Option<String>,
    pub custom: Vec<(String, String)>,
}

/// Animate CSS properties with a spring. Returns a style string signal.
pub fn css_spring(
    target: AnimatedStyle,
    config: SpringConfig,
) -> ReadSignal<String>

/// Animate CSS properties with a tween. Returns a style string signal.
pub fn css_tween(
    from: AnimatedStyle,
    to: AnimatedStyle,
    duration: f32,
    easing: Easing,
) -> ReadSignal<String>
```

#### `src/ssr.rs`

```rust
/// Returns true during hydration — animations skip to final state.
pub fn is_hydrating() -> bool

/// Guard that prevents animation hooks from running on the server.
/// On the server the signal returns the final target value immediately.
pub fn use_client_only<T: Animatable>(server_value: T) -> ReadSignal<T>

/// Wrapper that renders a static fallback during SSR and swaps in
/// the animated version after hydration.
#[component]
pub fn SsrFallback(
    fallback: View,
    children: Children,
) -> impl IntoView
```

#### `Cargo.toml`

```toml
[package]
name        = "animato-leptos"
version     = "1.1.0"
description = "Leptos integration for the Animato animation library — signal-backed hooks, scroll, presence, transitions, FLIP lists, gestures, and SSR."

[features]
default    = ["scroll", "presence", "transition", "list", "gesture", "css"]
scroll     = []
presence   = []
transition = ["dep:leptos_router"]
list       = []
gesture    = ["dep:animato-physics"]
css        = []
ssr        = []
path       = ["dep:animato-path"]
color      = ["dep:animato-color"]

[dependencies]
animato-core     = { workspace = true }
animato-tween    = { workspace = true }
animato-spring   = { workspace = true }
animato-timeline = { workspace = true }
animato-driver   = { workspace = true }
animato-wasm     = { workspace = true }
animato-path     = { workspace = true, optional = true }
animato-physics  = { workspace = true, optional = true }
animato-color    = { workspace = true, optional = true }
leptos           = { workspace = true }
leptos_router    = { workspace = true, optional = true }
wasm-bindgen     = { workspace = true }
web-sys          = { workspace = true, features = ["Window", "Document", "Element", "HtmlElement", "DomRect", "IntersectionObserver", "IntersectionObserverEntry", "IntersectionObserverInit", "ScrollToOptions"] }
```

---

### 4.13 `animato-dioxus`

**Responsibility:** Cross-platform Dioxus integration. Hook-based animation primitives that work identically on web (WASM), desktop, mobile, and TUI targets. Platform-adaptive rendering, native window animation helpers, scroll-driven animations, presence transitions, FLIP lists, page transitions, and gesture bindings.

**Depends on:** `animato-core`, `animato-tween`, `animato-spring`, `animato-timeline`, `animato-driver`, `animato-path`, `animato-physics`, `animato-wasm`, `dioxus`, `dioxus-router`

**Version:** Starts at `1.2.0`.

#### Module breakdown

| File | Contents |
|------|----------|
| `hooks.rs` | `use_tween`, `use_spring`, `use_timeline`, `use_keyframes` — core animation hooks |
| `motion.rs` | `use_motion` — unified all-in-one hook (tween, spring, or keyframes in one call) |
| `scroll.rs` | `use_scroll_progress`, `use_scroll_trigger`, `use_scroll_velocity` — scroll-driven animations |
| `presence.rs` | `AnimatePresence` — mount/unmount transitions with enter/exit configs |
| `transition.rs` | `PageTransition` — route-change animation wrapper with mode presets |
| `list.rs` | `AnimatedFor` — FLIP-powered list reordering with stagger support |
| `gesture.rs` | `use_drag`, `use_gesture`, `use_pinch`, `use_swipe` — cross-platform pointer/touch bindings |
| `platform.rs` | `PlatformAdapter`, `AnimationBackend` — platform-adaptive tick source and rendering strategy |
| `native.rs` | `use_window_animation`, `use_window_spring` — native window position/size animation (desktop/mobile) |

#### `src/hooks.rs`

```rust
/// Tween hook. Returns the current value and a control handle.
/// Works on all Dioxus targets: web, desktop, mobile, TUI.
pub fn use_tween<T: Animatable + Send + Sync + 'static>(
    from: T,
    to: T,
    config: impl FnOnce(TweenBuilder<T>) -> TweenBuilder<T>,
) -> (T, TweenHandle)

/// Spring hook. The returned value follows the target with physics.
pub fn use_spring<T: Animatable + Send + Sync + 'static>(
    initial: T,
    config: SpringConfig,
) -> (T, SpringHandle)

/// Timeline hook for composing multiple animations.
pub fn use_timeline(
    builder: impl FnOnce(&mut Timeline),
) -> TimelineHandle

/// Keyframe track hook for multi-stop animations.
pub fn use_keyframes<T: Animatable + Send + Sync + 'static>(
    builder: impl FnOnce(KeyframeTrack<T>) -> KeyframeTrack<T>,
) -> (T, KeyframeHandle)
```

#### `src/motion.rs`

```rust
/// All-in-one motion hook. Combines tween, spring, and keyframe capabilities
/// behind a single ergonomic API.
pub fn use_motion<T: Animatable + Send + Sync + 'static>(
    initial: T,
) -> MotionHandle<T>

impl<T: Animatable> MotionHandle<T> {
    pub fn value(&self) -> T;
    pub fn animate_to(&self, target: T, config: MotionConfig);
    pub fn spring_to(&self, target: T, config: SpringConfig);
    pub fn keyframes(&self, track: KeyframeTrack<T>);
    pub fn stop(&self);
    pub fn snap_to(&self, value: T);
    pub fn is_animating(&self) -> bool;
}

pub enum MotionConfig {
    Tween { duration: f32, easing: Easing, delay: f32 },
    Spring(SpringConfig),
}
```

#### `src/platform.rs`

```rust
/// Detects the current Dioxus rendering platform and selects
/// the optimal animation tick source.
pub struct PlatformAdapter;

impl PlatformAdapter {
    /// Detect platform at runtime.
    pub fn detect() -> AnimationBackend;
}

pub enum AnimationBackend {
    /// Web — uses requestAnimationFrame via animato-wasm RafDriver.
    WebRaf,
    /// Desktop/Mobile — uses std::time::Instant with a 60fps event loop tick.
    NativeClock,
    /// TUI — uses crossterm event poll intervals as the tick source.
    TerminalPoll,
}
```

#### `src/native.rs`

```rust
/// Animate the native window position on desktop.
pub fn use_window_animation(
    config: impl FnOnce(TweenBuilder<[f32; 2]>) -> TweenBuilder<[f32; 2]>,
) -> WindowAnimationHandle

/// Spring-based window position animation for desktop.
pub fn use_window_spring(
    config: SpringConfig,
) -> WindowSpringHandle

impl WindowAnimationHandle {
    pub fn move_to(&self, x: f32, y: f32);
    pub fn resize_to(&self, w: f32, h: f32);
    pub fn opacity_to(&self, opacity: f32);
}
```

The `presence.rs`, `transition.rs`, `list.rs`, `gesture.rs`, and `scroll.rs` modules follow the same API contract as `animato-leptos` but use Dioxus `Signal<T>` and RSX instead of Leptos `ReadSignal<T>` and `view!{}`. All hooks use `use_future` internally to drive the rAF/clock loop.

#### `Cargo.toml`

```toml
[package]
name        = "animato-dioxus"
version     = "1.2.0"
description = "Dioxus integration for the Animato animation library — cross-platform hooks, scroll, presence, transitions, FLIP lists, gestures, and native window animation."

[features]
default    = ["scroll", "presence", "transition", "list", "gesture", "motion"]
scroll     = []
presence   = []
transition = ["dep:dioxus-router"]
list       = []
gesture    = ["dep:animato-physics"]
motion     = []
native     = []
path       = ["dep:animato-path"]
color      = ["dep:animato-color"]

[dependencies]
animato-core     = { workspace = true }
animato-tween    = { workspace = true }
animato-spring   = { workspace = true }
animato-timeline = { workspace = true }
animato-driver   = { workspace = true }
animato-wasm     = { workspace = true, optional = true }
animato-path     = { workspace = true, optional = true }
animato-physics  = { workspace = true, optional = true }
animato-color    = { workspace = true, optional = true }
dioxus           = { workspace = true }
dioxus-router    = { workspace = true, optional = true }

[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen     = { workspace = true }
web-sys          = { workspace = true, features = ["Window", "Document", "Element", "HtmlElement", "DomRect", "IntersectionObserver", "IntersectionObserverEntry", "ScrollToOptions"] }
```

---

### 4.14 `animato-yew`

**Responsibility:** Yew integration providing hook-based and agent-based animation primitives. Functional component hooks for tweens, springs, timelines, and keyframes. Scroll-driven animations, mount/unmount presence transitions, FLIP list reordering, page transitions, gesture bindings, CSS helpers, and an `AnimationAgent` for cross-component animation coordination.

**Depends on:** `animato-core`, `animato-tween`, `animato-spring`, `animato-timeline`, `animato-driver`, `animato-path`, `animato-physics`, `animato-wasm`, `yew`, `yew-router`

**Version:** Starts at `1.3.0`.

#### Module breakdown

| File | Contents |
|------|----------|
| `hooks.rs` | `use_tween`, `use_spring`, `use_timeline`, `use_keyframes` — functional component hooks |
| `scroll.rs` | `use_scroll_progress`, `use_scroll_trigger`, `use_scroll_velocity` — scroll-driven animations |
| `presence.rs` | `AnimatePresence` — mount/unmount transitions with configurable enter/exit |
| `transition.rs` | `PageTransition` — route-change animation wrapper |
| `list.rs` | `AnimatedFor` — FLIP-powered list with insert/remove/reorder animations |
| `gesture.rs` | `use_drag`, `use_gesture`, `use_pinch`, `use_swipe` — pointer event bindings |
| `agent.rs` | `AnimationAgent` — Yew agent for cross-component animation message coordination |
| `css.rs` | `AnimatedStyle`, `css_transform()`, `css_spring()` — CSS property helpers |

#### `src/hooks.rs`

```rust
/// Tween hook for Yew functional components. Returns a UseStateHandle<T>
/// that re-renders the component when the animated value changes.
/// Uses use_raf internally to limit updates to one per frame.
pub fn use_tween<T: Animatable + 'static>(
    from: T,
    to: T,
    config: impl FnOnce(TweenBuilder<T>) -> TweenBuilder<T>,
) -> (UseStateHandle<T>, TweenHandle)

/// Spring hook. Re-renders the component per-frame while the spring is active.
pub fn use_spring<T: Animatable + 'static>(
    initial: T,
    config: SpringConfig,
) -> (UseStateHandle<T>, SpringHandle)

/// Timeline composition hook.
pub fn use_timeline(
    builder: impl FnOnce(&mut Timeline),
) -> TimelineHandle

/// Multi-stop keyframe animation hook.
pub fn use_keyframes<T: Animatable + 'static>(
    builder: impl FnOnce(KeyframeTrack<T>) -> KeyframeTrack<T>,
) -> (UseStateHandle<T>, KeyframeHandle)
```

#### `src/agent.rs`

```rust
/// Yew agent that coordinates animations across multiple components.
/// Components send messages to the agent to start, stop, or synchronize
/// animations without direct parent-child coupling.
pub struct AnimationAgent {
    driver: AnimationDriver,
    subscribers: HashMap<AnimationId, Vec<HandlerId>>,
}

pub enum AgentInput {
    AddTween { id: String, tween: Box<dyn Playable + Send> },
    AddSpring { id: String, spring: Box<dyn Update + Send> },
    Play(String),
    Pause(String),
    Reset(String),
    Cancel(String),
    CancelAll,
    Tick(f32),
}

pub enum AgentOutput {
    ValueChanged { id: String, progress: f32 },
    Completed { id: String },
    Settled { id: String },
}
```

The `scroll.rs`, `presence.rs`, `transition.rs`, `list.rs`, `gesture.rs`, and `css.rs` modules follow the same API contract as `animato-leptos` but use Yew `Html`, `UseStateHandle<T>`, `NodeRef`, and `Callback` instead of Leptos equivalents. Per-frame updates use `use_raf` from `gloo` or a custom rAF closure to avoid VDOM diffing overhead on non-animated nodes.

#### `Cargo.toml`

```toml
[package]
name        = "animato-yew"
version     = "1.3.0"
description = "Yew integration for the Animato animation library — hooks, agents, scroll, presence, transitions, FLIP lists, gestures, and CSS animation helpers."

[features]
default    = ["scroll", "presence", "transition", "list", "gesture", "css"]
scroll     = []
presence   = []
transition = ["dep:yew-router"]
list       = []
gesture    = ["dep:animato-physics"]
css        = []
agent      = []
path       = ["dep:animato-path"]
color      = ["dep:animato-color"]

[dependencies]
animato-core     = { workspace = true }
animato-tween    = { workspace = true }
animato-spring   = { workspace = true }
animato-timeline = { workspace = true }
animato-driver   = { workspace = true }
animato-wasm     = { workspace = true }
animato-path     = { workspace = true, optional = true }
animato-physics  = { workspace = true, optional = true }
animato-color    = { workspace = true, optional = true }
yew              = { workspace = true }
yew-router       = { workspace = true, optional = true }
wasm-bindgen     = { workspace = true }
web-sys          = { workspace = true, features = ["Window", "Document", "Element", "HtmlElement", "DomRect", "IntersectionObserver", "IntersectionObserverEntry", "ScrollToOptions"] }
gloo             = { version = "0.11" }
```

---

### 4.15 `animato-js`

**Responsibility:** WASM-compiled NPM package exposing Animato's animation engine to JavaScript frameworks (React, Svelte, Vue, Angular, vanilla JS). Provides `#[wasm_bindgen]` wrappers around core types, a string-based easing parser for JS ergonomics, and a ready-to-use rAF driver. Published to NPM as `@animato/core` via `wasm-pack`.

**Depends on:** `animato-core`, `animato-tween`, `animato-spring`, `animato-timeline`, `animato-driver`, `animato-path`, `animato-wasm`, `wasm-bindgen`, `js-sys`, `web-sys`

**Version:** Starts at `1.4.0`.

**Build command:** `wasm-pack build crates/animato-js --target web --scope animato`

#### Module breakdown

| File | Contents |
|------|----------|
| `tween.rs` | `JsTween` — `#[wasm_bindgen]` wrapper around `Tween<f32>` and `Tween<[f32; N]>` |
| `spring.rs` | `JsSpring` — wrapper around `Spring` and `SpringN<T>` |
| `timeline.rs` | `JsTimeline` — wrapper around `Timeline` with string-label API |
| `keyframe.rs` | `JsKeyframeTrack` — wrapper around `KeyframeTrack<f32>` |
| `driver.rs` | `JsRafDriver` — wrapper around `RafDriver` for JS rAF callbacks |
| `easing.rs` | `parse_easing(name: &str) -> Easing` — string-to-enum parser for JS ergonomics |
| `path.rs` | `JsMotionPath` — wrapper around `MotionPathTween` |

#### `src/tween.rs`

```rust
#[wasm_bindgen]
pub struct JsTween {
    inner: Tween<f32>,
}

#[wasm_bindgen]
impl JsTween {
    #[wasm_bindgen(constructor)]
    pub fn new(from: f32, to: f32, duration: f32) -> Self;

    /// Set easing by name: "linear", "easeOutCubic", "easeInOutBack", etc.
    pub fn set_easing(&mut self, name: &str);

    /// Set easing by CSS cubic-bezier control points.
    pub fn set_cubic_bezier(&mut self, x1: f32, y1: f32, x2: f32, y2: f32);

    pub fn update(&mut self, dt: f32) -> bool;
    pub fn value(&self) -> f32;
    pub fn progress(&self) -> f32;
    pub fn eased_progress(&self) -> f32;
    pub fn is_complete(&self) -> bool;
    pub fn pause(&mut self);
    pub fn resume(&mut self);
    pub fn reset(&mut self);
    pub fn reverse(&mut self);
    pub fn seek(&mut self, t: f32);
    pub fn set_time_scale(&mut self, ts: f32);
    pub fn set_delay(&mut self, delay: f32);
    pub fn set_loop_count(&mut self, count: u32);
    pub fn set_ping_pong(&mut self);
}

/// Multi-dimensional tween for [x, y] animations.
#[wasm_bindgen]
pub struct JsTween2D {
    inner: Tween<[f32; 2]>,
}

#[wasm_bindgen]
impl JsTween2D {
    #[wasm_bindgen(constructor)]
    pub fn new(from_x: f32, from_y: f32, to_x: f32, to_y: f32, duration: f32) -> Self;
    pub fn update(&mut self, dt: f32) -> bool;
    pub fn x(&self) -> f32;
    pub fn y(&self) -> f32;
}
```

#### `src/spring.rs`

```rust
#[wasm_bindgen]
pub struct JsSpring {
    inner: Spring,
}

#[wasm_bindgen]
impl JsSpring {
    #[wasm_bindgen(constructor)]
    pub fn new(initial: f32, target: f32) -> Self;

    /// Use a named preset: "gentle", "wobbly", "stiff", "slow", "snappy".
    pub fn set_preset(&mut self, name: &str);

    pub fn set_config(&mut self, stiffness: f32, damping: f32, mass: f32);
    pub fn set_target(&mut self, target: f32);
    pub fn update(&mut self, dt: f32) -> bool;
    pub fn position(&self) -> f32;
    pub fn velocity(&self) -> f32;
    pub fn is_settled(&self) -> bool;
    pub fn snap_to(&mut self, pos: f32);
}
```

#### `src/easing.rs`

```rust
/// Parses a JavaScript-friendly easing name into the Animato Easing enum.
/// Supports: "linear", "easeInQuad", "easeOutCubic", "easeInOutBack",
/// "easeOutBounce", "easeInElastic", "steps(5)",
/// "cubicBezier(0.4, 0, 0.2, 1)", and all 38 named variants.
pub fn parse_easing(name: &str) -> Easing;

/// Returns all available easing names as a JS array.
#[wasm_bindgen]
pub fn available_easings() -> Vec<JsValue>;
```

#### JavaScript usage (after `wasm-pack build`):

```js
// Install: npm install @animato/core
import init, { JsTween, JsSpring, available_easings } from '@animato/core';

await init(); // initialize WASM module

// Tween
const tween = new JsTween(0, 300, 1.0);
tween.set_easing('easeOutCubic');

let last = performance.now();
function tick(now) {
  if (tween.update((now - last) / 1000)) {
    last = now;
    element.style.transform = `translateX(${tween.value()}px)`;
    requestAnimationFrame(tick);
  }
}
requestAnimationFrame(tick);

// Spring
const spring = new JsSpring(0, 100);
spring.set_preset('wobbly');
```

#### `Cargo.toml`

```toml
[package]
name        = "animato-js"
version     = "1.4.0"
description = "WASM bindings for the Animato animation library — use Animato in React, Svelte, Vue, and any JavaScript framework."

[lib]
crate-type = ["cdylib", "rlib"]

[features]
default    = ["tween", "spring", "timeline", "driver"]
tween      = []
spring     = []
timeline   = []
driver     = []
path       = ["dep:animato-path"]
color      = ["dep:animato-color"]

[dependencies]
animato-core     = { workspace = true }
animato-tween    = { workspace = true }
animato-spring   = { workspace = true }
animato-timeline = { workspace = true }
animato-driver   = { workspace = true }
animato-wasm     = { workspace = true }
animato-path     = { workspace = true, optional = true }
animato-color    = { workspace = true, optional = true }
wasm-bindgen     = { workspace = true }
js-sys           = { workspace = true }
web-sys          = { workspace = true, features = ["Window", "Performance"] }
```

---

### 4.16 `animato` (facade)

**Responsibility:** The one crate users put in their `Cargo.toml`. Feature flags on this crate activate the matching sub-crates and re-export their public APIs.

```toml
[features]
default  = ["std", "tween", "timeline", "spring", "driver"]
std      = ["animato-core/std", "animato-driver/std", "animato-path?/std", "animato-color?/std"]
tween    = ["dep:animato-tween"]
timeline = ["dep:animato-timeline"]
spring   = ["dep:animato-spring"]
path     = ["dep:animato-path", "animato-path/std"]
physics  = ["dep:animato-physics"]
color    = ["dep:animato-color", "dep:palette"]
driver   = ["dep:animato-driver"]
gpu      = ["dep:animato-gpu"]
bevy     = ["dep:animato-bevy"]
wasm     = ["dep:animato-wasm"]
wasm-dom = ["wasm", "animato-wasm/wasm-dom"]
leptos   = ["dep:animato-leptos"]
dioxus   = ["dep:animato-dioxus"]
yew      = ["dep:animato-yew"]
js       = ["dep:animato-js"]
serde    = ["animato-core/serde", "animato-tween/serde", "animato-spring/serde", "animato-path?/serde", "animato-color?/serde"]
tokio    = ["animato-timeline/tokio"]
no_std   = []
```

`src/lib.rs` re-exports everything behind `#[cfg(feature = ...)]` guards.

---

## 5. Data Flow & Runtime Loop

### Standard Application (non-Bevy, non-WASM)

```
Application loop (60fps)
       │
       ▼
  WallClock::delta()         → dt: f32 (seconds since last frame)
       │
       ▼
  AnimationDriver::tick(dt)
       │
       ├── Tween::update(dt)              → advance elapsed, compute value()
       ├── KeyframeTrack::update(dt)      → advance elapsed, binary-search, lerp
       ├── Timeline::update(dt)           → tick entries in time window
       ├── Spring::update(dt)             → integrate velocity + position
       └── MotionPathTween::update(dt)    → advance path tween, evaluate position
       │
       ▼
  Application reads .value() or .position()
  from each animation, then renders.
```

### Bevy ECS Loop

```
Bevy scheduler (Update stage)
       │
       ▼
  tick_tweens system
  tick_springs system
       │
       ▼
  Query<(Entity, &mut AnimatoTween<T>)>
  Query<(Entity, &mut AnimatoSpring<T>)>
       │
       ▼
  .update(time.delta_secs())
       │
       ▼
  TweenCompleted / SpringSettled messages fired
       │
       ▼
  User systems react to messages or use built-in transform helpers
```

### WASM / Browser Loop

```
Browser
       │
       ▼
  requestAnimationFrame(timestamp_ms)
       │
       ▼
  RafDriver::tick(timestamp_ms)
       │
       ▼
  AnimationDriver::tick(dt)
       │
       ▼
  Write values to DOM via wasm-bindgen JS closures
```

### Leptos Signal Loop

```
Leptos component mounts
       │
       ▼
  use_tween() / use_spring() / use_keyframes()
       │
       ▼
  Spawns rAF loop via request_animation_frame closure
       │
       ├── Tween::update(dt) / Spring::update(dt)
       ├── WriteSignal::set(new_value)      ← fine-grained signal update
       └── Only the DOM node reading the signal re-renders
       │
       ▼
  On unmount: rAF closure dropped, animation cleaned up
  On SSR: signal returns target value immediately (no rAF)
```

### Dioxus Hook Loop (cross-platform)

```
Dioxus component renders (any platform)
       │
       ▼
  use_tween() / use_spring() / use_motion()
       │
       ▼
  PlatformAdapter::detect() → WebRaf | NativeClock | TerminalPoll
       │
       ├── Web:     use_future + RafDriver::tick(timestamp_ms)
       ├── Desktop:  use_future + WallClock::delta()
       ├── Mobile:   use_future + WallClock::delta()
       └── TUI:      use_future + crossterm poll interval
       │
       ▼
  Signal<T>::set(new_value) → component re-renders
```

### Yew Hook Loop

```
Yew functional component
       │
       ▼
  use_tween() / use_spring()
       │
       ▼
  use_effect → gloo::request_animation_frame loop
       │
       ├── Tween::update(dt) / Spring::update(dt)
       ├── UseStateHandle<T>::set(new_value)    ← triggers VDOM diff
       └── Only the changed node in the VDOM is patched
       │
       ▼
  AnimationAgent (optional) coordinates cross-component animations
  via message passing without parent-child coupling
```

## 6. Type System Design

### The `Animatable` hierarchy

```
Interpolate
  └── .lerp(&self, other: &Self, t: f32) -> Self

         │ blanket impl: Interpolate + Clone + 'static

Animatable   ← all generic bounds use this
  ├── Tween<T: Animatable>
  ├── KeyframeTrack<T: Animatable>
  └── SpringN<T: Animatable>
```

### Why `t: f32` everywhere

The progress parameter is always `f32`. This is intentional:
- Animation timing is a display-frequency concern — `f32` precision (24-bit mantissa) is imperceptible at 60fps.
- A second generic `<P>` for the time parameter would double the API surface for no real-world benefit.
- Types like `f64` world coordinates still get full `f64` precision in their `Interpolate` impl — only the incoming `t` is cast.

### Builder pattern everywhere

Every type with more than two fields uses a consuming builder:

```rust
// Every optional field has a sane default.
// No positional argument confusion.
// The compiler enforces T is Animatable before .build().
let t = Tween::new(0.0_f32, 100.0)
    .duration(1.0)
    .easing(Easing::EaseOutBack)
    .delay(0.1)
    .looping(Loop::PingPong)
    .build();
```

### `no_std` strategy

```
With default features:    Uses std heap allocation, wall clock, callbacks
With no_std:              Stack-only. Vec requires `extern crate alloc`.

Available in no_std:
  animato-core  → Easing, Interpolate, Animatable, Update
  animato-tween → Tween<T> (stack allocated), Loop, TweenState
  animato-spring → Spring (stack allocated), SpringConfig
  animato-physics → Inertia, GestureRecognizer, PointerData

NOT available in no_std (require allocation):
  KeyframeTrack<T>, Timeline, Sequence, AnimationDriver,
  WallClock, callbacks, InertiaN<T>, DragState, AnimatoPlugin, RafDriver
```

---

## 7. Feature Flag Strategy

| Feature | What it enables | Required crates |
|---------|----------------|-----------------|
| `default` | `std` + `tween` + `timeline` + `spring` + `driver` | All core crates |
| `std` | Wall clock, callbacks, heap allocation | OS |
| `tween` | `Tween<T>`, `KeyframeTrack<T>` | `animato-tween` |
| `timeline` | `Timeline`, `Sequence`, `stagger` | `animato-timeline` |
| `spring` | `Spring`, `SpringN<T>` | `animato-spring` |
| `path` | Bezier, MotionPath, SVG parser | `animato-path` |
| `physics` | Inertia, DragState, Gesture | `animato-physics` |
| `color` | Perceptual color interpolation | `animato-color`, `palette` |
| `driver` | `AnimationDriver`, Clocks, ScrollDriver | `animato-driver` |
| `gpu` | `GpuAnimationBatch` via wgpu | `animato-gpu`, `wgpu` |
| `bevy` | `AnimatoPlugin` | `animato-bevy`, bevy crates |
| `wasm` | `RafDriver` + WASM binding | `animato-wasm`, `wasm-bindgen` |
| `leptos` | Signal-backed hooks, scroll, presence, transitions, FLIP lists, gestures, SSR | `animato-leptos`, `leptos` |
| `dioxus` | Cross-platform hooks, scroll, presence, transitions, FLIP lists, gestures, native | `animato-dioxus`, `dioxus` |
| `yew` | Hook/agent animation, scroll, presence, transitions, FLIP lists, gestures | `animato-yew`, `yew` |
| `js` | WASM-compiled NPM package for React, Svelte, Vue, Angular, vanilla JS | `animato-js`, `wasm-bindgen` |
| `serde` | `Serialize`/`Deserialize` on all public types | `serde` |
| `tokio` | `.wait().await` on timelines | `tokio` |

**User decision guide:**

| You are building... | `Cargo.toml` features |
|---------------------|----------------------|
| TUI / CLI app | `default` |
| Bevy game | `bevy` |
| WASM web app (raw) | `wasm` |
| Leptos web app | `leptos` |
| Dioxus cross-platform app | `dioxus` |
| Yew web app | `yew` |
| React / Svelte / Vue (via WASM) | `js` (build with `wasm-pack`) |
| GPU particle system | `gpu` |
| Embedded / no_std | `default-features = false` |
| Everything | `default,path,physics,color,gpu,leptos,dioxus,yew,serde,tokio` |

---

## 8. Error Handling Strategy

Animato uses **no `Result` in hot paths**. Animation update functions never fail. They clamp, saturate, or silently correct invalid input.

| Situation | Behavior |
|-----------|----------|
| `t` outside `[0, 1]` in easing | Clamped to `[0, 1]` silently |
| `duration = 0.0` | Immediately complete, returns `end` value |
| `duration < 0.0` | Treated as `0.0` — immediately complete |
| `dt < 0.0` | Treated as `0.0` — no backward time |
| `KeyframeTrack` with 0 frames | Returns `None` |
| `KeyframeTrack` with 1 frame | Returns that frame's value always |
| Spring with `stiffness = 0.0` | Returns `target` immediately |
| Inertia reaches bounds | Position clamps to bounds and velocity becomes `0.0` |
| `seek()` with `t > 1.0` | Clamped to `1.0` |

`Result` is only returned by builders that validate user-provided data at construction time (e.g. if `duration < 0.0` is given, `TweenBuilder::build()` can return `Err(AnimatoError::InvalidDuration)`).

---

## 9. Testing Strategy

### Unit tests — inline in each source file

Every module has `#[cfg(test)]` at the bottom. Required tests:

| Crate / Module | Required tests |
|----------------|----------------|
| `animato-core / traits.rs` | `f32` lerp endpoints, midpoint, `[f32; 4]` independence |
| `animato-core / easing.rs` | Every variant: `apply(0)=0`, `apply(1)=1`, no panic on out-of-range `t` |
| `animato-tween / tween.rs` | Start value, end value, complete flag, delay, seek, reverse, large-dt safety, PingPong reversal |
| `animato-tween / keyframe.rs` | Single frame, two frames, multi-frame, looping, PingPong, out-of-range query |
| `animato-timeline / timeline.rs` | Concurrent play, sequential play, seek, pause/resume, loop, callback fires |
| `animato-spring / spring.rs` | Settles to target, stiff settles fast, damping=0 oscillates, SpringN for `[f32; 3]` |
| `animato-driver / driver.rs` | Completed removed automatically, cancel mid-animation, `active_count`, thread-safe add |
| `animato-driver / clock.rs` | MockClock returns correct fixed dt, ManualClock advance+delta |
| `animato-path / bezier.rs` | position(0)=start, position(1)=end, arc-length monotonicity |
| `animato-physics / inertia.rs` | friction settle, negative dt, bounds clamp/stop, multi-axis inertia |
| `animato-physics / drag.rs` | axis constraints, pointer id capture, velocity EMA, grid snap |
| `animato-physics / gesture.rs` | tap, double tap, long press, swipe, pinch, rotation |

### Integration tests — `tests/` at workspace root

```
tests/
├── tween_lifecycle.rs         — full tween lifecycle using MockClock
├── spring_settles.rs          — spring reaches target within N steps, all presets
├── keyframe_looping.rs        — long-running looping track stays in bounds
├── timeline_sequence.rs       — multi-step sequence completes in correct order
└── physics_input.rs           — drag, inertia, swipe, pinch, rotation via facade
```

### Benchmark suite — `benches/`

```
benches/
├── easing_bench.rs            — all shipped easing variants via criterion
├── tween_update_bench.rs      — update() throughput, 1 and 10,000 tweens
├── spring_bench.rs            — spring settle time across all presets
└── physics_bench.rs           — inertia, drag, and gesture throughput
```

Run with: `cargo bench`

### CI matrix (`.github/workflows/ci.yml`)

```yaml
- cargo test --workspace                          # all tests
- cargo test --workspace --no-default-features    # no_std compile check
- cargo clippy --workspace --all-features         # linting
- cargo doc --workspace --all-features            # docs build
- cargo bench --workspace --no-run               # benches compile
- wasm-pack test --headless --chrome              # wasm feature
```

---

## 10. Performance Guidelines

### Zero-cost in the common case

- All easing functions are `#[inline]` — compiled to 2–5 float operations at call site.
- `Tween<T>` is stack-allocated; its `update()` is a handful of float multiplications.
- `Interpolate` blanket impls on primitives compile to a scalar multiply-add.
- `Easing::apply()` is a match on a local enum — branch predictor handles it well after the first few frames.
- `KeyframeTrack::update()` binary-searches a `Vec` — fast for any reasonable number of keyframes (< 1000).

### When allocation happens

| Type | When | Cost |
|------|------|------|
| `KeyframeTrack<T>` | `.push()` at build time | One Vec realloc |
| `Timeline` | `.add()` at build time | One Vec realloc |
| `AnimationDriver` | `.add()` at runtime | One Box allocation |
| Callbacks | `on_complete()` at build time | One Box allocation |

No allocation happens during `.update()` or `.value()` calls in the normal path.

### Avoiding dynamic dispatch in tight loops

If you are updating thousands of values per frame (particles, procedural effects), skip `AnimationDriver` and keep a `Vec<Tween<f32>>` directly:

```rust
// Monomorphized — no vtable, compiler can vectorize:
for tween in tweens.iter_mut() {
    tween.update(dt);
}
```

### GPU batch for extreme scale

For 10,000+ concurrent `Tween<f32>` values, use `animato-gpu`. The batch API centralizes updates, embeds the WGSL easing shader, and falls back to exact CPU evaluation whenever GPU setup or easing support is unavailable.

---

## 11. Integration Targets

### TUI / CLI (ratatui)

```rust
use animato::{Tween, Easing, WallClock, Update};

struct App { progress: Tween<f32> }

fn main() {
    let mut app = App {
        progress: Tween::new(0.0_f32, 1.0)
            .duration(2.0)
            .easing(Easing::EaseInOutCubic)
            .build(),
    };
    let mut clock = WallClock::new();

    loop {
        app.progress.update(clock.delta());
        terminal.draw(|f| {
            let pct = (app.progress.value() * 100.0) as u16;
            f.render_widget(Gauge::default().percent(pct), area);
        })?;
        if app.progress.is_complete() { break; }
    }
}
```

**Example files to ship:**
- `examples/tui_progress.rs` — animated Gauge widget
- `examples/tui_spinner.rs` — braille spinner via `KeyframeTrack<&str>`
- `examples/tui_bounce.rs` — bouncing element via `Spring`

### Web / WASM

Build with `wasm-pack build --target web --features wasm`.

```rust
use wasm_bindgen::prelude::*;
use animato::{Tween, Easing, Update};
use animato_wasm::RafDriver;

#[wasm_bindgen]
pub struct App {
    tween:  Tween<f32>,
    driver: RafDriver,
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            tween: Tween::new(0.0_f32, 500.0)
                .duration(1.5)
                .easing(Easing::EaseOutBounce)
                .build(),
            driver: RafDriver::new(),
        }
    }
    pub fn tick(&mut self, ts: f64) {
        let dt = self.driver.tick(ts);
        self.tween.update(dt);
    }
    pub fn value(&self) -> f32 { self.tween.value() }
}
```

### Bevy

```rust
use bevy::prelude::*;
use animato_bevy::{AnimatoPlugin, TweenCompleted};
use animato::{AnimatoTween, Easing, Tween};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AnimatoPlugin)
        .add_systems(Startup, spawn)
        .add_systems(Update, on_done)
        .run();
}

fn spawn(mut commands: Commands) {
    commands.spawn((
        Sprite::default(),
        Transform::default(),
        AnimatoTween::translation(
            Tween::new([0.0_f32, 0.0, 0.0], [200.0, 0.0, 0.0])
                .duration(0.8)
                .easing(Easing::EaseOutBack)
                .build(),
        ),
    ));
}

fn on_done(mut messages: MessageReader<TweenCompleted>) {
    for message in messages.read() {
        println!("Entity {:?} finished animating", message.entity);
    }
}
```

### `no_std` / Embedded

```toml
[dependencies]
animato-core  = { version = "1.0", default-features = false }
animato-tween = { version = "1.0", default-features = false }
animato-spring = { version = "1.0", default-features = false }
animato-path = { version = "1.0", default-features = false }
animato-physics = { version = "1.0", default-features = false }
animato-color = { version = "1.0", default-features = false }
```

Available: `Easing`, `Tween<T>`, `Spring`, `SpringConfig`, fixed Bezier curves, `Inertia`, `GestureRecognizer`, `InLab<C>`, `InOklch<C>`, `InLinear<C>`, and all `Interpolate` blanket impls.

---

## 12. CI / CD Pipeline

### `ci.yml` — runs on every PR and push to main

```
Jobs:
  test:
    matrix: [stable, beta, nightly]
    steps:
      - cargo test --workspace --all-features
      - cargo test --workspace --no-default-features
      - cargo clippy --workspace --all-features -- -D warnings
      - cargo fmt --check

  docs:
    - cargo doc --workspace --all-features --no-deps

  wasm:
    - cargo check -p animato-wasm --target wasm32-unknown-unknown --features wasm-dom
    - cd examples/wasm_counter && wasm-pack test --headless --chrome

  bench:
    - cargo bench --workspace --no-run

  coverage:
    - cargo llvm-cov --workspace --all-features --fail-under-lines 90

  fuzz:
    - cargo +nightly fuzz run svg_path_parser -- -max_total_time=60
```

### `publish.yml` — runs on version tags (`v*`)

```
Steps:
  - Verify tag matches version in each Cargo.toml
  - Run fmt, clippy, tests, docs, examples, WASM, no_std, coverage, fuzz, and bench gates
  - cargo publish --dry-run immediately before each crate publish
  - cargo publish -p animato-core
  - cargo publish -p animato-tween
  - ... (in dependency order)
  - cargo publish -p animato
  - Create GitHub Release and deploy the WASM counter example to GitHub Pages
```

---

## 13. Publishing Checklist

Before `cargo publish` for any crate:

- [ ] All `pub` items have `///` doc comments with at least one example
- [ ] `README.md` has a quick-start example that compiles with `cargo test --doc`
- [ ] `CHANGELOG.md` has an entry for this version
- [ ] `LICENSE-MIT` and `LICENSE-APACHE` are present at workspace root
- [ ] `cargo test --workspace` passes — zero warnings
- [ ] `cargo test --workspace --no-default-features` passes
- [ ] `cargo test --workspace --all-features` passes
- [ ] `cargo clippy --workspace --all-features -- -D warnings` is clean
- [ ] `cargo doc --workspace --all-features --open` renders correctly
- [ ] `cargo bench --workspace --no-run` compiles without errors
- [ ] `cargo llvm-cov --workspace --all-features --fail-under-lines 90` passes
- [ ] `cargo +nightly fuzz run svg_path_parser -- -max_total_time=60` passes
- [ ] `cargo test -p animato --all-features --examples` compiles every registered example
- [ ] Version in `Cargo.toml` matches git tag and `CHANGELOG.md` entry
- [ ] `cargo publish --dry-run` succeeds for each crate immediately before publishing it

### Publish order (dependency chain)

```
animato-core → animato-tween → animato-spring → animato-path → animato-physics
          → animato-color → animato-driver → animato-timeline
          → animato-gpu → animato-bevy → animato-wasm
          → animato-leptos → animato-dioxus → animato-yew → animato-js → animato
```

---

## 14. Naming & Style Conventions

### Crate naming

`animato-{concern}` — Latin prefix, lowercase, hyphen-separated.  
The facade crate is simply `animato`.

### Type naming

| Type | Convention | Example |
|------|------------|---------|
| Structs | `PascalCase`, generic over `T` where needed | `Tween<T>`, `SpringN<T>` |
| Enums | `PascalCase` | `Easing`, `Loop`, `TweenState` |
| Traits | `PascalCase`, verb-like for behavior traits | `Interpolate`, `Update` |
| Config structs | `{Type}Config` | `SpringConfig`, `GestureConfig` |
| ID newtypes | `{Type}Id` over `u64` | `AnimationId` |
| State enums | `{Type}State` | `TweenState`, `TimelineState` |
| Events (Bevy) | Past tense `PascalCase` | `TweenCompleted`, `SpringSettled` |

### Public vs private fields

| Field type | Visibility |
|------------|------------|
| Configuration (`duration`, `easing`, `stiffness`) | `pub` — users may inspect and mutate |
| Internal state (`elapsed`, `velocity`, `loop_count`) | Private — managed exclusively by `Update` |

### Module-level documentation

Every `lib.rs` must have a crate-level `//!` doc block with:
1. One-sentence summary
2. Quick-start example (compiles as a `cargo test --doc`)
3. Feature flags table
4. Link to the `animato` facade crate

---

*Document version: 1.4.0 — covers architecture through Animato 1.0.0 core + Leptos 1.1.0 + Dioxus 1.2.0 + Yew 1.3.0 + JS 1.4.0*  
*Project: Aarambh Dev Hub — github.com/AarambhDevHub/animato*
