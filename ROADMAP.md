# Animato — Project Roadmap

> *Italian: animato — animated, lively, with life and movement.*
> A professional-grade, renderer-agnostic animation library for Rust.

This roadmap tracks every planned release from `v0.1.0` through `v1.3.0`.  
Each milestone is a working, published crate — not a draft. Nothing ships without tests, docs, and benchmarks.

---

## Status Legend

| Symbol | Meaning |
|--------|---------|
| ✅ | Complete |
| 🔄 | In progress |
| 📋 | Planned |
| 🔮 | Future / post-1.0 |

---

## Release Overview

| Version | Name | Focus | Status |
|---------|------|-------|--------|
| `v0.1.0` | Foundation | Core traits, easing, tween, spring, driver | ✅ |
| `v0.2.0` | Composition | Keyframe tracks, timeline, sequence, stagger | ✅ |
| `v0.3.0` | Control | Time scale, callbacks, advanced easing | ✅ |
| `v0.4.0` | Paths | Bezier, motion paths, CatmullRom, SVG parsing | ✅ |
| `v0.5.0` | Physics | Inertia, drag, gesture recognition | ✅ |
| `v0.6.0` | Color | Perceptual color interpolation (Lab, Oklch, Linear) | ✅ |
| `v0.7.0` | Integrations | Bevy plugin, WASM/rAF driver, DOM plugins | ✅ |
| `v0.8.0` | Advanced | Shape morphing, scroll-linked, layout animation (FLIP) | ✅ |
| `v0.9.0` | Performance | GPU batch compute, benchmarks, no_std hardening | ✅ |
| `v1.0.0` | Stable | API freeze, full docs, examples, all CI green | ✅ |
| `v1.1.0` | Leptos | Signal-backed hooks, scroll, presence, transitions, FLIP lists, gestures, SSR | 📋 |
| `v1.2.0` | Dioxus | Cross-platform hooks, scroll, presence, transitions, FLIP lists, gestures, native | 📋 |
| `v1.3.0` | Yew | Hook/agent animation, scroll, presence, transitions, FLIP lists, gestures | 📋 |

---

## v0.1.0 — Foundation

**Goal:** The smallest useful version of Animato. A developer can animate a single value from A to B, drive it with a clock, and use it in any Rust project.

### Crates shipped

- `animato-core` `v0.1.0`
- `animato-tween` `v0.1.0`
- `animato-spring` `v0.1.0`
- `animato-driver` `v0.1.0`
- `animato` `v0.1.0` (facade — default features only)

### Deliverables

**`animato-core`**
- [x] `Interpolate` trait with blanket impls for `f32`, `f64`, `[f32; 2]`, `[f32; 3]`, `[f32; 4]`, `i32`, `u8`
- [x] `Animatable` blanket impl (auto-derived from `Interpolate + Clone + Send + 'static`)
- [x] `Update` trait (`fn update(&mut self, dt: f32) -> bool`)
- [x] `Easing` enum — all 31 classic variants (Linear, Polynomial × 12, Sine × 3, Expo × 3, Circ × 3, Back × 3, Elastic × 3, Bounce × 3)
- [x] `Easing::apply(t: f32) -> f32` with internal `t` clamping
- [x] Free easing functions (`ease_out_cubic(t: f32) -> f32`, etc.) for zero-overhead use
- [x] `Easing::all_named() -> &'static [Easing]`
- [x] `no_std` compile gate (`#![cfg_attr(not(feature = "std"), no_std)]`)
- [x] Full doc comments on every public item
- [x] Test: every variant satisfies `apply(0.0) == 0.0` and `apply(1.0) == 1.0`
- [x] Test: no panic on `t` outside `[0, 1]`

**`animato-tween`**
- [x] `Tween<T: Animatable>` struct (stack-allocated)
- [x] `TweenBuilder<T>` with consuming builder pattern
- [x] `TweenState` enum (`Idle`, `Running`, `Paused`, `Completed`)
- [x] `Update for Tween<T>` — delay handling, elapsed advancement, completion detection
- [x] `.value() -> T` — hot path, no allocation
- [x] `.progress() -> f32` and `.eased_progress() -> f32`
- [x] `.is_complete()`, `.reset()`, `.seek(t: f32)`, `.reverse()`
- [x] `.pause()` and `.resume()`
- [x] `Loop` enum (`Once`, `Times(u32)`, `Forever`, `PingPong`)
- [x] Time scale support (`.time_scale(f32)`)
- [x] `snap_to(value, grid)` and `round_to(value, decimals)` free functions
- [x] `no_std` compatible — no heap allocation
- [x] Tests: start/end values, delay, seek, reverse, large-dt, PingPong direction

