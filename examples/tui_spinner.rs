use animato::{KeyframeTrack, Loop, Update};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::Style;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{Terminal, TerminalOptions, Viewport};

fn main() {
    let frames = ["⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"];
    let mut track = frames
        .iter()
        .enumerate()
        .fold(KeyframeTrack::new(), |track, (index, _)| {
            track.push(index as f32 * 0.08, index as i32)
        })
        .looping(Loop::Forever);

    let mut stdout = std::io::stdout();
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::with_options(
        backend,
        TerminalOptions {
            viewport: Viewport::Inline(4),
        },
    )
    .unwrap();

    for frame in 0..64 {
        let index = track.value().unwrap_or(0).rem_euclid(frames.len() as i32) as usize;
        terminal
            .draw(|f| {
                let area = f.area();
                let rows =
                    Layout::vertical([Constraint::Length(3), Constraint::Length(1)]).split(area);
                let cols = Layout::horizontal([
                    Constraint::Min(1),
                    Constraint::Length(26),
                    Constraint::Min(1),
                ])
                .split(rows[0]);
                let widget = Paragraph::new(format!(" {} ", frames[index])).block(
                    Block::default()
                        .title(" Animato Spinner ")
                        .borders(Borders::ALL),
                );
                f.render_widget(widget, cols[1]);
                let info = Paragraph::new(format!("  frame {frame:03}   {}", frames[index]))
                    .style(Style::default().dim());
                f.render_widget(info, rows[1]);
            })
            .unwrap();
        track.update(0.08);
        std::thread::sleep(std::time::Duration::from_millis(80));
    }

    terminal.clear().unwrap();
    println!("✓ Done!");
}
