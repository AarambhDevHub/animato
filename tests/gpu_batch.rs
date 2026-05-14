//! Integration tests for v0.9.0 GPU batch fallback behavior.

use animato::{Easing, GpuAnimationBatch, GpuBackend, Tween, Update};

#[test]
fn facade_exports_gpu_batch_types() {
    let batch = GpuAnimationBatch::new_cpu();
    assert_eq!(batch.backend(), GpuBackend::Cpu);
    assert_eq!(batch.len(), 0);
}

#[test]
fn cpu_fallback_matches_regular_tween() {
    let mut expected = Tween::new(0.0_f32, 100.0)
        .duration(2.0)
        .easing(Easing::EaseInOutSine)
        .build();
    let mut batch = GpuAnimationBatch::new_cpu();
    batch.push(
        Tween::new(0.0_f32, 100.0)
            .duration(2.0)
            .easing(Easing::EaseInOutSine)
            .build(),
    );

    for _ in 0..10 {
        expected.update(1.0 / 60.0);
        batch.tick(1.0 / 60.0);
    }

    assert!((batch.read_back()[0] - expected.value()).abs() < 0.0001);
}

#[test]
fn unsupported_easing_stays_correct_on_cpu_fallback() {
    let mut batch = GpuAnimationBatch::new_cpu();
    batch.push(
        Tween::new(0.0_f32, 1.0)
            .duration(1.0)
            .easing(Easing::RoughEase {
                strength: 0.5,
                points: 8,
            })
            .build(),
    );

    batch.tick(0.5);

    assert_eq!(batch.backend(), GpuBackend::Cpu);
    assert!(batch.read_back()[0].is_finite());
}

#[test]
fn shader_source_is_available_for_packaging() {
    let source = GpuAnimationBatch::shader_source();
    assert!(source.contains("@compute"));
    assert!(source.contains("ease_out_bounce"));
}
