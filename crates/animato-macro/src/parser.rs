//! Parser for the `animato!{}` Motion Macro DSL.
//!
//! Converts a `syn::ParseStream` into an AST [`MotionNode`](crate::ast::MotionNode)
//! tree.

use crate::ast::*;
use crate::error;
use proc_macro2::Span;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, LitFloat, LitInt, LitStr, Token};

/// The top-level input to `animato! { ... }`.
pub struct AnimatoInput {
    /// The parsed motion nodes.
    pub nodes: Vec<MotionNode>,
}

impl Parse for AnimatoInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut nodes = Vec::new();
        while !input.is_empty() {
            // Skip optional semicolons between top-level nodes.
            if input.peek(Token![;]) {
                input.parse::<Token![;]>()?;
                continue;
            }
            nodes.push(parse_node(input)?);
        }
        Ok(Self { nodes })
    }
}

/// Parse a single motion node.
pub fn parse_node(input: ParseStream) -> syn::Result<MotionNode> {
    let keyword: Ident = input.parse()?;
    let name = keyword.to_string();

    match name.as_str() {
        "tween" => parse_tween(input).map(MotionNode::Tween),
        "spring" => parse_spring(input).map(MotionNode::Spring),
        "keyframes" => parse_keyframes(input).map(MotionNode::Keyframes),
        "sequence" => parse_sequence(input),
        "parallel" => parse_parallel(input),
        "group" => parse_group(input),
        "stagger" => parse_stagger(input).map(MotionNode::Stagger),
        "path" => parse_path(input).map(MotionNode::Path),
        "morph" => parse_morph(input).map(MotionNode::Morph),
        "draw" => parse_draw(input).map(MotionNode::Draw),
        "color" => parse_color(input).map(MotionNode::Color),
        "waveform" => parse_waveform(input).map(MotionNode::Waveform),
        "preset" => parse_preset_call(input).map(MotionNode::Preset),
        "label" => parse_label(input).map(MotionNode::Label),
        "at" => parse_at(input).map(MotionNode::At),
        _ => Err(error::unknown_keyword(keyword.span(), &name)),
    }
}

// ── Tween ────────────────────────────────────────────────────────────────────

fn parse_tween(input: ParseStream) -> syn::Result<TweenSpec> {
    let span = Span::call_site();
    let target: Ident = input.parse()?;
    input.parse::<Token![:]>()?;

    let from = parse_value(input)?;
    input.parse::<Token![=>]>()?;
    let to = parse_value(input)?;

    // Comma-separated fields, terminated by `;` or end of block.
    let mut duration: Option<f32> = None;
    let mut easing: Option<EasingSpec> = None;
    let mut delay: Option<f32> = None;
    let mut time_scale: Option<f32> = None;
    let mut loop_mode: Option<LoopSpec> = None;
    let mut snap: Option<f32> = None;
    let mut round: Option<u32> = None;
    let mut reverse = false;

    parse_field_list(input, |field, stream| {
        match field {
            "duration" => {
                let v = parse_float(stream)?;
                duration = Some(v);
            }
            "easing" => {
                easing = Some(parse_easing(stream)?);
            }
            "delay" => {
                let v = parse_float(stream)?;
                delay = Some(v);
            }
            "time_scale" => {
                let v = parse_float(stream)?;
                time_scale = Some(v);
            }
            "loop" => {
                loop_mode = Some(parse_loop_mode(stream)?);
            }
            "snap" => {
                let v = parse_float(stream)?;
                snap = Some(v);
            }
            "round" => {
                let v = parse_uint(stream)?;
                round = Some(v);
            }
            "reverse" => {
                let v: syn::LitBool = stream.parse()?;
                if v.value {
                    reverse = true;
                }
            }
            _ => {
                return Err(error::unknown_keyword(
                    proc_macro2::Span::call_site(),
                    &field,
                ));
            }
        }
        Ok(())
    })?;

    let duration = duration.ok_or_else(|| error::missing_duration(span))?;

    Ok(TweenSpec {
        target: target.to_string(),
        from,
        to,
        duration,
        easing,
        delay,
        time_scale,
        loop_mode,
        snap,
        round,
        reverse,
        span,
    })
}

// ── Spring ───────────────────────────────────────────────────────────────────

