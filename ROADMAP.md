# Animato — Project Roadmap

> *Italian: animato — animated, lively, with life and movement.*
> A professional-grade, renderer-agnostic animation library for Rust.

This roadmap tracks every planned release from `v0.1.0` through `v1.0.0`.  
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
| `v0.5.0` | Physics | Inertia, drag, gesture recognition | 📋 |
| `v0.6.0` | Color | Perceptual color interpolation (Lab, Oklch, Linear) | 📋 |
| `v0.7.0` | Integrations | Bevy plugin, WASM/rAF driver, DOM plugins | 📋 |
| `v0.8.0` | Advanced | Shape morphing, scroll-linked, layout animation (FLIP) | 📋 |
| `v0.9.0` | Performance | GPU batch compute, benchmarks, no_std hardening | 📋 |
| `v1.0.0` | Stable | API freeze, full docs, examples, all CI green | 📋 |

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
- [ ] `InertiaConfig` with `friction`, `min_velocity`, and optional `bounds`
- [ ] Presets: `smooth()`, `snappy()`, `heavy()`
- [ ] `Inertia` — 1D friction deceleration from an initial velocity
- [ ] `InertiaN<T: Animatable>` — multi-dimensional inertia
- [ ] `.kick(velocity)` — start inertia from a velocity value
- [ ] `.is_settled() -> bool`

*`drag.rs`*
- [ ] `PointerData` struct (`x`, `y`, `pressure`, `pointer_id`)
- [ ] `DragAxis` enum (`Both`, `X`, `Y`)
- [ ] `DragConstraints` struct (`min_x`, `max_x`, `min_y`, `max_y`, optional `grid_snap`)
- [ ] `DragState` — tracks pointer position, velocity EMA, axis lock, constraints
- [ ] `.on_pointer_down(data)`, `.on_pointer_move(data, dt)`, `.on_pointer_up(data)` → `Option<InertiaN<[f32; 2]>>`

*`gesture.rs`*
- [ ] `GestureConfig` struct (`tap_max_distance`, `tap_max_duration`, `swipe_min_distance`, `long_press_duration`)
- [ ] `Gesture` enum: `Tap`, `DoubleTap`, `LongPress`, `Swipe`, `Pinch`, `Rotation`
- [ ] `SwipeDirection` enum: `Up`, `Down`, `Left`, `Right`
- [ ] `GestureRecognizer` — feeds pointer events, emits `Gesture` on pointer-up

**`animato` facade**
- [ ] `physics` feature flag

---

## v0.6.0 — Color

**Goal:** Animate colors in perceptually uniform spaces so gradients look correct to the human eye, not just mathematically correct.

### Crates shipped

- `animato-color` `v0.6.0` (new)

### Deliverables

**`animato-color`**
- [ ] `InLab<C>` wrapper — interpolates in CIE L\*a\*b\* space
- [ ] `InOklch<C>` wrapper — interpolates in Oklch (modern perceptual space)
- [ ] `InLinear<C>` wrapper — interpolates in linear light (gamma-correct sRGB lerp)
- [ ] `Interpolate` implemented for each wrapper via the `palette` crate
- [ ] Tests: `InLab` red-to-blue midpoint is not a muddy brown
- [ ] Tests: `InLinear` vs `InLab` produce different midpoints (proof the wrapper matters)

**`animato` facade**
- [ ] `color` feature flag (enables `dep:animato-color`, `dep:palette`)
- [ ] `examples/color_animation.rs` — animate background color in Lab space

---

## v0.7.0 — Integrations

**Goal:** First-class support for Bevy, WASM browsers, and ratatui TUIs. A developer can drop `AnimatoPlugin` into Bevy or call `RafDriver::tick()` from a `requestAnimationFrame` callback.

### Crates shipped

- `animato-bevy` `v0.7.0` (new)
- `animato-wasm` `v0.7.0` (new)

### Deliverables

