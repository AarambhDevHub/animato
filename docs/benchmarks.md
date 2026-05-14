# Animato v0.9.0 Benchmarks

Current benchmark suite:

```sh
cargo bench
cargo bench --bench gpu_batch_bench --features gpu
```

## Coverage

| Benchmark | Scenario |
|-----------|----------|
| `easing_bench` | All 38 named easing variants plus selected free functions |
| `tween_update_bench` | 1, 100, 1,000, and 10,000 concurrent `Tween<f32>` updates |
| `spring_bench` | Spring settle time and per-step Euler/RK4 cost |
| `path_bench` | Cubic path evaluation, motion path value reads, SVG parsing |
| `physics_bench` | Inertia settle, drag movement, swipe recognition |
| `timeline_bench` | 10-entry timeline update throughput |
| `gpu_batch_bench` | 1,000 and 10,000 tween batch updates through `GpuAnimationBatch` CPU fallback |

## Notes

- v0.9.0 adds the benchmark targets and keeps them in CI with `cargo bench --workspace --no-run`.
- Criterion HTML reports are generated under `target/criterion/` after a local run.
- Record hardware, operating system, Rust version, and the exact git revision when publishing benchmark numbers.

## Release Baseline

Baseline numbers should be captured on the release machine immediately before tagging `v0.9.0`:

```sh
rustc --version
cargo bench
```

The release gate is benchmark compilation plus no correctness regressions; absolute timing is hardware-dependent.