**`animato-spring`**
- [x] `Spring` struct (stack-allocated, `no_std`)
- [x] `SpringConfig` with `stiffness`, `damping`, `mass`, `epsilon`
- [x] Presets: `gentle()`, `wobbly()`, `stiff()`, `slow()`, `snappy()`
- [x] Semi-implicit Euler integration
- [x] RK4 integration behind `.use_rk4(true)` flag
- [x] `is_settled()` with epsilon-based detection
- [x] `snap_to(pos)` — teleport without animation
- [x] `SpringN<T: Animatable>` — multi-dimensional spring via component decomposition (sealed `Decompose` trait)
- [x] `Update for Spring` and `Update for SpringN<T>`
- [x] Tests: settles to target for all presets, damping=0 oscillates, SpringN for `[f32; 3]`

**`animato-driver`**
- [x] `AnimationDriver` — owns `Vec<Box<dyn Update + Send>>`, retain-drain pattern
- [x] `AnimationId` newtype over `u64` — `Copy + Hash + Eq`
- [x] `.add()` returns `AnimationId`
- [x] `.tick(dt)` — ticks all, auto-removes completed
- [x] `.cancel(id)`, `.cancel_all()`, `.active_count()`, `.is_active(id)`
- [x] `Clock` trait (`fn delta(&mut self) -> f32`)
- [x] `WallClock` (requires `std`)
- [x] `ManualClock` — caller provides dt via `.advance(dt)`
- [x] `MockClock` — fixed-step for deterministic tests
- [x] Tests: auto-removal, cancel, active_count, MockClock correctness

**`animato` facade**
- [x] Feature flags: `default`, `std`, `tween`, `spring`, `driver`, `serde`
- [x] Re-exports all public APIs behind `#[cfg(feature)]` guards
- [x] Facade-level `lib.rs` doc with quick-start example

**Documentation & Infrastructure**
- [x] `README.md` with installation, quick-start, feature table
- [x] `ARCHITECTURE.md` (done)
- [x] `ROADMAP.md` (this file)
- [x] `CONTRIBUTING.md`
- [x] `CHANGELOG.md` with `## [0.1.0]` entry
- [x] `LICENSE-MIT` and `LICENSE-APACHE`
- [x] `.github/workflows/ci.yml` — test (stable/beta/nightly), clippy, fmt, docs, no_std, bench compile
- [x] `.github/workflows/publish.yml` — pre-verify gate + dep-ordered crates.io publish
- [x] `examples/basic_tween.rs`
- [x] `examples/spring_demo.rs`
- [x] `benches/easing_bench.rs`, `tween_update_bench.rs`, `spring_bench.rs`
- [x] `tests/tween_lifecycle.rs`, `tests/spring_settles.rs`, `tests/driver_lifecycle.rs`
- [x] `cargo publish --dry-run` passes for all crates (run before tagging v0.1.0)

---

## v0.2.0 — Composition

**Goal:** Compose multiple animations. A developer can build a timeline of concurrent tweens or a sequence where each step plays after the previous one.

### Crates shipped

- `animato-timeline` `v0.2.0` (new)
- All previous crates bumped to `v0.2.0`

### Deliverables

**`animato-timeline`**
- [x] `Timeline` struct with `Vec<TimelineEntry>` internally
- [x] `TimelineState` enum (`Idle`, `Playing`, `Paused`, `Completed`)
- [x] `.add(label, anim, At)` builder method
- [x] `At` enum: `Absolute(f32)`, `Start`, `End`, `Label(&str)`, `Offset(f32)`
- [x] `.play()`, `.pause()`, `.resume()`, `.reset()`
- [x] `.seek(t: f32)` — normalized seek
- [x] `.seek_abs(secs: f32)` — absolute time seek
- [x] `.duration() -> f32`, `.progress() -> f32`, `.is_complete() -> bool`
- [x] `Loop` support on `Timeline`
- [x] `Sequence` builder: `.then(label, anim)`, `.then_for(label, anim, duration)`, `.gap(secs)`, `.build() -> Timeline`
- [x] `stagger(animations, delay) -> Timeline`
- [x] `Update for Timeline` — ticks entries within their time window for normal playback
- [x] Tests: concurrent play, sequential play, seek, pause, loop, stagger order

**`animato-tween`**
- [x] `KeyframeTrack<T: Animatable>` with sorted `Vec<Keyframe<T>>`
- [x] `Keyframe<T>` struct (`time: f32`, `value: T`, `easing: Easing`)
- [x] `.push()` and `.push_eased()` builder methods
- [x] Binary-search interpolation in `.value_at(t: f32) -> Option<T>`
- [x] PingPong loop logic in `KeyframeTrack`
- [x] Tests: empty, single frame, two frames, multi-frame, looping, PingPong

