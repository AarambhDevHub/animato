//! # animato-macro
//!
//! Procedural macro DSL for declarative Animato animation authoring.
//!
//! This crate is the compile-time authoring layer for Animato. It does **not**
//! introduce a new runtime. Every macro expansion generates normal Animato
//! primitives such as `Tween`, `Spring`, `Timeline`, `AnimationGroup`,
//! `KeyframeTrack`, `MotionPathTween`, and related types.
//!
//! ## Quick Start
//!
//! ```ignore
//! use animato::prelude::*;
//!
//! let intro = animato! {
//!     sequence {
//!         tween opacity: 0.0 => 1.0, duration: 0.35, easing: ease_out_cubic;
//!         spring scale: 0.92 => 1.0, preset: snappy;
//!     }
//! };
//! ```
//!
//! ## Macros
//!
//! | Macro | Purpose |
//! |-------|---------|
//! | `animato! { ... }` | Primary declarative DSL |
//! | `motion! { ... }` | Alias for `animato!` |
//! | `tween! { ... }` | Standalone `Tween<T>` generator |
//! | `spring! { ... }` | Standalone `Spring` / `SpringN<T>` generator |
//! | `timeline! { ... }` | Standalone `Timeline` generator |
//! | `keyframes! { ... }` | Standalone `KeyframeTrack<T>` generator |
//! | `preset! { ... }` | User-defined reusable preset generator |
//!
//! ## Feature Flags
//!
//! | Feature | Effect |
//! |---------|--------|
//! | `leptos` | Enables `leptos_motion!` |
//! | `dioxus` | Enables `dioxus_motion!` |
//! | `yew` | Enables `yew_motion!` |
//! | `bevy` | Enables `bevy_motion!` |
//! | `wasm` | Enables `wasm_motion!` |

#![deny(missing_docs)]
// Framework helpers and some error/diagnostic functions are feature-gated or
// reserved for future use — allow dead code in the proc-macro crate.
#![allow(dead_code)]
// The DSL parser uses explicit if-else chains and references that clippy
// would otherwise collapse; allow these patterns for readability.
#![allow(clippy::collapsible_if)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::needless_lifetimes)]

mod ast;
mod easing;
mod error;
mod expand;
mod framework;
mod parser;
mod presets;
mod validate;

use proc_macro::TokenStream;

/// Main declarative animation macro.
///
/// Parses a block of motion nodes (tweens, springs, keyframes, sequences,
/// parallels, staggers, paths, colors, waveforms, presets) and generates
/// the corresponding Animato primitive expression.
///
/// # Example
///
/// ```ignore
/// let intro = animato! {
///     sequence {
///         tween opacity: 0.0 => 1.0, duration: 0.35, easing: ease_out_cubic;
///         parallel {
///             tween y: 24.0 => 0.0, duration: 0.55, easing: ease_out_back;
///             spring scale: 0.92 => 1.0, preset: snappy;
///         }
///     }
/// };
/// ```
#[proc_macro]
pub fn animato(input: TokenStream) -> TokenStream {
    expand_macro(input, "animato")
}

/// Alias for [`animato!`](macro@animato) focused on UI-style motion authoring.
///
/// Produces identical expansion to `animato!{}`.
#[proc_macro]
pub fn motion(input: TokenStream) -> TokenStream {
    expand_macro(input, "motion")
}

/// Standalone tween macro.
///
/// Generates a single `Tween<T>` from a `tween ...;` statement.
///
/// # Example
///
/// ```ignore
/// let t = tween!{ opacity: 0.0 => 1.0, duration: 0.4, easing: ease_out_cubic };
/// ```
#[proc_macro]
pub fn tween(input: TokenStream) -> TokenStream {
    expand_macro(input, "tween")
}

