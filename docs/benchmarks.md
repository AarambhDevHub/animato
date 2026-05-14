# Animato v1.0.0 Benchmarks

Benchmarks are Criterion targets registered on the facade crate.

## Run

```sh
cargo bench
cargo bench --bench gpu_batch_bench --features gpu
```

## Coverage

| Benchmark | Scenario |
|-----------|----------|
| `easing_bench` | All 38 named easing variants and selected free functions. |
| `tween_update_bench` | 1, 100, 1,000, and 10,000 `Tween<f32>` updates. |
| `spring_bench` | Preset settle time and Euler/RK4 per-step cost. |
| `path_bench` | Curve evaluation, motion path reads, SVG parsing. |
| `physics_bench` | Inertia settle, drag movement, swipe recognition. |
| `timeline_bench` | 10-entry timeline update throughput. |
| `gpu_batch_bench` | 1,000 and 10,000 tween batch updates through CPU fallback. |

## Release Baseline

Before tagging v1.0.0, record:

```sh
rustc --version
cargo bench
```

Include operating system, CPU, GPU if relevant, and git revision in release
notes. Absolute timings are hardware-dependent; v1.0 CI requires benchmark
compilation and uses coverage/fuzz/test gates for correctness.

## Related Docs

- [Performance](./performance.md)
- [Release](./release.md)
