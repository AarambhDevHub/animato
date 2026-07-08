//! Compile-time easing name resolver.
//!
//! Converts an [`EasingSpec`](crate::ast::EasingSpec) into a `TokenStream`
//! that produces the matching `animato::Easing` variant.

use crate::ast::EasingSpec;
use proc_macro2::TokenStream;
use quote::quote;

/// All 31 named easing variants accepted by the DSL, in `snake_case`.
///
/// This is the single source of truth for valid easing names and is used both
/// for resolution and for "did you mean…?" suggestions.
pub const NAMED_EASINGS: &[&str] = &[
    "linear",
    "ease_in_quad",
    "ease_out_quad",
    "ease_in_out_quad",
    "ease_in_cubic",
    "ease_out_cubic",
    "ease_in_out_cubic",
    "ease_in_quart",
    "ease_out_quart",
    "ease_in_out_quart",
    "ease_in_quint",
    "ease_out_quint",
    "ease_in_out_quint",
    "ease_in_sine",
    "ease_out_sine",
    "ease_in_out_sine",
    "ease_in_expo",
    "ease_out_expo",
    "ease_in_out_expo",
    "ease_in_circ",
    "ease_out_circ",
    "ease_in_out_circ",
    "ease_in_back",
    "ease_out_back",
    "ease_in_out_back",
    "ease_in_elastic",
    "ease_out_elastic",
    "ease_in_out_elastic",
    "ease_in_bounce",
    "ease_out_bounce",
    "ease_in_out_bounce",
];

/// Map a `snake_case` easing name to its `PascalCase` enum variant.
///
/// Returns `None` for unknown names.
pub fn named_variant(name: &str) -> Option<&'static str> {
    // The enum variant is the name converted to PascalCase:
    //   ease_out_cubic -> EaseOutCubic
    //   linear -> Linear
    let pascal = to_pascal_case(name);
    if NAMED_EASINGS.contains(&name) {
        Some(leak_str(pascal))
    } else {
        None
    }
}

/// Suggest the closest known easing name for an unknown input.
///
/// Uses a simple Levenshtein-distance heuristic. Returns `None` if no name is
/// within distance 3.
pub fn suggest(name: &str) -> Option<&'static str> {
    let lower = name.to_lowercase();
    NAMED_EASINGS
        .iter()
        .map(|&candidate| (candidate, levenshtein(&lower, candidate)))
        .min_by_key(|&(_, d)| d)
        .and_then(|(candidate, d)| if d <= 3 { Some(candidate) } else { None })
}

/// Convert an [`EasingSpec`] into a `TokenStream` producing `animato::Easing`.
pub fn expand_easing(spec: &EasingSpec) -> TokenStream {
    match spec {
        EasingSpec::Named(name) => match named_variant(name) {
            Some(variant) => {
                let ident = syn::Ident::new(variant, proc_macro2::Span::call_site());
                quote! { animato::Easing::#ident }
            }
            None => {
                // Should have been caught by validation, but emit a fallback.
                quote! { animato::Easing::Linear }
            }
        },
        EasingSpec::CubicBezier(x1, y1, x2, y2) => {
            quote! { animato::Easing::CubicBezier(#x1, #y1, #x2, #y2) }
        }
        EasingSpec::Steps(n) => {
            quote! { animato::Easing::Steps(#n) }
        }
        EasingSpec::Wiggle(n) => {
            quote! { animato::Easing::Wiggle { wiggles: #n } }
        }
        EasingSpec::Rough { strength, points } => {
            quote! { animato::Easing::RoughEase { strength: #strength, points: #points } }
        }
        EasingSpec::SlowMo {
            linear_ratio,
            power,
        } => {
            quote! { animato::Easing::SlowMo { linear_ratio: #linear_ratio, power: #power } }
        }
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn to_pascal_case(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut capitalize_next = true;
    for ch in s.chars() {
        if ch == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(ch.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(ch);
        }
    }
    result
}

/// Leak a `String` into a `&'static str`.
///
/// This is acceptable in a proc-macro context because the macro process is
/// short-lived and the leaked strings are bounded by the number of unique
/// easing names (31).
fn leak_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let (m, n) = (a.len(), b.len());
    if m == 0 {
        return n;
    }
    if n == 0 {
        return m;
    }

    let mut prev: Vec<usize> = (0..=n).collect();
    let mut curr: Vec<usize> = vec![0; n + 1];

    for i in 1..=m {
        curr[0] = i;
        for j in 1..=n {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            curr[j] = (prev[j] + 1).min(curr[j - 1] + 1).min(prev[j - 1] + cost);
        }
        core::mem::swap(&mut prev, &mut curr);
    }
    prev[n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_easings_resolve() {
        assert_eq!(named_variant("linear"), Some("Linear"));
        assert_eq!(named_variant("ease_out_cubic"), Some("EaseOutCubic"));
        assert_eq!(named_variant("ease_in_out_back"), Some("EaseInOutBack"));
    }

    #[test]
    fn unknown_easings_return_none() {
        assert_eq!(named_variant("ease_out_magic"), None);
        assert_eq!(named_variant("banana"), None);
    }

    #[test]
    fn suggestions_are_close() {
        assert_eq!(suggest("ease_out_cubc"), Some("ease_out_cubic"));
        assert_eq!(suggest("easeinquad"), Some("ease_in_quad"));
    }

    #[test]
    fn pascal_case_converts() {
        assert_eq!(to_pascal_case("linear"), "Linear");
        assert_eq!(to_pascal_case("ease_out_cubic"), "EaseOutCubic");
        assert_eq!(to_pascal_case("ease_in_out_back"), "EaseInOutBack");
    }
}
