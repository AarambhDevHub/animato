//! Integration test: motion path tween behavior.

use animato::{
    CompoundPath, Easing, LineSegment, MotionPath, MotionPathTween, PathCommand, PathEvaluate,
    Update,
};

#[test]
fn motion_path_tween_drives_position() {
    let line = LineSegment::new([0.0, 0.0], [100.0, 0.0]);
    let mut motion = MotionPathTween::new(line)
        .duration(1.0)
        .easing(Easing::Linear)
        .build();

    motion.update(0.5);
    assert_eq!(motion.value(), [50.0, 0.0]);

    motion.update(0.5);
    assert!(motion.is_complete());
    assert_eq!(motion.value(), [100.0, 0.0]);
}

#[test]
fn motion_offsets_trim_path() {
    let line = LineSegment::new([0.0, 0.0], [100.0, 0.0]);
    let mut motion = MotionPathTween::new(line)
        .duration(1.0)
        .start_offset(0.2)
        .end_offset(0.8)
        .build();

    assert_eq!(motion.value(), [20.0, 0.0]);
    motion.update(1.0);
    assert_eq!(motion.value(), [80.0, 0.0]);
}

#[test]
fn auto_rotate_aligns_to_tangent() {
    let line = LineSegment::new([0.0, 0.0], [0.0, 100.0]);
    let motion = MotionPathTween::new(line).auto_rotate(true).build();

    assert!((motion.rotation_deg() - 90.0).abs() < 0.001);
}

#[test]
fn motion_path_from_commands_uses_compound_lengths() {
    let commands = [
        PathCommand::MoveTo([0.0, 0.0]),
        PathCommand::LineTo([100.0, 0.0]),
        PathCommand::LineTo([100.0, 100.0]),
    ];
    let path = MotionPath::from_commands(&commands);

    assert_eq!(path.position(0.25), [50.0, 0.0]);
    assert_eq!(path.position(0.75), [100.0, 50.0]);
    assert_eq!(path.arc_length(), 200.0);
}

#[test]
fn compound_path_from_svg_is_usable_as_motion_path() {
    let compound = CompoundPath::try_from_svg("M0 0 L100 0 L100 100").unwrap();
    let path = MotionPath::from(compound);

    assert_eq!(path.position(0.5), [100.0, 0.0]);
}