fn parse_spring(input: ParseStream) -> syn::Result<SpringSpec> {
    let span = Span::call_site();
    let target: Ident = input.parse()?;
    input.parse::<Token![:]>()?;

    let from = parse_value(input)?;
    input.parse::<Token![=>]>()?;
    let to = parse_value(input)?;

    let mut preset: Option<SpringPreset> = None;
    let mut stiffness: Option<f32> = None;
    let mut damping: Option<f32> = None;
    let mut mass: Option<f32> = None;
    let mut velocity: Option<ValueExpr> = None;
    let mut epsilon: Option<f32> = None;
    let mut integrator: Option<IntegratorSpec> = None;
    let mut damping_mode: Option<DampingMode> = None;

    parse_field_list(input, |field, stream| {
        match field {
            "preset" => {
                let ident: Ident = stream.parse()?;
                preset = Some(parse_spring_preset(ident)?);
            }
            "stiffness" => stiffness = Some(parse_float(stream)?),
            "damping" => damping = Some(parse_float(stream)?),
            "mass" => mass = Some(parse_float(stream)?),
            "velocity" => velocity = Some(parse_value(stream)?),
            "epsilon" => epsilon = Some(parse_float(stream)?),
            "integrator" => {
                let ident: Ident = stream.parse()?;
                integrator = Some(match &ident.to_string()[..] {
                    "semi_implicit_euler" => IntegratorSpec::SemiImplicitEuler,
                    "rk4" => IntegratorSpec::RungeKutta4,
                    _ => {
                        return Err(error::malformed(
                            ident.span(),
                            "expected `semi_implicit_euler` or `rk4`",
                        ));
                    }
                });
            }
            "critically_damped" => {
                let s = parse_float_in_parens(stream)?;
                damping_mode = Some(DampingMode::CriticallyDamped(s));
            }
            "overdamped" => {
                let (s, r) = parse_two_floats_in_parens(stream)?;
                damping_mode = Some(DampingMode::Overdamped(s, r));
            }
            "underdamped" => {
                let (s, r) = parse_two_floats_in_parens(stream)?;
                damping_mode = Some(DampingMode::Underdamped(s, r));
            }
            _ => {
                return Err(error::unknown_keyword(
                    proc_macro2::Span::call_site(),
                    &field,
                ));
            }
        }
        Ok(())
    })?;

    Ok(SpringSpec {
        target: target.to_string(),
        from,
        to,
        preset,
        stiffness,
        damping,
        mass,
        velocity,
        epsilon,
        integrator,
        damping_mode,
        span,
    })
}

fn parse_spring_preset(ident: Ident) -> syn::Result<SpringPreset> {
    match &ident.to_string()[..] {
        "gentle" => Ok(SpringPreset::Gentle),
        "wobbly" => Ok(SpringPreset::Wobbly),
        "stiff" => Ok(SpringPreset::Stiff),
        "slow" => Ok(SpringPreset::Slow),
        "snappy" => Ok(SpringPreset::Snappy),
        _ => Err(error::unknown_spring_preset(
            ident.span(),
            &ident.to_string(),
        )),
    }
}

// ── Keyframes ────────────────────────────────────────────────────────────────

fn parse_keyframes(input: ParseStream) -> syn::Result<KeyframeSpec> {
    let span = Span::call_site();
    let target: Ident = input.parse()?;
    let content;
    syn::braced!(content in input);

    let mut frames = Vec::new();
    let mut loop_mode: Option<LoopSpec> = None;

    while !content.is_empty() {
        // Allow `loop: ...` at the start of the block.
        if content.peek(Token![loop]) {
            content.parse::<Token![loop]>()?;
            content.parse::<Token![:]>()?;
            loop_mode = Some(parse_loop_mode(&content)?);
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
            continue;
        }

        let time = parse_keyframe_time(&content)?;
        content.parse::<Token![:]>()?;
        let value = parse_value(&content)?;
        let easing = if content.peek(Ident) && !content.peek(Token![,]) && !content.peek(Token![;])
        {
            // Peek to see if it's an easing name (not a comma/brace).
            let ahead = content.lookahead1();
            if ahead.peek(Ident) {
                Some(parse_easing(&content)?)
            } else {
                None
            }
        } else {
            None
        };

        frames.push(KeyframeEntry {
            time,
            value,
            easing,
        });

        if content.peek(Token![,]) {
            content.parse::<Token![,]>()?;
        } else if content.peek(Token![;]) {
            content.parse::<Token![;]>()?;
        } else {
            break;
        }
    }

    Ok(KeyframeSpec {
        target: target.to_string(),
        frames,
        loop_mode,
        span,
    })
}

