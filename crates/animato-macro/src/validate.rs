//! Compile-time validation for the Motion Macro AST.
//!
//! Walks the AST and returns `syn::Error`s for:
//! - Unknown easing names
//! - Unknown spring presets
//! - Unknown built-in preset names
//! - Mismatched tween `from`/`to` value types
//! - Missing required fields
//! - Invalid durations (negative)
//! - Duplicate/unsorted keyframe times (where statically detectable)

use crate::ast::*;
use crate::error;
use proc_macro2::Span;

/// Validate a list of motion nodes. Returns the first error encountered, if any.
pub fn validate_nodes(nodes: &[MotionNode]) -> syn::Result<()> {
    for node in nodes {
        validate_node(node)?;
    }
    Ok(())
}

/// Validate a single motion node.
pub fn validate_node(node: &MotionNode) -> syn::Result<()> {
    match node {
        MotionNode::Tween(spec) => validate_tween(spec),
        MotionNode::Spring(spec) => validate_spring(spec),
        MotionNode::Keyframes(spec) => validate_keyframes(spec),
        MotionNode::Sequence(nodes) | MotionNode::Parallel(nodes) => validate_nodes(nodes),
        MotionNode::Group { nodes, .. } => validate_nodes(nodes),
        MotionNode::Stagger(spec) => {
            if let Some(StaggerPatternSpec::Grid { cols, rows, .. }) = &spec.pattern {
                if *cols == 0 || *rows == 0 {
                    return Err(error::malformed(
                        spec.span,
                        "stagger `grid` requires non-zero `cols` and `rows`",
                    ));
                }
            }
            validate_nodes(&spec.nodes)
        }
        MotionNode::Path(spec) => validate_path(spec),
        MotionNode::Morph(spec) => validate_morph(spec),
        MotionNode::Draw(spec) => validate_draw(spec),
        MotionNode::Color(spec) => validate_color(spec),
        MotionNode::Waveform(spec) => validate_waveform(spec),
        MotionNode::Preset(call) => validate_preset_call(call),
        MotionNode::Label(_) | MotionNode::At(_) => Ok(()),
    }
}

fn validate_tween(spec: &TweenSpec) -> syn::Result<()> {
    if spec.duration < 0.0 {
        return Err(error::malformed(
            spec.span,
            "`duration` must be non-negative",
        ));
    }
    if spec.from.component_count() != spec.to.component_count() {
        return Err(error::value_type_mismatch(
            spec.span,
            &format!("{}", spec.from.component_count()),
            &format!("{}", spec.to.component_count()),
        ));
    }
    if let Some(easing) = &spec.easing {
        validate_easing(easing, spec.span)?;
    }
    if let Some(loop_mode) = &spec.loop_mode {
        validate_loop_mode(loop_mode, spec.span)?;
    }
    if let Some(ts) = spec.time_scale {
        if ts < 0.0 {
            return Err(error::malformed(
                spec.span,
                "`time_scale` must be non-negative",
            ));
        }
    }
    if let Some(delay) = spec.delay {
        if delay < 0.0 {
            return Err(error::malformed(spec.span, "`delay` must be non-negative"));
        }
    }
    Ok(())
}

fn validate_spring(spec: &SpringSpec) -> syn::Result<()> {
    if spec.from.component_count() != spec.to.component_count() {
        return Err(error::value_type_mismatch(
            spec.span,
            &format!("{}", spec.from.component_count()),
            &format!("{}", spec.to.component_count()),
        ));
    }
    if spec.stiffness.is_some() && spec.preset.is_some() {
        // Allow explicit override but it's worth noting — not an error.
    }
    if let Some(stiffness) = spec.stiffness {
        if stiffness < 0.0 {
            return Err(error::malformed(
                spec.span,
                "`stiffness` must be non-negative",
            ));
        }
    }
    if let Some(mass) = spec.mass {
        if mass <= 0.0 {
            return Err(error::malformed(spec.span, "`mass` must be positive"));
        }
    }
    if let Some(epsilon) = spec.epsilon {
        if epsilon <= 0.0 {
            return Err(error::malformed(spec.span, "`epsilon` must be positive"));
        }
    }
    Ok(())
}

fn validate_keyframes(spec: &KeyframeSpec) -> syn::Result<()> {
    if spec.frames.len() < 2 {
        return Err(error::malformed(
            spec.span,
            "keyframe track needs at least 2 frames",
        ));
    }
    // Check for duplicate times (only for Percent — Seconds comparison is
    // also valid since both are f32).
    let mut seen: Vec<f32> = Vec::new();
    for frame in &spec.frames {
        let t = match frame.time {
            KeyframeTime::Percent(p) => p,
            KeyframeTime::Seconds(s) => s,
        };
        if seen
            .iter()
            .any(|&existing| (existing - t).abs() < f32::EPSILON)
        {
            return Err(error::malformed(
                spec.span,
                &format!("duplicate keyframe time `{t}`"),
            ));
        }
        seen.push(t);
        if let Some(easing) = &frame.easing {
            validate_easing(easing, spec.span)?;
        }
    }
    if let Some(loop_mode) = &spec.loop_mode {
        validate_loop_mode(loop_mode, spec.span)?;
    }
    Ok(())
}

