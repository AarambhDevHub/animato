# Macro Recipes — Copy-Paste Animation Patterns

> Practical recipes for the `animato!{}` Motion Macro DSL (v1.7.0).

---

## UI Entrances

### Fade in

```rust,ignore
let mut t = tween! { opacity: 0.0 => 1.0, duration: 0.4, easing: ease_out_cubic };
```

### Slide in from left

```rust,ignore
let mut t = tween! { x: -100.0 => 0.0, duration: 0.5, easing: ease_out_back };
```

### Scale in with overshoot

```rust,ignore
let mut t = tween! { scale: 0.0 => 1.0, duration: 0.4, easing: ease_out_back };
```

### Combined fade + slide + scale

```rust,ignore
let mut group = animato! {
    parallel {
        tween opacity: 0.0 => 1.0, duration: 0.4, easing: ease_out_cubic;
        tween y: 24.0 => 0.0, duration: 0.5, easing: ease_out_back;
        spring scale: 0.92 => 1.0, preset: snappy;
    }
};
```

---

## UI Exits

### Fade out

```rust,ignore
let mut t = tween! { opacity: 1.0 => 0.0, duration: 0.3, easing: ease_in_cubic };
```

### Slide out to right

```rust,ignore
let mut t = tween! { x: 0.0 => 100.0, duration: 0.4, easing: ease_in_cubic };
```

---

## Modal Transitions

### Modal enter (fade + scale)

```rust,ignore
let mut modal = animato! { preset modal_enter };
```

Or explicitly:

```rust,ignore
let mut modal = animato! {
    parallel {
        tween opacity: 0.0 => 1.0, duration: 0.4, easing: ease_out_cubic;
        spring scale: 0.9 => 1.0, preset: snappy;
    }
};
```

### Modal exit

```rust,ignore
let mut modal = animato! { preset modal_exit };
```

---

## Toast Notifications

### Toast enter (slide up + fade)

```rust,ignore
let mut toast = animato! { preset toast_enter };
```

### Toast exit (fade + slide down)

```rust,ignore
let mut toast = animato! { preset toast_exit };
```

---

## Page Transitions

### Page enter

```rust,ignore
let mut page = animato! { preset page_enter };
```

### Page exit

```rust,ignore
let mut page = animato! { preset page_exit };
```

---

## List Stagger

### Linear stagger for list items

```rust,ignore
let mut timeline = animato! {
    stagger delay: 0.05 {
        tween opacity: 0.0 => 1.0, duration: 0.25;
        tween opacity: 0.0 => 1.0, duration: 0.25;
        tween opacity: 0.0 => 1.0, duration: 0.25;
        tween opacity: 0.0 => 1.0, duration: 0.25;
        tween opacity: 0.0 => 1.0, duration: 0.25;
    }
};
```

### Grid stagger (center-out)

```rust,ignore
let mut timeline = animato! {
    stagger pattern: grid(cols: 4, rows: 3, origin: center), delay: 0.06 {
        tween opacity: 0.0 => 1.0, duration: 0.25;
    }
};
```

### Random stagger

```rust,ignore
let mut timeline = animato! {
    stagger pattern: random(seed: 42, min: 0.02, max: 0.12), delay: 0.05 {
        tween opacity: 0.0 => 1.0, duration: 0.25;
    }
};
```

---

## Loading Animations

### Loading pulse

```rust,ignore
let mut pulse = animato! { preset loading_pulse };
```

### Loading wave (sine)

```rust,ignore
let mut wave = animato! {
    waveform sine frequency: 1.5, amplitude: 0.3, duration: 2.0
};
```

### Shimmer

```rust,ignore
let mut shimmer = animato! { preset shimmer };
```

---

## Spring Animations

### Card spring with velocity

```rust,ignore
let mut card = spring! {
    scale: 0.8 => 1.0,
    preset: snappy,
    velocity: 200.0,
};
```

### Wobbly button press

```rust,ignore
let mut button = spring! { scale: 1.0 => 0.95, preset: wobbly };
```

### Stiff panel slide

```rust,ignore
let mut panel = spring! { x: -320.0 => 0.0, preset: stiff };
```

---

## Motion Paths

### SVG path motion

```rust,ignore
let mut motion = animato! {
    path position along "M 0 0 C 50 100 150 100 200 0",
        duration: 1.2,
        easing: ease_in_out_sine,
        auto_rotate: true;
};
```

### Path with offset

```rust,ignore
let mut motion = animato! {
    path position along "M 0 0 L 100 100",
        duration: 1.0,
        offset: 0.1..0.9,
        easing: ease_in_out_sine;
};
```

---

## Color Animations

### Red to blue in Oklch

```rust,ignore
let mut color = animato! {
    color background: "#ff0000" => "#0000ff",
        duration: 0.8, space: oklch, easing: ease_in_out_sine;
};
```

### Named colors

```rust,ignore
let mut color = animato! {
    color bg: white => black, duration: 0.5, space: lab;
};
```

---

## Decorative Animations

### Shake

```rust,ignore
let mut shake = animato! { preset shake };
```

### Wiggle

```rust,ignore
let mut wiggle = animato! { preset wiggle };
```

### Heartbeat

```rust,ignore
let mut heart = animato! { preset heartbeat };
```

### Float

```rust,ignore
let mut float = animato! { preset float };
```

---

## Hero Intro (Full Example)

The complete hero intro from the v1.7.0 roadmap:

```rust,ignore
let mut intro = animato! {
    sequence {
        tween opacity: 0.0 => 1.0, duration: 0.35, easing: ease_out_cubic;

        parallel {
            tween y: 24.0 => 0.0, duration: 0.55, easing: ease_out_back;
            spring scale: 0.92 => 1.0, preset: snappy;
        }

        stagger pattern: grid(cols: 3, rows: 2, origin: center), delay: 0.06 {
            tween card[0].opacity: 0.0 => 1.0, duration: 0.25;
            tween card[1].opacity: 0.0 => 1.0, duration: 0.25;
            tween card[2].opacity: 0.0 => 1.0, duration: 0.25;
            tween card[3].opacity: 0.0 => 1.0, duration: 0.25;
            tween card[4].opacity: 0.0 => 1.0, duration: 0.25;
            tween card[5].opacity: 0.0 => 1.0, duration: 0.25;
        }
    }
};
```

---

## Advanced Easing Recipes

### Cubic bezier (CSS-style)

```rust,ignore
let mut t = tween! {
    x: 0.0 => 1.0, duration: 0.5,
    easing: cubic_bezier(0.22, 1.0, 0.36, 1.0),
};
```

### Stepped

```rust,ignore
let mut t = tween! {
    x: 0.0 => 100.0, duration: 1.0,
    easing: steps(10),
};
```

### Wiggle easing

```rust,ignore
let mut t = tween! {
    x: 0.0 => 100.0, duration: 2.0,
    easing: wiggle(8),
};
```
