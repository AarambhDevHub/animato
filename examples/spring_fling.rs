use animato::{Spring, SpringConfig, Update};

fn main() {
    let mut spring =
        Spring::from_velocity(0.0, 850.0, 320.0, SpringConfig::underdamped(180.0, 0.65));

    for frame in 0..180 {
        if !spring.update(1.0 / 60.0) {
            break;
        }
        if frame % 12 == 0 {
            println!(
                "frame {frame:03}: x={:.2} velocity={:.2} energy={:.2} overshoots={}",
                spring.position(),
                spring.velocity(),
                spring.energy(),
                spring.overshoot_count()
            );
        }
    }
}
