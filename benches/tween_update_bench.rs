//! Benchmark: Tween<f32> update throughput at 1, 100, and 10,000 concurrent tweens.
//!
//! Run with: `cargo bench --bench tween_update_bench`

use animato_core::{Easing, Update};
use animato_tween::Tween;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};

const DT: f32 = 1.0 / 60.0;

fn make_tween() -> Tween<f32> {
    Tween::new(0.0_f32, 100.0)
        .duration(2.0)
        .easing(Easing::EaseOutCubic)
        .build()
}

fn bench_single(c: &mut Criterion) {
    c.bench_function("tween/update/1", |b| {
        let mut t = make_tween();
        b.iter(|| {
            t.update(black_box(DT));
            black_box(t.value())
        });
    });
}

fn bench_n_tweens(c: &mut Criterion) {
    let mut group = c.benchmark_group("tween/update/n");

    for &n in &[100_usize, 1_000, 10_000] {
        group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
            let mut tweens: Vec<Tween<f32>> = (0..n).map(|_| make_tween()).collect();
            b.iter(|| {
                let mut sum = 0.0_f32;
                for t in tweens.iter_mut() {
                    t.update(black_box(DT));
                    sum += t.value();
                }
                black_box(sum)
            });
        });
    }
    group.finish();
}

fn bench_value_hot_path(c: &mut Criterion) {
    c.bench_function("tween/value_only", |b| {
        let mut t = make_tween();
        t.update(0.5); // advance to midpoint
        b.iter(|| black_box(t.value()));
    });
}

criterion_group!(benches, bench_single, bench_n_tweens, bench_value_hot_path);
criterion_main!(benches);
