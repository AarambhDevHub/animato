//! Diagnostic helpers for producing clear, span-aware compiler errors.

use proc_macro2::Span;
use syn::Error;

/// Create an "unexpected token" error.
pub fn unexpected_token(span: Span, expected: &str, found: &str) -> Error {
    Error::new(span, format!("expected {expected}, found `{found}`"))
}

/// Create a "missing field" error.
pub fn missing_field(span: Span, field: &str) -> Error {
    Error::new(span, format!("missing required field `{field}`"))
}

/// Create an "unknown easing" error with a suggestion.
pub fn unknown_easing(span: Span, name: &str) -> Error {
    match crate::easing::suggest(name) {
        Some(suggestion) => Error::new(
            span,
            format!("unknown easing `{name}`; did you mean `{suggestion}`?"),
        ),
        None => Error::new(span, format!("unknown easing `{name}`")),
    }
}

/// Create an "unknown spring preset" error.
pub fn unknown_spring_preset(span: Span, name: &str) -> Error {
    Error::new(
        span,
        format!(
            "unknown spring preset `{name}`; valid presets are: gentle, wobbly, stiff, slow, snappy"
        ),
    )
}

/// Create an "unknown preset" error.
pub fn unknown_preset(span: Span, name: &str) -> Error {
    Error::new(
        span,
        format!(
            "unknown preset `{name}`; see docs/macro-reference.md for the full list of built-in presets"
        ),
    )
}

/// Create a "type mismatch" error for tween from/to values.
pub fn value_type_mismatch(span: Span, from: &str, to: &str) -> Error {
    Error::new(
        span,
        format!("tween `from` and `to` value types do not match: {from} vs {to}"),
    )
}

/// Create a generic "malformed" error.
pub fn malformed(span: Span, msg: &str) -> Error {
    Error::new(span, msg)
}

/// Create an "unsupported nesting" error.
pub fn unsupported_nesting(span: Span, parent: &str, child: &str) -> Error {
    Error::new(span, format!("`{child}` is not allowed inside `{parent}`"))
}

/// Create an "unknown DSL keyword" error.
pub fn unknown_keyword(span: Span, keyword: &str) -> Error {
    Error::new(
        span,
        format!(
            "unknown DSL keyword `{keyword}`; expected one of: tween, spring, keyframes, sequence, parallel, group, stagger, path, morph, draw, color, waveform, preset, label, at"
        ),
    )
}

/// Create a "missing duration" error.
pub fn missing_duration(span: Span) -> Error {
    Error::new(span, "missing required field `duration`")
}

/// Create a "stagger grid missing cols/rows" error.
pub fn stagger_grid_incomplete(span: Span) -> Error {
    Error::new(
        span,
        "stagger `grid` pattern requires both `cols` and `rows`",
    )
}

/// Create an "invalid color" error.
pub fn invalid_color(span: Span, color: &str) -> Error {
    Error::new(
        span,
        format!(
            "invalid color `{color}`; expected hex (#rgb, #rgba, #rrggbb, #rrggbbaa) or a named color"
        ),
    )
}

/// Create a "feature not enabled" error for framework macros.
pub fn feature_not_enabled(span: Span, feature: &str) -> Error {
    Error::new(
        span,
        format!("the `{feature}` feature must be enabled on `animato-macro` to use this macro"),
    )
}
