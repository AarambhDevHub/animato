# Changelog

All notable changes to Animato will be documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.0] — 2026-05-07 — Foundation

### Added
- Initial workspace structure with 12 focused crates
- `animato-core`: `Interpolate`, `Animatable`, `Update` traits
- `animato-core`: 43 easing functions — Linear, Polynomial (12), Sine (3), Expo (3), Circular (3), Back (3), Elastic (3), Bounce (3), CSS (`CubicBezier`, `Steps`), Advanced (5)
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
- `animato-tween`: `KeyframeTrack<T>` with binary-search interpolation
- `animato-tween`: `Keyframe<T>` with per-segment easing
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

## [0.1.0] — Unreleased

*Initial release. See [Unreleased] above for full change list.*

---

[Unreleased]: https://github.com/AarambhDevHub/animato/compare/HEAD
[0.1.0]: https://github.com/AarambhDevHub/animato/releases/tag/v0.1.0
