// Compile-pass test: the motion alias.
use animato::{motion, Update};

#[allow(dead_code)]
fn motion_alias() {
    let mut tl = motion! {
        sequence {
            tween opacity: 0.0 => 1.0, duration: 0.3;
        }
    };
    tl.update(0.1);
}

fn main() {}
