use animato::{AnimationDriver, DevToolsState, DevToolsTuiPanel, TimelineInspector, Tween};

fn main() {
    let mut driver = AnimationDriver::new();
    driver.add_inspectable("progress", Tween::new(0.0_f32, 1.0).duration(1.0).build());
    driver.tick(0.25);

    let mut inspector = TimelineInspector::new();
    inspector.capture(&driver);

    let state = DevToolsState::new();
    let panel = DevToolsTuiPanel::new();
    println!("{}", panel.render_summary(&state));
    for snapshot in inspector.snapshots() {
        println!(
            "{} {} {:.0}%",
            snapshot.label.as_deref().unwrap_or("animation"),
            snapshot.progress_bar(16),
            snapshot.progress * 100.0
        );
    }
}