fn parse_keyframe_time(input: ParseStream) -> syn::Result<KeyframeTime> {
    // Try percentage: `50%`
    if input.peek(LitInt) {
        let lit: LitInt = input.parse()?;
        let repr = lit.to_string();
        if input.peek(Token![%]) {
            input.parse::<Token![%]>()?;
            let pct: f32 = repr
                .parse()
                .map_err(|_| error::malformed(lit.span(), "invalid percentage"))?;
            return Ok(KeyframeTime::Percent(pct / 100.0));
        }
        // Otherwise it might be `0.25s` — check for `s` suffix
        if repr.ends_with('s') {
            let num: f32 = repr[..repr.len() - 1]
                .parse()
                .map_err(|_| error::malformed(lit.span(), "invalid seconds"))?;
            return Ok(KeyframeTime::Seconds(num));
        }
        // Bare integer — treat as seconds
        let num: f32 = repr
            .parse()
            .map_err(|_| error::malformed(lit.span(), "invalid number"))?;
        return Ok(KeyframeTime::Seconds(num));
    }
    if input.peek(LitFloat) {
        let lit: LitFloat = input.parse()?;
        let repr = lit.to_string();
        if repr.ends_with('s') {
            let num: f32 = repr[..repr.len() - 1]
                .parse()
                .map_err(|_| error::malformed(lit.span(), "invalid seconds"))?;
            return Ok(KeyframeTime::Seconds(num));
        }
        let num: f32 = repr
            .parse()
            .map_err(|_| error::malformed(lit.span(), "invalid number"))?;
        return Ok(KeyframeTime::Seconds(num));
    }
    Err(error::malformed(
        Span::call_site(),
        "expected keyframe time (e.g. `50%` or `0.25s`)",
    ))
}

// ── Sequence / Parallel / Group ─────────────────────────────────────────────

fn parse_sequence(input: ParseStream) -> syn::Result<MotionNode> {
    let content;
    syn::braced!(content in input);
    let mut nodes = Vec::new();
    while !content.is_empty() {
        if content.peek(Token![;]) {
            content.parse::<Token![;]>()?;
            continue;
        }
        nodes.push(parse_node(&content)?);
    }
    Ok(MotionNode::Sequence(nodes))
}

fn parse_parallel(input: ParseStream) -> syn::Result<MotionNode> {
    let content;
    syn::braced!(content in input);
    let mut nodes = Vec::new();
    while !content.is_empty() {
        if content.peek(Token![;]) {
            content.parse::<Token![;]>()?;
            continue;
        }
        nodes.push(parse_node(&content)?);
    }
    Ok(MotionNode::Parallel(nodes))
}

fn parse_group(input: ParseStream) -> syn::Result<MotionNode> {
    let mode_ident: Ident = input.parse()?;
    let mode = match &mode_ident.to_string()[..] {
        "sequence" => GroupMode::Sequence,
        "parallel" => GroupMode::Parallel,
        _ => {
            return Err(error::malformed(
                mode_ident.span(),
                "expected `sequence` or `parallel` after `group`",
            ));
        }
    };
    let content;
    syn::braced!(content in input);
    let mut nodes = Vec::new();
    while !content.is_empty() {
        if content.peek(Token![;]) {
            content.parse::<Token![;]>()?;
            continue;
        }
        nodes.push(parse_node(&content)?);
    }
    Ok(MotionNode::Group { mode, nodes })
}

// ── Stagger ──────────────────────────────────────────────────────────────────

fn parse_stagger(input: ParseStream) -> syn::Result<StaggerSpec> {
    let span = Span::call_site();
    let mut pattern: Option<StaggerPatternSpec> = None;
    let mut delay: Option<f32> = None;

    // Parse optional `pattern: ...` and/or `delay: ...` before the block.
    loop {
        if input.peek(syn::token::Brace) {
            break;
        }
        let field: Ident = input.parse()?;
        match &field.to_string()[..] {
            "pattern" => {
                input.parse::<Token![:]>()?;
                pattern = Some(parse_stagger_pattern(input)?);
            }
            "delay" => {
                input.parse::<Token![:]>()?;
                delay = Some(parse_float(input)?);
            }
            _ => {
                return Err(error::unknown_keyword(field.span(), &field.to_string()));
            }
        }
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        } else {
            break;
        }
    }

    let content;
    syn::braced!(content in input);
    let mut nodes = Vec::new();
    while !content.is_empty() {
        if content.peek(Token![;]) {
            content.parse::<Token![;]>()?;
            continue;
        }
        nodes.push(parse_node(&content)?);
    }

    Ok(StaggerSpec {
        pattern,
        delay,
        nodes,
        span,
    })
}

