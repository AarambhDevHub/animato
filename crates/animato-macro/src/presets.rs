//! Built-in animation presets for the Motion Macro DSL.
//!
//! Each preset is a function returning a [`MotionNode`] tree. The expander
//! invokes these when a `preset <name>` call is encountered.

use crate::ast::*;

/// Returns `true` if `name` is a known built-in preset.
pub fn is_known_preset(name: &str) -> bool {
    matches!(
        name,
        "fade_in"
            | "fade_out"
            | "slide_in"
            | "slide_out"
            | "scale_in"
            | "scale_out"
            | "bounce_in"
            | "bounce_out"
            | "modal_enter"
            | "modal_exit"
            | "drawer_open"
            | "drawer_close"
            | "toast_enter"
            | "toast_exit"
            | "page_enter"
            | "page_exit"
            | "stagger_children"
            | "loading_pulse"
            | "loading_wave"
            | "shake"
            | "wiggle"
            | "heartbeat"
            | "float"
            | "shimmer"
    )
}

/// Returns the AST for a built-in preset, with optional duration/easing
/// overrides applied.
///
/// Returns `None` for unknown presets.
pub fn build_preset(call: &PresetCall) -> Option<MotionNode> {
    let span = call.span;
    let duration = call.duration;
    let easing = call.easing.clone();
    let node = match call.name.as_str() {
        "fade_in" => fade_in(duration, easing.clone(), span),
        "fade_out" => fade_out(duration, easing.clone(), span),
        "slide_in" => slide_in(duration, easing.clone(), span),
        "slide_out" => slide_out(duration, easing.clone(), span),
        "scale_in" => scale_in(duration, easing.clone(), span),
        "scale_out" => scale_out(duration, easing.clone(), span),
        "bounce_in" => bounce_in(duration, easing.clone(), span),
        "bounce_out" => bounce_out(duration, easing.clone(), span),
        "modal_enter" => modal_enter(duration, easing.clone(), span),
        "modal_exit" => modal_exit(duration, easing.clone(), span),
        "drawer_open" => drawer_open(duration, easing.clone(), span),
        "drawer_close" => drawer_close(duration, easing.clone(), span),
        "toast_enter" => toast_enter(duration, easing.clone(), span),
        "toast_exit" => toast_exit(duration, easing.clone(), span),
        "page_enter" => page_enter(duration, easing.clone(), span),
        "page_exit" => page_exit(duration, easing.clone(), span),
        "stagger_children" => stagger_children(duration, easing.clone(), span),
        "loading_pulse" => loading_pulse(duration, easing.clone(), span),
        "loading_wave" => loading_wave(duration, easing.clone(), span),
        "shake" => shake(duration, easing.clone(), span),
        "wiggle" => wiggle(duration, easing.clone(), span),
        "heartbeat" => heartbeat(duration, easing.clone(), span),
        "float" => float(duration, easing.clone(), span),
        "shimmer" => shimmer(duration, easing.clone(), span),
        _ => return None,
    };
    Some(node)
}

/// Returns `true` if `name` is a recognized CSS named color.
pub fn is_named_color(name: &str) -> bool {
    matches!(
        name.to_lowercase().as_str(),
        "black"
            | "white"
            | "red"
            | "green"
            | "blue"
            | "yellow"
            | "cyan"
            | "magenta"
            | "gray"
            | "grey"
            | "orange"
            | "purple"
            | "pink"
            | "brown"
            | "lime"
            | "teal"
            | "navy"
            | "maroon"
            | "olive"
            | "silver"
            | "aqua"
            | "fuchsia"
    )
}

/// Resolve a named color to an `(r, g, b)` tuple in `0..=255`.
pub fn named_color_rgb(name: &str) -> Option<(u8, u8, u8)> {
    Some(match name.to_lowercase().as_str() {
        "black" => (0, 0, 0),
        "white" => (255, 255, 255),
        "red" => (255, 0, 0),
        "green" => (0, 128, 0),
        "blue" => (0, 0, 255),
        "yellow" => (255, 255, 0),
        "cyan" | "aqua" => (0, 255, 255),
        "magenta" | "fuchsia" => (255, 0, 255),
        "gray" | "grey" => (128, 128, 128),
        "orange" => (255, 165, 0),
        "purple" => (128, 0, 128),
        "pink" => (255, 192, 203),
        "brown" => (165, 42, 42),
        "lime" => (0, 255, 0),
        "teal" => (0, 128, 128),
        "navy" => (0, 0, 128),
        "maroon" => (128, 0, 0),
        "olive" => (128, 128, 0),
        "silver" => (192, 192, 192),
        _ => return None,
    })
}

