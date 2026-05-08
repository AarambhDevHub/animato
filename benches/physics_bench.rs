//! Benchmark: physics inertia, drag tracking, and gesture recognition.
//!
//! Run with: `cargo bench --bench physics_bench`

use animato_core::Update;
use animato_physics::{
    DragConstraints, DragState, GestureRecognizer, Inertia, InertiaConfig, PointerData,
};
use criterion::{Criterion, black_box, criterion_group, criterion_main};

const DT: f32 = 1.0 / 60.0;

fn bench_inertia_settle(c: &mut Criterion) {
    c.bench_function("physics/inertia_settle", |b| {
        b.iter(|| {
            let mut inertia = Inertia::new(InertiaConfig::smooth());
            inertia.kick(black_box(1200.0));
            while inertia.update(DT) {}
            black_box(inertia.position())
        });
    });
}

fn bench_drag_move(c: &mut Criterion) {
    c.bench_function("physics/drag_move", |b| {
        b.iter(|| {
            let mut drag = DragState::new([0.0, 0.0])
                .constraints(DragConstraints::bounded(-500.0, 500.0, -500.0, 500.0));
            drag.on_pointer_down(PointerData::new(0.0, 0.0, 1));
            for i in 0..120 {
                drag.on_pointer_move(PointerData::new(i as f32, i as f32 * 0.5, 1), DT);
            }
            black_box(drag.velocity())
        });
    });
}

fn bench_gesture_swipe(c: &mut Criterion) {
    c.bench_function("physics/gesture_swipe", |b| {
        b.iter(|| {
            let mut recognizer = GestureRecognizer::default();
            recognizer.on_pointer_down(PointerData::new(0.0, 0.0, 1), 0.0);
            black_box(recognizer.on_pointer_up(PointerData::new(100.0, 0.0, 1), 0.2))
        });
    });
}

criterion_group!(
    benches,
    bench_inertia_settle,
    bench_drag_move,
    bench_gesture_swipe
);
criterion_main!(benches);
