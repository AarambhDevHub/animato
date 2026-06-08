use animato::{AnimationGroup, At, Timeline, Tween, Update};

fn main() {
    let fade = Tween::new(0.0_f32, 1.0).duration(0.4).build();
    let slide = Tween::new(0.0_f32, 120.0).duration(0.8).build();
    let mut parallel = AnimationGroup::parallel(vec![fade, slide]);
    parallel.play();

    while parallel.update(1.0 / 30.0) {}
    println!("parallel group complete: {}", parallel.is_complete());

    let nested = Timeline::new().add(
        "pulse",
        Tween::new(1.0_f32, 1.25).duration(0.3).build(),
        At::Start,
    );
    let mut parent = Timeline::new().add_timeline("nested", nested, At::Start);
    parent.play();
    parent.update(0.15);
    println!("nested timeline progress: {:.2}", parent.progress());
}
