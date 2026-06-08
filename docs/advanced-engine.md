# Advanced Engine

Animato v1.5.0 adds advanced engine primitives that work across Rust,
WASM, and JavaScript bindings.

## Spring From Velocity

Use release velocity from drag/fling gestures to continue motion into a spring:

```rust
use animato::{Spring, SpringConfig, Update};

let mut spring = Spring::from_velocity(0.0, 850.0, 320.0, SpringConfig::underdamped(180.0, 0.65));
while spring.update(1.0 / 60.0) {}

assert!(spring.is_settled());
```

`Spring::energy()` reports kinetic plus potential energy, and
`Spring::overshoot_count()` counts target crossings.

## Waveforms

`Waveform` generates procedural scalar values and can bake them into
`KeyframeTrack<f32>`:

```rust
use animato::Waveform;

let wave = Waveform::Sine { frequency: 1.0, amplitude: 24.0, phase: 0.0 };
let track = wave.to_keyframe_track(1.0, 60.0);
```

Supported waveforms are sine, sawtooth, square, triangle, and deterministic
smoothed noise.

## Rotation And Matrix Interpolation

`Angle` interpolates along the shortest angular path. `Quaternion` uses slerp,
and `Mat4` interpolates affine transforms by translation, rotation, and scale:

```rust
use animato::{Angle, Quaternion, Tween};

let start = Quaternion::IDENTITY;
let end = Quaternion::from_axis_angle([0.0, 1.0, 0.0], Angle::from_degrees(180.0));
let tween = Tween::new(start, end).duration(1.0).build();
```

## Stagger Patterns And Groups

`StaggerPattern` calculates deterministic delays for grid, random,
center-out, and edges-in layouts. `AnimationGroup` controls a parallel,
sequence, or staggered set as one `Playable`.

```rust
use animato::{AnimationGroup, GridOrigin, StaggerPattern, Tween};

let pattern = StaggerPattern::Grid {
    cols: 4,
    rows: 3,
    origin: GridOrigin::Center,
    step: 0.08,
};

let mut group = AnimationGroup::stagger(
    vec![Tween::new(0.0_f32, 1.0).duration(0.3).build()],
    pattern,
);
group.play();
```

## Recorder

`AnimationRecorder` captures scalar samples for DevTools-style replay:

```rust
use animato::AnimationRecorder;

let mut recorder = AnimationRecorder::new();
recorder.start();
recorder.record("opacity", 0.0, 0.0);
recorder.record("opacity", 1.0, 1.0);

let json = recorder.export_json();
let restored = AnimationRecorder::import_json(&json).unwrap();
assert_eq!(restored.replay("opacity", 0.5), Some(0.5));
```

The binary format is deterministic and dependency-free.

## JavaScript

The NPM package exports `Spring.fromVelocity`, `Waveform`, `StaggerPattern`,
`Angle`, `Quaternion`, `Mat4`, `AngleTween`, `QuaternionTween`, `Mat4Tween`,
`AnimationGroup`, and `AnimationRecorder`.
