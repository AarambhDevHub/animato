# Changelog

All notable changes to Animato will be documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.4.0] — 2026-05-23 — JavaScript

### Added
- `animato-js`: new Rust/WASM crate built with `wasm-bindgen` and published to NPM as `@animato/core`.
- JavaScript exports for tweens, vector tweens, keyframes, timelines, springs, rAF/scroll drivers, motion paths, morphing, draw values, inertia, drag, gestures, color interpolation, DOM helpers, and batch tween evaluation.
- JavaScript utility exports: `version`, `initAnimato`, `availableEasings`, `ease`, `parseEasing`, `snapTo`, `roundTo`, and `interpolateColor`.
- `animato` facade feature `js`, re-exported as `animato::js::*`.
- JavaScript examples for vanilla timeline, React tween, Svelte spring, Vue motion path, and Angular color usage.
- NPM publish workflow, package preparation script, package size check, and local CI mirror script.
- `docs/javascript.md` and `crates/animato-js/README.md`.

### Changed
- Bumped the workspace version and internal dependency pins from `1.3.0`/`1.3` to `1.4.0`/`1.4`.
- Updated README, architecture notes, feature flags, API map, examples, testing, release docs, roadmap, and CI for the JavaScript release.
- Aligned the Yew scroll trigger example with the richer Leptos visual system and kept Yew on the current `0.23` release line.
- Enabled release LTO, single codegen unit, panic abort, and stripping for optimized WASM package output.

### Verification
- `cargo check -p animato-js`
- `cargo test -p animato-js`
- `cargo check -p animato-js --target wasm32-unknown-unknown --all-features`
- `bash scripts/build-js-package.sh`
- `npm run typecheck` and `npm run build` for all JavaScript examples

## [1.3.0] — 2026-05-21 — Yew

### Added
- `animato-yew`: new Yew 0.23 integration crate with `UseStateHandle`-backed tween, spring, timeline, and keyframe hooks.
- Yew CSS, scroll, presence, page transition, keyed list, gesture, and serializable `f32` agent coordination APIs.
- `animato` facade features: `yew`, `yew-csr`, `yew-hydration`, `yew-ssr`, and `yew-agent`.
- Yew examples for basic tweening, scroll triggers, animated lists, drag gestures, page transitions, and agent coordination.
- `docs/yew.md` integration guide.

### Changed
- Bumped all workspace crates and internal dependency pins from `1.2.0`/`1.2` to `1.3.0`/`1.3`.
- Updated README, feature-flag docs, API map, architecture notes, roadmap, CI, and publish workflow for `v1.3.0 — Yew`.
- Added Yew 0.23.0, Yew Router 0.20.0, and Yew Agent 0.5.0 as workspace dependencies.

### Verification
- `cargo check -p animato-yew --all-features`
- `cargo check --manifest-path examples/yew_basic_tween/Cargo.toml --target wasm32-unknown-unknown`

## [1.2.0] — 2026-05-19 — Dioxus

### Added
- `animato-dioxus`: new Dioxus 0.7.9 integration crate with signal-backed tween, spring, timeline, keyframe, and motion hooks.
- Dioxus CSS, presence, page transition, animated list, scroll, gesture, platform, and portable native window helper APIs.
- `animato` facade features: `dioxus`, `dioxus-web`, `dioxus-desktop`, `dioxus-router`, and `dioxus-native`.
- Dioxus examples for web tweening, desktop springs, cross-platform motion, and TUI-style progress.
- `docs/dioxus.md` integration guide.

### Changed
- Bumped all workspace crates and internal dependency pins from `1.1.0`/`1.1` to `1.2.0`/`1.2`.
- Updated README, feature-flag docs, API map, architecture notes, roadmap, CI, and publish workflow for `v1.2.0 — Dioxus`.
- Added Dioxus 0.7.9 and Dioxus Router 0.7.9 as workspace dependencies with minimal library features.

### Verification
- `cargo check -p animato-dioxus`
- `cargo test -p animato-dioxus --all-features`
- `cargo test -p animato --features dioxus --test dioxus_facade`

## [1.1.0] — 2026-05-16 — Leptos

