# Macro Reference — Full DSL Grammar

> Complete keyword and syntax reference for the `animato!{}` Motion Macro DSL
> (v1.7.0).

---

## Top-Level Macros

```rust
animato! { <motion_block> }       // primary DSL
motion!  { <motion_block> }       // alias (identical expansion)
tween!     { <tween_stmt> }       // standalone Tween<T>
spring!    { <spring_stmt> }      // standalone Spring / SpringN<T>
timeline!  { <motion_block> }     // standalone Timeline
keyframes! { <keyframe_def> }     // standalone KeyframeTrack<T>
preset!    { <preset_def> }       // user-defined preset
```

## Motion Block

A motion block is a sequence of motion nodes:

```
motion_block := { <node>* }
node := tween_stmt | spring_stmt | keyframes_block | sequence_block
      | parallel_block | group_block | stagger_block | path_stmt
      | morph_stmt | draw_stmt | color_stmt | waveform_stmt
      | preset_call | label_stmt | at_stmt
```

---

## Tween Statement

```
tween_stmt := 'tween' <target> ':' <value> '=>' <value> ',' <tween_field>+ ';'
tween_field := 'duration' ':' <number>
             | 'easing' ':' <easing>
             | 'delay' ':' <number>
             | 'time_scale' ':' <number>
             | 'loop' ':' <loop_mode>
             | 'snap' ':' <number>
             | 'round' ':' <int>
             | 'reverse' ':' 'true'
target := <ident>
value := <number> | '[' <number> (',' <number>)* ']'
```

### Examples

```rust,ignore
// Scalar tween
tween! { opacity: 0.0 => 1.0, duration: 0.4, easing: ease_out_cubic }

// Vector tween (2D)
tween! { position: [0.0, 20.0] => [10.0, 30.0], duration: 0.6 }

// With delay, time scale, and ping-pong loop
tween! {
    x: 0.0 => 100.0,
    duration: 0.5,
    delay: 0.2,
    time_scale: 1.5,
    loop: ping_pong,
}
```

---

## Easing

```
easing := <easing_name>                         // ease_out_cubic
        | 'Easing::' <PascalName>               // Easing::EaseOutCubic
        | 'cubic_bezier' '(' <n> ',' <n> ',' <n> ',' <n> ')'
        | 'steps' '(' <int> ')'
        | 'wiggle' '(' <int> ')'
        | 'rough' '(' 'strength' ':' <n> ',' 'points' ':' <int> ')'
        | 'slowmo' '(' 'linear_ratio' ':' <n> ',' 'power' ':' <n> ')'
```

### Named easings (31)

`linear`, `ease_in_quad`, `ease_out_quad`, `ease_in_out_quad`,
`ease_in_cubic`, `ease_out_cubic`, `ease_in_out_cubic`,
`ease_in_quart`, `ease_out_quart`, `ease_in_out_quart`,
`ease_in_quint`, `ease_out_quint`, `ease_in_out_quint`,
`ease_in_sine`, `ease_out_sine`, `ease_in_out_sine`,
`ease_in_expo`, `ease_out_expo`, `ease_in_out_expo`,
`ease_in_circ`, `ease_out_circ`, `ease_in_out_circ`,
`ease_in_back`, `ease_out_back`, `ease_in_out_back`,
`ease_in_elastic`, `ease_out_elastic`, `ease_in_out_elastic`,
`ease_in_bounce`, `ease_out_bounce`, `ease_in_out_bounce`

### Examples

```rust,ignore
easing: ease_out_cubic
easing: Easing::EaseOutCubic
easing: cubic_bezier(0.22, 1.0, 0.36, 1.0)
easing: steps(5)
easing: wiggle(4)
easing: rough(strength: 0.5, points: 8)
easing: slowmo(linear_ratio: 0.7, power: 2.0)
```

---

## Loop Mode

```
loop_mode := 'once' | 'times' '(' <int> ')' | 'forever'
           | 'ping_pong' | 'ping_pong_times' '(' <int> ')'
```

### Examples

```rust,ignore
loop: once
loop: times(3)
loop: forever
loop: ping_pong
loop: ping_pong_times(4)
```

---

## Spring Statement

