//! Benchmark: path evaluation and SVG parsing.
//!
//! Run with: `cargo bench --bench path_bench`

use animato_core::Easing;
use animato_path::{CompoundPath, CubicBezierCurve, MotionPathTween, PathEvaluate, SvgPathParser};
use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn bench_cubic_eval(c: &mut Criterion) {
    let curve = CubicBezierCurve::new([0.0, 0.0], [25.0, 100.0], [75.0, -100.0], [100.0, 0.0]);

    c.bench_function("path/cubic_position", |b| {
        b.iter(|| black_box(curve.position(black_box(0.5))))
    });
}

fn bench_motion_path_tween_value(c: &mut Criterion) {
    let curve = CubicBezierCurve::new([0.0, 0.0], [25.0, 100.0], [75.0, -100.0], [100.0, 0.0]);
    let motion = MotionPathTween::new(curve)
        .duration(2.0)
        .easing(Easing::EaseOutCubic)
        .build();

    c.bench_function("path/motion_value", |b| {
        b.iter(|| black_box(motion.value()))
    });
}

fn bench_svg_parse(c: &mut Criterion) {
    let d = "M0 0 C10 0 30 40 50 50 L100 50 A25 25 0 0 1 150 50 Z";

    c.bench_function("path/svg_parse", |b| {
        b.iter(|| black_box(SvgPathParser::try_parse(black_box(d)).unwrap()))
    });

    c.bench_function("path/compound_from_svg", |b| {
        b.iter(|| black_box(CompoundPath::try_from_svg(black_box(d)).unwrap()))
    });
}

criterion_group!(
    benches,
    bench_cubic_eval,
    bench_motion_path_tween_value,
    bench_svg_parse
);
criterion_main!(benches);