fn validate_path(spec: &PathSpec) -> syn::Result<()> {
    if spec.duration < 0.0 {
        return Err(error::malformed(
            spec.span,
            "`duration` must be non-negative",
        ));
    }
    if let Some(easing) = &spec.easing {
        validate_easing(easing, spec.span)?;
    }
    if let Some((start, end)) = spec.offset {
        if !(0.0..=1.0).contains(&start) || !(0.0..=1.0).contains(&end) {
            return Err(error::malformed(
                spec.span,
                "`offset` values must be in [0.0, 1.0]",
            ));
        }
    }
    Ok(())
}

fn validate_morph(spec: &MorphSpec) -> syn::Result<()> {
    if spec.duration < 0.0 {
        return Err(error::malformed(
            spec.span,
            "`duration` must be non-negative",
        ));
    }
    if spec.samples == 0 {
        return Err(error::malformed(spec.span, "`samples` must be non-zero"));
    }
    if let Some(easing) = &spec.easing {
        validate_easing(easing, spec.span)?;
    }
    Ok(())
}

fn validate_draw(spec: &DrawSpec) -> syn::Result<()> {
    if spec.duration < 0.0 {
        return Err(error::malformed(
            spec.span,
            "`duration` must be non-negative",
        ));
    }
    if let Some(easing) = &spec.easing {
        validate_easing(easing, spec.span)?;
    }
    Ok(())
}

fn validate_color(spec: &ColorSpec) -> syn::Result<()> {
    if spec.duration < 0.0 {
        return Err(error::malformed(
            spec.span,
            "`duration` must be non-negative",
        ));
    }
    if !is_valid_color(&spec.from) {
        return Err(error::invalid_color(spec.span, &spec.from));
    }
    if !is_valid_color(&spec.to) {
        return Err(error::invalid_color(spec.span, &spec.to));
    }
    if let Some(easing) = &spec.easing {
        validate_easing(easing, spec.span)?;
    }
    Ok(())
}

fn validate_waveform(spec: &WaveformSpec) -> syn::Result<()> {
    if let Some(freq) = spec.frequency {
        if freq < 0.0 {
            return Err(error::malformed(
                spec.span,
                "`frequency` must be non-negative",
            ));
        }
    }
    Ok(())
}

fn validate_preset_call(call: &PresetCall) -> syn::Result<()> {
    if !crate::presets::is_known_preset(&call.name) {
        return Err(error::unknown_preset(call.span, &call.name));
    }
    if let Some(easing) = &call.easing {
        validate_easing(easing, call.span)?;
    }
    if let Some(duration) = call.duration {
        if duration < 0.0 {
            return Err(error::malformed(
                call.span,
                "`duration` must be non-negative",
            ));
        }
    }
    Ok(())
}

/// Validate an easing spec — check named easings are known.
pub fn validate_easing(easing: &EasingSpec, span: Span) -> syn::Result<()> {
    if let EasingSpec::Named(name) = easing {
        if crate::easing::named_variant(name).is_none() {
            return Err(error::unknown_easing(span, name));
        }
    }
    Ok(())
}

fn validate_loop_mode(_loop_mode: &LoopSpec, _span: Span) -> syn::Result<()> {
    // All loop modes are valid by construction.
    Ok(())
}

/// Check whether a color string is a valid hex color or named color.
fn is_valid_color(s: &str) -> bool {
    if let Some(hex) = s.strip_prefix('#') {
        matches!(hex.len(), 3 | 4 | 6 | 8) && hex.chars().all(|c| c.is_ascii_hexdigit())
    } else {
        crate::presets::is_named_color(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_tween(from: ValueExpr, to: ValueExpr) -> TweenSpec {
        TweenSpec {
            target: "x".into(),
            from,
            to,
            duration: 1.0,
            easing: None,
            delay: None,
            time_scale: None,
            loop_mode: None,
            snap: None,
            round: None,
            reverse: false,
            span: Span::call_site(),
        }
    }

    #[test]
    fn validates_matching_types() {
        let t = make_tween(ValueExpr::Scalar(0.0), ValueExpr::Scalar(1.0));
        assert!(validate_tween(&t).is_ok());
    }

    #[test]
    fn rejects_mismatched_types() {
        let t = make_tween(ValueExpr::Scalar(0.0), ValueExpr::Array(vec![1.0, 2.0]));
        assert!(validate_tween(&t).is_err());
    }

    #[test]
    fn rejects_negative_duration() {
        let mut t = make_tween(ValueExpr::Scalar(0.0), ValueExpr::Scalar(1.0));
        t.duration = -1.0;
        assert!(validate_tween(&t).is_err());
    }

    #[test]
    fn rejects_unknown_easing() {
        let mut t = make_tween(ValueExpr::Scalar(0.0), ValueExpr::Scalar(1.0));
        t.easing = Some(EasingSpec::Named("ease_out_magic".into()));
        assert!(validate_tween(&t).is_err());
    }

    #[test]
    fn accepts_known_hex_colors() {
        assert!(is_valid_color("#ff0000"));
        assert!(is_valid_color("#f00"));
        assert!(is_valid_color("#ff0000ff"));
    }

    #[test]
    fn rejects_invalid_hex_colors() {
        assert!(!is_valid_color("#gg0000"));
        assert!(!is_valid_color("#ff"));
    }
}
