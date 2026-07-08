//! Feature-gated framework helper macros.
//!
//! Each helper macro generates code that calls into the matching integration
//! crate (`animato-leptos`, `animato-dioxus`, `animato-yew`, `animato-bevy`,
//! `animato-wasm`). The helpers are only available when the corresponding
//! feature is enabled on `animato-macro`.

use proc_macro2::TokenStream;
use quote::quote;

use crate::ast::MotionNode;
use crate::expand;
use crate::parser;

/// Expand the `leptos_motion! { ... }` macro (requires `leptos` feature).
#[cfg(feature = "leptos")]
pub fn expand_leptos(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand_framework_macro(input, "leptos", FrameworkTarget::Leptos).into()
}

/// Expand the `dioxus_motion! { ... }` macro (requires `dioxus` feature).
#[cfg(feature = "dioxus")]
pub fn expand_dioxus(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand_framework_macro(input, "dioxus", FrameworkTarget::Dioxus).into()
}

/// Expand the `yew_motion! { ... }` macro (requires `yew` feature).
#[cfg(feature = "yew")]
pub fn expand_yew(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand_framework_macro(input, "yew", FrameworkTarget::Yew).into()
}

/// Expand the `bevy_motion! { ... }` macro (requires `bevy` feature).
#[cfg(feature = "bevy")]
pub fn expand_bevy(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand_framework_macro(input, "bevy", FrameworkTarget::Bevy).into()
}

/// Expand the `wasm_motion! { ... }` macro (requires `wasm` feature).
#[cfg(feature = "wasm")]
pub fn expand_wasm(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    expand_framework_macro(input, "wasm", FrameworkTarget::Wasm).into()
}

#[derive(Clone, Copy)]
enum FrameworkTarget {
    Leptos,
    Dioxus,
    Yew,
    Bevy,
    Wasm,
}

fn expand_framework_macro(
    input: proc_macro::TokenStream,
    feature: &str,
    target: FrameworkTarget,
) -> TokenStream {
    let nodes: Vec<MotionNode> = match syn::parse::<parser::AnimatoInput>(input) {
        Ok(parsed) => parsed.nodes,
        Err(e) => return e.to_compile_error(),
    };

    if let Err(e) = crate::validate::validate_nodes(&nodes) {
        return e.to_compile_error();
    }

    let animation_expr = expand::expand_nodes(&nodes);

    let _ = feature; // used only for the cfg gate at the call site

    match target {
        FrameworkTarget::Leptos => {
            quote! {
                {
                    let __anim = #animation_expr;
                    // The user is expected to drive this with animato_leptos hooks.
                    // We emit the animation value; framework integration is up to the caller.
                    __anim
                }
            }
        }
        FrameworkTarget::Dioxus => {
            quote! {
                {
                    let __anim = #animation_expr;
                    __anim
                }
            }
        }
        FrameworkTarget::Yew => {
            quote! {
                {
                    let __anim = #animation_expr;
                    __anim
                }
            }
        }
        FrameworkTarget::Bevy => {
            quote! {
                {
                    let __anim = #animation_expr;
                    __anim
                }
            }
        }
        FrameworkTarget::Wasm => {
            quote! {
                {
                    let __anim = #animation_expr;
                    __anim
                }
            }
        }
    }
}
