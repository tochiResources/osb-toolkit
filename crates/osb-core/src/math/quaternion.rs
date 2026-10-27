use super::{Matrix4, Vec3};

// A unit quaternion, matching OpenTK's `Quaternion` layout (`xyz` vector, `w` scalar).
/// A unit quaternion (`xyz` vector, `w` scalar), matching OpenTK.
#[derive(
    Debug, Clone, Copy, PartialEq, Default, serde::Serialize, serde::Deserialize, utoipa::ToSchema,
)]
pub struct Quaternion {
    /// X component.
    pub x: f64,
    /// Y component.
    pub y: f64,
    /// Z component.
    pub z: f64,
    /// W (scalar) component.
    pub w: f64,
}

impl Quaternion {
    pub const IDENTITY: Quaternion = Quaternion {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    };

    pub const fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Quaternion { x, y, z, w }
    }

    /// Builds a quaternion from intrinsic X/Y/Z Euler angles in radians.
    pub fn from_euler_angles(x: f64, y: f64, z: f64) -> Self {
        let (sx, cx) = (x * 0.5).sin_cos();
        let (sy, cy) = (y * 0.5).sin_cos();
        let (sz, cz) = (z * 0.5).sin_cos();
        Quaternion::new(
            sx * cy * cz - cx * sy * sz,
            cx * sy * cz + sx * cy * sz,
            cx * cy * sz - sx * sy * cz,
            cx * cy * cz + sx * sy * sz,
        )
    }

    pub fn from_axis_angle(axis: Vec3, angle: f64) -> Self {
        let axis = axis.normalized();
        let (s, c) = (angle * 0.5).sin_cos();
        Quaternion::new(axis.x * s, axis.y * s, axis.z * s, c)
    }

    pub fn length(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalized(self) -> Quaternion {
        let len = self.length();
        if len == 0.0 {
            Quaternion::IDENTITY
        } else {
            Quaternion::new(self.x / len, self.y / len, self.z / len, self.w / len)
        }
    }

    pub fn to_matrix(self) -> Matrix4 {
        let q = self.normalized();
        let (x, y, z, w) = (q.x, q.y, q.z, q.w);
        let xx = x * x;
        let yy = y * y;
        let zz = z * z;
        let xy = x * y;
        let xz = x * z;
        let yz = y * z;
        let wx = w * x;
        let wy = w * y;
        let wz = w * z;
        let mut m = Matrix4::identity().m;
        m[0] = 1.0 - 2.0 * (yy + zz);
        m[1] = 2.0 * (xy + wz);
        m[2] = 2.0 * (xz - wy);
        m[4] = 2.0 * (xy - wz);
        m[5] = 1.0 - 2.0 * (xx + zz);
        m[6] = 2.0 * (yz + wx);
        m[8] = 2.0 * (xz + wy);
        m[9] = 2.0 * (yz - wx);
        m[10] = 1.0 - 2.0 * (xx + yy);
        Matrix4 { m }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Vec4;

    #[test]
    fn identity_rotation_is_identity_matrix() {
        let m = Quaternion::IDENTITY.to_matrix();
        assert_eq!(
            m.transform_vec4(Vec4::new(1.0, 2.0, 3.0, 1.0)),
            Vec4::new(1.0, 2.0, 3.0, 1.0)
        );
    }

    #[test]
    fn quarter_turn_about_z() {
        let q = Quaternion::from_euler_angles(0.0, 0.0, std::f64::consts::FRAC_PI_2);
        let m = q.to_matrix();
        let out = m.transform_vec4(Vec4::new(1.0, 0.0, 0.0, 1.0));
        assert!(out.x.abs() < 1e-5 && (out.y - 1.0).abs() < 1e-5);
    }
}