// ── Hex color parser ────────────────────────────────────────────────────────

/// Parse a hex color string into `(r, g, b, a)` where each component is `0..=255`.
///
/// Returns `None` for invalid strings.
pub fn parse_hex_color(s: &str) -> Option<(u8, u8, u8, u8)> {
    let hex = s.strip_prefix('#')?;
    match hex.len() {
        3 => {
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
            Some((r, g, b, 255))
        }
        4 => {
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
            let a = u8::from_str_radix(&hex[3..4].repeat(2), 16).ok()?;
            Some((r, g, b, a))
        }
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some((r, g, b, 255))
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
            Some((r, g, b, a))
        }
        _ => None,
    }
}

/// Resolve a color string (hex or named) into `(r, g, b, a)` in `0..=255`.
pub fn resolve_color(s: &str) -> Option<(u8, u8, u8, u8)> {
    if s.starts_with('#') {
        parse_hex_color(s)
    } else {
        named_color_rgb(s).map(|(r, g, b)| (r, g, b, 255))
    }
}

// ── Preset definitions ──────────────────────────────────────────────────────

fn default_duration(override_: Option<f32>, default: f32) -> f32 {
    override_.unwrap_or(default)
}

fn make_tween(
    target: &str,
    from: f32,
    to: f32,
    duration: Option<f32>,
    easing: Option<EasingSpec>,
    span: proc_macro2::Span,
) -> MotionNode {
    MotionNode::Tween(TweenSpec {
        target: target.into(),
        from: ValueExpr::Scalar(from),
        to: ValueExpr::Scalar(to),
        duration: default_duration(duration, 0.4),
        easing: easing.or_else(|| Some(EasingSpec::Named("ease_out_cubic".into()))),
        delay: None,
        time_scale: None,
        loop_mode: None,
        snap: None,
        round: None,
        reverse: false,
        span,
    })
}

fn fade_in(
    duration: Option<f32>,
    easing: Option<EasingSpec>,
    span: proc_macro2::Span,
) -> MotionNode {
    make_tween(
        "opacity",
        0.0,
        1.0,
        Some(default_duration(duration, 0.4)),
        easing,
        span,
    )
}

fn fade_out(
    duration: Option<f32>,
    easing: Option<EasingSpec>,
    span: proc_macro2::Span,
) -> MotionNode {
    let mut node = make_tween(
        "opacity",
        1.0,
        0.0,
        Some(default_duration(duration, 0.4)),
        easing,
        span,
    );
    if let MotionNode::Tween(ref mut t) = node {
        t.easing = t
            .easing
            .clone()
            .or_else(|| Some(EasingSpec::Named("ease_in_cubic".into())));
    }
    node
}

fn slide_in(
    duration: Option<f32>,
    easing: Option<EasingSpec>,
    span: proc_macro2::Span,
) -> MotionNode {
    make_tween(
        "x",
        -100.0,
        0.0,
        Some(default_duration(duration, 0.5)),
        easing,
        span,
    )
}

fn slide_out(
    duration: Option<f32>,
    easing: Option<EasingSpec>,
    span: proc_macro2::Span,
) -> MotionNode {
    make_tween(
        "x",
        0.0,
        100.0,
        Some(default_duration(duration, 0.5)),
        easing,
        span,
    )
}

fn scale_in(
    duration: Option<f32>,
    easing: Option<EasingSpec>,
    span: proc_macro2::Span,
) -> MotionNode {
    let mut node = make_tween(
        "scale",
        0.0,
        1.0,
        Some(default_duration(duration, 0.4)),
        easing,
        span,
    );
    if let MotionNode::Tween(ref mut t) = node {
        t.easing = t
            .easing
            .clone()
            .or_else(|| Some(EasingSpec::Named("ease_out_back".into())));
    }
    node
}

fn scale_out(
    duration: Option<f32>,
    easing: Option<EasingSpec>,
    span: proc_macro2::Span,
) -> MotionNode {
    let mut node = make_tween(
        "scale",
        1.0,
        0.0,
        Some(default_duration(duration, 0.4)),
        easing,
        span,
    );
    if let MotionNode::Tween(ref mut t) = node {
        t.easing = t
            .easing
            .clone()
            .or_else(|| Some(EasingSpec::Named("ease_in_back".into())));
    }
    node
}

