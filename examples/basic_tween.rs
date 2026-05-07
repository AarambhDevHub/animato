//! Example: Animate a single f32 value from 0 → 100 and render a progress bar.
//!
//! Run with:
//! ```sh
//! cargo run --example basic_tween
//! ```

use motus::{AnimationDriver, Clock, Easing, Loop, Tween, Update, WallClock};

fn render_bar(value: f32, width: usize) {
    let filled = ((value / 100.0) * width as f32).round() as usize;
    let filled = filled.min(width);
    let bar: String = "█".repeat(filled) + &"░".repeat(width - filled);
    print!("\r  [{bar}] {value:6.2}%");
    // Flush without newline so the bar updates in-place
    use std::io::Write;
    std::io::stdout().flush().unwrap();
}

fn main() {
    println!("Motus v0.1.0 — basic_tween example");
    println!("  Animating 0 → 100 over 2.0s with EaseOutCubic\n");

    // ── Build a tween ────────────────────────────────────────────────────────
    let tween = Tween::new(0.0_f32, 100.0)
        .duration(2.0)
        .easing(Easing::EaseOutCubic)
        .delay(0.3)
        .build();

    // ── Register with a driver ───────────────────────────────────────────────
    let mut driver = AnimationDriver::new();
    let id = driver.add(tween);

    // ── We need to read the value back, so keep a separate tween too ─────────
    // In real code you'd store the tween yourself and pass a reference to a system.
    // Here we keep a second local tween just for display purposes:
    let mut display = Tween::new(0.0_f32, 100.0)
        .duration(2.0)
        .easing(Easing::EaseOutCubic)
        .delay(0.3)
        .build();

    let mut clock = WallClock::new();

    println!("  Delay: 0.3s  Duration: 2.0s  Easing: EaseOutCubic\n");

    while driver.is_active(id) {
        let dt = clock.delta();
        driver.tick(dt);
        display.update(dt);
        render_bar(display.value(), 40);
        std::thread::sleep(std::time::Duration::from_millis(16)); // ~60fps
    }
    render_bar(100.0, 40);
    println!("\n\n  ✓ Complete!\n");

    // ── Demonstrate Loop::PingPong ───────────────────────────────────────────
    println!("  PingPong demo — 3 cycles (0 → 100 → 0 × 3)");
    let mut ping = Tween::new(0.0_f32, 100.0)
        .duration(0.8)
        .easing(Easing::EaseInOutSine)
        .looping(Loop::Times(6)) // 6 half-cycles = 3 full PingPong cycles
        .build();

    let mut clock2 = WallClock::new();
    while !ping.is_complete() {
        let dt = clock2.delta();
        ping.update(dt);
        render_bar(ping.value(), 40);
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
    println!("\n\n  ✓ PingPong complete!\n");
}