**`animato` facade**
- [x] Add `timeline` feature flag
- [x] Re-export `Timeline`, `Sequence`, `At`, `stagger`
- [x] Re-export `Keyframe`, `KeyframeTrack`, and `Playable`
- [x] `examples/timeline_sequence.rs`
- [x] `examples/keyframe_track.rs`

---

## v0.3.0 — Control

**Goal:** Fine-grained runtime control and ergonomics. Time scale, callbacks, and advanced easing.

### Deliverables

**`animato-timeline`**
- [x] Callbacks (`std` feature): `.on_entry_complete(label, f)`, `.on_complete(f)`
- [x] `tokio` feature: `.wait().await` resolves when timeline completes
- [x] Time scale on `Timeline` (`.time_scale(f32)`, `.set_time_scale(f32)`)

**`animato-core`**
- [x] `CubicBezier(f32, f32, f32, f32)` easing variant (CSS-compatible)
- [x] `Steps(u32)` easing variant
- [x] Tests for new easing variants

Advanced GSAP-style easing variants remain assigned to `v0.8.0 — Advanced`.

**`animato` facade**
- [x] `serde` feature exports `Serialize`/`Deserialize` on supported concrete core types
- [x] `tokio` feature passes through to `animato-timeline`
- [x] `examples/keyframe_track.rs` with looping + PingPong demo

---

## v0.4.0 — Paths

**Goal:** Animate along curves. A developer can move an object along a quadratic Bezier, a CatmullRom spline, or a path parsed from an SVG `d` attribute.

### Crates shipped

- `animato-path` `v0.4.0` (new)

### Deliverables

**`animato-path`**

*`bezier.rs`*
- [x] `QuadBezier` — quadratic Bezier curve with `position(t)` and `tangent(t)`
- [x] `CubicBezierCurve` — cubic Bezier path curve with `position(t)` and `tangent(t)`
- [x] `CatmullRomSpline` — smooth interpolating spline through control points
- [x] `PathEvaluate` trait: `position(t)`, `tangent(t)`, `rotation_deg(t)`, `arc_length()`
- [x] Arc-length parameterization via numerical integration (uniform `t` → uniform distance)

*`motion.rs`*
- [x] `MotionPath` — chain of `PathEvaluate` segments into one unified path
- [x] `MotionPathTween` — drives `t ∈ [0, 1]` via an internal `Tween<f32>`, returns `[f32; 2]`
- [x] Auto-rotate: `.auto_rotate(true)` aligns the object's heading to the path tangent
- [x] Start/end offsets: `.start_offset(0.1).end_offset(0.9)` trims the path

*`poly.rs`*
- [x] `PolyPath` — smooth path through arbitrary points via CatmullRom + arc-length param
- [x] `CompoundPath` — sequence of heterogeneous segments (line, quad, cubic, arc)
- [x] `PathCommand` enum used internally by `SvgPathParser` and `CompoundPath`

*`svg.rs`*
- [x] `SvgPathParser::parse(d: &str) -> Vec<PathCommand>`
- [x] Support for `M`, `L`, `H`, `V`, `C`, `Q`, `A`, `Z` commands
- [x] Support for relative (lowercase) variants of all commands

**`animato` facade**
- [x] `path` feature flag
- [x] `examples/motion_path.rs` — object moves along a Bezier curve
- [x] `tests/path_arc_length.rs` — arc-length monotonicity and endpoint tests

---

## v0.5.0 — Physics

**Goal:** Input-driven physics. Inertia (friction deceleration after a drag), drag tracking with velocity estimation, and gesture recognition.

### Crates shipped

- `animato-physics` `v0.5.0` (new)

### Deliverables

**`animato-physics`**

*`inertia.rs`*
- [x] `InertiaConfig` with `friction`, `min_velocity`, and optional `bounds`
- [x] Presets: `smooth()`, `snappy()`, `heavy()`
- [x] `Inertia` — 1D friction deceleration from an initial velocity
- [x] `InertiaN<T: Animatable>` — multi-dimensional inertia
- [x] `.kick(velocity)` — start inertia from a velocity value
- [x] `.is_settled() -> bool`

