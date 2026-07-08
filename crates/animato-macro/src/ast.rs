//! Internal AST for the `animato!{}` Motion Macro DSL.
//!
//! The parser converts `syn` tokens into these types. The expander then walks
//! the AST and emits stable Animato builder code via `quote!`.

use proc_macro2::Span;

/// One top-level node inside an `animato! { ... }` block.
#[derive(Debug, Clone)]
pub enum MotionNode {
    /// `tween opacity: 0.0 => 1.0, duration: 0.4, easing: ease_out_cubic;`
    Tween(TweenSpec),
    /// `spring scale: 0.8 => 1.0, preset: snappy;`
    Spring(SpringSpec),
    /// `keyframes opacity { 0%: 0.0, 50%: 0.7 ease_out_cubic, 100%: 1.0 }`
    Keyframes(KeyframeSpec),
    /// `sequence { ... }`
    Sequence(Vec<MotionNode>),
    /// `parallel { ... }`
    Parallel(Vec<MotionNode>),
    /// `group sequence { ... }` or `group parallel { ... }`
    Group {
        /// Whether the group plays in sequence or parallel.
        mode: GroupMode,
        /// Child nodes.
        nodes: Vec<MotionNode>,
    },
    /// `stagger delay: 0.05 { ... }` or `stagger pattern: grid(...), delay: 0.06 { ... }`
    Stagger(StaggerSpec),
    /// `path x along "M0 0 L100 100", duration: 1.0`
    Path(PathSpec),
    /// `morph "M0 0" => "M100 100", samples: 64, duration: 0.8`
    Morph(MorphSpec),
    /// `draw "M0 0 L100 100", duration: 1.0`
    Draw(DrawSpec),
    /// `color bg: "#ff0000" => "#0000ff", space: oklch, duration: 0.5`
    Color(ColorSpec),
    /// `waveform sine frequency: 2.0, amplitude: 1.0`
    Waveform(WaveformSpec),
    /// `preset fade_in` or `preset modal_enter(duration: 0.5)`
    Preset(PresetCall),
    /// `label "intro";`
    Label(String),
    /// `at "+0.2"` or `at 1.25`
    At(AtSpec),
}

/// Group playback mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupMode {
    /// One after another.
    Sequence,
    /// All at once.
    Parallel,
}

/// A tween specification parsed from the DSL.
#[derive(Debug, Clone)]
pub struct TweenSpec {
    /// The property name being animated (e.g. `opacity`, `x`, `scale`).
    pub target: String,
    /// Starting value.
    pub from: ValueExpr,
    /// Ending value.
    pub to: ValueExpr,
    /// Duration in seconds.
    pub duration: f32,
    /// Optional easing curve.
    pub easing: Option<EasingSpec>,
    /// Optional pre-animation delay in seconds.
    pub delay: Option<f32>,
    /// Optional time-scale multiplier.
    pub time_scale: Option<f32>,
    /// Optional looping mode.
    pub loop_mode: Option<LoopSpec>,
    /// Optional snap-to grid value.
    pub snap: Option<f32>,
    /// Optional decimal rounding count.
    pub round: Option<u32>,
    /// Whether the tween should play in reverse.
    pub reverse: bool,
    /// Source span for diagnostics.
    pub span: Span,
}

/// A spring specification parsed from the DSL.
#[derive(Debug, Clone)]
pub struct SpringSpec {
    /// The property name being animated.
    pub target: String,
    /// Starting value.
    pub from: ValueExpr,
    /// Ending value (target).
    pub to: ValueExpr,
    /// Named preset, if any.
    pub preset: Option<SpringPreset>,
    /// Explicit stiffness (overrides preset).
    pub stiffness: Option<f32>,
    /// Explicit damping (overrides preset).
    pub damping: Option<f32>,
    /// Explicit mass (overrides preset).
    pub mass: Option<f32>,
    /// Initial velocity for fling-style springs.
    pub velocity: Option<ValueExpr>,
    /// Settle epsilon.
    pub epsilon: Option<f32>,
    /// Integration method.
    pub integrator: Option<IntegratorSpec>,
    /// Damping-mode helper, if specified.
    pub damping_mode: Option<DampingMode>,
    /// Source span for diagnostics.
    pub span: Span,
}

