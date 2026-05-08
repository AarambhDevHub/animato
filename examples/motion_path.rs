//! Example: animate a point along a motion path.
//!
//! Run with:
//! ```sh
//! cargo run --example motion_path --features path
//! ```

use animato::{CubicBezierCurve, Easing, MotionPathTween, Update};

fn main() {
    println!("Animato v0.4.0 - motion_path example\n");

    let curve = CubicBezierCurve::new([0.0, 0.0], [40.0, 90.0], [140.0, -90.0], [200.0, 0.0]);

    let mut motion = MotionPathTween::new(curve)
        .duration(1.0)
        .easing(Easing::EaseInOutSine)
        .auto_rotate(true)
        .build();

    while !motion.is_complete() {
        motion.update(1.0 / 12.0);
        let [x, y] = motion.value();
        let rotation = motion.rotation_deg();
        println!("  point=({x:7.2}, {y:7.2}) rotation={rotation:7.2}deg");
    }
}
