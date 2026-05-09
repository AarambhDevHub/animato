//! Integration tests for v0.7.0 WASM rAF driver exports.

use animato::{RafDriver, Tween};

#[test]
fn facade_exports_raf_driver() {
    let mut driver = RafDriver::new();
    let id = driver.add(Tween::new(0.0_f32, 1.0).duration(0.01).build());

    driver.tick(1000.0);
    assert!(driver.is_active(id));

    driver.tick(1020.0);
    assert!(!driver.is_active(id));
}

#[test]
fn raf_driver_pause_and_resume_are_facade_accessible() {
    let mut driver = RafDriver::new();
    driver.pause();
    assert!(driver.is_paused());
    driver.resume();
    assert!(!driver.is_paused());
}
