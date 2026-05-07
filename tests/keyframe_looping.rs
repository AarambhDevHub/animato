//! Integration test: KeyframeTrack evaluation and looping.

use animato::{Easing, KeyframeTrack, Loop, Playable, Update};

#[test]
fn empty_keyframe_track_is_safe() {
    let track: KeyframeTrack<f32> = KeyframeTrack::new();
    assert!(track.value().is_none());
    assert!(track.value_at(1.0).is_none());
    assert!(track.is_complete());
}

#[test]
fn keyframes_interpolate_with_segment_easing() {
    let track = KeyframeTrack::new()
        .push_eased(0.0, 0.0_f32, Easing::EaseOutCubic)
        .push(1.0, 100.0);

    assert!(track.value_at(0.5).unwrap() > 50.0);
}

#[test]
fn forever_looping_track_remains_in_range() {
    let mut track = KeyframeTrack::new()
        .push(0.0, 0.0_f32)
        .push(1.0, 100.0)
        .looping(Loop::Forever);

    for _ in 0..1_000 {
        assert!(track.update(1.0 / 60.0));
        let value = track.value().unwrap();
        assert!((0.0..=100.0).contains(&value));
    }
}

#[test]
fn ping_pong_track_reverses_after_duration() {
    let mut track = KeyframeTrack::new()
        .push(0.0, 0.0_f32)
        .push(1.0, 100.0)
        .looping(Loop::PingPong);

    track.update(1.25);
    assert_eq!(track.value(), Some(75.0));
}

#[test]
fn times_loop_completes_after_requested_cycles() {
    let mut track = KeyframeTrack::new()
        .push(0.0, 0.0_f32)
        .push(1.0, 1.0)
        .looping(Loop::Times(3));

    assert!(track.update(2.5));
    assert!(!track.update(0.5));
    assert!(track.is_complete());
    assert_eq!(track.value(), Some(1.0));
}

#[test]
fn playable_seek_uses_full_playback_duration() {
    let mut track = KeyframeTrack::new()
        .push(0.0, 0.0_f32)
        .push(1.0, 100.0)
        .looping(Loop::Times(2));

    track.seek_to(0.75);
    assert_eq!(track.value(), Some(50.0));
}
