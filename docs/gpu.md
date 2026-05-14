# GPU Batch Evaluation

Feature: `gpu`.

```toml
[dependencies]
animato = { version = "1.0", features = ["gpu"] }
```

`GpuAnimationBatch` batches many `Tween<f32>` values. It uses GPU resources when
available and falls back to exact CPU evaluation when GPU setup fails or when an
easing cannot be represented in the shader.

## CPU Fallback

```rust
use animato::{Easing, GpuAnimationBatch, Tween};

let mut batch = GpuAnimationBatch::new_cpu();
for i in 0..100 {
    batch.push(
        Tween::new(0.0_f32, i as f32)
            .duration(1.0)
            .easing(Easing::EaseOutCubic)
            .build(),
    );
}

batch.tick(0.5);
assert_eq!(batch.read_back().len(), 100);
```

## Auto Backend

```rust,no_run
use animato::GpuAnimationBatch;

let batch = GpuAnimationBatch::new_auto();
println!("backend: {:?}", batch.backend());
```

## Notes

- Use direct `Vec<Tween<f32>>` for small batches.
- Use `GpuAnimationBatch` for thousands of scalar tweens.
- `read_back()` returns the latest value slice.
- Unsupported custom/advanced easings stay correct via CPU fallback.

## Related Docs

- [Performance](./performance.md)
- [Benchmarks](./benchmarks.md)