**`animato-bevy`**
- [ ] `AnimatoPlugin` — registers all systems and events
- [ ] `tick_tweens` system — runs in `Update`, calls `.update(time.delta_secs())`
- [ ] `tick_springs` system — same pattern for `SpringN<T>`
- [ ] `TweenCompleted` event — fired when a `Tween` component finishes
- [ ] `SpringSettled` event — fired when a `SpringN` component settles
- [ ] `AnimationLabel` component — optional label for identifying animations in events
- [ ] Tests: Bevy integration test with `App::new()` + plugin, asserts event fires

**`animato-wasm`**
- [ ] `RafDriver` — wraps `AnimationDriver`, converts `timestamp_ms: f64` to `dt: f32`
- [ ] `.pause()`, `.resume()`, `.set_time_scale(f32)`
- [ ] `FlipState` and `FlipAnimation` — FLIP layout transition helpers (`wasm-dom` sub-feature)
- [ ] `SplitText` — splits a DOM text node into character/word spans for individual animation
- [ ] `ScrollSmoother` — momentum scrolling overlay
- [ ] `Draggable` — DOM element drag binding, emits pointer events to `DragState`
- [ ] `Observer` — unified pointer/touch/wheel event abstraction
- [ ] `examples/wasm_counter/` — wasm-pack example with rAF loop

**`animato` facade**
- [ ] `bevy` feature flag
- [ ] `wasm` feature flag (enables `animato-wasm` core)
- [ ] `wasm-dom` sub-feature (enables DOM plugin types)
- [ ] `examples/tui_progress.rs` — ratatui animated progress bar
- [ ] `examples/tui_spinner.rs` — braille spinner via KeyframeTrack

---

## v0.8.0 — Advanced

**Goal:** GSAP-class features — shape morphing, scroll-linked animation, advanced easing, and FLIP layout transitions.

### Deliverables

**`animato-path`**
- [ ] `MorphPath` — point-by-point shape morph with auto-resampling
- [ ] `resample(points: &[[f32; 2]], count: usize) -> Vec<[f32; 2]>` — uniform resampling
- [ ] `DrawSvg` trait — `draw_on(progress: f32) -> f32` and `draw_on_reverse(progress: f32) -> f32` for `stroke-dashoffset` animation

**`animato-driver`**
- [ ] `ScrollDriver` — drives animations from scroll position instead of time
- [ ] `ScrollClock` — `Clock` implementation backed by scroll position

**`animato-core`**
- [ ] Advanced easing variants:
  - [ ] `RoughEase { strength: f32, points: u32 }`
  - [ ] `SlowMo { linear_ratio: f32, power: f32 }`
  - [ ] `Wiggle { wiggles: u32 }`
  - [ ] `CustomBounce { strength: f32 }`
  - [ ] `ExpoScale { start: f32, end: f32 }`

**`animato-wasm`**
- [ ] `LayoutAnimator` — FLIP-style layout transitions with `compute_transitions()` and `css_transform()`
- [ ] `SharedElementTransition` — animate an element between two layout positions

**`animato` facade**
- [ ] `examples/scroll_linked.rs` — scroll-driven animation
- [ ] `examples/morph_path.rs` — shape morphing between two polygons

---

## v0.9.0 — Performance

**Goal:** GPU batch compute for extreme-scale animations, hardened `no_std` support, comprehensive benchmark suite.

### Crates shipped

- `animato-gpu` `v0.9.0` (new)

### Deliverables

**`animato-gpu`**
- [ ] `GpuAnimationBatch` — uploads tween state to GPU, dispatches WGSL compute shader, reads back results
- [ ] `shaders/tween.wgsl` — evaluates all classic easing variants on GPU
- [ ] CPU fallback mode when GPU is unavailable (`new_auto()`)
- [ ] Benchmark: 10,000 tweens per frame on GPU vs CPU

**`animato-core` / `animato-tween` / `animato-spring`**
- [ ] Audit every type for `no_std` correctness
- [ ] `cargo test --workspace --no-default-features` passes with zero warnings
- [ ] Binary size measurement: `no_std` build of core + tween + spring < 10 KB `.text`