### Added
- `animato-leptos`: new Leptos 0.8 integration crate with signal-backed `use_tween`, `use_spring`, `use_timeline`, and `use_keyframes` hooks.
- Leptos scroll, presence, page transition, FLIP-ready list, gesture, CSS, and SSR helper APIs.
- `animato` facade features: `leptos`, `leptos-csr`, `leptos-hydrate`, and `leptos-ssr`.
- Leptos examples for basic tweening, scroll triggers, page transitions, animated lists, and drag/pinch gestures.
- `docs/leptos.md` integration guide.

### Changed
- Bumped all workspace crates and internal dependency pins from `1.0.0`/`1.0` to `1.1.0`/`1.1`.
- Updated README, feature-flag docs, architecture notes, roadmap, CI, and publish workflow for `v1.1.0 — Leptos`.
- Added Leptos 0.8.19 and Leptos Router 0.8.13 as workspace dependencies.

### Verification
- `cargo test -p animato-leptos --all-features`
- `cargo check -p animato-leptos --all-features`

## [1.0.0] — 2026-05-14 — Stable

### Added
- Stable v1.0.0 documentation set under `docs/`, including `README.md`, `api-full.md`, feature guides, recipes, testing, release, troubleshooting, and FAQ pages.
- v1.0.0 release gates for examples, benchmark compilation, coverage, fuzzing, WASM checks, no_std checks, dry-run publishing, and tag validation.
- `cargo-fuzz` scaffold with a `SvgPathParser` fuzz target.
- GitHub Pages release job for the WASM counter example artifact.

### Changed
- Bumped all workspace crates and internal dependency pins from `0.9.0`/`0.9` to `1.0.0`/`1.0`.
- Stabilized the existing v0.9 public API as the v1.0 API without public renames or removals.
- Updated README, architecture notes, roadmap, contributor guidance, examples, benchmark docs, and release workflow copy for `v1.0.0 — Stable`.
- Hardened `publish.yml` so releases validate the tag, run dry-runs before publishing, and tolerate only explicit already-published reruns.

### Verification
- `cargo fmt --check`
- `cargo clippy --workspace --all-features -- -D warnings`
- `cargo test --workspace --all-features`
- `cargo test --workspace --no-default-features`
- `cargo test -p animato --all-features --examples`
- `cargo doc --workspace --all-features --no-deps`
- `cargo check -p animato-wasm --target wasm32-unknown-unknown --features wasm-dom`
- `cargo bench --workspace --no-run`

## [0.9.0] — 2026-05-14 — Performance

### Added
- `animato-gpu`: new crate with `GpuAnimationBatch`, `GpuBackend`, and `GpuBatchError`.
- `GpuAnimationBatch::new_cpu()`, `new(device, queue)`, `try_new_auto()`, `new_auto()`, `push()`, `tick()`, `read_back()`, `backend()`, `len()`, and `clear()`.
- Embedded WGSL shader source for the 31 classic easing variants.
- `animato` facade: `gpu` feature flag and GPU batch re-exports.
- `TweenSnapshot` and runtime state getters for batch evaluators.
- `MorphPath::evaluate_into()` for allocation-free shape evaluation into a reusable buffer.
- Benchmarks: `timeline_bench.rs` and `gpu_batch_bench.rs`.
- Example: `examples/gpu_particles.rs`.
- Benchmark guide: `docs/benchmarks.md`.
- Integration tests for GPU CPU fallback parity and facade exports.

### Changed
- Bumped all workspace crates and internal dependency pins from `0.8.0` to `0.9.0`.
- `animato-spring` and `animato-physics` now compile as `no_std` when `alloc` is enabled without `std`.
- `MorphPath::bounds_at()` no longer allocates.
- `ScrollDriver` tests now prove registered animations receive proportional scroll deltas.
- `publish.yml`, README, roadmap, architecture notes, and examples updated for `v0.9.0 — Performance`.

### Fixed
- `SharedElementTransition::reset()` now resets the underlying `FlipAnimation` instead of only clearing the completion flag.
- Facade integration tests now declare the features they actually use, so the broad `cargo test --workspace --no-default-features` release gate can run.

## [0.8.0] — 2026-05-09 — Advanced

