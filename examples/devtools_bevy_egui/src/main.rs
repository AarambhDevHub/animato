use animato::{AnimationDriver, DevToolsEguiPanel, DevToolsState, TimelineInspector, Tween};

fn main() {
    let mut driver = AnimationDriver::new();
    driver.add_inspectable("bevy-panel", Tween::new(0.0_f32, 1.0).duration(1.0).build());
    driver.tick(0.5);

    let mut inspector = TimelineInspector::new();
    inspector.capture(&driver);

    let panel = DevToolsEguiPanel::new();
    let state = DevToolsState::new();
    println!("{}", panel.render_summary(&state));
    println!("snapshots={}", inspector.snapshots().len());
}
