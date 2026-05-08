//! Example: Spring physics — all 5 presets and a live position display.
//!
//! Run with:
//! ```sh
//! cargo run --example spring_demo
//! ```

use animato::{Clock, Spring, SpringConfig, Update, WallClock};

fn render_position(position: f32, target: f32, range: f32) {
    let width = 60usize;
    let normalized = ((position / range) * width as f32).round() as isize;
    let cursor = normalized.clamp(0, width as isize) as usize;

    let target_col = ((target / range) * width as f32).round() as usize;
    let target_col = target_col.min(width);

    let mut bar: Vec<char> = " ".repeat(width + 1).chars().collect();
    if target_col < bar.len() {
        bar[target_col] = '│';
    }
    if cursor < bar.len() {
        bar[cursor] = '●';
    }
    let bar_str: String = bar.into_iter().collect();
    print!("\r  |{bar_str}|  pos={position:7.2}  vel=");
    use std::io::Write;
    std::io::stdout().flush().unwrap();
}

fn run_preset(label: &str, config: SpringConfig, target: f32) {
    println!("  ┌─ {label}");
    println!(
        "  │  stiffness={:.0}  damping={:.0}  mass={:.1}",
        config.stiffness, config.damping, config.mass
    );
    print!("  │  ");

    let mut spring = Spring::new(config);
    spring.set_target(target);

    let mut clock = WallClock::new();
    let mut frames = 0usize;

    while !spring.is_settled() {
        let dt = clock.delta();
        spring.update(dt);
        render_position(spring.position(), target, 200.0);
        frames += 1;
        std::thread::sleep(std::time::Duration::from_millis(16));
    }

    println!(
        "\n  └─ Settled in ~{frames} frames  final={:.4}\n",
        spring.position()
    );
}

fn main() {
    println!("Animato v0.6.0 - spring_demo example");
    println!("  Target: 100.0  |  Legend: │=target  ●=position\n");

    let target = 100.0_f32;

    run_preset("gentle  (slow, soft)", SpringConfig::gentle(), target);
    run_preset("wobbly  (bouncy, playful)", SpringConfig::wobbly(), target);
    run_preset("stiff   (fast, firm)", SpringConfig::stiff(), target);
    run_preset("slow    (very lazy)", SpringConfig::slow(), target);
    run_preset("snappy  (near-instant)", SpringConfig::snappy(), target);

    // ── SpringN<[f32; 3]> demo ───────────────────────────────────────────────
    use animato::SpringN;
    println!("  ── SpringN<[f32; 3]> targeting [100, 200, 300] ──");
    let mut sn: SpringN<[f32; 3]> = SpringN::new(SpringConfig::stiff(), [0.0; 3]);
    sn.set_target([100.0, 200.0, 300.0]);

    let mut clock = WallClock::new();
    while !sn.is_settled() {
        sn.update(clock.delta());
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
    let p = sn.position();
    println!("  Settled: [{:.3}, {:.3}, {:.3}]\n", p[0], p[1], p[2]);
    println!("  ✓ All springs settled!");
}
