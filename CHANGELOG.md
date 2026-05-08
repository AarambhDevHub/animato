# Changelog

All notable changes to Animato will be documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

[Unreleased]: https://github.com/AarambhDevHub/animato/compare/v0.6.0...HEAD
[0.6.0]: https://github.com/AarambhDevHub/animato/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/AarambhDevHub/animato/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/AarambhDevHub/animato/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/AarambhDevHub/animato/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/AarambhDevHub/animato/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/AarambhDevHub/animato/releases/tag/v0.1.0