fn bounce_in(
    duration: Option<f32>,
    easing: Option<EasingSpec>,
    span: proc_macro2::Span,
) -> MotionNode {
    let _dur = default_duration(duration, 0.6);
    let ease = easing.unwrap_or_else(|| EasingSpec::Named("ease_out_bounce".into()));
    MotionNode::Keyframes(KeyframeSpec {
        target: "scale".into(),
        frames: vec![
            KeyframeEntry {
                time: KeyframeTime::Percent(0.0),
                value: ValueExpr::Scalar(0.0),
                easing: None,
            },
            KeyframeEntry {
                time: KeyframeTime::Percent(0.6),
                value: ValueExpr::Scalar(1.1),
                easing: None,
            },
            KeyframeEntry {
                time: KeyframeTime::Percent(0.8),
                value: ValueExpr::Scalar(0.95),
                easing: None,
            },
            KeyframeEntry {
                time: KeyframeTime::Percent(1.0),
                value: ValueExpr::Scalar(1.0),
                easing: Some(ease),
            },
        ],
        loop_mode: None,
        span,
    })
}

fn bounce_out(
    duration: Option<f32>,
    easing: Option<EasingSpec>,
    span: proc_macro2::Span,
) -> MotionNode {
    let _dur = default_duration(duration, 0.6);
    let ease = easing.unwrap_or_else(|| EasingSpec::Named("ease_in_bounce".into()));
    MotionNode::Keyframes(KeyframeSpec {
        target: "scale".into(),
        frames: vec![
            KeyframeEntry {
                time: KeyframeTime::Percent(0.0),
                value: ValueExpr::Scalar(1.0),
                easing: Some(ease),
            },
            KeyframeEntry {
                time: KeyframeTime::Percent(0.2),
                value: ValueExpr::Scalar(0.95),
                easing: None,
            },
            KeyframeEntry {
                time: KeyframeTime::Percent(0.4),
                value: ValueExpr::Scalar(1.1),
                easing: None,
            },
            KeyframeEntry {
                time: KeyframeTime::Percent(1.0),
                value: ValueExpr::Scalar(0.0),
                easing: None,
            },
        ],
        loop_mode: None,
        span,
    })
}

fn modal_enter(
    duration: Option<f32>,
    easing: Option<EasingSpec>,
    span: proc_macro2::Span,
) -> MotionNode {
    let dur = default_duration(duration, 0.4);
    MotionNode::Parallel(vec![
        make_tween("opacity", 0.0, 1.0, Some(dur), easing.clone(), span),
        {
            let mut s = make_tween("scale", 0.9, 1.0, Some(dur), easing, span);
            if let MotionNode::Tween(ref mut t) = s {
                t.easing = t
                    .easing
                    .clone()
                    .or_else(|| Some(EasingSpec::Named("ease_out_back".into())));
            }
            s
        },
    ])
}

fn modal_exit(
    duration: Option<f32>,
    easing: Option<EasingSpec>,
    span: proc_macro2::Span,
) -> MotionNode {
    let dur = default_duration(duration, 0.3);
    MotionNode::Parallel(vec![
        make_tween("opacity", 1.0, 0.0, Some(dur), easing.clone(), span),
        make_tween("scale", 1.0, 0.9, Some(dur), easing, span),
    ])
}

fn drawer_open(
    duration: Option<f32>,
    easing: Option<EasingSpec>,
    span: proc_macro2::Span,
) -> MotionNode {
    let mut node = make_tween(
        "x",
        -320.0,
        0.0,
        Some(default_duration(duration, 0.4)),
        easing,
        span,
    );
    if let MotionNode::Tween(ref mut t) = node {
        t.easing = t
            .easing
            .clone()
            .or_else(|| Some(EasingSpec::Named("ease_out_cubic".into())));
    }
    node
}

fn drawer_close(
    duration: Option<f32>,
    easing: Option<EasingSpec>,
    span: proc_macro2::Span,
) -> MotionNode {
    let mut node = make_tween(
        "x",
        0.0,
        -320.0,
        Some(default_duration(duration, 0.4)),
        easing,
        span,
    );
    if let MotionNode::Tween(ref mut t) = node {
        t.easing = t
            .easing
            .clone()
            .or_else(|| Some(EasingSpec::Named("ease_in_cubic".into())));
    }
    node
}

fn toast_enter(
    duration: Option<f32>,
    easing: Option<EasingSpec>,
    span: proc_macro2::Span,
) -> MotionNode {
    let dur = default_duration(duration, 0.4);
    MotionNode::Sequence(vec![
        make_tween("y", 40.0, 0.0, Some(dur), easing.clone(), span),
        make_tween("opacity", 0.0, 1.0, Some(dur), easing, span),
    ])
}

