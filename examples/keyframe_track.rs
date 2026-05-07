//! Example: Multi-stop animation with KeyframeTrack.
//!
//! Run with:
//! ```sh
//! cargo run --example keyframe_track
//! ```

use animato::{Easing, KeyframeTrack, Loop, Update};

fn main() {
    println!("Animato v0.2.0 - keyframe_track example");
    println!("  Three keyframes, looping forever for one short preview\n");

    let mut track = KeyframeTrack::new()
        .push_eased(0.0, 0.0_f32, Easing::EaseOutCubic)
        .push_eased(0.6, 100.0, Easing::EaseInOutSine)
        .push(1.2, 40.0)
        .looping(Loop::Forever);

    for frame in 0..90 {
        track.update(1.0 / 60.0);
        let value = track.value().unwrap_or(0.0);
        println!("  frame {frame:02}: value={value:6.2}");
    }
}
