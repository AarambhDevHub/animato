//! Example: scroll-driven animation using ScrollDriver and ScrollClock.
//!
//! Run with:
//! ```sh
//! cargo run --example scroll_linked
//! ```

use animato::{Clock, Easing, ScrollClock, ScrollDriver, Tween, Update};

fn render_bar(value: f32, label: &str) {
    let width = 40usize;
    let filled = ((value.clamp(0.0, 100.0) / 100.0) * width as f32).round() as usize;
    let bar = "█".repeat(filled) + &"░".repeat(width - filled);
    println!("  {label:<12} [{bar}] {value:6.1}");
}

fn main() {
    println!("Animato v0.9.0 - scroll_linked example");
    println!("  Scroll range 0–1000 px drives three tweens\n");

    // ── ScrollDriver approach ─────────────────────────────────────────────────
    let fade = Tween::new(0.0_f32, 100.0)
        .duration(1.0)
        .easing(Easing::Linear)
        .build();
    let slide = Tween::new(0.0_f32, 300.0)
        .duration(1.0)
        .easing(Easing::EaseOutCubic)
        .build();

    // Keep local copies for reading.
    let mut fade_local = Tween::new(0.0_f32, 100.0)
        .duration(1.0)
        .easing(Easing::Linear)
        .build();
    let mut slide_local = Tween::new(0.0_f32, 300.0)
        .duration(1.0)
        .easing(Easing::EaseOutCubic)
        .build();

    let mut driver = ScrollDriver::new(0.0, 1000.0);
    driver.add(fade);
    driver.add(slide);

    println!("  ── ScrollDriver (progress-delta ticks) ──────────────────");
    for step in (0..=1000_u32).step_by(100) {
        let pos = step as f32;
        let progress = pos / 1000.0;
        // Update local tweens directly by scroll progress for display.
        fade_local.seek(progress);
        slide_local.seek(progress);
        driver.set_position(pos);
        println!(
            "  scroll={pos:5}  progress={:.2}  animations={}",
            driver.progress(),
            driver.animation_count()
        );
        render_bar(fade_local.value(), "fade");
        render_bar((slide_local.value() / 3.0).min(100.0), "slide");
        println!();
    }

    // ── ScrollClock approach ──────────────────────────────────────────────────
    println!("  ── ScrollClock (Clock-compatible) ───────────────────────");
    let mut clock = ScrollClock::new(0.0, 500.0);
    let mut anim = Tween::new(0.0_f32, 100.0)
        .duration(1.0)
        .easing(Easing::EaseInOutSine)
        .build();

    for pos in (0..=500_u32).step_by(50) {
        clock.set_scroll(pos as f32);
        let dt = clock.delta();
        anim.update(dt);
        render_bar(anim.value(), "clock anim");
    }

    println!("\n  ✓ Scroll demo complete!");
}