fn parse_stagger_pattern(input: ParseStream) -> syn::Result<StaggerPatternSpec> {
    let name: Ident = input.parse()?;
    match &name.to_string()[..] {
        "grid" => {
            let content;
            syn::parenthesized!(content in input);
            let mut cols: Option<u32> = None;
            let mut rows: Option<u32> = None;
            let mut origin = GridOriginSpec::Center;
            while !content.is_empty() {
                let field: Ident = content.parse()?;
                content.parse::<Token![:]>()?;
                match &field.to_string()[..] {
                    "cols" => cols = Some(parse_uint(&content)?),
                    "rows" => rows = Some(parse_uint(&content)?),
                    "origin" => origin = parse_grid_origin(&content)?,
                    _ => {
                        return Err(error::unknown_keyword(field.span(), &field.to_string()));
                    }
                }
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                } else {
                    break;
                }
            }
            let cols = cols.ok_or_else(|| error::stagger_grid_incomplete(name.span()))?;
            let rows = rows.ok_or_else(|| error::stagger_grid_incomplete(name.span()))?;
            Ok(StaggerPatternSpec::Grid { cols, rows, origin })
        }
        "random" => {
            let content;
            syn::parenthesized!(content in input);
            let mut seed: Option<u32> = None;
            let mut min: Option<f32> = None;
            let mut max: Option<f32> = None;
            while !content.is_empty() {
                let field: Ident = content.parse()?;
                content.parse::<Token![:]>()?;
                match &field.to_string()[..] {
                    "seed" => seed = Some(parse_uint(&content)?),
                    "min" => min = Some(parse_float(&content)?),
                    "max" => max = Some(parse_float(&content)?),
                    _ => {
                        return Err(error::unknown_keyword(field.span(), &field.to_string()));
                    }
                }
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                } else {
                    break;
                }
            }
            Ok(StaggerPatternSpec::Random {
                seed: seed.unwrap_or(0),
                min: min.unwrap_or(0.0),
                max: max.unwrap_or(0.1),
            })
        }
        "center_out" => Ok(StaggerPatternSpec::CenterOut),
        "edges_in" => Ok(StaggerPatternSpec::EdgesIn),
        _ => Err(error::malformed(
            name.span(),
            "expected `grid`, `random`, `center_out`, or `edges_in`",
        )),
    }
}

fn parse_grid_origin(input: ParseStream) -> syn::Result<GridOriginSpec> {
    let ident: Ident = input.parse()?;
    match &ident.to_string()[..] {
        "top_left" => Ok(GridOriginSpec::TopLeft),
        "top_right" => Ok(GridOriginSpec::TopRight),
        "bottom_left" => Ok(GridOriginSpec::BottomLeft),
        "bottom_right" => Ok(GridOriginSpec::BottomRight),
        "center" => Ok(GridOriginSpec::Center),
        "top" => Ok(GridOriginSpec::Top),
        "bottom" => Ok(GridOriginSpec::Bottom),
        "left" => Ok(GridOriginSpec::Left),
        "right" => Ok(GridOriginSpec::Right),
        _ => Err(error::malformed(
            ident.span(),
            "expected a grid origin: top_left, top_right, bottom_left, bottom_right, center, top, bottom, left, right",
        )),
    }
}

// ── Path / Morph / Draw ──────────────────────────────────────────────────────

fn parse_path(input: ParseStream) -> syn::Result<PathSpec> {
    let span = Span::call_site();
    let target: Ident = input.parse()?;
    // `along "M..."`
    let along: Ident = input.parse()?;
    if along != "along" {
        return Err(error::malformed(
            along.span(),
            "expected `along` after path target",
        ));
    }
    let svg: LitStr = input.parse()?;

    let mut duration: Option<f32> = None;
    let mut easing: Option<EasingSpec> = None;
    let mut auto_rotate: Option<bool> = None;
    let mut offset: Option<(f32, f32)> = None;
    let mut loop_mode: Option<LoopSpec> = None;

    parse_field_list(input, |field, stream| {
        match field {
            "duration" => duration = Some(parse_float(stream)?),
            "easing" => easing = Some(parse_easing(stream)?),
            "auto_rotate" => {
                let v: syn::LitBool = stream.parse()?;
                auto_rotate = Some(v.value);
            }
            "offset" => {
                let start = parse_float(stream)?;
                stream.parse::<Token![..]>()?;
                let end = parse_float(stream)?;
                offset = Some((start, end));
            }
            "loop" => loop_mode = Some(parse_loop_mode(stream)?),
            _ => {
                return Err(error::unknown_keyword(
                    proc_macro2::Span::call_site(),
                    &field,
                ));
            }
        }
        Ok(())
    })?;

    let duration = duration.ok_or_else(|| error::missing_duration(span))?;

    Ok(PathSpec {
        target: target.to_string(),
        svg: svg.value(),
        duration,
        easing,
        auto_rotate,
        offset,
        loop_mode,
        span,
    })
}

