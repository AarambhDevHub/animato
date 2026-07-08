# Macro Expansion — What Code the Macro Generates

> The `animato!{}` macro is a pure compile-time authoring layer. It generates
> ordinary Rust code that uses the stable Animato builder API. This document
> shows representative input → output pairs so you can understand and debug
> macro expansions.

To inspect the generated code yourself, use `cargo expand`:

```sh
cargo install cargo-expand
cargo expand --example macro_basic_tween --features macro
```

---

## 1. Simple Tween

### Input

```rust,ignore
let t = tween! {
    opacity: 0.0 => 1.0,
    duration: 0.4,
    easing: ease_out_cubic
};
```

### Generated (approximate)

```rust
let t = {
    let mut __t = animato::Tween::new(0.0 as f32, 1.0 as f32)
        .duration(0.4)
        .easing(animato::Easing::EaseOutCubic)
        .build();
    __t
};
```

---

## 2. Tween with Delay and Loop

### Input

```rust,ignore
let t = tween! {
    x: 0.0 => 100.0,
    duration: 0.5,
    delay: 0.2,
    loop: ping_pong
};
```

### Generated (approximate)

```rust
let t = {
    let mut __t = animato::Tween::new(0.0 as f32, 100.0 as f32)
        .duration(0.5)
        .delay(0.2)
        .looping(animato::Loop::PingPong)
        .build();
    __t
};
```

---

## 3. Spring with Preset

### Input

```rust,ignore
let s = spring! { scale: 0.8 => 1.0, preset: snappy };
```

### Generated (approximate)

```rust
let s = {
    let __cfg = animato::SpringConfig::snappy();
    let mut __s = animato::Spring::new(__cfg);
    __s.snap_to(0.8);
    __s.set_target(1.0);
    __s
};
```

---

## 4. Spring with Velocity

### Input

```rust,ignore
let s = spring! {
    x: 0.0 => 320.0,
    velocity: 900.0,
    preset: snappy
};
```

### Generated (approximate)

```rust
let s = {
    let __cfg = animato::SpringConfig::snappy();
    animato::Spring::from_velocity(0.0, 900.0, 320.0, __cfg)
};
```

---

## 5. Keyframes

### Input

```rust,ignore
let track = keyframes! {
    opacity {
        0%: 0.0,
        50%: 0.7 ease_out_cubic,
        100%: 1.0,
    }
};
```

### Generated (approximate)

```rust
let track = {
    let mut __track: animato::KeyframeTrack<f32> = animato::KeyframeTrack::new();
    __track = __track.push(0.0, 0.0 as f32);
    __track = __track.push_eased(0.5, 0.7 as f32, animato::Easing::EaseOutCubic);
    __track = __track.push(1.0, 1.0 as f32);
    __track
};
```

---

## 6. Sequence

### Input

```rust,ignore
let timeline = animato! {
    sequence {
        tween opacity: 0.0 => 1.0, duration: 0.3, easing: ease_out_cubic;
        spring scale: 0.8 => 1.0, preset: snappy;
    }
};
```

### Generated (approximate)

```rust
let timeline = {
    let mut __seq = animato::Sequence::new();
    __seq = __seq.then(
        "node_0",
        {
            let mut __t = animato::Tween::new(0.0 as f32, 1.0 as f32)
                .duration(0.3)
                .easing(animato::Easing::EaseOutCubic)
                .build();
            __t
        },
    );
    __seq = __seq.then(
        "node_1",
        {
            let __cfg = animato::SpringConfig::snappy();
            let mut __s = animato::Spring::new(__cfg);
            __s.snap_to(0.8);
            __s.set_target(1.0);
            __s
        },
    );
    __seq.build()
};
```

---

## 7. Parallel

### Input

```rust,ignore
let group = animato! {
    parallel {
        tween x: 0.0 => 100.0, duration: 1.0;
        tween y: 0.0 => 50.0, duration: 1.0;
    }
};
```

### Generated (approximate)

```rust
let group = animato::AnimationGroup::parallel(::std::vec![
    {
        let mut __t = animato::Tween::new(0.0 as f32, 100.0 as f32)
            .duration(1.0)
            .build();
        __t
    },
    {
        let mut __t = animato::Tween::new(0.0 as f32, 50.0 as f32)
            .duration(1.0)
            .build();
        __t
    },
]);
```

---

## 8. Preset Call

### Input

```rust,ignore
let anim = animato! { preset fade_in };
```

### Generated (approximate)

The preset expands into its underlying tween definition:

```rust
let anim = {
    let mut __t = animato::Tween::new(0.0 as f32, 1.0 as f32)
        .duration(0.4)
        .easing(animato::Easing::EaseOutCubic)
        .build();
    __t
};
```

---

## Key Properties

1. **No hidden runtime** — every expansion uses public `animato::` API.
2. **Readable** — generated code uses intermediate `let` bindings and `__` prefixed hygienic identifiers.
3. **Type-annotated** — `as f32` / `as [f32; N]` annotations ensure correct monomorphization.
4. **Composable** — nested nodes expand recursively.
5. **Zero-cost** — the macro adds no runtime overhead vs. hand-written builders.
