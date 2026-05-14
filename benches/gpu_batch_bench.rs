//! Benchmark: 10,000 tweens per tick through `GpuAnimationBatch`.
//!
//! Run with: `cargo bench --bench gpu_batch_bench --features gpu`

use animato_core::Easing;
use animato_gpu::GpuAnimationBatch;
use animato_tween::Tween;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};

const DT: f32 = 1.0 / 60.0;

fn make_tween(index: usize) -> Tween<f32> {
    Tween::new(0.0_f32, 1.0 + index as f32)
        .duration(2.0)
        .easing(Easing::EaseOutCubic)
        .build()
}

fn bench_cpu_fallback_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("gpu_batch/cpu_fallback");
    for &n in &[1_000_usize, 10_000] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            let mut batch = GpuAnimationBatch::new_cpu();
            for i in 0..n {
                batch.push(make_tween(i));
            }
            b.iter(|| {
                batch.tick(black_box(DT));
                let output = batch.read_back();
                black_box(output.len());
                black_box(output.last().copied().unwrap_or_default());
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_cpu_fallback_batch);
criterion_main!(benches);
