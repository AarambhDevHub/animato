use animato::{Update, Waveform};

fn main() {
    let sine = Waveform::Sine {
        frequency: 1.0,
        amplitude: 24.0,
        phase: 0.0,
    };
    let triangle = Waveform::Triangle {
        frequency: 0.5,
        amplitude: 12.0,
    };
    let mut track = sine.to_keyframe_track(1.0, 12.0);

    for frame in 0..=12 {
        let time = frame as f32 / 12.0;
        track.update(1.0 / 12.0);
        println!(
            "t={time:.2} sine={:.2} triangle={:.2} keyframe={:.2}",
            sine.sample(time),
            triangle.sample(time),
            track.value().unwrap_or_default()
        );
    }
}
