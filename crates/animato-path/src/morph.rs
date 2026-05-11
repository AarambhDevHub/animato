//! Shape morphing between two polylines.

use crate::math;
use alloc::vec::Vec;
use animato_core::Interpolate;

/// Uniformly resample a polyline to exactly `count` evenly-spaced points
/// measured by arc length.
///
/// # Parameters
/// - `points` — input polyline (at least 1 point)
/// - `count`  — desired number of output points (at least 1)
///
/// # Returns
/// A new `Vec` with exactly `count` points. If `points` is empty the output is
/// empty. If `points` has a single point, that point is repeated `count` times.
///
/// ```rust,ignore
/// use animato_path::morph::resample;
/// let square = vec![[0.0f32, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];
/// let resampled = resample(&square, 8);
/// assert_eq!(resampled.len(), 8);
/// ```
pub fn resample(points: &[[f32; 2]], count: usize) -> Vec<[f32; 2]> {
    if points.is_empty() || count == 0 {
        return Vec::new();
    }
    if points.len() == 1 || count == 1 {
        return alloc::vec![points[0]; count.max(1)];
    }

    // Build cumulative arc-length table.
    let mut cumulative = alloc::vec![0.0_f32; points.len()];
    for i in 1..points.len() {
        cumulative[i] = cumulative[i - 1] + math::distance(points[i - 1], points[i]);
    }
    let total = cumulative[points.len() - 1];

    if total <= f32::EPSILON {
        return alloc::vec![points[0]; count];
    }

    let mut result = Vec::with_capacity(count);
    result.push(points[0]);

    for i in 1..count.saturating_sub(1) {
        let target = total * i as f32 / (count - 1) as f32;
        let upper = cumulative.partition_point(|&l| l < target);
        let upper = upper.min(points.len() - 1);
        let lower = upper.saturating_sub(1);

        let point = if lower == upper {
            points[upper]
        } else {
            let span = (cumulative[upper] - cumulative[lower]).max(f32::EPSILON);
            let local_t = ((target - cumulative[lower]) / span).clamp(0.0, 1.0);
            math::lerp_point(points[lower], points[upper], local_t)
        };
        result.push(point);
    }

    result.push(*points.last().unwrap());
    result
}

/// A shape morph between two polylines.
///
/// Both shapes are resampled to the same point count during construction so
/// that every vertex has a clear correspondence for interpolation.
///
/// # Example
///
/// ```rust,ignore
/// use animato_path::MorphPath;
///
/// let square = vec![[0.0f32,0.0],[100.0,0.0],[100.0,100.0],[0.0,100.0]];
/// let circle = (0..=8)
///     .map(|i| {
///         let a = i as f32 * std::f32::consts::TAU / 8.0;
///         [50.0 + 50.0 * a.cos(), 50.0 + 50.0 * a.sin()]
///     })
///     .collect();
///
/// let morph = MorphPath::new(square, circle);
/// let midway = morph.evaluate(0.5);   // halfway between square and circle
/// assert_eq!(midway.len(), morph.point_count());
/// ```
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MorphPath {
    from: Vec<[f32; 2]>,
    to: Vec<[f32; 2]>,
}

impl MorphPath {
    /// Create a morph between `from` and `to`.
    ///
    /// Both shapes are resampled to `max(from.len(), to.len(), 2)` points.
    pub fn new(from: Vec<[f32; 2]>, to: Vec<[f32; 2]>) -> Self {
        let count = from.len().max(to.len()).max(2);
        Self::with_resolution(from, to, count)
    }

    /// Create a morph with an explicit resolution (minimum 2 points).
    pub fn with_resolution(from: Vec<[f32; 2]>, to: Vec<[f32; 2]>, resolution: usize) -> Self {
        let count = resolution.max(2);
        Self {
            from: resample(&from, count),
            to: resample(&to, count),
        }
    }

    /// Interpolated shape at `t` ∈ `[0.0, 1.0]`.
    ///
    /// - `t = 0.0` → `from` shape
    /// - `t = 1.0` → `to` shape
    pub fn evaluate(&self, t: f32) -> Vec<[f32; 2]> {
        let t = t.clamp(0.0, 1.0);
        self.from
            .iter()
            .zip(self.to.iter())
            .map(|(a, b)| a.lerp(b, t))
            .collect()
    }

    /// The `from` shape after resampling.
    pub fn from_shape(&self) -> &[[f32; 2]] {
        &self.from
    }

    /// The `to` shape after resampling.
    pub fn to_shape(&self) -> &[[f32; 2]] {
        &self.to
    }

    /// Number of points in each shape (they are always equal).
    pub fn point_count(&self) -> usize {
        self.from.len()
    }

