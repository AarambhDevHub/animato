//! # Animato
//!
//! > *Italian: animato — animated, lively, with life and movement.*
//!
//! A professional-grade, renderer-agnostic animation library for Rust.
//! Zero mandatory dependencies. `no_std`-ready.
//!
//! Works everywhere: TUIs, Web (WASM), Bevy games, embedded targets, and native apps.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use animato::{Tween, Easing, Update};
//!
//! let mut tween = Tween::new(0.0_f32, 100.0)
//!     .duration(1.0)
//!     .easing(Easing::EaseOutCubic)
//!     .build();
//!
//! tween.update(1.0);
//! assert_eq!(tween.value(), 100.0);
//! assert!(tween.is_complete());
//! ```
//!
//! ## Spring Physics
//!
//! ```rust,ignore
//! use animato::{Spring, SpringConfig, Update};
//!
//! let mut spring = Spring::new(SpringConfig::wobbly());
//! spring.set_target(200.0);
//!
//! while !spring.is_settled() {
//!     spring.update(1.0 / 60.0);
//! }
//! assert!((spring.position() - 200.0).abs() < 0.01);
//! ```
//!
//! ## Input Physics
//!
//! ```rust,ignore
//! use animato::{Inertia, InertiaConfig, Update};
//!
//! let mut inertia = Inertia::new(InertiaConfig::smooth());
//! inertia.kick(800.0);
//! while inertia.update(1.0 / 60.0) {}
//! ```
//!
//! ## AnimationDriver
//!
//! ```rust,ignore
//! use animato::{Tween, Easing, AnimationDriver, WallClock, Clock};
//!
//! let mut driver = AnimationDriver::new();
//! let id = driver.add(
//!     Tween::new(0.0_f32, 1.0).duration(2.0).easing(Easing::EaseInOutSine).build()
//! );
//!
//! let mut clock = WallClock::new();
//! // In your loop: driver.tick(clock.delta());
//! ```
//!
//! ## `no_std` Usage
//!
//! For `no_std` targets, depend on the sub-crates directly:
//!
//! ```toml
//! [dependencies]
//! animato-core   = { version = "1.1", default-features = false }
//! animato-tween  = { version = "1.1", default-features = false }
//! animato-spring = { version = "1.1", default-features = false }
//! animato-physics = { version = "1.1", default-features = false }
//! animato-color = { version = "1.1", default-features = false }
//! ```
//!
//! ## Feature Flags
//!
//! | Feature | What it adds |
//! |---------|-------------|
//! | `default` | `std` + `tween` + `timeline` + `spring` + `driver` |
//! | `std` | Wall clock, heap-backed types |
//! | `tween` | [`Tween<T>`], [`KeyframeTrack<T>`], [`Loop`] |
//! | `timeline` | [`Timeline`], [`Sequence`], [`stagger()`] |
//! | `spring` | [`Spring`], [`SpringConfig`], [`SpringN<T>`] |
//! | `path` | [`MotionPath`], [`MotionPathTween`], [`SvgPathParser`] |
//! | `physics` | [`Inertia`], [`DragState`], [`GestureRecognizer`] |
//! | `color` | [`InLab<T>`], [`InOklch<T>`], [`InLinear<T>`] |
//! | `driver` | [`AnimationDriver`], all [`Clock`] variants |
//! | `gpu` | [`GpuAnimationBatch`] for high-volume `Tween<f32>` batches |
//! | `bevy` | [`AnimatoPlugin`], Bevy tween/spring wrapper components |
//! | `wasm` | [`RafDriver`] for `requestAnimationFrame` loops |
//! | `leptos` | Signal-backed Leptos hooks and components |
//! | `tokio` | [`Timeline::wait()`] async completion waiting |
//! | `serde` | `Serialize`/`Deserialize` on all public types |

// ── Core — always present ────────────────────────────────────────────────────
pub use animato_core::{Animatable, Easing, Interpolate, Playable, Update};

// ── Serde convenience re-export ─────────────────────────────────────────────
#[cfg(feature = "serde")]
pub use serde::{Deserialize, Serialize};

/// All free easing functions (`ease_out_cubic`, `cubic_bezier`, etc.) re-exported at crate root.
///
/// These are `#[inline]` free functions — use them when you want zero-overhead
/// easing without the `Easing` enum indirection.
pub mod easing {
    pub use animato_core::easing::*;
}

// ── Tween ────────────────────────────────────────────────────────────────────
#[cfg(feature = "tween")]
pub use animato_tween::{
    Keyframe, KeyframeTrack, Loop, Tween, TweenBuilder, TweenSnapshot, TweenState, round_to,
    snap_to,
};

// ── Timeline ────────────────────────────────────────────────────────────────
#[cfg(feature = "timeline")]
pub use animato_timeline::{At, Sequence, Timeline, TimelineState, stagger};

// ── Spring ───────────────────────────────────────────────────────────────────
#[cfg(feature = "spring")]
pub use animato_spring::{Integrator, Spring, SpringConfig};

#[cfg(feature = "spring")]
pub use animato_spring::SpringN;

// ── Path ─────────────────────────────────────────────────────────────────────
#[cfg(feature = "path")]
pub use animato_path::{
    CatmullRomSpline, CompoundPath, CubicBezierCurve, DrawSvg, DrawValues, EllipticalArc,
    LineSegment, MorphPath, MotionPath, MotionPathTween, MotionPathTweenBuilder, PathCommand,
    PathEvaluate, PathSegment, PolyPath, QuadBezier, SvgPathError, SvgPathParser, resample,
};

// ── Physics ─────────────────────────────────────────────────────────────────
#[cfg(feature = "physics")]
pub use animato_physics::{
    DragAxis, DragConstraints, DragState, Gesture, GestureConfig, GestureRecognizer, Inertia,
    InertiaBounds, InertiaConfig, InertiaN, PointerData, SwipeDirection,
};

// ── Color ───────────────────────────────────────────────────────────────────
#[cfg(feature = "color")]
pub use animato_color::{InLab, InLinear, InOklch};
#[cfg(feature = "color")]
pub use palette;

// ── Driver ───────────────────────────────────────────────────────────────────
#[cfg(feature = "driver")]
pub use animato_driver::{
    AnimationDriver, AnimationId, Clock, ManualClock, MockClock, ScrollClock, ScrollDriver,
    WallClock,
};

// ── GPU ──────────────────────────────────────────────────────────────────────
#[cfg(feature = "gpu")]
pub use animato_gpu::{GpuAnimationBatch, GpuBackend, GpuBatchError};

// ── Bevy ─────────────────────────────────────────────────────────────────────
#[cfg(feature = "bevy")]
pub use animato_bevy::{
    AnimationChannel, AnimationLabel, AnimatoPlugin, AnimatoSet, AnimatoSpring,
    AnimatoSpringPlugin, AnimatoTween, AnimatoTweenPlugin, SpringSettled, TweenCompleted,
};

// ── WASM ─────────────────────────────────────────────────────────────────────
#[cfg(feature = "wasm")]
pub use animato_wasm::{RafDriver, ScrollSmoother};

#[cfg(all(feature = "wasm-dom", target_arch = "wasm32"))]
pub use animato_wasm::{
    Draggable, FlipAnimation, FlipState, LayoutAnimator, Observer, ObserverEvent,
    SharedElementTransition, SplitMode, SplitText,
};

// ── Leptos ──────────────────────────────────────────────────────────────────
#[cfg(feature = "leptos")]
pub use animato_leptos::*;
