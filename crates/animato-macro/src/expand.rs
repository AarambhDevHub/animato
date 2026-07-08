//! Code generation for the Motion Macro DSL.
//!
//! Each [`MotionNode`](crate::ast::MotionNode) variant expands into a
//! `TokenStream` that constructs the matching stable Animato primitive.

use crate::ast::*;
use crate::easing;
use proc_macro2::TokenStream;
use quote::quote;

/// Expand a list of motion nodes into a single expression.
///
/// If there is exactly one node, it expands to that node's expression.
/// If there are multiple nodes, they are wrapped in a `Sequence`.
pub fn expand_nodes(nodes: &[MotionNode]) -> TokenStream {
    match nodes.len() {
        0 => quote! { animato::Sequence::new().build() },
        1 => expand_node(&nodes[0]),
        _ => {
            let parts: Vec<TokenStream> = nodes.iter().map(expand_node).collect();
            // Wrap in a sequence with auto-generated labels.
            quote! {{
                let mut __seq = animato::Sequence::new();
                #(
                    __seq = __seq.then(
                        concat!("node_", stringify!(#parts)),
                        #parts,
                    );
                )*
                __seq.build()
            }}
        }
    }
}

/// Expand a single motion node into a `TokenStream` expression.
pub fn expand_node(node: &MotionNode) -> TokenStream {
    match node {
        MotionNode::Tween(spec) => expand_tween(spec),
        MotionNode::Spring(spec) => expand_spring(spec),
        MotionNode::Keyframes(spec) => expand_keyframes(spec),
        MotionNode::Sequence(nodes) => expand_sequence(nodes),
        MotionNode::Parallel(nodes) => expand_parallel(nodes),
        MotionNode::Group { mode, nodes } => expand_group(*mode, nodes),
        MotionNode::Stagger(spec) => expand_stagger(spec),
        MotionNode::Path(spec) => expand_path(spec),
        MotionNode::Morph(spec) => expand_morph(spec),
        MotionNode::Draw(spec) => expand_draw(spec),
        MotionNode::Color(spec) => expand_color(spec),
        MotionNode::Waveform(spec) => expand_waveform(spec),
        MotionNode::Preset(call) => expand_preset(call),
        MotionNode::Label(_) | MotionNode::At(_) => {
            // Labels and `at` offsets are handled during sequence expansion.
            // Standalone, they produce a unit value (should not appear at top level).
            quote! { () }
        }
    }
}

// ── Tween ────────────────────────────────────────────────────────────────────

fn expand_tween(spec: &TweenSpec) -> TokenStream {
    let from = expand_value(&spec.from);
    let to = expand_value(&spec.to);
    let duration = spec.duration;

    let easing = match &spec.easing {
        Some(e) => {
            let e_tokens = easing::expand_easing(e);
            quote! { .easing(#e_tokens) }
        }
        None => quote! {},
    };

    let delay = spec
        .delay
        .map(|d| quote! { .delay(#d) })
        .unwrap_or_default();
    let time_scale = spec
        .time_scale
        .map(|ts| quote! { .time_scale(#ts) })
        .unwrap_or_default();
    let loop_mode = spec
        .loop_mode
        .as_ref()
        .map(expand_loop)
        .map(|l| quote! { .looping(#l) })
        .unwrap_or_default();
    let reverse = if spec.reverse {
        quote! { ; let __t = { let mut __r = __t; __r.reverse(); __r } }
    } else {
        quote! {}
    };

    // Snap and round require wrapping the built tween's value() in snap_to/round_to.
    let snap_wrap = if let Some(grid) = spec.snap {
        quote! {
            let __t = animato::snap_to(__t.value(), #grid);
        }
    } else if let Some(decimals) = spec.round {
        quote! {
            let __t = animato::round_to(__t.value(), #decimals);
        }
    } else {
        quote! {}
    };

    let ty_annotation = value_type_annotation(&spec.from);

    quote! {{
        let mut __t = animato::Tween::new(#from as #ty_annotation, #to as #ty_annotation)
            .duration(#duration)
            #easing
            #delay
            #time_scale
            #loop_mode
            .build();
        #reverse
        #snap_wrap
        __t
    }}
}

fn expand_value(value: &ValueExpr) -> TokenStream {
    match value {
        ValueExpr::Scalar(f) => quote! { #f },
        ValueExpr::Array(values) => {
            quote! { [#(#values),*] }
        }
    }
}

fn value_type_annotation(value: &ValueExpr) -> TokenStream {
    match value.component_count() {
        1 => quote! { f32 },
        2 => quote! { [f32; 2] },
        3 => quote! { [f32; 3] },
        4 => quote! { [f32; 4] },
        _ => quote! { f32 },
    }
}

fn expand_loop(mode: &LoopSpec) -> TokenStream {
    match mode {
        LoopSpec::Once => quote! { animato::Loop::Once },
        LoopSpec::Times(n) => quote! { animato::Loop::Times(#n) },
        LoopSpec::Forever => quote! { animato::Loop::Forever },
        LoopSpec::PingPong => quote! { animato::Loop::PingPong },
        LoopSpec::PingPongTimes(n) => quote! { animato::Loop::PingPongTimes(#n) },
    }
}

// ── Spring ───────────────────────────────────────────────────────────────────

fn expand_spring(spec: &SpringSpec) -> TokenStream {
    let from = expand_value(&spec.from);
    let to = expand_value(&spec.to);

    let config = expand_spring_config(spec);
    let use_rk4 = if matches!(spec.integrator, Some(IntegratorSpec::RungeKutta4)) {
        quote! { .use_rk4(true) }
    } else {
        quote! {}
    };

    // For scalar springs with velocity, use from_velocity.
    if let Some(vel) = &spec.velocity {
        let v = expand_value(vel);
        if spec.from.component_count() == 1 {
            return quote! {{
                let __cfg = #config;
                animato::Spring::from_velocity(#from, #v, #to, __cfg) #use_rk4
            }};
        }

        // Multi-dimensional spring with velocity.
        let ty = value_type_annotation(&spec.from);
        return quote! {{
            let __cfg = #config;
            animato::SpringN::from_velocity(#from as #ty, #v as #ty, #to as #ty, __cfg)
        }};
    }

    if spec.from.component_count() == 1 {
        quote! {{
            let __cfg = #config;
            let mut __s = animato::Spring::new(__cfg) #use_rk4;
            __s.snap_to(#from);
            __s.set_target(#to);
            __s
        }}
    } else {
        let ty = value_type_annotation(&spec.from);
        quote! {{
            let __cfg = #config;
            let mut __s: animato::SpringN<#ty> = animato::SpringN::new(__cfg, #from as #ty);
            __s.set_target(#to as #ty);
            __s
        }}
    }
}

fn expand_spring_config(spec: &SpringSpec) -> TokenStream {
    // Damping mode takes priority.
    if let Some(mode) = &spec.damping_mode {
        return match mode {
            DampingMode::CriticallyDamped(s) => {
                quote! { animato::SpringConfig::critically_damped(#s) }
            }
            DampingMode::Overdamped(s, r) => {
                quote! { animato::SpringConfig::overdamped(#s, #r) }
            }
            DampingMode::Underdamped(s, r) => {
                quote! { animato::SpringConfig::underdamped(#s, #r) }
            }
        };
    }

    // Explicit stiffness/damping/mass.
    if spec.stiffness.is_some() || spec.damping.is_some() || spec.mass.is_some() {
        let stiffness = spec.stiffness.unwrap_or(100.0);
        let damping = spec.damping.unwrap_or(10.0);
        let mass = spec.mass.unwrap_or(1.0);
        let epsilon = spec.epsilon.unwrap_or(0.001);
        return quote! {
            animato::SpringConfig {
                stiffness: #stiffness,
                damping: #damping,
                mass: #mass,
                epsilon: #epsilon,
            }
        };
    }

    // Named preset.
    match spec.preset {
        Some(SpringPreset::Gentle) => quote! { animato::SpringConfig::gentle() },
        Some(SpringPreset::Wobbly) => quote! { animato::SpringConfig::wobbly() },
        Some(SpringPreset::Stiff) => quote! { animato::SpringConfig::stiff() },
        Some(SpringPreset::Slow) => quote! { animato::SpringConfig::slow() },
        Some(SpringPreset::Snappy) => quote! { animato::SpringConfig::snappy() },
        None => quote! { animato::SpringConfig::default() },
    }
}

// ── Keyframes ────────────────────────────────────────────────────────────────

fn expand_keyframes(spec: &KeyframeSpec) -> TokenStream {
    let ty = if spec.frames.is_empty() {
        quote! { f32 }
    } else {
        value_type_from_value(&spec.frames[0].value)
    };

    let pushes: Vec<TokenStream> = spec
        .frames
        .iter()
        .map(|frame| {
            let time = match frame.time {
                KeyframeTime::Percent(p) => p,
                KeyframeTime::Seconds(s) => s,
            };
            let value = expand_value(&frame.value);
            match &frame.easing {
                Some(e) => {
                    let e_tokens = easing::expand_easing(e);
                    quote! { __track = __track.push_eased(#time, #value as #ty, #e_tokens); }
                }
                None => {
                    quote! { __track = __track.push(#time, #value as #ty); }
                }
            }
        })
        .collect();

    let loop_mode = spec
        .loop_mode
        .as_ref()
        .map(expand_loop)
        .map(|l| quote! { __track = __track.looping(#l); })
        .unwrap_or_default();

    quote! {{
        let mut __track: animato::KeyframeTrack<#ty> = animato::KeyframeTrack::new();
        #(#pushes)*
        #loop_mode
        __track
    }}
}

fn value_type_from_value(value: &ValueExpr) -> TokenStream {
    match value.component_count() {
        1 => quote! { f32 },
        2 => quote! { [f32; 2] },
        3 => quote! { [f32; 3] },
        4 => quote! { [f32; 4] },
        _ => quote! { f32 },
    }
}

// ── Sequence / Parallel / Group ─────────────────────────────────────────────

fn expand_sequence(nodes: &[MotionNode]) -> TokenStream {
    let parts: Vec<TokenStream> = nodes.iter().map(expand_node).collect();
    let labels: Vec<String> = (0..nodes.len()).map(|i| format!("node_{i}")).collect();
    quote! {{
        let mut __seq = animato::Sequence::new();
        #(
            __seq = __seq.then(#labels, #parts);
        )*
        __seq.build()
    }}
}

fn expand_parallel(nodes: &[MotionNode]) -> TokenStream {
    let parts: Vec<TokenStream> = nodes.iter().map(expand_node).collect();
    let labels: Vec<String> = (0..nodes.len()).map(|i| format!("item_{i}")).collect();
    // Build a Timeline with all children starting at At::Start (parallel).
    // This avoids the homogeneous-type requirement of AnimationGroup::parallel.
    quote! {{
        let mut __tl = animato::Timeline::new();
        #(
            __tl = __tl.add(#labels, #parts, animato::At::Start);
        )*
        __tl
    }}
}

fn expand_group(mode: GroupMode, nodes: &[MotionNode]) -> TokenStream {
    let parts: Vec<TokenStream> = nodes.iter().map(expand_node).collect();
    let labels: Vec<String> = (0..nodes.len()).map(|i| format!("item_{i}")).collect();
    match mode {
        GroupMode::Sequence => {
            quote! {{
                let mut __seq = animato::Sequence::new();
                #(
                    __seq = __seq.then(#labels, #parts);
                )*
                __seq.build()
            }}
        }
        GroupMode::Parallel => {
            quote! {{
                let mut __tl = animato::Timeline::new();
                #(
                    __tl = __tl.add(#labels, #parts, animato::At::Start);
                )*
                __tl
            }}
        }
    }
}

// ── Stagger ──────────────────────────────────────────────────────────────────

fn expand_stagger(spec: &StaggerSpec) -> TokenStream {
    let parts: Vec<TokenStream> = spec.nodes.iter().map(expand_node).collect();

    let pattern_tokens = match &spec.pattern {
        Some(StaggerPatternSpec::Grid { cols, rows, origin }) => {
            let origin_tokens = expand_grid_origin(*origin);
            let delay = spec.delay.unwrap_or(0.05);
            let cols = *cols as usize;
            let rows = *rows as usize;
            quote! {
                animato::StaggerPattern::Grid {
                    cols: #cols,
                    rows: #rows,
                    origin: #origin_tokens,
                    step: #delay,
                }
            }
        }
        Some(StaggerPatternSpec::Random { seed, min, max }) => {
            quote! {
                animato::StaggerPattern::Random {
                    seed: #seed,
                    min_delay: #min,
                    max_delay: #max,
                }
            }
        }
        Some(StaggerPatternSpec::CenterOut) => {
            let count = spec.nodes.len();
            let delay = spec.delay.unwrap_or(0.05);
            quote! {
                animato::StaggerPattern::CenterOut {
                    count: #count,
                    step: #delay,
                }
            }
        }
        Some(StaggerPatternSpec::EdgesIn) => {
            let count = spec.nodes.len();
            let delay = spec.delay.unwrap_or(0.05);
            quote! {
                animato::StaggerPattern::EdgesIn {
                    count: #count,
                    step: #delay,
                }
            }
        }
        None => {
            // Linear stagger using AnimationGroup::stagger with a linear pattern.
            let delay = spec.delay.unwrap_or(0.05);
            quote! {
                animato::StaggerPattern::Grid {
                    cols: 1,
                    rows: 1,
                    origin: animato::GridOrigin::TopLeft,
                    step: #delay,
                }
            }
        }
    };

    // Generate index literals for each child animation.
    let indices: Vec<u32> = (0..spec.nodes.len() as u32).collect();
    let total = spec.nodes.len();

    quote! {{
        let __pattern = #pattern_tokens;
        let __animations = ::std::vec![#(#parts),*];
        let mut __timeline = animato::Timeline::new();
        #(
            let __idx = #indices as usize;
            let __delay = __pattern.delay(__idx, #total);
            __timeline = __timeline.add(
                concat!("item_", stringify!(#indices)),
                __animations[__idx].clone(),
                animato::At::Absolute(__delay),
            );
        )*
        __timeline
    }}
}

fn expand_grid_origin(origin: GridOriginSpec) -> TokenStream {
    match origin {
        GridOriginSpec::TopLeft => quote! { animato::GridOrigin::TopLeft },
        GridOriginSpec::TopRight => quote! { animato::GridOrigin::TopRight },
        GridOriginSpec::BottomLeft => quote! { animato::GridOrigin::BottomLeft },
        GridOriginSpec::BottomRight => quote! { animato::GridOrigin::BottomRight },
        GridOriginSpec::Center => quote! { animato::GridOrigin::Center },
        GridOriginSpec::Top => quote! { animato::GridOrigin::Top },
        GridOriginSpec::Bottom => quote! { animato::GridOrigin::Bottom },
        GridOriginSpec::Left => quote! { animato::GridOrigin::Left },
        GridOriginSpec::Right => quote! { animato::GridOrigin::Right },
    }
}

// ── Path ─────────────────────────────────────────────────────────────────────

fn expand_path(spec: &PathSpec) -> TokenStream {
    let svg = &spec.svg;
    let duration = spec.duration;
    let easing_tokens = spec
        .easing
        .as_ref()
        .map(|e| {
            let t = easing::expand_easing(e);
            quote! { .easing(#t) }
        })
        .unwrap_or_default();
    let auto_rotate = spec
        .auto_rotate
        .map(|b| quote! { .auto_rotate(#b) })
        .unwrap_or_default();
    let offset = spec
        .offset
        .map(|(s, e)| quote! { .start_offset(#s).end_offset(#e) })
        .unwrap_or_default();
    let loop_mode = spec
        .loop_mode
        .as_ref()
        .map(expand_loop)
        .map(|l| quote! { .looping(#l) })
        .unwrap_or_default();

    quote! {
        animato::MotionPathTween::new(animato::MotionPath::from_svg(#svg))
            .duration(#duration)
            #easing_tokens
            #auto_rotate
            #offset
            #loop_mode
            .build()
    }
}

// ── Morph ────────────────────────────────────────────────────────────────────

fn expand_morph(spec: &MorphSpec) -> TokenStream {
    let from = &spec.from;
    let to = &spec.to;
    let samples = spec.samples;
    let duration = spec.duration;
    let easing_tokens = spec
        .easing
        .as_ref()
        .map(|e| {
            let t = easing::expand_easing(e);
            quote! { .easing(#t) }
        })
        .unwrap_or_default();

    quote! {{
        let __morph = animato::MorphPath::new(
            animato::MotionPath::from_svg(#from),
            animato::MotionPath::from_svg(#to),
        );
        let __path = __morph.samples(#samples);
        animato::MotionPathTween::new(__path)
            .duration(#duration)
            #easing_tokens
            .build()
    }}
}

// ── Draw ─────────────────────────────────────────────────────────────────────

fn expand_draw(spec: &DrawSpec) -> TokenStream {
    let svg = &spec.svg;
    let duration = spec.duration;
    let easing_tokens = spec
        .easing
        .as_ref()
        .map(|e| {
            let t = easing::expand_easing(e);
            quote! { .easing(#t) }
        })
        .unwrap_or_default();

    quote! {{
        let __path = animato::MotionPath::from_svg(#svg);
        let __draw = animato::DrawSvg::new(__path);
        let mut __tween = animato::Tween::new(0.0_f32, 1.0_f32)
            .duration(#duration)
            #easing_tokens
            .build();
        __tween
    }}
}

// ── Color ────────────────────────────────────────────────────────────────────

fn expand_color(spec: &ColorSpec) -> TokenStream {
    let from_rgb = crate::presets::resolve_color(&spec.from);
    let to_rgb = crate::presets::resolve_color(&spec.to);

    let (fr, _, _, _) = from_rgb.unwrap_or((0, 0, 0, 255));
    let (tr, _, _, _) = to_rgb.unwrap_or((255, 255, 255, 255));

    let duration = spec.duration;
    let easing_tokens = spec
        .easing
        .as_ref()
        .map(|e| {
            let t = easing::expand_easing(e);
            quote! { .easing(#t) }
        })
        .unwrap_or_default();

    // Normalize the red channel to [0.0, 1.0] as a representative color value.
    let fr = fr as f32 / 255.0;
    let tr = tr as f32 / 255.0;

    quote! {{
        // We construct a tween over f32 (opacity-like) for the color.
        // Full color-space interpolation requires the animato-color wrappers,
        // which need palette::Srgba. Here we emit a scalar tween on a
        // representative channel as a simplified color animation.
        animato::Tween::new(#fr, #tr)
            .duration(#duration)
            #easing_tokens
            .build()
    }}
}

// ── Waveform ─────────────────────────────────────────────────────────────────

fn expand_waveform(spec: &WaveformSpec) -> TokenStream {
    let frequency = spec.frequency.unwrap_or(1.0);
    let amplitude = spec.amplitude.unwrap_or(1.0);
    let phase = spec.phase.unwrap_or(0.0);
    let duty_cycle = spec.duty_cycle.unwrap_or(0.5);
    let seed = spec.seed.unwrap_or(0);
    let smoothness = spec.smoothness.unwrap_or(0.25);

    // Build the waveform value with the right fields.
    let waveform_expr = match spec.kind {
        WaveformKind::Sine => {
            quote! { animato::Waveform::Sine { frequency: #frequency, amplitude: #amplitude, phase: #phase } }
        }
        WaveformKind::Sawtooth => {
            quote! { animato::Waveform::Sawtooth { frequency: #frequency, amplitude: #amplitude } }
        }
        WaveformKind::Square => {
            quote! { animato::Waveform::Square { frequency: #frequency, amplitude: #amplitude, duty_cycle: #duty_cycle } }
        }
        WaveformKind::Triangle => {
            quote! { animato::Waveform::Triangle { frequency: #frequency, amplitude: #amplitude } }
        }
        WaveformKind::Noise => {
            quote! { animato::Waveform::Noise { seed: #seed, smoothness: #smoothness } }
        }
    };

    let duration = spec.duration.unwrap_or(2.0);
    let samples = 64_u32;
    let sample_dt = if duration > 0.0 {
        duration / samples as f32
    } else {
        0.0
    };

    quote! {{
        let __wf = #waveform_expr;
        let mut __track: animato::KeyframeTrack<f32> = animato::KeyframeTrack::new();
        let __n: u32 = #samples;
        let __dt: f32 = #sample_dt;
        let mut __i: u32 = 0;
        while __i <= __n {
            let __t = __i as f32 * __dt;
            let __v = __wf.sample(__t);
            __track = __track.push(__t, __v);
            __i += 1;
        }
        __track
    }}
}

// ── Preset call ──────────────────────────────────────────────────────────────

fn expand_preset(call: &PresetCall) -> TokenStream {
    if let Some(node) = crate::presets::build_preset(call) {
        expand_node(&node)
    } else {
        // Unknown preset — should have been caught by validation.
        quote! { () }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::Span;

    #[test]
    fn expand_simple_tween() {
        let spec = TweenSpec {
            target: "opacity".into(),
            from: ValueExpr::Scalar(0.0),
            to: ValueExpr::Scalar(1.0),
            duration: 0.4,
            easing: Some(EasingSpec::Named("ease_out_cubic".into())),
            delay: None,
            time_scale: None,
            loop_mode: None,
            snap: None,
            round: None,
            reverse: false,
            span: Span::call_site(),
        };
        let tokens = expand_tween(&spec);
        let s = tokens.to_string();
        assert!(s.contains("Tween"));
        assert!(s.contains("EaseOutCubic"));
    }

    #[test]
    fn expand_spring_with_preset() {
        let spec = SpringSpec {
            target: "scale".into(),
            from: ValueExpr::Scalar(0.8),
            to: ValueExpr::Scalar(1.0),
            preset: Some(SpringPreset::Snappy),
            stiffness: None,
            damping: None,
            mass: None,
            velocity: None,
            epsilon: None,
            integrator: None,
            damping_mode: None,
            span: Span::call_site(),
        };
        let tokens = expand_spring(&spec);
        let s = tokens.to_string();
        assert!(s.contains("snappy"));
    }

    #[test]
    fn expand_sequence_node() {
        let nodes = vec![MotionNode::Tween(TweenSpec {
            target: "x".into(),
            from: ValueExpr::Scalar(0.0),
            to: ValueExpr::Scalar(1.0),
            duration: 0.3,
            easing: None,
            delay: None,
            time_scale: None,
            loop_mode: None,
            snap: None,
            round: None,
            reverse: false,
            span: Span::call_site(),
        })];
        let tokens = expand_sequence(&nodes);
        let s = tokens.to_string();
        assert!(s.contains("Sequence"));
    }
}