fn toast_exit(
    duration: Option<f32>,
    easing: Option<EasingSpec>,
    span: proc_macro2::Span,
) -> MotionNode {
    let dur = default_duration(duration, 0.3);
    MotionNode::Sequence(vec![
        make_tween("opacity", 1.0, 0.0, Some(dur), easing.clone(), span),
        make_tween("y", 0.0, 40.0, Some(dur), easing, span),
    ])
}

fn page_enter(
    duration: Option<f32>,
    easing: Option<EasingSpec>,
    span: proc_macro2::Span,
) -> MotionNode {
    let dur = default_duration(duration, 0.5);
    MotionNode::Parallel(vec![
        make_tween("opacity", 0.0, 1.0, Some(dur), easing.clone(), span),
        make_tween("y", 20.0, 0.0, Some(dur), easing, span),
    ])
}

fn page_exit(
    duration: Option<f32>,
    easing: Option<EasingSpec>,
    span: proc_macro2::Span,
) -> MotionNode {
    let dur = default_duration(duration, 0.4);
    MotionNode::Parallel(vec![
        make_tween("opacity", 1.0, 0.0, Some(dur), easing.clone(), span),
        make_tween("y", 0.0, -20.0, Some(dur), easing, span),
    ])
}

fn stagger_children(
    duration: Option<f32>,
    easing: Option<EasingSpec>,
    span: proc_macro2::Span,
) -> MotionNode {
    // This preset is a marker — actual stagger is applied by the caller.
    // For standalone use, it expands to a simple fade_in.
    fade_in(duration, easing, span)
}

fn loading_pulse(
    duration: Option<f32>,
    easing: Option<EasingSpec>,
    span: proc_macro2::Span,
) -> MotionNode {
    let _ = duration;
    let _ = easing;
    MotionNode::Keyframes(KeyframeSpec {
        target: "opacity".into(),
        frames: vec![
            KeyframeEntry {
                time: KeyframeTime::Percent(0.0),
                value: ValueExpr::Scalar(0.4),
                easing: None,
            },
            KeyframeEntry {
                time: KeyframeTime::Percent(0.5),
                value: ValueExpr::Scalar(1.0),
                easing: Some(EasingSpec::Named("ease_in_out_sine".into())),
            },
            KeyframeEntry {
                time: KeyframeTime::Percent(1.0),
                value: ValueExpr::Scalar(0.4),
                easing: Some(EasingSpec::Named("ease_in_out_sine".into())),
            },
        ],
        loop_mode: Some(LoopSpec::Forever),
        span,
    })
}

fn loading_wave(
    duration: Option<f32>,
    easing: Option<EasingSpec>,
    span: proc_macro2::Span,
) -> MotionNode {
    let _ = duration;
    let _ = easing;
    MotionNode::Waveform(WaveformSpec {
        kind: WaveformKind::Sine,
        frequency: Some(1.5),
        amplitude: Some(0.3),
        phase: Some(0.0),
        duty_cycle: None,
        seed: None,
        smoothness: None,
        duration: Some(2.0),
        span,
    })
}

fn shake(duration: Option<f32>, easing: Option<EasingSpec>, span: proc_macro2::Span) -> MotionNode {
    let _ = duration;
    let _ = easing;
    MotionNode::Keyframes(KeyframeSpec {
        target: "x".into(),
        frames: vec![
            KeyframeEntry {
                time: KeyframeTime::Percent(0.0),
                value: ValueExpr::Scalar(0.0),
                easing: None,
            },
            KeyframeEntry {
                time: KeyframeTime::Percent(0.2),
                value: ValueExpr::Scalar(-10.0),
                easing: None,
            },
            KeyframeEntry {
                time: KeyframeTime::Percent(0.4),
                value: ValueExpr::Scalar(10.0),
                easing: None,
            },
            KeyframeEntry {
                time: KeyframeTime::Percent(0.6),
                value: ValueExpr::Scalar(-8.0),
                easing: None,
            },
            KeyframeEntry {
                time: KeyframeTime::Percent(0.8),
                value: ValueExpr::Scalar(8.0),
                easing: None,
            },
            KeyframeEntry {
                time: KeyframeTime::Percent(1.0),
                value: ValueExpr::Scalar(0.0),
                easing: None,
            },
        ],
        loop_mode: None,
        span,
    })
}

