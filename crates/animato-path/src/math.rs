//! Internal math helpers for `animato-path`.

use core::f32::consts::PI;

#[cfg(feature = "std")]
#[inline]
pub(crate) fn sqrt(x: f32) -> f32 {
    x.sqrt()
}

#[cfg(not(feature = "std"))]
#[inline]
pub(crate) fn sqrt(x: f32) -> f32 {
    libm::sqrtf(x)
}

#[cfg(all(feature = "std", any(feature = "std", feature = "alloc")))]
#[inline]
pub(crate) fn sin(x: f32) -> f32 {
    x.sin()
}

#[cfg(all(not(feature = "std"), any(feature = "std", feature = "alloc")))]
#[inline]
pub(crate) fn sin(x: f32) -> f32 {
    libm::sinf(x)
}

#[cfg(all(feature = "std", any(feature = "std", feature = "alloc")))]
#[inline]
pub(crate) fn cos(x: f32) -> f32 {
    x.cos()
}

#[cfg(all(not(feature = "std"), any(feature = "std", feature = "alloc")))]
#[inline]
pub(crate) fn cos(x: f32) -> f32 {
    libm::cosf(x)
}

#[cfg(feature = "std")]
#[inline]
pub(crate) fn atan2(y: f32, x: f32) -> f32 {
    y.atan2(x)
}

#[cfg(not(feature = "std"))]
#[inline]
pub(crate) fn atan2(y: f32, x: f32) -> f32 {
    libm::atan2f(y, x)
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[inline]
pub(crate) fn deg_to_rad(deg: f32) -> f32 {
    deg * PI / 180.0
}

#[inline]
pub(crate) fn rad_to_deg(rad: f32) -> f32 {
    rad * 180.0 / PI
}

#[inline]
pub(crate) fn clamp01(t: f32) -> f32 {
    if t.is_nan() { 0.0 } else { t.clamp(0.0, 1.0) }
}

#[inline]
pub(crate) fn add(a: [f32; 2], b: [f32; 2]) -> [f32; 2] {
    [a[0] + b[0], a[1] + b[1]]
}

#[inline]
pub(crate) fn sub(a: [f32; 2], b: [f32; 2]) -> [f32; 2] {
    [a[0] - b[0], a[1] - b[1]]
}

#[inline]
pub(crate) fn scale(v: [f32; 2], s: f32) -> [f32; 2] {
    [v[0] * s, v[1] * s]
}

#[inline]
pub(crate) fn dot(a: [f32; 2], b: [f32; 2]) -> f32 {
    a[0] * b[0] + a[1] * b[1]
}

#[inline]
pub(crate) fn length(v: [f32; 2]) -> f32 {
    sqrt(dot(v, v))
}

#[inline]
pub(crate) fn distance(a: [f32; 2], b: [f32; 2]) -> f32 {
    length(sub(b, a))
}

#[inline]
pub(crate) fn normalize(v: [f32; 2]) -> [f32; 2] {
    let len = length(v);
    if len <= f32::EPSILON {
        [0.0, 0.0]
    } else {
        [v[0] / len, v[1] / len]
    }
}

#[inline]
pub(crate) fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

#[inline]
pub(crate) fn lerp_point(a: [f32; 2], b: [f32; 2], t: f32) -> [f32; 2] {
    [lerp(a[0], b[0], t), lerp(a[1], b[1], t)]
}

#[inline]
pub(crate) fn rotation_deg(tangent: [f32; 2]) -> f32 {
    if tangent == [0.0, 0.0] {
        0.0
    } else {
        rad_to_deg(atan2(tangent[1], tangent[0]))
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[inline]
pub(crate) fn angle_between(u: [f32; 2], v: [f32; 2]) -> f32 {
    let sign = if u[0] * v[1] - u[1] * v[0] < 0.0 {
        -1.0
    } else {
        1.0
    };
    let denom = (length(u) * length(v)).max(f32::EPSILON);
    let value = (dot(u, v) / denom).clamp(-1.0, 1.0);
    sign * atan2(sqrt((1.0 - value * value).max(0.0)), value)
}
