use animato_js::{
    EasingCurveEditor, PerformanceMonitor, RafDriver, SpringVisualizer, TimelineInspector, Tween,
    available_easings, ease, init_animato, version,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn wasm_exports_core_animation_api() {
    init_animato();
    assert_eq!(version(), "1.6.0");
    assert!(available_easings().length() > 30);

    let tween = Tween::new(0.0, 100.0, 1.0);
    tween.set_easing("easeOutCubic").unwrap();
    tween.update(0.5);
    assert!(tween.value() > 50.0);
    assert!(ease("linear", 0.4).unwrap() == 0.4);
}

#[wasm_bindgen_test]
fn raf_driver_clamps_and_ticks() {
    let tween = Tween::new(0.0, 1.0, 1.0);
    let mut driver = RafDriver::new();
    let id = driver.add_tween(&tween);
    assert!(driver.is_active(id));
    assert_eq!(driver.tick(1000.0), 0.0);
    assert!(driver.tick(1500.0) <= 0.25);
    assert!(tween.value() > 0.0);
}

#[wasm_bindgen_test]
fn devtools_exports_capture_driver_and_graph_data() {
    let tween = Tween::new(0.0, 1.0, 1.0);
    let mut driver = RafDriver::new();
    driver.add_tween(&tween);
    driver.tick(1000.0);
    driver.tick(1250.0);

    let mut inspector = TimelineInspector::new();
    inspector.capture_raf_driver(&driver);
    assert_eq!(inspector.snapshot_count(), 1);
    assert_eq!(inspector.snapshot_kind(0), "tween");
    assert!(inspector.snapshot_progress(0) > 0.0);

    let mut easing = EasingCurveEditor::new("easeOutCubic").unwrap();
    easing.set_sample_count(8);
    assert_eq!(easing.sample_points().length(), 16);

    let mut spring = SpringVisualizer::new();
    spring.simulate(1.0, 1.0 / 60.0, 30);
    assert!(spring.frame_count() > 0);

    let mut perf = PerformanceMonitor::new(4);
    perf.record_frame(1.0 / 60.0);
    assert!(perf.fps() > 0.0);
}