fn parse_morph(input: ParseStream) -> syn::Result<MorphSpec> {
    let span = Span::call_site();
    let from: LitStr = input.parse()?;
    input.parse::<Token![=>]>()?;
    let to: LitStr = input.parse()?;

    let mut samples: Option<u32> = None;
    let mut duration: Option<f32> = None;
    let mut easing: Option<EasingSpec> = None;

    parse_field_list(input, |field, stream| {
        match field {
            "samples" => samples = Some(parse_uint(stream)?),
            "duration" => duration = Some(parse_float(stream)?),
            "easing" => easing = Some(parse_easing(stream)?),
            _ => {
                return Err(error::unknown_keyword(
                    proc_macro2::Span::call_site(),
                    &field,
                ));
            }
        }
        Ok(())
    })?;

    let duration = duration.ok_or_else(|| error::missing_duration(span))?;
    let samples = samples.unwrap_or(32);

    Ok(MorphSpec {
        from: from.value(),
        to: to.value(),
        samples,
        duration,
        easing,
        span,
    })
}

fn parse_draw(input: ParseStream) -> syn::Result<DrawSpec> {
    let span = Span::call_site();
    let svg: LitStr = input.parse()?;

    let mut duration: Option<f32> = None;
    let mut easing: Option<EasingSpec> = None;

    parse_field_list(input, |field, stream| {
        match field {
            "duration" => duration = Some(parse_float(stream)?),
            "easing" => easing = Some(parse_easing(stream)?),
            _ => {
                return Err(error::unknown_keyword(
                    proc_macro2::Span::call_site(),
                    &field,
                ));
            }
        }
        Ok(())
    })?;

    let duration = duration.ok_or_else(|| error::missing_duration(span))?;

    Ok(DrawSpec {
        svg: svg.value(),
        duration,
        easing,
        span,
    })
}

// ── Color ────────────────────────────────────────────────────────────────────

fn parse_color(input: ParseStream) -> syn::Result<ColorSpec> {
    let span = Span::call_site();
    let target: Ident = input.parse()?;
    input.parse::<Token![:]>()?;
    let from: LitStr = input.parse()?;
    input.parse::<Token![=>]>()?;
    let to: LitStr = input.parse()?;

    let mut duration: Option<f32> = None;
    let mut space = ColorSpace::Linear;
    let mut easing: Option<EasingSpec> = None;

    parse_field_list(input, |field, stream| {
        match field {
            "duration" => duration = Some(parse_float(stream)?),
            "space" => {
                let ident: Ident = stream.parse()?;
                space = match &ident.to_string()[..] {
                    "linear" => ColorSpace::Linear,
                    "lab" => ColorSpace::Lab,
                    "oklch" => ColorSpace::Oklch,
                    _ => {
                        return Err(error::malformed(
                            ident.span(),
                            "expected `linear`, `lab`, or `oklch`",
                        ));
                    }
                };
            }
            "easing" => easing = Some(parse_easing(stream)?),
            _ => {
                return Err(error::unknown_keyword(
                    proc_macro2::Span::call_site(),
                    &field,
                ));
            }
        }
        Ok(())
    })?;

    let duration = duration.ok_or_else(|| error::missing_duration(span))?;

    Ok(ColorSpec {
        target: target.to_string(),
        from: from.value(),
        to: to.value(),
        duration,
        space,
        easing,
        span,
    })
}

// ── Waveform ─────────────────────────────────────────────────────────────────

