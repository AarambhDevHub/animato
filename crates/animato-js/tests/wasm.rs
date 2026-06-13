use animato_js::{RafDriver, Tween, available_easings, ease, init_animato, version};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn wasm_exports_core_animation_api() {
    init_animato();
    assert_eq!(version(), "1.5.1");
    assert!(available_easings().length() > 30);

    let tween = Tween::new(0.0, 100.0, 1.0);
    tween.set_easing("easeOutCubic").unwrap();
    tween.update(0.5);
    assert!(tween.value() > 50.0);
    assert!(ease("linear", 0.4).unwrap() == 0.4);
}

#[wasm_bindgen_test]
fn raf_driver_clamps_and_ticks() {
    let tween = Tween::new(0.0, 1.0, 1.0);
    let mut driver = RafDriver::new();
    let id = driver.add_tween(&tween);
    assert!(driver.is_active(id));
    assert_eq!(driver.tick(1000.0), 0.0);
    assert!(driver.tick(1500.0) <= 0.25);
    assert!(tween.value() > 0.0);
}