**`animato-core` — 5 new advanced easing variants**
- `Easing::RoughEase { strength, points }` — organic, rough motion using deterministic sine harmonics. Zero at `t=0`, one at `t=1`.
- `Easing::SlowMo { linear_ratio, power }` — motion that accelerates at the edges and crawls through the middle; CSS-friendly slow-motion feel.
- `Easing::Wiggle { wiggles }` — sinusoidal oscillation around the linear trend, fading to zero at both endpoints.
- `Easing::CustomBounce { strength }` — blend between linear (`strength=0`) and `EaseOutBounce` (`strength=1`).
- `Easing::ExpoScale { start, end }` — exponential time-warping: `f(t) = (k^t − 1) / (k − 1)` where `k = end / start`.
- Free functions `rough_ease`, `slow_mo`, `wiggle`, `custom_bounce`, `expo_scale` exported from `animato_core::easing`.
- `math::log` and `math::exp` added to the `no_std`-portable math shim.
- `Easing::all_named()` now returns **38** variants (was 33).

**`animato-path` — shape morphing and SVG draw animation**
- `MorphPath` — morphs between two polylines with automatic arc-length resampling. `evaluate(t)` returns the interpolated shape at progress `t`.
- `resample(points, count)` — uniformly resample any polyline to an exact point count by arc length. Also available as a standalone public free function.
- `DrawSvg` trait — blanket-implemented for every `PathEvaluate` type. Provides `draw_on(progress) -> DrawValues` and `draw_on_reverse(progress) -> DrawValues` for CSS `stroke-dashoffset` animation.
- `DrawValues` struct — holds `dash_array`, `dash_offset`, `progress()`, and `to_css()`.

**`animato-driver` — scroll-linked animation**
- `ScrollDriver` — drives registered animations by a normalised scroll-position delta. Animations receive `|Δpos| / range` as their `dt`.
- `ScrollClock` — adapts scroll-position changes to the `Clock` trait; accumulates multiple moves before `delta()` is consumed.
- Both types re-exported from `animato-driver` root and the `animato` facade.

**`animato-wasm` — layout animation helpers (wasm-dom feature)**
- `LayoutAnimator` — orchestrates FLIP-style layout transitions for multiple named DOM elements. Supports `snapshot`, `compute_transitions`, `update`, `apply`, and `css_transform`.
- `SharedElementTransition` — single-element FLIP transition (hero animation). `capture`, `update`, `apply_to`, `css_transform`, and `is_complete`.

**Examples**
- `examples/morph_path.rs` — morphs a square into a circle using `MorphPath` + `Tween`.
- `examples/scroll_linked.rs` — simulates scroll-driven animation with both `ScrollDriver` and `ScrollClock`.

**Tests**
- `tests/advanced_easing.rs` — endpoint invariants, monotonicity, tween integration for all five new easing variants.
- `tests/morph_path_integration.rs` — `resample`, `MorphPath`, and `DrawSvg` integration tests.
- `tests/scroll_driver.rs` — `ScrollDriver` and `ScrollClock` integration tests.

### Changed
- Bumped all workspace crates from `0.7.0` → `0.8.0`.
- `animato-driver/src/lib.rs` now re-exports `ScrollDriver` and `ScrollClock` at the crate root.
- `animato/src/lib.rs` re-exports all new v0.8.0 symbols under the appropriate feature flags.
- `animato-path/src/lib.rs` exposes the new `draw` and `morph` modules.
- `animato-wasm/src/lib.rs` exposes `LayoutAnimator` and `SharedElementTransition` (wasm-dom + wasm32).

### Fixed
- `easing.rs` unit test `all_named_count` updated from 33 to 38 to reflect the five new variants.

---

## [0.7.0] — 2026-05-09 — Integrations

### Added
- `animato-bevy`: new crate with `AnimatoPlugin`, `AnimatoTween<T>`, `AnimatoSpring<T>`, `AnimationLabel`, and transform helpers.
- `animato-bevy`: Bevy 0.18 completion messages `TweenCompleted` and `SpringSettled`.
- `animato-wasm`: new crate with `RafDriver` for `requestAnimationFrame` timestamps.
- `animato-wasm`: `ScrollSmoother` and `wasm-dom` helpers for FLIP, SplitText, Draggable, and Observer.
- `animato` facade: `bevy`, `wasm`, and `wasm-dom` feature flags and re-exports.
- Examples: `bevy_transform.rs`, `tui_progress.rs`, `tui_spinner.rs`, and `examples/wasm_counter/`.
- Integration tests for Bevy plugin behavior and facade-level WASM driver exports.

### Changed
- Bumped all workspace crates and internal dependency pins to `0.7.0`.
- Bumped workspace MSRV to Rust `1.89` for latest Bevy compatibility.
- Updated README, roadmap, architecture, CI, and publish workflow for `v0.7.0 — Integrations`.