**Benchmarks**
- [ ] `benches/easing_bench.rs` — all easing variants via criterion
- [ ] `benches/tween_update_bench.rs` — 1, 100, 10,000 tweens per tick
- [ ] `benches/spring_bench.rs` — settle time for all presets
- [ ] `benches/timeline_bench.rs` — 10-entry timeline tick throughput
- [ ] Benchmark results published to `docs/benchmarks.md`

**`animato` facade**
- [ ] `gpu` feature flag
- [ ] `examples/gpu_particles.rs` — 10,000 particle tweens on GPU

---

## v1.0.0 — Stable

**Goal:** API freeze. Every public item is documented, every example compiles, every feature has integration tests, CI is fully green on stable + beta + nightly.

### Deliverables

**API Stability**
- [ ] Review every `pub` item — deprecate or stabilize
- [ ] `#[deprecated]` on anything being removed before 1.0
- [ ] No `pub` item without a `///` doc comment and a runnable example

**Documentation**
- [ ] `docs/` folder with:
  - [ ] `getting-started.md` — 5-minute guide from `cargo add` to first animation
  - [ ] `concepts.md` — explains Interpolate, Animatable, Update, Clock
  - [ ] `easing-guide.md` — visual descriptions of every easing variant
  - [ ] `migration.md` — guide for anyone migrating from Spanda or other libs
  - [ ] `benchmarks.md` — current benchmark results
- [ ] `cargo doc --all-features` renders zero warnings
- [ ] All examples compile and run with `cargo run --example {name}`

**Testing**
- [ ] ≥ 90% test coverage measured via `cargo-llvm-cov`
- [ ] Integration test for every major integration target (ratatui, WASM, Bevy)
- [ ] Fuzz testing on `SvgPathParser` via `cargo-fuzz`

**CI**
- [ ] `stable`, `beta`, `nightly` all green
- [ ] WASM build (`wasm-pack test --headless --chrome`) green
- [ ] `no_std` compile check green
- [ ] Clippy `--all-features -- -D warnings` green
- [ ] `cargo fmt --check` green
- [ ] Benchmark regression check — fail if easing perf drops > 10%

**Release**
- [ ] `CHANGELOG.md` complete — every change from 0.1.0 → 1.0.0 documented
- [ ] GitHub Release with prebuilt WASM example hosted on GitHub Pages
- [ ] Announcement post on r/rust and Dev.to

---

## Post-1.0 Ideas (Future / `v1.x`)

These are not committed — they are ideas to revisit after the stable release.

| Idea | Notes |
|------|-------|
| `animato-egui` | `EguiAnimatoPlugin` for egui animation helpers |
| `animato-tauri` | Tauri IPC bridge for driving Animato from the JS frontend |
| `animato-dioxus` | Dioxus signal integration for reactive animations |
| `animato-leptos` | Leptos signal/resource integration |
| Declarative animation DSL | A `animato!{ }` proc macro for GSAP-style chaining |
| Spring from velocity | Start a spring with an initial velocity, not just a target |
| Animation recording | Record and replay animation sequences as data |
| `f64` time precision | Optional `dt: f64` for high-precision simulation targets |
| Waveform generators | Sine, sawtooth, square wave as `KeyframeTrack` presets |
| Interpolation extensions | Quaternion slerp, matrix lerp for 3D work |

---

## Contributing to Animato

See [`CONTRIBUTING.md`](./CONTRIBUTING.md) for how to set up the workspace, run tests, and submit pull requests.

The best way to contribute right now is to pick any unchecked item from `v0.5.0` above and open a PR.

---

*Roadmap version: 0.4.0 — last updated May 2026*  
*v0.4.0 shipped — next milestone: v0.5.0 — Physics*  
*Project: Aarambh Dev Hub — github.com/AarambhDevHub/animato*