*`drag.rs`*
- [x] `PointerData` struct (`x`, `y`, `pressure`, `pointer_id`)
- [x] `DragAxis` enum (`Both`, `X`, `Y`)
- [x] `DragConstraints` struct (`min_x`, `max_x`, `min_y`, `max_y`, optional `grid_snap`)
- [x] `DragState` — tracks pointer position, velocity EMA, axis lock, constraints
- [x] `.on_pointer_down(data)`, `.on_pointer_move(data, dt)`, `.on_pointer_up(data)` → `Option<InertiaN<[f32; 2]>>`

*`gesture.rs`*
- [x] `GestureConfig` struct (`tap_max_distance`, `tap_max_duration`, `swipe_min_distance`, `long_press_duration`)
- [x] `Gesture` enum: `Tap`, `DoubleTap`, `LongPress`, `Swipe`, `Pinch`, `Rotation`
- [x] `SwipeDirection` enum: `Up`, `Down`, `Left`, `Right`
- [x] `GestureRecognizer` — feeds pointer events, emits `Gesture` on pointer-up

**`animato` facade**
- [x] `physics` feature flag

---

## v0.6.0 — Color

**Goal:** Animate colors in perceptually uniform spaces so gradients look correct to the human eye, not just mathematically correct.

### Crates shipped

- `animato-color` `v0.6.0` (new)

### Deliverables

**`animato-color`**
- [x] `InLab<C>` wrapper — interpolates in CIE L\*a\*b\* space
- [x] `InOklch<C>` wrapper — interpolates in Oklch (modern perceptual space)
- [x] `InLinear<C>` wrapper — interpolates in linear light (gamma-correct sRGB lerp)
- [x] `Interpolate` implemented for each wrapper via the `palette` crate
- [x] Tests: `InLab` red-to-blue midpoint is not a muddy brown
- [x] Tests: `InLinear` vs `InLab` produce different midpoints (proof the wrapper matters)

**`animato` facade**
- [x] `color` feature flag (enables `dep:animato-color`, `dep:palette`)
- [x] `examples/color_animation.rs` — animate background color in Lab space

---

## v0.7.0 — Integrations

**Goal:** First-class support for Bevy, WASM browsers, and ratatui TUIs. A developer can drop `AnimatoPlugin` into Bevy or call `RafDriver::tick()` from a `requestAnimationFrame` callback.

### Crates shipped

- `animato-bevy` `v0.7.0` (new)
- `animato-wasm` `v0.7.0` (new)

### Deliverables

**`animato-bevy`**
- [x] `AnimatoPlugin` — registers common tween/spring systems and completion messages
- [x] `tick_tweens` system — runs in `Update`, calls `.update(time.delta_secs())`
- [x] `tick_springs` system — same pattern for `SpringN<T>`
- [x] `TweenCompleted` message — fired when an `AnimatoTween<T>` finishes
- [x] `SpringSettled` message — fired when an `AnimatoSpring<T>` settles
- [x] `AnimationLabel` component — optional label for identifying animations in messages
- [x] Tests: Bevy integration test with `App::new()` + plugin, asserts message fires

**`animato-wasm`**
- [x] `RafDriver` — wraps `AnimationDriver`, converts `timestamp_ms: f64` to `dt: f32`
- [x] `.pause()`, `.resume()`, `.set_time_scale(f32)`
- [x] `FlipState` and `FlipAnimation` — FLIP layout transition helpers (`wasm-dom` sub-feature)
- [x] `SplitText` — splits a DOM text node into character/word spans for individual animation
- [x] `ScrollSmoother` — momentum scrolling overlay
- [x] `Draggable` — DOM element drag binding, emits pointer events to `DragState`
- [x] `Observer` — unified pointer/wheel event abstraction
- [x] `examples/wasm_counter/` — wasm-pack example with rAF loop

**`animato` facade**
- [x] `bevy` feature flag
- [x] `wasm` feature flag (enables `animato-wasm` core)
- [x] `wasm-dom` sub-feature (enables DOM plugin types)
- [x] `examples/tui_progress.rs` — ratatui animated progress bar
- [x] `examples/tui_spinner.rs` — braille spinner via KeyframeTrack

---

## v0.8.0 — Advanced

**Goal:** GSAP-class features — shape morphing, scroll-linked animation, advanced easing, and FLIP layout transitions.

### Deliverables

**`animato-path`**
- [x] `MorphPath` — point-by-point shape morph with auto-resampling
- [x] `resample(points: &[[f32; 2]], count: usize) -> Vec<[f32; 2]>` — uniform resampling
- [x] `DrawSvg` trait — `draw_on(progress: f32) -> f32` and `draw_on_reverse(progress: f32) -> f32` for `stroke-dashoffset` animation
- [x] `DrawValues` struct with `to_css()` helper