---

## [0.6.0] — 2026-05-08 — Color

### Added
- `animato-color`: new crate for perceptual color interpolation.
- `animato-color`: `InLab<C>` wrapper for CIE L\*a\*b\* interpolation.
- `animato-color`: `InOklch<C>` wrapper for modern perceptual Oklch interpolation.
- `animato-color`: `InLinear<C>` wrapper for linear-light sRGB interpolation.
- `animato-color`: `Interpolate` implementations backed by `palette` conversions and `Mix`.
- `animato` facade: `color` feature flag, color wrapper re-exports, and `palette` re-export.
- Example: `color_animation.rs`.
- Integration tests for facade color exports and `Tween<InLab<Srgb>>`.

### Changed
- Bumped all workspace crates and internal dependency pins to `0.6.0`.
- Updated README, roadmap, architecture snippets, CI, and publish workflow for `v0.6.0 — Color`.

---

## [0.5.0] — 2026-05-08 — Physics

### Added
- `animato-physics`: new crate for input-driven physics, drag tracking, and gesture recognition.
- `animato-physics`: `InertiaConfig`, `InertiaBounds`, `Inertia`, and `InertiaN<T>` with friction deceleration.
- `animato-physics`: presets `smooth()`, `snappy()`, and `heavy()`.
- `animato-physics`: clamp-and-stop bounds for 1D and multi-dimensional inertia.
- `animato-physics`: `PointerData`, `DragAxis`, `DragConstraints`, and `DragState`.
- `animato-physics`: pointer capture, axis locks, rectangular constraints, grid snap, and velocity EMA.
- `animato-physics`: `GestureConfig`, `Gesture`, `SwipeDirection`, and `GestureRecognizer`.
- `animato-physics`: tap, double tap, long press, swipe, pinch, and rotation recognition.
- `animato` facade: `physics` feature flag and physics API re-exports.
- Example: `physics_drag.rs`.
- Integration tests for physics facade exports, bounded inertia, drag release, swipe, pinch, and rotation.
- Benchmark: `physics_bench.rs`.

### Changed
- Bumped all workspace crates and internal dependency pins to `0.5.0`.
- Updated README, roadmap, architecture snippets, CI, and publish workflow for `v0.5.0 — Physics`.

---

## [0.4.0] — 2026-05-08 — Paths

### Added
- `animato-path`: new crate for Bezier curves, CatmullRom splines, compound paths, and SVG path parsing.
- `animato-path`: `PathEvaluate` trait with `position(t)`, `tangent(t)`, `rotation_deg(t)`, and `arc_length()`.
- `animato-path`: `QuadBezier` and `CubicBezierCurve` with arc-length-normalized evaluation.
- `animato-path`: `CatmullRomSpline` and `PolyPath` for smooth paths through arbitrary points.
- `animato-path`: `CompoundPath`, `PathSegment`, `LineSegment`, `EllipticalArc`, and `PathCommand`.
- `animato-path`: `SvgPathParser::parse()` and `try_parse()` with `M`, `L`, `H`, `V`, `C`, `Q`, `A`, `Z`, and lowercase relative command support.
- `animato-path`: `MotionPath`, `MotionPathTween`, auto-rotation, and start/end offsets.
- `animato` facade: `path` feature flag and path API re-exports.
- Example: `motion_path.rs`.
- Integration tests for path arc length, motion path tweening, and SVG parsing.
- Benchmark: `path_bench.rs`.

### Changed
- Bumped all workspace crates and internal dependency pins to `0.4.0`.
- Updated README, roadmap, architecture snippets, CI, and publish workflow for `v0.4.0 — Paths`.

---

## [0.3.0] — 2026-05-07 — Control

### Added
- `animato-core`: `Easing::CubicBezier(f32, f32, f32, f32)` with CSS-compatible x-control clamping.
- `animato-core`: `Easing::Steps(u32)` with CSS `jump-end` behavior.
- `animato-core`: `cubic_bezier()` and `steps()` free easing helpers.
- `animato-timeline`: timeline-level `.time_scale(f32)` and `.set_time_scale(f32)`.
- `animato-timeline`: `std`-gated `.on_entry_complete(label, f)` and `.on_complete(f)` callbacks.
- `animato-timeline`: `tokio` feature and `.wait().await` completion future.
- `animato` facade: `tokio` feature pass-through and serde trait re-exports.
- Integration tests for v0.3.0 control features.

