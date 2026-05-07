//! Multi-dimensional [`SpringN<T>`] using one [`Spring`] per component of `T`.

use crate::config::SpringConfig;
use crate::decompose::Decompose;
use crate::spring::Spring;
use alloc::vec::Vec;
use core::marker::PhantomData;
use motus_core::Update;

/// A multi-dimensional spring that animates any type that can be decomposed
/// into independent `f32` components (see the sealed `Decompose` trait).
///
/// Internally holds one [`Spring`] per component of `T` and reconstructs
/// the full value each frame.
///
/// Requires the `alloc` or `std` feature.
///
/// # Example
///
/// ```rust
/// use motus_spring::{SpringN, SpringConfig};
/// use motus_core::Update;
///
/// let mut spring: SpringN<[f32; 3]> = SpringN::new(SpringConfig::wobbly(), [0.0; 3]);
/// spring.set_target([100.0, 200.0, 300.0]);
///
/// while !spring.is_settled() {
///     spring.update(1.0 / 60.0);
/// }
/// let pos = spring.position();
/// assert!((pos[0] - 100.0).abs() < 0.01);
/// assert!((pos[1] - 200.0).abs() < 0.01);
/// assert!((pos[2] - 300.0).abs() < 0.01);
/// ```
#[derive(Debug)]
pub struct SpringN<T: Decompose> {
    components: Vec<Spring>,
    _marker: PhantomData<T>,
}

impl<T: Decompose> SpringN<T> {
    /// Create a new multi-dimensional spring at `initial` position.
    pub fn new(config: SpringConfig, initial: T) -> Self {
        let n = T::component_count();
        let mut buf = alloc::vec![0.0_f32; n];
        initial.write_components(&mut buf);

        let components = buf
            .iter()
            .map(|&pos| {
                let mut s = Spring::new(config.clone());
                s.snap_to(pos);
                s
            })
            .collect();

        Self {
            components,
            _marker: PhantomData,
        }
    }

    /// Set the target for all component springs simultaneously.
    pub fn set_target(&mut self, target: T) {
        let n = T::component_count();
        let mut buf = alloc::vec![0.0_f32; n];
        target.write_components(&mut buf);
        for (spring, &t) in self.components.iter_mut().zip(buf.iter()) {
            spring.set_target(t);
        }
    }

    /// Current position, reconstructed from component springs.
    pub fn position(&self) -> T {
        let values: Vec<f32> = self.components.iter().map(|s| s.position()).collect();
        T::from_components(&values)
    }

    /// `true` when all component springs have settled.
    pub fn is_settled(&self) -> bool {
        self.components.iter().all(|s| s.is_settled())
    }

    /// Teleport all components to `pos` instantly — velocity zeroed, target set to `pos`.
    pub fn snap_to(&mut self, pos: T) {
        let n = T::component_count();
        let mut buf = alloc::vec![0.0_f32; n];
        pos.write_components(&mut buf);
        for (spring, &p) in self.components.iter_mut().zip(buf.iter()) {
            spring.set_target(p);
            spring.snap_to(p);
        }
    }
}

impl<T: Decompose> Update for SpringN<T> {
    fn update(&mut self, dt: f32) -> bool {
        if self.is_settled() {
            return false;
        }
        for s in self.components.iter_mut() {
            s.update(dt);
        }
        !self.is_settled()
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Tests
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    const DT: f32 = 1.0 / 60.0;

    fn settle<T: Decompose>(spring: &mut SpringN<T>) {
        for _ in 0..10_000 {
            if !spring.update(DT) {
                break;
            }
        }
        assert!(spring.is_settled(), "SpringN did not settle");
    }

    #[test]
    fn spring_n_f32_settles() {
        let mut s: SpringN<f32> = SpringN::new(SpringConfig::stiff(), 0.0);
        s.set_target(100.0);
        settle(&mut s);
        assert!((s.position() - 100.0).abs() < 0.01);
    }

    #[test]
    fn spring_n_vec2_settles() {
        let mut s: SpringN<[f32; 2]> = SpringN::new(SpringConfig::wobbly(), [0.0; 2]);
        s.set_target([50.0, -50.0]);
        settle(&mut s);
        let pos = s.position();
        assert!((pos[0] - 50.0).abs() < 0.01);
        assert!((pos[1] - (-50.0)).abs() < 0.01);
    }

    #[test]
    fn spring_n_vec3_settles() {
        let mut s: SpringN<[f32; 3]> = SpringN::new(SpringConfig::stiff(), [0.0; 3]);
        s.set_target([100.0, 200.0, 300.0]);
        settle(&mut s);
        let pos = s.position();
        assert!((pos[0] - 100.0).abs() < 0.01);
        assert!((pos[1] - 200.0).abs() < 0.01);
        assert!((pos[2] - 300.0).abs() < 0.01);
    }

    #[test]
    fn spring_n_snap_to() {
        let mut s: SpringN<[f32; 2]> = SpringN::new(SpringConfig::default(), [0.0; 2]);
        s.snap_to([10.0, 20.0]);
        let pos = s.position();
        assert_eq!(pos[0], 10.0);
        assert_eq!(pos[1], 20.0);
        assert!(s.is_settled());
    }
}
