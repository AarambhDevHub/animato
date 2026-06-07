//! JavaScript and TypeScript bindings for Animato.
//!
//! This crate is compiled with `wasm-pack` and published as `@aarambhdevhub/animato-core`.
//! It exposes the renderer-agnostic Animato engines to JavaScript while keeping
//! framework adapters in examples and application code.

#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

mod advanced;
mod batch;
mod color;
mod driver;
mod easing;
mod error;
mod keyframe;
mod path;
mod physics;
mod recorder;
mod spring;
mod timeline;
mod tween;
mod types;
mod wasm_dom;

pub use advanced::{
    Angle, AngleTween, AnimationGroup, Mat4, Mat4Tween, Quaternion, QuaternionTween,
    StaggerPattern, Waveform,
};
pub use batch::TweenBatch;
pub use color::{ColorTween, interpolate_color};
pub use driver::{RafDriver, ScrollDriver};
pub use easing::{available_easings, ease, parse_easing_for_js};
pub use keyframe::{KeyframeTrack, KeyframeTrack2D, KeyframeTrack3D, KeyframeTrack4D};
pub use path::{MorphPath, MotionPath};
pub use physics::{DragState, GestureRecognizer, Inertia, Inertia2D};
pub use recorder::AnimationRecorder;
pub use spring::{Spring, Spring2D, Spring3D, Spring4D};
pub use tween::{Tween, Tween2D, Tween3D, Tween4D};
pub use wasm_dom::{Draggable, FlipAnimation, LayoutAnimator, Observer, ScrollSmoother, SplitText};

use wasm_bindgen::prelude::*;

/// Initialize Animato's optional browser runtime hooks.
///
/// This is safe to call more than once. The current implementation installs a
/// panic hook when the crate is built with the `console_error_panic_hook`
/// dependency enabled by downstream builds.
#[wasm_bindgen(js_name = initAnimato)]
pub fn init_animato() {
    #[cfg(all(target_arch = "wasm32", feature = "panic-hook"))]
    console_error_panic_hook::set_once();
}

/// Return the Animato package version.
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}

/// Snap a value to a grid size.
#[wasm_bindgen(js_name = snapTo)]
pub fn snap_to(value: f32, grid: f32) -> f32 {
    animato_tween::snap_to(value, grid)
}

/// Round a value to a fixed number of decimal places.
#[wasm_bindgen(js_name = roundTo)]
pub fn round_to(value: f32, decimals: u32) -> f32 {
    animato_tween::round_to(value, decimals)
}
