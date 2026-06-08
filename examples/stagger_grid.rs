use animato::{GridOrigin, StaggerPattern};

fn main() {
    let pattern = StaggerPattern::Grid {
        cols: 4,
        rows: 3,
        origin: GridOrigin::Center,
        step: 0.08,
    };

    for row in 0..3 {
        for col in 0..4 {
            let index = row * 4 + col;
            print!("{:.2}s ", pattern.delay(index, 12));
        }
        println!();
    }
}
