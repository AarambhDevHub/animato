//! Integration tests for v0.5.0 physics input APIs.

use animato::{
    DragConstraints, DragState, Gesture, GestureRecognizer, Inertia, InertiaBounds, InertiaConfig,
    PointerData, SwipeDirection, Update,
};

const DT: f32 = 1.0 / 60.0;

#[test]
fn facade_exports_inertia() {
    let mut inertia = Inertia::new(InertiaConfig::smooth());
    inertia.kick(500.0);
    assert!(inertia.update(DT));
    assert!(inertia.position() > 0.0);
}

#[test]
fn inertia_clamps_to_bounds_and_stops() {
    let config = InertiaConfig::smooth().with_bounds(InertiaBounds::new(0.0, 20.0));
    let mut inertia = Inertia::with_position(config, 10.0);
    inertia.kick(1200.0);

    for _ in 0..120 {
        if !inertia.update(DT) {
            break;
        }
    }

    assert_eq!(inertia.position(), 20.0);
    assert_eq!(inertia.velocity(), 0.0);
}

#[test]
fn drag_release_creates_bounded_inertia() {
    let mut drag = DragState::new([0.0, 0.0])
        .constraints(DragConstraints::bounded(0.0, 100.0, 0.0, 100.0))
        .velocity_smoothing(1.0);

    drag.on_pointer_down(PointerData::new(0.0, 0.0, 1));
    drag.on_pointer_move(PointerData::new(60.0, 40.0, 1), 0.016);

    let mut inertia = drag
        .on_pointer_up(PointerData::new(60.0, 40.0, 1))
        .expect("drag release should create inertia");

    for _ in 0..240 {
        if !inertia.update(DT) {
            break;
        }
    }

    let pos = inertia.position();
    assert!((0.0..=100.0).contains(&pos[0]));
    assert!((0.0..=100.0).contains(&pos[1]));
}

#[test]
fn recognizer_emits_swipe() {
    let mut recognizer = GestureRecognizer::default();
    recognizer.on_pointer_down(PointerData::new(0.0, 0.0, 1), 0.0);
    let gesture = recognizer.on_pointer_up(PointerData::new(-80.0, 0.0, 1), 0.2);

    assert!(matches!(
        gesture,
        Some(Gesture::Swipe {
            direction: SwipeDirection::Left,
            ..
        })
    ));
}

#[test]
fn recognizer_emits_pinch_and_rotation() {
    let mut pinch = GestureRecognizer::default();
    pinch.on_pointer_down(PointerData::new(0.0, 0.0, 1), 0.0);
    pinch.on_pointer_down(PointerData::new(10.0, 0.0, 2), 0.0);
    pinch.on_pointer_move(PointerData::new(20.0, 0.0, 2), 0.1);
    assert!(matches!(
        pinch.on_pointer_up(PointerData::new(0.0, 0.0, 1), 0.2),
        Some(Gesture::Pinch { scale, .. }) if (scale - 2.0).abs() < 0.01
    ));

    let mut rotation = GestureRecognizer::default();
    rotation.on_pointer_down(PointerData::new(0.0, 0.0, 1), 0.0);
    rotation.on_pointer_down(PointerData::new(10.0, 0.0, 2), 0.0);
    rotation.on_pointer_move(PointerData::new(0.0, 10.0, 2), 0.1);
    assert!(matches!(
        rotation.on_pointer_up(PointerData::new(0.0, 0.0, 1), 0.2),
        Some(Gesture::Rotation { angle_delta, .. }) if (angle_delta - 90.0).abs() < 0.01
    ));
}
