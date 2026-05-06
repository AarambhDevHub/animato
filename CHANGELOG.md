# Changelog

All notable changes to Motus will be documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
This project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Added
- Initial workspace structure with 12 focused crates
- `motus-core`: `Interpolate`, `Animatable`, `Update` traits
- `motus-core`: 43 easing functions — Linear, Polynomial (12), Sine (3), Expo (3), Circular (3), Back (3), Elastic (3), Bounce (3), CSS (`CubicBezier`, `Steps`), Advanced (5)
- `motus-core`: Free easing functions (`ease_out_cubic(t)`, etc.) for zero-overhead use
- `motus-core`: `Easing::all_named()` for picker UIs and test sweeps
- `motus-core`: `no_std` support
- `motus-tween`: `Tween<T>` with builder pattern
- `motus-tween`: `TweenBuilder<T>` — `.duration()`, `.easing()`, `.delay()`, `.time_scale()`, `.looping()`
- `motus-tween`: `TweenState` enum (`Idle`, `Running`, `Paused`, `Completed`)
- `motus-tween`: `Loop` enum (`Once`, `Times(u32)`, `Forever`, `PingPong`)
- `motus-tween`: `.value()`, `.progress()`, `.eased_progress()`, `.is_complete()`
- `motus-tween`: `.seek()`, `.reset()`, `.reverse()`, `.pause()`, `.resume()`
- `motus-tween`: `snap_to()` and `round_to()` value modifier free functions
- `motus-tween`: `KeyframeTrack<T>` with binary-search interpolation
- `motus-tween`: `Keyframe<T>` with per-segment easing
- `motus-spring`: `Spring` with semi-implicit Euler integration
- `motus-spring`: `SpringConfig` with `gentle()`, `wobbly()`, `stiff()`, `slow()`, `snappy()` presets
- `motus-spring`: Optional RK4 integration via `.use_rk4(true)`
- `motus-spring`: `SpringN<T>` — multi-dimensional spring via component decomposition
- `motus-spring`: `is_settled()`, `snap_to()`, `set_target()`
- `motus-driver`: `AnimationDriver` with `AnimationId`, auto-removal of completed animations
- `motus-driver`: `Clock` trait, `WallClock`, `ManualClock`, `MockClock`
- `motus` facade: feature flags for all sub-crates
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

[Unreleased]: https://github.com/AarambhDevHub/motus/compare/HEAD
[0.1.0]: https://github.com/AarambhDevHub/motus/releases/tag/v0.1.0
