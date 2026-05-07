//! Benchmark: all 33 named/representative easing variants via Criterion.
//!
//! Run with: `cargo bench --bench easing_bench`

use animato_core::Easing;
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};

fn bench_all_named(c: &mut Criterion) {
    let mut group = c.benchmark_group("easing/named");

    for easing in Easing::all_named() {
        let label = format!("{:?}", easing);
        group.bench_with_input(BenchmarkId::new("apply", &label), easing, |b, e| {
            b.iter(|| {
                // Sweep 100 t values across [0, 1]
                let mut sum = 0.0_f32;
                for i in 0..=100 {
                    let t = i as f32 / 100.0;
                    sum += e.apply(black_box(t));
                }
                black_box(sum)
            })
        });
    }
    group.finish();
}

fn bench_free_functions(c: &mut Criterion) {
    use animato_core::easing::*;
    let mut group = c.benchmark_group("easing/free_fn");

    macro_rules! bench_fn {
        ($name:ident) => {
            group.bench_function(stringify!($name), |b| {
                b.iter(|| {
                    let mut s = 0.0_f32;
                    for i in 0..=100 {
                        s += $name(black_box(i as f32 / 100.0));
                    }
                    black_box(s)
                })
            });
        };
    }

    bench_fn!(ease_in_quad);
    bench_fn!(ease_out_quad);
    bench_fn!(ease_in_out_cubic);
    bench_fn!(ease_out_cubic);
    bench_fn!(ease_out_bounce);
    bench_fn!(ease_out_elastic);
    bench_fn!(ease_out_back);
    bench_fn!(ease_in_out_sine);
    group.bench_function("cubic_bezier_css_ease", |b| {
        b.iter(|| {
            let mut s = 0.0_f32;
            for i in 0..=100 {
                s += cubic_bezier(black_box(i as f32 / 100.0), 0.25, 0.1, 0.25, 1.0);
            }
            black_box(s)
        })
    });
    group.bench_function("steps_4", |b| {
        b.iter(|| {
            let mut s = 0.0_f32;
            for i in 0..=100 {
                s += steps(black_box(i as f32 / 100.0), 4);
            }
            black_box(s)
        })
    });

    group.finish();
}

criterion_group!(benches, bench_all_named, bench_free_functions);
criterion_main!(benches);