fn parse_waveform(input: ParseStream) -> syn::Result<WaveformSpec> {
    let span = Span::call_site();
    let kind_ident: Ident = input.parse()?;
    let kind = match &kind_ident.to_string()[..] {
        "sine" => WaveformKind::Sine,
        "sawtooth" => WaveformKind::Sawtooth,
        "square" => WaveformKind::Square,
        "triangle" => WaveformKind::Triangle,
        "noise" => WaveformKind::Noise,
        _ => {
            return Err(error::malformed(
                kind_ident.span(),
                "expected `sine`, `sawtooth`, `square`, `triangle`, or `noise`",
            ));
        }
    };

    let mut frequency: Option<f32> = None;
    let mut amplitude: Option<f32> = None;
    let mut phase: Option<f32> = None;
    let mut duty_cycle: Option<f32> = None;
    let mut seed: Option<u32> = None;
    let mut smoothness: Option<f32> = None;
    let mut duration: Option<f32> = None;

    parse_field_list(input, |field, stream| {
        match field {
            "frequency" => frequency = Some(parse_float(stream)?),
            "amplitude" => amplitude = Some(parse_float(stream)?),
            "phase" => phase = Some(parse_float(stream)?),
            "duty_cycle" => duty_cycle = Some(parse_float(stream)?),
            "seed" => seed = Some(parse_uint(stream)?),
            "smoothness" => smoothness = Some(parse_float(stream)?),
            "duration" => duration = Some(parse_float(stream)?),
            _ => {
                return Err(error::unknown_keyword(
                    proc_macro2::Span::call_site(),
                    &field,
                ));
            }
        }
        Ok(())
    })?;

    Ok(WaveformSpec {
        kind,
        frequency,
        amplitude,
        phase,
        duty_cycle,
        seed,
        smoothness,
        duration,
        span,
    })
}

// ── Preset call ──────────────────────────────────────────────────────────────

fn parse_preset_call(input: ParseStream) -> syn::Result<PresetCall> {
    let span = Span::call_site();
    let name: Ident = input.parse()?;
    let mut duration: Option<f32> = None;
    let mut easing: Option<EasingSpec> = None;

    // Optional `(duration: 0.5, easing: ease_out_cubic)`
    if input.peek(syn::token::Paren) {
        let content;
        syn::parenthesized!(content in input);
        while !content.is_empty() {
            let field: Ident = content.parse()?;
            content.parse::<Token![:]>()?;
            match &field.to_string()[..] {
                "duration" => duration = Some(parse_float(&content)?),
                "easing" => easing = Some(parse_easing(&content)?),
                _ => {
                    return Err(error::unknown_keyword(field.span(), &field.to_string()));
                }
            }
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            } else {
                break;
            }
        }
    }

    // Optional trailing fields without parens: `, duration: 0.5`
    if input.peek(Token![,]) {
        input.parse::<Token![,]>()?;
        parse_field_list(input, |field, stream| {
            match field {
                "duration" => duration = Some(parse_float(stream)?),
                "easing" => easing = Some(parse_easing(stream)?),
                _ => {
                    return Err(error::unknown_keyword(
                        proc_macro2::Span::call_site(),
                        &field,
                    ));
                }
            }
            Ok(())
        })?;
    }

    Ok(PresetCall {
        name: name.to_string(),
        duration,
        easing,
        span,
    })
}

// ── Label / At ───────────────────────────────────────────────────────────────

fn parse_label(input: ParseStream) -> syn::Result<String> {
    let s: LitStr = input.parse()?;
    input.parse::<Token![;]>()?;
    Ok(s.value())
}

fn parse_at(input: ParseStream) -> syn::Result<AtSpec> {
    let relative;
    let value;

    if input.peek(Token![+]) {
        input.parse::<Token![+]>()?;
        relative = true;
        value = parse_float(input)?;
    } else if input.peek(Token![-]) {
        input.parse::<Token![-]>()?;
        relative = true;
        value = -parse_float(input)?;
    } else {
        relative = false;
        value = parse_float(input)?;
    }

    if input.peek(Token![;]) {
        input.parse::<Token![;]>()?;
    }

    Ok(AtSpec { value, relative })
}

// ── Value / Easing / Loop primitives ────────────────────────────────────────

fn parse_value(input: ParseStream) -> syn::Result<ValueExpr> {
    if input.peek(syn::token::Bracket) {
        let content;
        syn::bracketed!(content in input);
        let mut values = Vec::new();
        while !content.is_empty() {
            values.push(parse_float(&content)?);
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            } else {
                break;
            }
        }
        Ok(ValueExpr::Array(values))
    } else {
        let f = parse_float(input)?;
        Ok(ValueExpr::Scalar(f))
    }
}

