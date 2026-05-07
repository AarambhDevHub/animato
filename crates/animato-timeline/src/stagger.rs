//! Stagger helper for offsetting many animations.

use crate::timeline::{At, Timeline};
use alloc::format;
use alloc::vec::Vec;
use animato_core::Playable;

/// Create a timeline where each animation starts `delay` seconds after the previous one.
///
/// Entries are labeled `item_0`, `item_1`, and so on.
pub fn stagger<A>(animations: Vec<A>, delay: f32) -> Timeline
where
    A: Playable + Send + 'static,
{
    let mut timeline = Timeline::new();
    let delay = delay.max(0.0);
    for (index, animation) in animations.into_iter().enumerate() {
        timeline = timeline.add(
            format!("item_{index}"),
            animation,
            At::Absolute(index as f32 * delay),
        );
    }
    timeline
}

#[cfg(test)]
mod tests {
    use super::*;
    use animato_core::Update;
    use animato_tween::Tween;

    #[test]
    fn stagger_offsets_entries() {
        let animations = alloc::vec![
            Tween::new(0.0_f32, 10.0).duration(1.0).build(),
            Tween::new(0.0_f32, 10.0).duration(1.0).build(),
            Tween::new(0.0_f32, 10.0).duration(1.0).build(),
        ];
        let mut timeline = stagger(animations, 0.25);
        timeline.play();
        timeline.update(0.5);

        assert_eq!(timeline.get::<Tween<f32>>("item_0").unwrap().value(), 5.0);
        assert_eq!(timeline.get::<Tween<f32>>("item_1").unwrap().value(), 2.5);
        assert_eq!(timeline.get::<Tween<f32>>("item_2").unwrap().value(), 0.0);
    }
}
