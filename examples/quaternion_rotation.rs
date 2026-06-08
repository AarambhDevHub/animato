use animato::{Angle, Quaternion, Tween};

fn main() {
    let start = Quaternion::IDENTITY;
    let end = Quaternion::from_axis_angle([0.0, 1.0, 0.0], Angle::from_degrees(180.0));
    let mut tween = Tween::new(start, end).duration(1.0).build();

    for frame in 0..=10 {
        tween.seek(frame as f32 / 10.0);
        let q = tween.value();
        println!(
            "progress={:.1} quaternion=[{:.3}, {:.3}, {:.3}, {:.3}]",
            tween.progress(),
            q.x,
            q.y,
            q.z,
            q.w
        );
    }
}
