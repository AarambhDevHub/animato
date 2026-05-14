# Performance

Animato keeps the common update path small and predictable.

## Hot Path

- `Tween<T>::update(dt)` only advances scalar timing state.
- `Tween<T>::value()` performs easing plus one `Interpolate::lerp`.
- `Spring::update(dt)` integrates one scalar spring.
- `MorphPath::evaluate_into(t, output)` reuses caller-provided allocation.
- `GpuAnimationBatch` batches scalar tweens and falls back to CPU correctly.

## Avoiding Allocation

Use stack-only types in inner loops:

```rust
use animato::{Tween, Update};

let mut tweens: Vec<_> = (0..1000)
    .map(|i| Tween::new(0.0_f32, i as f32).duration(1.0).build())
    .collect();

for tween in &mut tweens {
    tween.update(1.0 / 60.0);
}
```

The `Vec` allocation belongs to your storage. Individual tween updates do not
allocate.

## Choosing A Strategy

| Count | Strategy |
|-------|----------|
| 1 to 100 | Plain `Tween<T>` or `AnimationDriver`. |
| 100 to 10,000 | `Vec<Tween<f32>>` for scalar values. |
| 10,000+ scalar tweens | `GpuAnimationBatch`. |
| Complex values | Monomorphized `Vec<Tween<T>>`. |

## Benchmarks

Use:

```sh
cargo bench
cargo bench --bench gpu_batch_bench --features gpu
```

See [benchmarks.md](./benchmarks.md).