fn parse_easing(input: ParseStream) -> syn::Result<EasingSpec> {
    if input.peek(syn::token::Paren) {
        // Function-style: cubic_bezier(...), steps(...), etc.
        // Already consumed by the field parser — but here we handle the
        // case where easing is `Easing::Foo` or a bare name.
        return Err(error::malformed(
            Span::call_site(),
            "unexpected `(` in easing position",
        ));
    }

    // Peek: is it `cubic_bezier(...)`, `steps(...)`, etc.?
    let ident: Ident = input.parse()?;
    let name = ident.to_string();

    match name.as_str() {
        "cubic_bezier" => {
            let content;
            syn::parenthesized!(content in input);
            let x1 = parse_float(&content)?;
            content.parse::<Token![,]>()?;
            let y1 = parse_float(&content)?;
            content.parse::<Token![,]>()?;
            let x2 = parse_float(&content)?;
            content.parse::<Token![,]>()?;
            let y2 = parse_float(&content)?;
            Ok(EasingSpec::CubicBezier(x1, y1, x2, y2))
        }
        "steps" => {
            let content;
            syn::parenthesized!(content in input);
            let n = parse_uint(&content)?;
            Ok(EasingSpec::Steps(n))
        }
        "wiggle" => {
            let content;
            syn::parenthesized!(content in input);
            let n = parse_uint(&content)?;
            Ok(EasingSpec::Wiggle(n))
        }
        "rough" => {
            let content;
            syn::parenthesized!(content in input);
            let mut strength = 0.5;
            let mut points = 8;
            while !content.is_empty() {
                let field: Ident = content.parse()?;
                content.parse::<Token![:]>()?;
                match &field.to_string()[..] {
                    "strength" => strength = parse_float(&content)?,
                    "points" => points = parse_uint(&content)?,
                    _ => {
                        return Err(error::unknown_keyword(field.span(), &field.to_string()));
                    }
                }
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                } else {
                    break;
                }
            }
            Ok(EasingSpec::Rough { strength, points })
        }
        "slowmo" => {
            let content;
            syn::parenthesized!(content in input);
            let mut linear_ratio = 0.7;
            let mut power = 2.0;
            while !content.is_empty() {
                let field: Ident = content.parse()?;
                content.parse::<Token![:]>()?;
                match &field.to_string()[..] {
                    "linear_ratio" => linear_ratio = parse_float(&content)?,
                    "power" => power = parse_float(&content)?,
                    _ => {
                        return Err(error::unknown_keyword(field.span(), &field.to_string()));
                    }
                }
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                } else {
                    break;
                }
            }
            Ok(EasingSpec::SlowMo {
                linear_ratio,
                power,
            })
        }
        _ => {
            // Bare named easing, or `Easing::Variant` path.
            if name == "Easing" && input.peek(Token![::]) {
                input.parse::<Token![::]>()?;
                let variant: Ident = input.parse()?;
                let snake = to_snake_case(&variant.to_string());
                Ok(EasingSpec::Named(snake))
            } else {
                Ok(EasingSpec::Named(name))
            }
        }
    }
}

fn parse_loop_mode(input: ParseStream) -> syn::Result<LoopSpec> {
    let ident: Ident = input.parse()?;
    match &ident.to_string()[..] {
        "once" => Ok(LoopSpec::Once),
        "forever" => Ok(LoopSpec::Forever),
        "ping_pong" => {
            if input.peek(syn::token::Paren) {
                let content;
                syn::parenthesized!(content in input);
                let n = parse_uint(&content)?;
                Ok(LoopSpec::PingPongTimes(n))
            } else {
                Ok(LoopSpec::PingPong)
            }
        }
        "times" => {
            let content;
            syn::parenthesized!(content in input);
            let n = parse_uint(&content)?;
            Ok(LoopSpec::Times(n))
        }
        _ => Err(error::malformed(
            ident.span(),
            "expected `once`, `times(n)`, `forever`, `ping_pong`, or `ping_pong_times(n)`",
        )),
    }
}

// ── Field-list helper ───────────────────────────────────────────────────────