**`animato-driver`**
- [x] `ScrollDriver` — drives animations from scroll position instead of time
- [x] `ScrollClock` — `Clock` implementation backed by scroll position

**`animato-core`**
- [x] Advanced easing variants:
  - [x] `RoughEase { strength: f32, points: u32 }`
  - [x] `SlowMo { linear_ratio: f32, power: f32 }`
  - [x] `Wiggle { wiggles: u32 }`
  - [x] `CustomBounce { strength: f32 }`
  - [x] `ExpoScale { start: f32, end: f32 }`

**`animato-wasm`**
- [x] `LayoutAnimator` — FLIP-style layout transitions with `compute_transitions()` and `css_transform()`
- [x] `SharedElementTransition` — animate an element between two layout positions

**`animato` facade**
- [x] `examples/scroll_linked.rs` — scroll-driven animation
- [x] `examples/morph_path.rs` — shape morphing between two polygons
- [x] `tests/advanced_easing.rs`
- [x] `tests/morph_path_integration.rs`
- [x] `tests/scroll_driver.rs`

---

## v0.9.0 — Performance

**Goal:** GPU batch compute for extreme-scale animations, hardened `no_std` support, comprehensive benchmark suite.

### Crates shipped

- `animato-gpu` `v0.9.0` (new)

### Deliverables

**`animato-gpu`**
- [x] `GpuAnimationBatch` — batches `Tween<f32>` values with deterministic CPU fallback
- [x] `shaders/tween.wgsl` — evaluates all classic easing variants on GPU-compatible WGSL
- [x] CPU fallback mode when GPU is unavailable (`new_auto()`)
- [x] Benchmark: 10,000 tweens per frame through batch API

**`animato-core` / `animato-tween` / `animato-spring`**
- [x] Audit every type for `no_std` correctness
- [x] `cargo test --workspace --no-default-features` passes with zero warnings
- [x] Bare-metal `alloc` builds for spring/path/physics covered in CI/release gate

**Benchmarks**
- [x] `benches/easing_bench.rs` — all easing variants via criterion
- [x] `benches/tween_update_bench.rs` — 1, 100, 10,000 tweens per tick
- [x] `benches/spring_bench.rs` — settle time for all presets
- [x] `benches/timeline_bench.rs` — 10-entry timeline tick throughput
- [x] Benchmark guide published to `docs/benchmarks.md`

**`animato` facade**
- [x] `gpu` feature flag
- [x] `examples/gpu_particles.rs` — 10,000 particle tweens through the batch API

---

## v1.0.0 — Stable

**Goal:** API freeze. Every public item is documented, every example compiles, every feature has integration tests, CI is fully green on stable + beta + nightly.

### Deliverables

**API Stability**
- [x] Review every `pub` item — existing API stabilized without breaking changes
- [x] No removals before 1.0; no deprecations required
- [x] Every public item guarded by crate-level `#![deny(missing_docs)]`; runnable or target-gated examples documented

**Documentation**
- [x] `docs/` folder with:
  - [x] `README.md` — documentation index
  - [x] `api-full.md` — complete stable API map
  - [x] `getting-started.md` — 5-minute guide from install to first animation
  - [x] `concepts.md` — explains Interpolate, Animatable, Update, Clock
  - [x] feature guides for tween, timeline, spring, path, physics, color, driver, GPU, Bevy, and WASM
  - [x] `migration.md`, `testing.md`, `release.md`, `troubleshooting.md`, `faq.md`, and `benchmarks.md`
- [x] `cargo doc --workspace --all-features --no-deps` renders zero warnings
- [x] All registered examples compile with `cargo test -p animato --all-features --examples`

**Testing**
- [x] >= 90% test coverage gate added via `cargo-llvm-cov`
- [x] Integration test coverage exists for ratatui examples compile, WASM rAF, Bevy, GPU fallback, path, physics, color, drivers, timelines, springs, and tweens
- [x] Fuzz testing scaffold added for `SvgPathParser` via `cargo-fuzz`

**CI**
- [x] `stable`, `beta`, `nightly` test matrix retained
- [x] WASM check and `wasm-pack test --headless --chrome` gate added
- [x] `no_std` compile check retained
- [x] Clippy `--all-features -- -D warnings` gate retained
- [x] `cargo fmt --check` gate retained
- [x] Benchmark compile gate retained; release notes require benchmark baseline capture

**Release**
- [x] `CHANGELOG.md` complete — every change from 0.1.0 to 1.0.0 documented
- [x] GitHub Release workflow updated for v1.0.0 and GitHub Pages WASM example deployment
- [x] Announcement checklist documented in release notes workflow

---