/// A keyframe track specification.
#[derive(Debug, Clone)]
pub struct KeyframeSpec {
    /// The property name being animated.
    pub target: String,
    /// The keyframe entries, in source order.
    pub frames: Vec<KeyframeEntry>,
    /// Optional looping mode.
    pub loop_mode: Option<LoopSpec>,
    /// Source span for diagnostics.
    pub span: Span,
}

/// One keyframe entry inside `keyframes { ... }`.
#[derive(Debug, Clone)]
pub struct KeyframeEntry {
    /// Time specification (percentage or seconds).
    pub time: KeyframeTime,
    /// Value at this keyframe.
    pub value: ValueExpr,
    /// Optional per-segment easing (applies from this frame to the next).
    pub easing: Option<EasingSpec>,
}

/// Keyframe time specification.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyframeTime {
    /// `50%` → normalized `0.5`.
    Percent(f32),
    /// `0.25s` → absolute seconds.
    Seconds(f32),
}

/// A stagger specification.
#[derive(Debug, Clone)]
pub struct StaggerSpec {
    /// The stagger pattern, if specified.
    pub pattern: Option<StaggerPatternSpec>,
    /// Linear delay between items in seconds.
    pub delay: Option<f32>,
    /// Child nodes to stagger.
    pub nodes: Vec<MotionNode>,
    /// Source span for diagnostics.
    pub span: Span,
}

/// Stagger pattern specification.
#[derive(Debug, Clone)]
pub enum StaggerPatternSpec {
    /// `grid(cols: 4, rows: 3, origin: center)`
    Grid {
        /// Number of columns.
        cols: u32,
        /// Number of rows.
        rows: u32,
        /// Origin cell.
        origin: GridOriginSpec,
    },
    /// `random(seed: 42, min: 0.02, max: 0.12)`
    Random {
        /// Deterministic seed.
        seed: u32,
        /// Minimum delay in seconds.
        min: f32,
        /// Maximum delay in seconds.
        max: f32,
    },
    /// `center_out`
    CenterOut,
    /// `edges_in`
    EdgesIn,
}

/// Grid origin for stagger patterns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridOriginSpec {
    /// Top-left cell starts first.
    TopLeft,
    /// Top-right cell starts first.
    TopRight,
    /// Bottom-left cell starts first.
    BottomLeft,
    /// Bottom-right cell starts first.
    BottomRight,
    /// Center cell(s) start first.
    Center,
    /// Top edge starts first.
    Top,
    /// Bottom edge starts first.
    Bottom,
    /// Left edge starts first.
    Left,
    /// Right edge starts first.
    Right,
}

/// A motion-path specification.
#[derive(Debug, Clone)]
pub struct PathSpec {
    /// The property name being animated.
    pub target: String,
    /// SVG path string (`M0 0 L100 100`).
    pub svg: String,
    /// Duration in seconds.
    pub duration: f32,
    /// Optional easing.
    pub easing: Option<EasingSpec>,
    /// Whether to auto-rotate to the path tangent.
    pub auto_rotate: Option<bool>,
    /// Optional normalized `[start, end]` offset along the path.
    pub offset: Option<(f32, f32)>,
    /// Optional loop mode.
    pub loop_mode: Option<LoopSpec>,
    /// Source span for diagnostics.
    pub span: Span,
}

/// A shape-morph specification.
#[derive(Debug, Clone)]
pub struct MorphSpec {
    /// Source SVG path.
    pub from: String,
    /// Destination SVG path.
    pub to: String,
    /// Number of resample points.
    pub samples: u32,
    /// Duration in seconds.
    pub duration: f32,
    /// Optional easing.
    pub easing: Option<EasingSpec>,
    /// Source span for diagnostics.
    pub span: Span,
}

/// An SVG draw specification.
#[derive(Debug, Clone)]
pub struct DrawSpec {
    /// SVG path string.
    pub svg: String,
    /// Duration in seconds.
    pub duration: f32,
    /// Optional easing.
    pub easing: Option<EasingSpec>,
    /// Source span for diagnostics.
    pub span: Span,
}

/// A color tween specification.
#[derive(Debug, Clone)]
pub struct ColorSpec {
    /// The property name being animated.
    pub target: String,
    /// Starting color (hex string or named color).
    pub from: String,
    /// Ending color.
    pub to: String,
    /// Duration in seconds.
    pub duration: f32,
    /// Interpolation space.
    pub space: ColorSpace,
    /// Optional easing.
    pub easing: Option<EasingSpec>,
    /// Source span for diagnostics.
    pub span: Span,
}

