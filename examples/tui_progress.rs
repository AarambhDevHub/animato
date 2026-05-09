use animato::{Easing, Tween, Update};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::Style;
use ratatui::widgets::{Block, Borders, Gauge, Paragraph};
use ratatui::{Terminal, TerminalOptions, Viewport};

fn main() {
    let mut tween = Tween::new(0.0_f32, 1.0)
        .duration(2.0)
        .easing(Easing::EaseOutCubic)
        .build();

    let mut stdout = std::io::stdout();
    let backend = CrosstermBackend::new(&mut stdout);
    let mut terminal = Terminal::with_options(
        backend,
        TerminalOptions {
            viewport: Viewport::Inline(4),
        },
    )
    .unwrap();

    for frame in 0..120 {
        let ratio = tween.value().clamp(0.0, 1.0);
        terminal
            .draw(|f| {
                let area = f.area();
                let rows =
                    Layout::vertical([Constraint::Length(3), Constraint::Length(1)]).split(area);
                let cols = Layout::horizontal([
                    Constraint::Min(1),
                    Constraint::Length(52),
                    Constraint::Min(1),
                ])
                .split(rows[0]);
                let gauge = Gauge::default()
                    .block(
                        Block::default()
                            .title(" Animato Progress ")
                            .borders(Borders::ALL),
                    )
                    .ratio(ratio as f64)
                    .label(format!("{:.1}%", ratio * 100.0));
                f.render_widget(gauge, cols[1]);
                let info = Paragraph::new(format!("  frame {frame:03}   {:.1}%", ratio * 100.0))
                    .style(Style::default().dim());
                f.render_widget(info, rows[1]);
            })
            .unwrap();
        tween.update(1.0 / 60.0);
        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    terminal.clear().unwrap();
    println!("✓ Done!");
}
