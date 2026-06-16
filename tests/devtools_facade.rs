use animato::{
    AnimationDriver, DevToolsState, Easing, EasingCurveEditor, PerformanceMonitor,
    RecorderControls, SpringConfig, SpringVisualizer, TimelineInspector, Tween,
};

#[test]
fn devtools_facade_exports_runtime_tools() {
    let mut driver = AnimationDriver::new();
    driver.add_inspectable(
        "opacity",
        Tween::new(0.0_f32, 1.0)
            .duration(1.0)
            .easing(Easing::EaseOutCubic)
            .build(),
    );
    driver.tick(0.5);

    let mut inspector = TimelineInspector::new();
    inspector.capture(&driver);
    assert_eq!(inspector.snapshots().len(), 1);
    assert_eq!(inspector.snapshots()[0].label.as_deref(), Some("opacity"));

    let mut easing = EasingCurveEditor::new(Easing::Linear);
    easing.set_control_points(0.1, 0.2, 0.3, 0.4);
    assert!(easing.copy_code().contains("CubicBezier"));

    let mut spring = SpringVisualizer::new(SpringConfig::snappy());
    spring.simulate(1.0, 1.0 / 60.0, 60);
    assert!(!spring.history.is_empty());

    let mut recorder = RecorderControls::new();
    recorder.start();
    recorder.record("x", 0.0, 0.0);
    recorder.record("x", 1.0, 1.0);
    assert_eq!(recorder.replay("x", 0.5), Some(0.5));

    let mut perf = PerformanceMonitor::new(4);
    perf.record_frame(1.0 / 60.0);
    assert!(perf.fps() > 0.0);

    let mut state = DevToolsState::new();
    state.toggle();
    assert!(!state.is_open());
}
