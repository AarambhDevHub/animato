//! Example: animate a ratatui spinner with a keyframe track.
//!
//! Run with:
//! ```sh
//! cargo run --example tui_spinner
//! ```

use animato::{KeyframeTrack, Loop, Update};
use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::widgets::{Block, Borders, Paragraph};

fn main() {
    println!("Animato v0.7.0 - tui_spinner example\n");

    let frames = ["⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"];
    let mut track = frames
        .iter()
        .enumerate()
        .fold(KeyframeTrack::new(), |track, (index, _)| {
            track.push(index as f32 * 0.08, index as i32)
        })
        .looping(Loop::Forever);

    let backend = TestBackend::new(24, 3);
    let mut terminal = Terminal::new(backend).expect("TestBackend is infallible");

    for frame in 0..16 {
        let index = track.value().unwrap_or(0).rem_euclid(frames.len() as i32) as usize;
        terminal
            .draw(|f| {
                let widget = Paragraph::new(frames[index]).block(
                    Block::default()
                        .title("Animato spinner")
                        .borders(Borders::ALL),
                );
                f.render_widget(widget, f.area());
            })
            .expect("TestBackend draw is infallible");
        println!("frame {frame:02}: {}", frames[index]);
        track.update(0.08);
    }
}