/// Parse a comma-separated list of `field: value` pairs, terminated by `;` or
/// the end of the current block/stream.
fn parse_field_list(
    input: ParseStream,
    mut handler: impl FnMut(&str, ParseStream) -> syn::Result<()>,
) -> syn::Result<()> {
    while !input.is_empty() && !input.peek(syn::token::Brace) {
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            continue;
        }
        if input.peek(Token![;]) {
            input.parse::<Token![;]>()?;
            break;
        }
        // Parse field name — handle keywords like `loop` by using Token![loop].
        let field = if input.peek(Token![loop]) {
            input.parse::<Token![loop]>()?;
            "loop".to_string()
        } else {
            let ident: Ident = input.parse()?;
            ident.to_string()
        };
        input.parse::<Token![:]>()?;
        handler(&field, input)?;
        // After each field, expect `,` or `;` or end.
        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        } else if input.peek(Token![;]) {
            input.parse::<Token![;]>()?;
            break;
        } else if input.is_empty() || input.peek(syn::token::Brace) {
            break;
        }
    }
    Ok(())
}

// ── Numeric helpers ─────────────────────────────────────────────────────────

fn parse_float(input: ParseStream) -> syn::Result<f32> {
    if input.peek(LitFloat) {
        let lit: LitFloat = input.parse()?;
        lit.base10_parse()
            .map_err(|e| error::malformed(lit.span(), &format!("invalid float: {e}")))
    } else if input.peek(LitInt) {
        let lit: LitInt = input.parse()?;
        lit.base10_parse()
            .map_err(|e| error::malformed(lit.span(), &format!("invalid float: {e}")))
    } else {
        Err(error::malformed(Span::call_site(), "expected a number"))
    }
}

fn parse_uint(input: ParseStream) -> syn::Result<u32> {
    let lit: LitInt = input.parse()?;
    lit.base10_parse()
        .map_err(|e| error::malformed(lit.span(), &format!("invalid integer: {e}")))
}

fn parse_float_in_parens(input: ParseStream) -> syn::Result<f32> {
    let content;
    syn::parenthesized!(content in input);
    parse_float(&content)
}

fn parse_two_floats_in_parens(input: ParseStream) -> syn::Result<(f32, f32)> {
    let content;
    syn::parenthesized!(content in input);
    let a = parse_float(&content)?;
    content.parse::<Token![,]>()?;
    let b = parse_float(&content)?;
    Ok((a, b))
}

fn to_snake_case(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 4);
    for (i, ch) in s.chars().enumerate() {
        if ch.is_ascii_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(ch.to_ascii_lowercase());
        } else {
            result.push(ch);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::TokenStream;
    use syn::parse_str;

    fn parse_input(src: &str) -> syn::Result<AnimatoInput> {
        let ts: TokenStream = src.parse().unwrap();
        syn::parse2(ts)
    }

    #[test]
    fn parse_simple_tween() {
        let input =
            parse_input("tween opacity: 0.0 => 1.0, duration: 0.4, easing: ease_out_cubic;");
        assert!(input.is_ok(), "parse failed: {:?}", input.err());
        let input = input.unwrap();
        assert_eq!(input.nodes.len(), 1);
        match &input.nodes[0] {
            MotionNode::Tween(t) => {
                assert_eq!(t.target, "opacity");
                assert_eq!(t.duration, 0.4);
            }
            _ => panic!("expected Tween"),
        }
    }

    #[test]
    fn parse_vector_tween() {
        let input = parse_input("tween pos: [0.0, 20.0] => [10.0, 30.0], duration: 0.6;");
        assert!(input.is_ok());
    }

    #[test]
    fn parse_spring_with_preset() {
        let input = parse_input("spring scale: 0.8 => 1.0, preset: snappy;");
        assert!(input.is_ok());
    }

    #[test]
    fn parse_keyframes() {
        let input =
            parse_input("keyframes opacity { 0%: 0.0, 50%: 0.7 ease_out_cubic, 100%: 1.0 }");
        assert!(input.is_ok(), "parse failed: {:?}", input.err());
    }

    #[test]
    fn parse_sequence_block() {
        let input = parse_input(
            "sequence { tween x: 0.0 => 1.0, duration: 0.3; spring y: 0.0 => 1.0, preset: gentle; }",
        );
        assert!(input.is_ok(), "parse failed: {:?}", input.err());
    }

    #[test]
    fn parse_stagger_grid() {
        let input = parse_input(
            "stagger pattern: grid(cols: 3, rows: 2, origin: center), delay: 0.06 { tween x: 0.0 => 1.0, duration: 0.25; }",
        );
        assert!(input.is_ok(), "parse failed: {:?}", input.err());
    }

    #[test]
    fn parse_cubic_bezier_easing() {
        let input = parse_input(
            "tween x: 0.0 => 1.0, duration: 0.3, easing: cubic_bezier(0.22, 1.0, 0.36, 1.0);",
        );
        assert!(input.is_ok());
    }
}
