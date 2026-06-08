//! Interpolatable value wrappers for advanced animation targets.

use crate::Interpolate;
use crate::math::{acos, cos, sin, sqrt};

const PI: f32 = core::f32::consts::PI;

/// An angle stored in degrees.
///
/// Interpolation follows the shortest angular path, so `359deg` to `1deg`
/// moves forward by two degrees instead of backward by 358 degrees.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Angle(pub f32);

impl Angle {
    /// Create an angle from degrees.
    #[inline]
    pub fn from_degrees(degrees: f32) -> Self {
        Self(degrees)
    }

    /// Create an angle from radians.
    #[inline]
    pub fn from_radians(radians: f32) -> Self {
        Self(radians * 180.0 / PI)
    }

    /// Return the angle in degrees.
    #[inline]
    pub fn degrees(self) -> f32 {
        self.0
    }

    /// Return the angle in radians.
    #[inline]
    pub fn radians(self) -> f32 {
        self.0 * PI / 180.0
    }

    /// Return an equivalent angle in `[0, 360)`.
    pub fn normalized(self) -> Self {
        let mut degrees = self.0 % 360.0;
        if degrees < 0.0 {
            degrees += 360.0;
        }
        Self(degrees)
    }
}

impl Interpolate for Angle {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        let delta = ((other.0 - self.0 + 540.0) % 360.0) - 180.0;
        Self(self.0 + delta * t)
    }
}

/// Unit quaternion used for 3D rotation interpolation.
///
/// The components are stored as `(x, y, z, w)`. [`Interpolate`] uses
/// shortest-path spherical linear interpolation.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Quaternion {
    /// X component of the vector part.
    pub x: f32,
    /// Y component of the vector part.
    pub y: f32,
    /// Z component of the vector part.
    pub z: f32,
    /// Scalar part.
    pub w: f32,
}

impl Quaternion {
    /// Identity rotation.
    pub const IDENTITY: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };

    /// Create a quaternion from components.
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }.normalized()
    }

    /// Create a quaternion from an axis and angle in degrees.
    pub fn from_axis_angle(axis: [f32; 3], angle: Angle) -> Self {
        let len = sqrt(axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]);
        if len <= f32::EPSILON {
            return Self::IDENTITY;
        }
        let half = angle.radians() * 0.5;
        let s = sin(half) / len;
        Self::new(axis[0] * s, axis[1] * s, axis[2] * s, cos(half))
    }

    /// Dot product between two quaternions.
    #[inline]
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    /// Magnitude of the quaternion.
    #[inline]
    pub fn length(self) -> f32 {
        sqrt(self.dot(self))
    }

    /// Return a normalized quaternion, or identity when length is invalid.
    pub fn normalized(self) -> Self {
        let len = self.length();
        if !len.is_finite() || len <= f32::EPSILON {
            return Self::IDENTITY;
        }
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
            w: self.w / len,
        }
    }

    /// Return the negated representation of the same rotation.
    #[inline]
    pub fn negated(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }

    /// Spherical linear interpolation using the shortest rotation path.
    pub fn slerp(self, other: Self, t: f32) -> Self {
        let a = self.normalized();
        let mut b = other.normalized();
        let mut dot = a.dot(b);
        if dot < 0.0 {
            b = b.negated();
            dot = -dot;
        }

        if dot > 0.9995 {
            return Self {
                x: a.x + (b.x - a.x) * t,
                y: a.y + (b.y - a.y) * t,
                z: a.z + (b.z - a.z) * t,
                w: a.w + (b.w - a.w) * t,
            }
            .normalized();
        }

        let theta_0 = acos(dot.clamp(-1.0, 1.0));
        let theta = theta_0 * t;
        let sin_theta = sin(theta);
        let sin_theta_0 = sin(theta_0);
        if sin_theta_0.abs() <= f32::EPSILON {
            return a;
        }
        let s0 = cos(theta) - dot * sin_theta / sin_theta_0;
        let s1 = sin_theta / sin_theta_0;
        Self {
            x: a.x * s0 + b.x * s1,
            y: a.y * s0 + b.y * s1,
            z: a.z * s0 + b.z * s1,
            w: a.w * s0 + b.w * s1,
        }
        .normalized()
    }

    /// Convert this rotation into a column-major 3x3 matrix.
    pub fn to_mat3(self) -> [f32; 9] {
        let q = self.normalized();
        let xx = q.x * q.x;
        let yy = q.y * q.y;
        let zz = q.z * q.z;
        let xy = q.x * q.y;
        let xz = q.x * q.z;
        let yz = q.y * q.z;
        let wx = q.w * q.x;
        let wy = q.w * q.y;
        let wz = q.w * q.z;

        [
            1.0 - 2.0 * (yy + zz),
            2.0 * (xy + wz),
            2.0 * (xz - wy),
            2.0 * (xy - wz),
            1.0 - 2.0 * (xx + zz),
            2.0 * (yz + wx),
            2.0 * (xz + wy),
            2.0 * (yz - wx),
            1.0 - 2.0 * (xx + yy),
        ]
    }
}