## v1.1.0 — Leptos

**Goal:** First-class Leptos integration. A developer can animate any value with a signal-backed hook, build scroll-triggered animations, mount/unmount transitions, FLIP list reordering, page transitions, drag/gesture-driven motion, and SSR-safe hydration — all with fine-grained reactivity and zero VDOM overhead.

### Crates shipped

- `animato-leptos` `v1.1.0` (new)

### Deliverables

**`animato-leptos` — hooks**
- [ ] `use_tween(from, to, config)` → `(ReadSignal<T>, TweenHandle)` — signal-backed tween with play/pause/resume/reset/reverse/seek/time_scale control
- [ ] `use_spring(initial, config)` → `(ReadSignal<T>, SpringHandle)` — signal-backed spring with set_target/snap_to/is_settled
- [ ] `use_timeline(builder)` → `TimelineHandle` — compose multiple animations with `At` scheduling
- [ ] `use_keyframes(builder)` → `(ReadSignal<T>, KeyframeHandle)` — multi-stop keyframe animation
- [ ] rAF loop management: auto-start on mount, auto-cleanup on unmount, pause on tab visibility change
- [ ] `TweenHandle` and `SpringHandle` expose `is_complete()` and `progress()` as `ReadSignal`

**`animato-leptos` — scroll**
- [ ] `use_scroll_progress(target, config)` → `ReadSignal<f32>` — 0.0..1.0 scroll progress of an element
- [ ] `use_scroll_trigger(target, config)` → `ScrollTriggerHandle` — viewport enter/exit callbacks with threshold, once, scrub, and pin options
- [ ] `use_scroll_velocity()` → `ReadSignal<f32>` — current scroll velocity in px/sec
- [ ] `SmoothScroll` component — momentum scroll container with overscroll damping
- [ ] `ScrollConfig` with axis, offset_start, offset_end, smooth, smooth_factor
- [ ] `ScrollTriggerConfig` with GSAP-style `start`/`end` strings, scrub linking, pin support

**`animato-leptos` — presence**
- [ ] `AnimatePresence` component — mount/unmount transitions with configurable enter/exit animations
- [ ] `PresenceAnimation` struct with duration, easing, from/to `AnimatedStyle`
- [ ] Presets: `fade()`, `slide_up()`, `slide_down()`, `slide_left()`, `slide_right()`, `zoom_in()`, `zoom_out()`, `flip_x()`, `flip_y()`, `blur_in()`, `spring(config)`
- [ ] `wait_exit` flag — delay DOM removal until exit animation completes

**`animato-leptos` — transitions**
- [ ] `PageTransition` component — route-change animation wrapper
- [ ] `TransitionMode` enum: `Sequential`, `Parallel`, `CrossFade`, `SlideOver`, `MorphHero`
- [ ] Integration with `leptos_router` for automatic route detection

**`animato-leptos` — list**
- [ ] `AnimatedFor` component — FLIP-powered list reordering with insert/remove/move animations
- [ ] Configurable enter/exit animations per item
- [ ] `move_duration`, `move_easing`, `stagger_delay` props
- [ ] Automatic layout snapshot and FLIP calculation

**`animato-leptos` — gesture**
- [ ] `use_drag(target, config)` → `(ReadSignal<[f32; 2]>, DragHandle)` — draggable element with axis lock, constraints, inertia, snap points, elastic edges
- [ ] `use_gesture(target, config)` → `ReadSignal<Option<Gesture>>` — tap, double tap, long press, swipe, pinch, rotation
- [ ] `use_pinch(target)` → `(ReadSignal<f32>, PinchHandle)` — pinch-zoom scale signal
- [ ] `use_swipe(target, config)` → `ReadSignal<Option<SwipeEvent>>` — swipe detection with direction and velocity

**`animato-leptos` — CSS**
- [ ] `AnimatedStyle` struct — CSS property bag (opacity, transform, scale, translate, rotate, skew, blur, background_color, border_radius, width, height, clip_path, custom)
- [ ] `css_spring(target, config)` → `ReadSignal<String>` — animate CSS properties with a spring
- [ ] `css_tween(from, to, duration, easing)` → `ReadSignal<String>` — animate CSS properties with a tween

**`animato-leptos` — SSR**
- [ ] `is_hydrating()` → `bool` — skip animations during hydration
- [ ] `use_client_only(server_value)` → `ReadSignal<T>` — returns target value on server, animates on client
- [ ] `SsrFallback` component — renders static fallback during SSR, swaps in animated version after hydration

**`animato` facade**
- [ ] `leptos` feature flag
- [ ] Re-exports all `animato-leptos` public APIs