fn wiggle(
    duration: Option<f32>,
    easing: Option<EasingSpec>,
    span: proc_macro2::Span,
) -> MotionNode {
    let _ = duration;
    let _ = easing;
    MotionNode::Keyframes(KeyframeSpec {
        target: "rotation".into(),
        frames: vec![
            KeyframeEntry {
                time: KeyframeTime::Percent(0.0),
                value: ValueExpr::Scalar(0.0),
                easing: None,
            },
            KeyframeEntry {
                time: KeyframeTime::Percent(0.25),
                value: ValueExpr::Scalar(5.0),
                easing: None,
            },
            KeyframeEntry {
                time: KeyframeTime::Percent(0.75),
                value: ValueExpr::Scalar(-5.0),
                easing: None,
            },
            KeyframeEntry {
                time: KeyframeTime::Percent(1.0),
                value: ValueExpr::Scalar(0.0),
                easing: None,
            },
        ],
        loop_mode: Some(LoopSpec::PingPong),
        span,
    })
}

fn heartbeat(
    duration: Option<f32>,
    easing: Option<EasingSpec>,
    span: proc_macro2::Span,
) -> MotionNode {
    let _ = duration;
    let _ = easing;
    MotionNode::Keyframes(KeyframeSpec {
        target: "scale".into(),
        frames: vec![
            KeyframeEntry {
                time: KeyframeTime::Percent(0.0),
                value: ValueExpr::Scalar(1.0),
                easing: None,
            },
            KeyframeEntry {
                time: KeyframeTime::Percent(0.15),
                value: ValueExpr::Scalar(1.1),
                easing: Some(EasingSpec::Named("ease_out_quad".into())),
            },
            KeyframeEntry {
                time: KeyframeTime::Percent(0.3),
                value: ValueExpr::Scalar(1.0),
                easing: None,
            },
            KeyframeEntry {
                time: KeyframeTime::Percent(0.45),
                value: ValueExpr::Scalar(1.05),
                easing: Some(EasingSpec::Named("ease_out_quad".into())),
            },
            KeyframeEntry {
                time: KeyframeTime::Percent(1.0),
                value: ValueExpr::Scalar(1.0),
                easing: None,
            },
        ],
        loop_mode: Some(LoopSpec::Forever),
        span,
    })
}

fn float(duration: Option<f32>, easing: Option<EasingSpec>, span: proc_macro2::Span) -> MotionNode {
    let _ = duration;
    let _ = easing;
    MotionNode::Waveform(WaveformSpec {
        kind: WaveformKind::Sine,
        frequency: Some(0.5),
        amplitude: Some(10.0),
        phase: Some(0.0),
        duty_cycle: None,
        seed: None,
        smoothness: None,
        duration: Some(4.0),
        span,
    })
}

fn shimmer(
    duration: Option<f32>,
    easing: Option<EasingSpec>,
    span: proc_macro2::Span,
) -> MotionNode {
    let _ = duration;
    let _ = easing;
    MotionNode::Keyframes(KeyframeSpec {
        target: "background_position_x".into(),
        frames: vec![
            KeyframeEntry {
                time: KeyframeTime::Percent(0.0),
                value: ValueExpr::Scalar(-200.0),
                easing: Some(EasingSpec::Named("ease_in_out_sine".into())),
            },
            KeyframeEntry {
                time: KeyframeTime::Percent(1.0),
                value: ValueExpr::Scalar(200.0),
                easing: None,
            },
        ],
        loop_mode: Some(LoopSpec::Forever),
        span,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_presets_recognized() {
        assert!(is_known_preset("fade_in"));
        assert!(is_known_preset("modal_enter"));
        assert!(is_known_preset("shimmer"));
        assert!(!is_known_preset("not_a_preset"));
    }

    #[test]
    fn hex_colors_parse() {
        assert_eq!(parse_hex_color("#fff"), Some((255, 255, 255, 255)));
        assert_eq!(parse_hex_color("#ff0000"), Some((255, 0, 0, 255)));
        assert_eq!(parse_hex_color("#ff000080"), Some((255, 0, 0, 128)));
        assert_eq!(parse_hex_color("#gg0000"), None);
    }

    #[test]
    fn named_colors_resolve() {
        assert_eq!(named_color_rgb("red"), Some((255, 0, 0)));
        assert_eq!(named_color_rgb("BLUE"), Some((0, 0, 255)));
        assert_eq!(named_color_rgb("notacolor"), None);
    }

    #[test]
    fn preset_builds_return_nodes() {
        let call = PresetCall {
            name: "fade_in".into(),
            duration: None,
            easing: None,
            span: proc_macro2::Span::call_site(),
        };
        assert!(build_preset(&call).is_some());
    }
}