/// Standalone spring macro.
///
/// Generates a `Spring` or `SpringN<T>` from a `spring ...;` statement.
///
/// # Example
///
/// ```ignore
/// let s = spring!{ scale: 0.8 => 1.0, preset: snappy };
/// ```
#[proc_macro]
pub fn spring(input: TokenStream) -> TokenStream {
    expand_macro(input, "spring")
}

/// Standalone timeline macro.
///
/// Generates a `Timeline` from a block of motion nodes.
///
/// # Example
///
/// ```ignore
/// let tl = timeline! {
///     sequence {
///         tween x: 0.0 => 1.0, duration: 0.3;
///         spring y: 0.0 => 1.0, preset: gentle;
///     }
/// };
/// ```
#[proc_macro]
pub fn timeline(input: TokenStream) -> TokenStream {
    expand_macro(input, "timeline")
}

/// Standalone keyframe-track macro.
///
/// Generates a `KeyframeTrack<T>` from a `keyframes { ... }` block.
///
/// # Example
///
/// ```ignore
/// let track = keyframes!{ opacity { 0%: 0.0, 50%: 0.7 ease_out_cubic, 100%: 1.0 } };
/// ```
#[proc_macro]
pub fn keyframes(input: TokenStream) -> TokenStream {
    expand_macro(input, "keyframes")
}

/// User-defined preset macro.
///
/// Defines a reusable named preset that expands into an Animato primitive.
///
/// # Example
///
/// ```ignore
/// preset! { card_enter {
///     sequence {
///         tween opacity: 0.0 => 1.0, duration: 0.3;
///         spring y: 20.0 => 0.0, preset: snappy;
///     }
/// } }
/// ```
#[proc_macro]
pub fn preset(input: TokenStream) -> TokenStream {
    expand_macro(input, "preset")
}

/// Leptos framework helper macro (requires `leptos` feature).
#[cfg(feature = "leptos")]
#[proc_macro]
pub fn leptos_motion(input: TokenStream) -> TokenStream {
    framework::expand_leptos(input)
}

/// Dioxus framework helper macro (requires `dioxus` feature).
#[cfg(feature = "dioxus")]
#[proc_macro]
pub fn dioxus_motion(input: TokenStream) -> TokenStream {
    framework::expand_dioxus(input)
}

/// Yew framework helper macro (requires `yew` feature).
#[cfg(feature = "yew")]
#[proc_macro]
pub fn yew_motion(input: TokenStream) -> TokenStream {
    framework::expand_yew(input)
}

/// Bevy framework helper macro (requires `bevy` feature).
#[cfg(feature = "bevy")]
#[proc_macro]
pub fn bevy_motion(input: TokenStream) -> TokenStream {
    framework::expand_bevy(input)
}

/// WASM framework helper macro (requires `wasm` feature).
#[cfg(feature = "wasm")]
#[proc_macro]
pub fn wasm_motion(input: TokenStream) -> TokenStream {
    framework::expand_wasm(input)
}

// ── Internal entry point ────────────────────────────────────────────────────

fn expand_macro(input: TokenStream, name: &str) -> TokenStream {
    // For standalone helper macros (tween!, spring!, keyframes!), prepend the
    // keyword so the parser sees a complete motion node statement.
    let needs_keyword = matches!(name, "tween" | "spring" | "keyframes");
    if needs_keyword {
        let keyword = proc_macro2::Ident::new(name, proc_macro2::Span::call_site());
        let input_ts: proc_macro2::TokenStream = input.into();
        let combined = quote::quote! { #keyword #input_ts };
        return expand_parsed(combined.into());
    }
    expand_parsed(input)
}

fn expand_parsed(input: TokenStream) -> TokenStream {
    let parsed: parser::AnimatoInput = match syn::parse(input) {
        Ok(p) => p,
        Err(e) => return e.to_compile_error().into(),
    };

    if let Err(e) = validate::validate_nodes(&parsed.nodes) {
        return e.to_compile_error().into();
    }

    let tokens = expand::expand_nodes(&parsed.nodes);
    tokens.into()
}