**Documentation & Examples**
- [ ] `docs/leptos.md` — Leptos integration guide
- [ ] `examples/leptos_basic_tween/` — Leptos app with animated div
- [ ] `examples/leptos_scroll_trigger/` — scroll-triggered entrance animations
- [ ] `examples/leptos_page_transition/` — route transition demo
- [ ] `examples/leptos_animated_list/` — FLIP list reordering demo
- [ ] `examples/leptos_drag_gesture/` — draggable element with inertia

**Testing**
- [ ] Unit tests for all hooks (mock rAF, deterministic dt)
- [ ] Integration tests for SSR guards (signal returns target value on server)
- [ ] WASM compile check: `cargo check -p animato-leptos --target wasm32-unknown-unknown`
- [ ] All examples compile: `cargo test -p animato-leptos --examples`

---

## v1.2.0 — Dioxus

**Goal:** Cross-platform Dioxus integration. The same animation hooks work on web (WASM), desktop (Windows/macOS/Linux), mobile (iOS/Android), and TUI — with platform-adaptive tick sources and native window animation helpers.

### Crates shipped

- `animato-dioxus` `v1.2.0` (new)

### Deliverables

**`animato-dioxus` — hooks**
- [ ] `use_tween(from, to, config)` → `(T, TweenHandle)` — tween hook working on all Dioxus targets
- [ ] `use_spring(initial, config)` → `(T, SpringHandle)` — spring hook with physics
- [ ] `use_timeline(builder)` → `TimelineHandle` — timeline composition
- [ ] `use_keyframes(builder)` → `(T, KeyframeHandle)` — keyframe track
- [ ] Platform-adaptive rAF/clock loop via `PlatformAdapter::detect()`

**`animato-dioxus` — motion**
- [ ] `use_motion(initial)` → `MotionHandle<T>` — all-in-one hook combining tween, spring, and keyframes
- [ ] `MotionHandle::animate_to(target, config)` — tween or spring transition
- [ ] `MotionHandle::keyframes(track)` — play a keyframe track
- [ ] `MotionHandle::stop()`, `snap_to()`, `is_animating()`
- [ ] `MotionConfig` enum: `Tween { duration, easing, delay }`, `Spring(SpringConfig)`

**`animato-dioxus` — scroll**
- [ ] `use_scroll_progress(target, config)` → scroll progress signal (web only)
- [ ] `use_scroll_trigger(target, config)` → viewport enter/exit with scrub and pin (web only)
- [ ] `use_scroll_velocity()` → scroll velocity signal (web only)
- [ ] Graceful no-op on non-web platforms

**`animato-dioxus` — presence, transition, list, gesture**
- [ ] `AnimatePresence` component — same API as `animato-leptos` but using Dioxus `Signal<T>` and RSX
- [ ] `PageTransition` component with `TransitionMode` enum and `dioxus-router` integration
- [ ] `AnimatedFor` component — FLIP-powered list with stagger support
- [ ] `use_drag`, `use_gesture`, `use_pinch`, `use_swipe` — cross-platform pointer/touch bindings
- [ ] Touch gestures work on mobile targets via Dioxus event system

**`animato-dioxus` — platform**
- [ ] `PlatformAdapter::detect()` → `AnimationBackend` (`WebRaf`, `NativeClock`, `TerminalPoll`)
- [ ] Web: uses `RafDriver` from `animato-wasm`
- [ ] Desktop/Mobile: uses `WallClock` with 60fps event loop tick
- [ ] TUI: uses crossterm event poll intervals as tick source

**`animato-dioxus` — native**
- [ ] `use_window_animation(config)` → `WindowAnimationHandle` — animate native window position on desktop
- [ ] `use_window_spring(config)` → `WindowSpringHandle` — spring-based window animation
- [ ] `WindowAnimationHandle::move_to()`, `resize_to()`, `opacity_to()`

**`animato` facade**
- [ ] `dioxus` feature flag
- [ ] Re-exports all `animato-dioxus` public APIs

**Documentation & Examples**
- [ ] `docs/dioxus.md` — Dioxus integration guide (web + desktop + mobile + TUI)
- [ ] `examples/dioxus_web_tween/` — web app with animated elements
- [ ] `examples/dioxus_desktop_spring/` — desktop app with spring-animated window
- [ ] `examples/dioxus_cross_platform/` — single codebase running on web + desktop
- [ ] `examples/dioxus_tui_progress/` — TUI progress bar with Dioxus