impl Interpolate for Quaternion {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        self.slerp(*other, t)
    }
}

/// A column-major 4x4 affine transform matrix.
///
/// Interpolation decomposes translation, scale, and rotation, interpolates each
/// component, then recomposes. Non-affine perspective terms are ignored.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mat4(pub [f32; 16]);

impl Mat4 {
    /// Identity matrix.
    pub const IDENTITY: Self = Self([
        1.0, 0.0, 0.0, 0.0, //
        0.0, 1.0, 0.0, 0.0, //
        0.0, 0.0, 1.0, 0.0, //
        0.0, 0.0, 0.0, 1.0,
    ]);

    /// Create a matrix from translation, rotation, and scale.
    pub fn from_translation_rotation_scale(
        translation: [f32; 3],
        rotation: Quaternion,
        scale: [f32; 3],
    ) -> Self {
        let r = rotation.to_mat3();
        Self([
            r[0] * scale[0],
            r[1] * scale[0],
            r[2] * scale[0],
            0.0,
            r[3] * scale[1],
            r[4] * scale[1],
            r[5] * scale[1],
            0.0,
            r[6] * scale[2],
            r[7] * scale[2],
            r[8] * scale[2],
            0.0,
            translation[0],
            translation[1],
            translation[2],
            1.0,
        ])
    }

    /// Extract translation from the affine matrix.
    #[inline]
    pub fn translation(self) -> [f32; 3] {
        [self.0[12], self.0[13], self.0[14]]
    }

    /// Extract approximate scale from the affine basis columns.
    pub fn scale(self) -> [f32; 3] {
        [
            column_len(&self.0, 0),
            column_len(&self.0, 1),
            column_len(&self.0, 2),
        ]
    }

    /// Decompose into translation, rotation, and scale.
    pub fn decompose(self) -> ([f32; 3], Quaternion, [f32; 3]) {
        let translation = self.translation();
        let scale = self.scale();
        let rotation = rotation_from_scaled_mat4(self.0, scale);
        (translation, rotation, scale)
    }
}

impl Interpolate for Mat4 {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        let (ta, ra, sa) = self.decompose();
        let (tb, rb, sb) = other.decompose();
        Self::from_translation_rotation_scale(ta.lerp(&tb, t), ra.slerp(rb, t), sa.lerp(&sb, t))
    }
}

/// A lightweight linear RGBA color value.
///
/// Perceptual color interpolation remains available through `animato-color`.
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Color(pub [f32; 4]);

impl Color {
    /// Create a linear RGBA color.
    #[inline]
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self([r, g, b, a])
    }

    /// Return the red component.
    #[inline]
    pub fn r(self) -> f32 {
        self.0[0]
    }

    /// Return the green component.
    #[inline]
    pub fn g(self) -> f32 {
        self.0[1]
    }

    /// Return the blue component.
    #[inline]
    pub fn b(self) -> f32 {
        self.0[2]
    }

    /// Return the alpha component.
    #[inline]
    pub fn a(self) -> f32 {
        self.0[3]
    }
}

