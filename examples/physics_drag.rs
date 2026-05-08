//! Example: drag release into bounded inertia.
//!
//! Run with:
//! ```sh
//! cargo run --example physics_drag --features physics
//! ```

use animato::{DragConstraints, DragState, PointerData, Update};

fn main() {
    println!("Animato v0.5.0 - physics_drag example");

    let mut drag = DragState::new([0.0, 0.0])
        .constraints(DragConstraints::bounded(0.0, 200.0, 0.0, 120.0))
        .velocity_smoothing(1.0);

    drag.on_pointer_down(PointerData::new(0.0, 0.0, 1));
    drag.on_pointer_move(PointerData::new(120.0, 50.0, 1), 1.0 / 60.0);
    println!(
        "drag position: [{:.1}, {:.1}] velocity: [{:.1}, {:.1}]",
        drag.position()[0],
        drag.position()[1],
        drag.velocity()[0],
        drag.velocity()[1]
    );

    let mut inertia = drag
        .on_pointer_up(PointerData::new(120.0, 50.0, 1))
        .expect("drag velocity should start inertia");

    for frame in 0..120 {
        let running = inertia.update(1.0 / 60.0);
        let position = inertia.position();
        println!(
            "frame {:03}: position [{:7.2}, {:7.2}]",
            frame, position[0], position[1]
        );
        if !running {
            break;
        }
    }
}