```
spring_stmt := 'spring' <target> ':' <value> '=>' <value> ',' <spring_field>+ ';'
spring_field := 'preset' ':' <spring_preset>
              | 'stiffness' ':' <number>
              | 'damping' ':' <number>
              | 'mass' ':' <number>
              | 'velocity' ':' <value>
              | 'epsilon' ':' <number>
              | 'integrator' ':' 'rk4'
              | 'critically_damped' '(' <number> ')'
              | 'overdamped' '(' <number> ',' <number> ')'
              | 'underdamped' '(' <number> ',' <number> ')'
spring_preset := 'gentle' | 'wobbly' | 'stiff' | 'slow' | 'snappy'
```

### Examples

```rust,ignore
// Preset
spring! { scale: 0.8 => 1.0, preset: snappy }

// Explicit config
spring! { x: 0.0 => 100.0, stiffness: 350.0, damping: 28.0, mass: 1.0 }

// With initial velocity (fling-to-snap)
spring! { x: 0.0 => 320.0, velocity: 900.0, preset: snappy }

// Damping mode
spring! { x: 0.0 => 1.0, underdamped(120.0, 0.5) }

// RK4 integrator
spring! { x: 0.0 => 1.0, preset: wobbly, integrator: rk4 }
```

### Spring presets

| Preset | Stiffness | Damping | Feel |
|--------|-----------|---------|------|
| `gentle` | 60 | 14 | Slow, soft |
| `wobbly` | 180 | 12 | Bouncy, playful |
| `stiff` | 210 | 20 | Fast, firm |
| `slow` | 37 | 14 | Very slow, lazy |
| `snappy` | 300 | 30 | Near-instant |

---

## Keyframes

```
keyframes_block := 'keyframes' <target> '{' <kf_entry> (',' <kf_entry>)* '}'
kf_entry := <time> ':' <value> [<easing>]
time := <percent> | <number> 's'
percent := <int> '%'
```

### Examples

```rust,ignore
keyframes! {
    opacity {
        0%: 0.0,
        50%: 0.7 ease_out_cubic,
        100%: 1.0,
    }
}

// With loop
keyframes! {
    scale {
        0%: 1.0,
        50%: 1.1 ease_out_quad,
        100%: 1.0,
        loop: forever,
    }
}
```

---

## Composition

### Sequence

```rust,ignore
animato! {
    sequence {
        tween opacity: 0.0 => 1.0, duration: 0.3;
        spring scale: 0.8 => 1.0, preset: snappy;
    }
}
```

### Parallel

```rust,ignore
animato! {
    parallel {
        tween x: 0.0 => 100.0, duration: 1.0;
        tween y: 0.0 => 50.0, duration: 1.0;
    }
}
```

### Group

```rust,ignore
animato! {
    group sequence {
        tween x: 0.0 => 1.0, duration: 0.3;
        tween y: 0.0 => 1.0, duration: 0.3;
    }
}
```

### Labels and Offsets

```rust,ignore
animato! {
    sequence {
        label "intro";
        tween opacity: 0.0 => 1.0, duration: 0.3;
        at "+0.2";  // 0.2s after the previous entry ends
        spring scale: 0.8 => 1.0, preset: snappy;
    }
}
```

---

## Stagger

```
stagger_block := 'stagger' [<stagger_pattern>] ',' 'delay' ':' <number> '{' <node>* '}'
stagger_pattern := 'pattern' ':' ('grid' '(' ... ')'
                             | 'random' '(' ... ')'
                             | 'center_out'
                             | 'edges_in')
```

### Examples

```rust,ignore
// Linear stagger
animato! {
    stagger delay: 0.05 {
        tween opacity: 0.0 => 1.0, duration: 0.25;
        tween opacity: 0.0 => 1.0, duration: 0.25;
        tween opacity: 0.0 => 1.0, duration: 0.25;
    }
}

// Grid stagger
animato! {
    stagger pattern: grid(cols: 4, rows: 3, origin: center), delay: 0.06 {
        tween opacity: 0.0 => 1.0, duration: 0.25;
    }
}

// Random stagger
animato! {
    stagger pattern: random(seed: 42, min: 0.02, max: 0.12), delay: 0.05 {
        tween opacity: 0.0 => 1.0, duration: 0.25;
    }
}
```

### Grid origins

`top_left`, `top_right`, `bottom_left`, `bottom_right`, `center`, `top`, `bottom`, `left`, `right`

---

## Path