impl Interpolate for Color {
    fn lerp(&self, other: &Self, t: f32) -> Self {
        Self(self.0.lerp(&other.0, t))
    }
}

fn column_len(m: &[f32; 16], col: usize) -> f32 {
    let i = col * 4;
    sqrt(m[i] * m[i] + m[i + 1] * m[i + 1] + m[i + 2] * m[i + 2])
}

fn rotation_from_scaled_mat4(m: [f32; 16], scale: [f32; 3]) -> Quaternion {
    let sx = if scale[0].abs() <= f32::EPSILON {
        1.0
    } else {
        scale[0]
    };
    let sy = if scale[1].abs() <= f32::EPSILON {
        1.0
    } else {
        scale[1]
    };
    let sz = if scale[2].abs() <= f32::EPSILON {
        1.0
    } else {
        scale[2]
    };

    let r00 = m[0] / sx;
    let r01 = m[4] / sy;
    let r02 = m[8] / sz;
    let r10 = m[1] / sx;
    let r11 = m[5] / sy;
    let r12 = m[9] / sz;
    let r20 = m[2] / sx;
    let r21 = m[6] / sy;
    let r22 = m[10] / sz;
    let trace = r00 + r11 + r22;

    if trace > 0.0 {
        let s = sqrt(trace + 1.0) * 2.0;
        return Quaternion::new((r21 - r12) / s, (r02 - r20) / s, (r10 - r01) / s, 0.25 * s);
    }
    if r00 > r11 && r00 > r22 {
        let s = sqrt(1.0 + r00 - r11 - r22) * 2.0;
        return Quaternion::new(0.25 * s, (r01 + r10) / s, (r02 + r20) / s, (r21 - r12) / s);
    }
    if r11 > r22 {
        let s = sqrt(1.0 + r11 - r00 - r22) * 2.0;
        return Quaternion::new((r01 + r10) / s, 0.25 * s, (r12 + r21) / s, (r02 - r20) / s);
    }
    let s = sqrt(1.0 + r22 - r00 - r11) * 2.0;
    Quaternion::new((r02 + r20) / s, (r12 + r21) / s, 0.25 * s, (r10 - r01) / s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn angle_uses_shortest_path() {
        let a = Angle::from_degrees(359.0);
        let b = Angle::from_degrees(1.0);
        assert_eq!(a.lerp(&b, 0.5).normalized().degrees(), 0.0);
    }

    #[test]
    fn quaternion_slerp_midpoint_is_normalized() {
        let a = Quaternion::IDENTITY;
        let b = Quaternion::from_axis_angle([0.0, 0.0, 1.0], Angle::from_degrees(180.0));
        let mid = a.lerp(&b, 0.5);
        assert!((mid.length() - 1.0).abs() < 0.0001);
        assert!((mid.w.abs() - 0.70710677).abs() < 0.0002);
    }

    #[test]
    fn mat4_interpolates_translation_scale_and_rotation() {
        let a = Mat4::IDENTITY;
        let b = Mat4::from_translation_rotation_scale(
            [10.0, 20.0, 30.0],
            Quaternion::from_axis_angle([0.0, 0.0, 1.0], Angle::from_degrees(90.0)),
            [2.0, 4.0, 6.0],
        );
        let mid = a.lerp(&b, 0.5);
        let (translation, rotation, scale) = mid.decompose();
        assert_eq!(translation, [5.0, 10.0, 15.0]);
        assert!((rotation.length() - 1.0).abs() < 0.0001);
        assert!((scale[0] - 1.5).abs() < 0.0001);
        assert!((scale[1] - 2.5).abs() < 0.0001);
        assert!((scale[2] - 3.5).abs() < 0.0001);
    }

    #[test]
    fn color_lerps_components() {
        let a = Color::rgba(0.0, 0.25, 0.5, 1.0);
        let b = Color::rgba(1.0, 0.75, 0.0, 0.5);
        assert_eq!(a.lerp(&b, 0.5), Color::rgba(0.5, 0.5, 0.25, 0.75));
    }
}