### Changed
- Bumped all workspace crates and internal dependency pins to `0.3.0`.
- Updated README, roadmap, examples, benchmark labels, and publish workflow for `v0.3.0 — Control`.
- Expanded serde coverage for concrete animation state types such as `Tween<T>` and `Spring`.

---

## [0.2.0] — 2026-05-07 — Composition

### Added
- `animato-core`: `Playable` trait for object-safe animation composition, reset, seek, duration, and downcasting.
- `animato-tween`: `Keyframe<T>` and `KeyframeTrack<T>` with sorted insertion, duplicate replacement, binary-search interpolation, looping, and PingPong support.
- `animato-timeline`: new crate with `Timeline`, `TimelineState`, `At`, `Sequence`, and `stagger`.
- `animato` facade: `timeline` feature added to default features and re-exports for all composition APIs.
- Examples: `keyframe_track.rs` and `timeline_sequence.rs`.
- Integration tests for keyframe looping and timeline sequencing.

### Changed
- Bumped all workspace crates and internal dependency pins to `0.2.0`.
- Updated roadmap and README so keyframes are part of `v0.2.0 — Composition`.

---

## [0.1.0] — 2026-05-07 — Foundation

### Added
- Initial workspace structure with 12 focused crates
- `animato-core`: `Interpolate`, `Animatable`, `Update` traits
- `animato-core`: 31 easing functions — Linear, Polynomial (12), Sine (3), Expo (3), Circular (3), Back (3), Elastic (3), Bounce (3)
- `animato-core`: Free easing functions (`ease_out_cubic(t)`, etc.) for zero-overhead use
- `animato-core`: `Easing::all_named()` for picker UIs and test sweeps
- `animato-core`: `no_std` support
- `animato-tween`: `Tween<T>` with builder pattern
- `animato-tween`: `TweenBuilder<T>` — `.duration()`, `.easing()`, `.delay()`, `.time_scale()`, `.looping()`
- `animato-tween`: `TweenState` enum (`Idle`, `Running`, `Paused`, `Completed`)
- `animato-tween`: `Loop` enum (`Once`, `Times(u32)`, `Forever`, `PingPong`)
- `animato-tween`: `.value()`, `.progress()`, `.eased_progress()`, `.is_complete()`
- `animato-tween`: `.seek()`, `.reset()`, `.reverse()`, `.pause()`, `.resume()`
- `animato-tween`: `snap_to()` and `round_to()` value modifier free functions
- `animato-spring`: `Spring` with semi-implicit Euler integration
- `animato-spring`: `SpringConfig` with `gentle()`, `wobbly()`, `stiff()`, `slow()`, `snappy()` presets
- `animato-spring`: Optional RK4 integration via `.use_rk4(true)`
- `animato-spring`: `SpringN<T>` — multi-dimensional spring via component decomposition
- `animato-spring`: `is_settled()`, `snap_to()`, `set_target()`
- `animato-driver`: `AnimationDriver` with `AnimationId`, auto-removal of completed animations
- `animato-driver`: `Clock` trait, `WallClock`, `ManualClock`, `MockClock`
- `animato` facade: feature flags for all sub-crates
- Workspace-level `Cargo.toml` with shared dependency versions
- `ARCHITECTURE.md` — full design document
- `ROADMAP.md` — versioned plan through v1.0.0
- `CONTRIBUTING.md` — workspace setup, commit format, PR process
- `LICENSE-MIT` and `LICENSE-APACHE`
- CI workflow: test, clippy, fmt, no_std check, docs build

---

[Unreleased]: https://github.com/AarambhDevHub/animato/compare/v1.2.0...HEAD
[1.2.0]: https://github.com/AarambhDevHub/animato/compare/v1.1.0...v1.2.0
[1.1.0]: https://github.com/AarambhDevHub/animato/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/AarambhDevHub/animato/compare/v0.9.0...v1.0.0
[0.9.0]: https://github.com/AarambhDevHub/animato/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/AarambhDevHub/animato/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/AarambhDevHub/animato/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/AarambhDevHub/animato/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/AarambhDevHub/animato/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/AarambhDevHub/animato/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/AarambhDevHub/animato/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/AarambhDevHub/animato/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/AarambhDevHub/animato/releases/tag/v0.1.0