    /// Bounding box `[min_x, min_y, max_x, max_y]` of the morphed shape at `t`.
    pub fn bounds_at(&self, t: f32) -> [f32; 4] {
        let shape = self.evaluate(t);
        if shape.is_empty() {
            return [0.0, 0.0, 0.0, 0.0];
        }
        let mut min_x = shape[0][0];
        let mut min_y = shape[0][1];
        let mut max_x = shape[0][0];
        let mut max_y = shape[0][1];
        for p in &shape[1..] {
            min_x = min_x.min(p[0]);
            min_y = min_y.min(p[1]);
            max_x = max_x.max(p[0]);
            max_y = max_y.max(p[1]);
        }
        [min_x, min_y, max_x, max_y]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn square(size: f32) -> Vec<[f32; 2]> {
        vec![
            [0.0, 0.0],
            [size, 0.0],
            [size, size],
            [0.0, size],
            [0.0, 0.0],
        ]
    }

    fn diamond(radius: f32) -> Vec<[f32; 2]> {
        vec![
            [radius, 0.0],
            [radius * 2.0, radius],
            [radius, radius * 2.0],
            [0.0, radius],
        ]
    }

    #[test]
    fn resample_preserves_endpoints() {
        let pts = square(100.0);
        let r = resample(&pts, 8);
        assert_eq!(r.len(), 8);
        assert_eq!(r[0], [0.0, 0.0]);
        assert_eq!(r[7], [0.0, 0.0]); // square starts and ends at origin
    }

    #[test]
    fn resample_single_point_repeated() {
        let pts = vec![[5.0_f32, 10.0]];
        let r = resample(&pts, 4);
        assert_eq!(r.len(), 4);
        assert!(r.iter().all(|&p| p == [5.0, 10.0]));
    }

    #[test]
    fn resample_count_one() {
        let pts = square(10.0);
        let r = resample(&pts, 1);
        assert_eq!(r.len(), 1);
        assert_eq!(r[0], pts[0]);
    }

    #[test]
    fn resample_empty_returns_empty() {
        let r = resample(&[], 5);
        assert!(r.is_empty());
    }

    #[test]
    fn morph_at_zero_is_from_shape() {
        let from = square(100.0);
        let to = diamond(100.0);
        let morph = MorphPath::new(from.clone(), to);
        let result = morph.evaluate(0.0);
        let expected = resample(&from, morph.point_count());
        for (a, b) in result.iter().zip(expected.iter()) {
            assert!((a[0] - b[0]).abs() < 0.001);
            assert!((a[1] - b[1]).abs() < 0.001);
        }
    }

    #[test]
    fn morph_at_one_is_to_shape() {
        let from = square(100.0);
        let to = diamond(100.0);
        let morph = MorphPath::new(from, to.clone());
        let result = morph.evaluate(1.0);
        let expected = resample(&to, morph.point_count());
        for (a, b) in result.iter().zip(expected.iter()) {
            assert!((a[0] - b[0]).abs() < 0.001);
            assert!((a[1] - b[1]).abs() < 0.001);
        }
    }

    #[test]
    fn morph_midpoint_is_interpolated() {
        let from = vec![[0.0_f32, 0.0], [100.0, 0.0]];
        let to = vec![[0.0_f32, 100.0], [100.0, 100.0]];
        let morph = MorphPath::with_resolution(from, to, 2);
        let mid = morph.evaluate(0.5);
        assert!((mid[0][1] - 50.0).abs() < 0.01);
        assert!((mid[1][1] - 50.0).abs() < 0.01);
    }

    #[test]
    fn morph_point_count_equals_larger_input() {
        let from = square(10.0); // 5 points
        let to = diamond(10.0); // 4 points
        let morph = MorphPath::new(from, to);
        assert_eq!(morph.point_count(), 5);
    }

    #[test]
    fn morph_with_resolution() {
        let from = square(50.0);
        let to = diamond(50.0);
        let morph = MorphPath::with_resolution(from, to, 16);
        assert_eq!(morph.point_count(), 16);
        let shape = morph.evaluate(0.5);
        assert_eq!(shape.len(), 16);
    }

    #[test]
    fn bounds_at_is_reasonable() {
        let from = vec![[0.0_f32, 0.0], [100.0, 0.0], [100.0, 100.0], [0.0, 100.0]];
        let to = from.clone();
        let morph = MorphPath::new(from, to);
        let bounds = morph.bounds_at(0.5);
        assert!(bounds[0] >= -0.01);
        assert!(bounds[1] >= -0.01);
        assert!(bounds[2] <= 100.01);
        assert!(bounds[3] <= 100.01);
    }
}
