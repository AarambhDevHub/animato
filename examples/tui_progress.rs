//! Example: render an animated ratatui progress bar from a tween.
//!
//! Run with:
//! ```sh
//! cargo run --example tui_progress
//! ```

use animato::{Easing, Tween, Update};
use ratatui::Terminal;
use ratatui::backend::TestBackend;
use ratatui::widgets::{Block, Borders, Gauge};

fn main() {
    println!("Animato v0.7.0 - tui_progress example\n");

    let backend = TestBackend::new(48, 3);
    let mut terminal = Terminal::new(backend).expect("TestBackend is infallible");
    let mut tween = Tween::new(0.0_f32, 1.0)
        .duration(1.0)
        .easing(Easing::EaseOutCubic)
        .build();

    for frame in 0..=10 {
        let ratio = tween.value().clamp(0.0, 1.0);
        terminal
            .draw(|f| {
                let gauge = Gauge::default()
                    .block(
                        Block::default()
                            .title("Animato progress")
                            .borders(Borders::ALL),
                    )
                    .ratio(ratio as f64);
                f.render_widget(gauge, f.area());
            })
            .expect("TestBackend draw is infallible");
        println!("frame {frame:02}: {:>5.1}%", ratio * 100.0);
        tween.update(0.1);
    }
}