/// Color interpolation space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorSpace {
    /// Linear RGB (gamma-correct sRGB lerp).
    #[default]
    Linear,
    /// CIE L*a*b* (perceptually uniform).
    Lab,
    /// Oklch (modern perceptual).
    Oklch,
}

/// A waveform specification.
#[derive(Debug, Clone)]
pub struct WaveformSpec {
    /// Waveform kind.
    pub kind: WaveformKind,
    /// Cycles per second.
    pub frequency: Option<f32>,
    /// Peak absolute value.
    pub amplitude: Option<f32>,
    /// Phase offset in radians (sine only).
    pub phase: Option<f32>,
    /// Duty cycle (square only).
    pub duty_cycle: Option<f32>,
    /// Deterministic seed (noise only).
    pub seed: Option<u32>,
    /// Smoothness factor (noise only).
    pub smoothness: Option<f32>,
    /// Total duration of the generated track in seconds.
    pub duration: Option<f32>,
    /// Source span for diagnostics.
    pub span: Span,
}

/// Waveform kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaveformKind {
    /// Continuous sine wave.
    Sine,
    /// Linear ramp sawtooth.
    Sawtooth,
    /// Two-state square wave.
    Square,
    /// Triangle wave.
    Triangle,
    /// Deterministic smoothed noise.
    Noise,
}

/// A preset call.
#[derive(Debug, Clone)]
pub struct PresetCall {
    /// Preset name (e.g. `fade_in`, `modal_enter`).
    pub name: String,
    /// Optional duration override.
    pub duration: Option<f32>,
    /// Optional easing override.
    pub easing: Option<EasingSpec>,
    /// Source span for diagnostics.
    pub span: Span,
}

/// An `at` offset specification.
#[derive(Debug, Clone)]
pub struct AtSpec {
    /// The offset value.
    pub value: f32,
    /// Whether the offset is relative (`+`/`-`) or absolute.
    pub relative: bool,
}

/// A value expression in the DSL.
#[derive(Debug, Clone)]
pub enum ValueExpr {
    /// A single float.
    Scalar(f32),
    /// An array of floats (`[0.0, 20.0]`).
    Array(Vec<f32>),
}

impl ValueExpr {
    /// Returns the number of scalar components (1 for scalar, N for array).
    pub fn component_count(&self) -> usize {
        match self {
            Self::Scalar(_) => 1,
            Self::Array(v) => v.len(),
        }
    }
}

/// An easing specification.
#[derive(Debug, Clone)]
pub enum EasingSpec {
    /// A named easing (`ease_out_cubic`, `linear`, etc.).
    Named(String),
    /// `cubic_bezier(x1, y1, x2, y2)`.
    CubicBezier(f32, f32, f32, f32),
    /// `steps(n)`.
    Steps(u32),
    /// `wiggle(n)`.
    Wiggle(u32),
    /// `rough(strength: s, points: n)`.
    Rough {
        /// Noise amplitude `0.0..=1.0`.
        strength: f32,
        /// Number of noise harmonics.
        points: u32,
    },
    /// `slowmo(linear_ratio: r, power: p)`.
    SlowMo {
        /// Fraction of time spent slow.
        linear_ratio: f32,
        /// Sharpness of edge acceleration.
        power: f32,
    },
}

/// A loop-mode specification.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoopSpec {
    /// `once`.
    Once,
    /// `times(n)`.
    Times(u32),
    /// `forever`.
    Forever,
    /// `ping_pong`.
    PingPong,
    /// `ping_pong_times(n)`.
    PingPongTimes(u32),
}

/// A spring preset name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpringPreset {
    /// Slow, soft spring.
    Gentle,
    /// Bouncy, playful spring.
    Wobbly,
    /// Fast, firm spring.
    Stiff,
    /// Very slow, lazy spring.
    Slow,
    /// Near-instant response.
    Snappy,
}

/// Integration method specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegratorSpec {
    /// Semi-implicit Euler (default).
    SemiImplicitEuler,
    /// 4th-order Runge-Kutta.
    RungeKutta4,
}

/// Damping-mode helper specification.
#[derive(Debug, Clone, Copy)]
pub enum DampingMode {
    /// `critically_damped(stiffness)`.
    CriticallyDamped(f32),
    /// `overdamped(stiffness, ratio)`.
    Overdamped(f32, f32),
    /// `underdamped(stiffness, ratio)`.
    Underdamped(f32, f32),
}