**Testing**
- [ ] Unit tests for all hooks (mock clock, deterministic dt)
- [ ] Platform adapter tests (backend detection)
- [ ] WASM compile check: `cargo check -p animato-dioxus --target wasm32-unknown-unknown`
- [ ] Desktop compile check: `cargo check -p animato-dioxus`
- [ ] All examples compile

---

## v1.3.0 — Yew

**Goal:** Full Yew integration with functional component hooks and an `AnimationAgent` for cross-component coordination. Scroll-driven animations, mount/unmount transitions, FLIP list reordering, page transitions, gesture bindings, and CSS helpers.

### Crates shipped

- `animato-yew` `v1.3.0` (new)

### Deliverables

**`animato-yew` — hooks**
- [ ] `use_tween(from, to, config)` → `(UseStateHandle<T>, TweenHandle)` — tween with rAF-gated state updates
- [ ] `use_spring(initial, config)` → `(UseStateHandle<T>, SpringHandle)` — spring with physics
- [ ] `use_timeline(builder)` → `TimelineHandle` — timeline composition
- [ ] `use_keyframes(builder)` → `(UseStateHandle<T>, KeyframeHandle)` — keyframe track
- [ ] Per-frame updates via `gloo::request_animation_frame` to minimize VDOM diff overhead

**`animato-yew` — scroll**
- [ ] `use_scroll_progress(target, config)` → scroll progress state
- [ ] `use_scroll_trigger(target, config)` → viewport enter/exit callbacks with scrub and pin
- [ ] `use_scroll_velocity()` → scroll velocity state

**`animato-yew` — presence, transition, list, gesture, CSS**
- [ ] `AnimatePresence` component — mount/unmount transitions using Yew `Html` and `Callback`
- [ ] `PageTransition` component with `TransitionMode` and `yew-router` integration
- [ ] `AnimatedFor` component — FLIP-powered list reordering
- [ ] `use_drag`, `use_gesture`, `use_pinch`, `use_swipe` — pointer event bindings via Yew `NodeRef`
- [ ] `AnimatedStyle` struct and `css_spring()`, `css_tween()` CSS helpers

**`animato-yew` — agent**
- [ ] `AnimationAgent` — Yew agent for cross-component animation coordination
- [ ] `AgentInput` enum: `AddTween`, `AddSpring`, `Play`, `Pause`, `Reset`, `Cancel`, `CancelAll`, `Tick`
- [ ] `AgentOutput` enum: `ValueChanged`, `Completed`, `Settled`
- [ ] Components subscribe to agent outputs without direct parent-child coupling
- [ ] Agent manages an `AnimationDriver` internally and ticks all registered animations

**`animato` facade**
- [ ] `yew` feature flag
- [ ] Re-exports all `animato-yew` public APIs

**Documentation & Examples**
- [ ] `docs/yew.md` — Yew integration guide
- [ ] `examples/yew_basic_tween/` — Yew app with animated div
- [ ] `examples/yew_scroll_trigger/` — scroll-triggered entrance animations
- [ ] `examples/yew_animated_list/` — FLIP list reordering demo
- [ ] `examples/yew_agent_coordination/` — cross-component animation via agent

**Testing**
- [ ] Unit tests for all hooks (mock rAF, deterministic dt)
- [ ] Agent integration tests (message round-trip, completion events)
- [ ] WASM compile check: `cargo check -p animato-yew --target wasm32-unknown-unknown`
- [ ] All examples compile

---

## Post-1.3 Ideas (Future / `v1.x+`)

These are not committed — they are ideas to revisit after the framework integrations ship.

| Idea | Notes |
|------|-------|
| `animato-egui` | `EguiAnimatoPlugin` for egui animation helpers |
| `animato-tauri` | Tauri IPC bridge for driving Animato from the JS frontend |
| Declarative animation DSL | A `animato!{ }` proc macro for GSAP-style chaining |
| Spring from velocity | Start a spring with an initial velocity, not just a target |
| Animation recording | Record and replay animation sequences as data |
| `f64` time precision | Optional `dt: f64` for high-precision simulation targets |
| Waveform generators | Sine, sawtooth, square wave as `KeyframeTrack` presets |
| Interpolation extensions | Quaternion slerp, matrix lerp for 3D work |

---

## Contributing to Animato

See [`CONTRIBUTING.md`](./CONTRIBUTING.md) for how to set up the workspace, run tests, and submit pull requests.

The best way to contribute right now is to use the v1.0 stable API and open focused issues for bugs, documentation gaps, or post-1.0 feature proposals.

---

*Roadmap version: 1.3.0 — last updated May 2026*  
*v1.0.0 core shipped — framework integrations in progress*  
*Project: Aarambh Dev Hub — github.com/AarambhDevHub/animato*
