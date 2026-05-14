//! Benchmark: Spring settle time and per-frame update cost across all presets.
//!
//! Run with: `cargo bench --bench spring_bench`

use animato_core::Update;
use animato_spring::{Spring, SpringConfig};
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};

const DT: f32 = 1.0 / 60.0;
type SpringPreset = (&'static str, fn() -> SpringConfig);

fn settle(config: SpringConfig) -> usize {
    let mut s = Spring::new(config);
    s.set_target(black_box(100.0));
    let mut steps = 0;
    while s.update(DT) {
        steps += 1;
    }
    steps
}

fn bench_settle_all_presets(c: &mut Criterion) {
    let mut group = c.benchmark_group("spring/settle");

    let presets: &[SpringPreset] = &[
        ("gentle", SpringConfig::gentle),
        ("wobbly", SpringConfig::wobbly),
        ("stiff", SpringConfig::stiff),
        ("slow", SpringConfig::slow),
        ("snappy", SpringConfig::snappy),
    ];

    for (name, make) in presets {
        group.bench_with_input(BenchmarkId::from_parameter(name), name, |b, _| {
            b.iter(|| black_box(settle(make())))
        });
    }
    group.finish();
}

fn bench_single_step(c: &mut Criterion) {
    let mut group = c.benchmark_group("spring/step");

    let configs: &[(&str, SpringConfig)] = &[
        ("euler_default", SpringConfig::default()),
        ("rk4_default", SpringConfig::default()),
    ];

    for (name, config) in configs {
        let use_rk4 = name.starts_with("rk4");
        group.bench_with_input(
            BenchmarkId::from_parameter(name),
            &(config.clone(), use_rk4),
            |b, (cfg, rk4)| {
                let mut s = Spring::new(cfg.clone());
                if *rk4 {
                    s = s.use_rk4(true);
                }
                s.set_target(100.0);
                b.iter(|| {
                    s.update(black_box(DT));
                    black_box(s.position())
                });
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_settle_all_presets, bench_single_step);
criterion_main!(benches);
