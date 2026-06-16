use animato::{DevToolsState, DevToolsWebPanel};

fn main() {
    let mut state = DevToolsState::new();
    let panel = DevToolsWebPanel::with_shortcut("Ctrl+Shift+A");
    panel.toggle(&mut state);
    println!("{}", panel.render_summary(&state));
}
