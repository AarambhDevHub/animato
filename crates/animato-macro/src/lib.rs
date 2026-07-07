//! # animato-macro
//!
//! Procedural macro DSL for declarative Animato animation authoring.
//!
//! This crate is the compile-time authoring layer for Animato. It does not
//! introduce a new runtime. Every macro expansion should eventually generate
//! normal Animato primitives such as `Tween`, `Spring`, `Timeline`,
//! `AnimationGroup`, `KeyframeTrack`, `MotionPathTween`, and related types.
//!
//! The first implementation step only exposes macro entry points. The parser,
//! AST, validation, and code generation modules will be added incrementally.

use proc_macro::TokenStream;

/// Main declarative animation macro.
///
/// Planned syntax:
///
/// ```ignore
/// let intro = animato! {
///     sequence {
///         tween opacity: 0.0 => 1.0, duration: 0.3, easing: ease_out_cubic;
///         spring scale: 0.8 => 1.0, preset: snappy;
///     }
/// };
/// ```
#[proc_macro]
pub fn animato(_input: TokenStream) -> TokenStream {
    not_implemented("animato!")
}

/// Alias for [`animato!`] focused on UI-style motion authoring.
#[proc_macro]
pub fn motion(_input: TokenStream) -> TokenStream {
    not_implemented("motion!")
}

/// Standalone tween macro.
#[proc_macro]
pub fn tween(_input: TokenStream) -> TokenStream {
    not_implemented("tween!")
}

/// Standalone spring macro.
#[proc_macro]
pub fn spring(_input: TokenStream) -> TokenStream {
    not_implemented("spring!")
}

/// Standalone timeline macro.
#[proc_macro]
pub fn timeline(_input: TokenStream) -> TokenStream {
    not_implemented("timeline!")
}

/// Standalone keyframe-track macro.
#[proc_macro]
pub fn keyframes(_input: TokenStream) -> TokenStream {
    not_implemented("keyframes!")
}

/// User-defined preset macro.
#[proc_macro]
pub fn preset(_input: TokenStream) -> TokenStream {
    not_implemented("preset!")
}

fn not_implemented(name: &str) -> TokenStream {
    let message = format!(
        "{name} is registered, but the Animato v1.7.0 Motion Macro parser is not implemented yet"
    );

    quote::quote! {
        compile_error!(#message);
    }
    .into()
}