```
path_stmt := 'path' <target> 'along' <string> ',' 'duration' ':' <number>
             (',' 'auto_rotate' ':' 'true')?
             (',' 'offset' ':' <number> '..' <number>)?
             (',' 'easing' ':' <easing>)?
             (',' 'loop' ':' <loop_mode>)?
```

### Example

```rust,ignore
animato! {
    path position along "M 0 0 C 50 100 150 100 200 0",
        duration: 1.2,
        easing: ease_in_out_sine,
        auto_rotate: true,
        offset: 0.1..0.9;
}
```

---

## Morph

```
morph_stmt := 'morph' <string> '=>' <string> ','
             'samples' ':' <int> ',' 'duration' ':' <number>
             (',' 'easing' ':' <easing>)?
```

### Example

```rust,ignore
animato! {
    morph "M 0 0 L 100 0 L 100 100 L 0 100 Z" => "M 50 0 L 100 50 L 50 100 L 0 50 Z",
        samples: 64, duration: 0.8, easing: ease_in_out_sine;
}
```

---

## Draw

```
draw_stmt := 'draw' <string> ',' 'duration' ':' <number>
             (',' 'easing' ':' <easing>)?
```

### Example

```rust,ignore
animato! { draw "M 0 0 L 100 100", duration: 1.0, easing: ease_out_cubic }
```

---

## Color

```
color_stmt := 'color' <target> ':' <hex_or_name> '=>' <hex_or_name> ','
             'duration' ':' <number>
             (',' 'space' ':' <color_space>)?
             (',' 'easing' ':' <easing>)?
color_space := 'linear' | 'lab' | 'oklch'
```

### Color formats

- Hex: `#rgb`, `#rgba`, `#rrggbb`, `#rrggbbaa`
- Named: `red`, `green`, `blue`, `white`, `black`, `yellow`, `cyan`, `magenta`, `gray`/`grey`, `orange`, `purple`, `pink`, `brown`, `lime`, `teal`, `navy`, `maroon`, `olive`, `silver`, `aqua`, `fuchsia`

### Example

```rust,ignore
animato! {
    color background: "#ff0000" => "#0000ff",
        duration: 0.5, space: oklch, easing: ease_in_out_sine;
}
```

---

## Waveform

```
waveform_stmt := 'waveform' <wave_kind> <wf_fields>
wave_kind := 'sine' | 'sawtooth' | 'square' | 'triangle' | 'noise'
wf_fields := 'frequency' ':' <n> ',' 'amplitude' ':' <n>
           | 'phase' ':' <n>  // sine only
           | 'duty_cycle' ':' <n>  // square only
           | 'seed' ':' <int> ',' 'smoothness' ':' <n>  // noise only
           | 'duration' ':' <n>
```

### Example

```rust,ignore
animato! { waveform sine frequency: 2.0, amplitude: 1.0, phase: 0.0, duration: 2.0 }
animato! { waveform noise seed: 42, smoothness: 0.8, duration: 3.0 }
```

---

## Presets

### Built-in presets (24)

```
fade_in, fade_out, slide_in, slide_out, scale_in, scale_out,
bounce_in, bounce_out, modal_enter, modal_exit, drawer_open, drawer_close,
toast_enter, toast_exit, page_enter, page_exit, stagger_children,
loading_pulse, loading_wave, shake, wiggle, heartbeat, float, shimmer
```

### Usage

```rust,ignore
animato! { preset fade_in }
animato! { preset modal_enter(duration: 0.5, easing: ease_out_back) }
```

### User-defined presets

```rust,ignore
preset! { card_enter {
    sequence {
        tween opacity: 0.0 => 1.0, duration: 0.3;
        spring y: 20.0 => 0.0, preset: snappy;
    }
} }
```

---

## Error Messages

The macro produces clear, span-aware diagnostics:

- `missing required field `duration``
- `unknown easing `ease_out_magic`; did you mean `ease_out_cubic`?`
- `unknown spring preset `bouncy`; valid presets are: gentle, wobbly, stiff, slow, snappy`
- `tween `from` and `to` value types do not match: 1 vs 2`
- `unknown preset `not_a_real_preset``
- `unknown DSL keyword `banana``
- `stagger `grid` requires both `cols` and `rows``
- `invalid color `#gg0000`; expected hex (#rgb, #rgba, #rrggbb, #rrggbbaa) or a named color`
