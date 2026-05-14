//! Benchmark: 10-entry timeline tick throughput.
//!
//! Run with: `cargo bench --bench timeline_bench`

use animato_core::{Easing, Update};
use animato_timeline::{At, Timeline};
use animato_tween::Tween;
use criterion::{Criterion, black_box, criterion_group, criterion_main};

const DT: f32 = 1.0 / 60.0;

fn make_timeline() -> Timeline {
    let mut timeline = Timeline::new();
    for i in 0..10 {
        let tween = Tween::new(0.0_f32, 100.0 + i as f32)
            .duration(1.0)
            .easing(Easing::EaseOutCubic)
            .build();
        timeline = timeline.add(format!("entry-{i}"), tween, At::Absolute(i as f32 * 0.05));
    }
    timeline.play();
    timeline
}

fn bench_timeline_tick(c: &mut Criterion) {
    c.bench_function("timeline/tick_10_entries", |b| {
        let mut timeline = make_timeline();
        b.iter(|| {
            if timeline.is_complete() {
                timeline.reset();
                timeline.play();
            }
            black_box(timeline.update(black_box(DT)))
        });
    });
}

criterion_group!(benches, bench_timeline_tick);
criterion_main!(benches);
