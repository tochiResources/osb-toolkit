use super::{MathHelper, Vec3, Vec4};

// A 4x4 matrix stored column-major (OpenGL convention), matching the transform
// semantics OpenTK exposes to effect scripts.
/// A 4x4 matrix, column-major (OpenGL convention): element `m[col * 4 + row]`.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct Matrix4 {
    /// The 16 column-major elements.
    pub m: [f64; 16],
}

impl Default for Matrix4 {
    fn default() -> Self {
        Matrix4::identity()
    }
}

impl Matrix4 {
    pub const fn identity() -> Self {
        Matrix4 {
            m: [
                1.0, 0.0, 0.0, 0.0, //
                0.0, 1.0, 0.0, 0.0, //
                0.0, 0.0, 1.0, 0.0, //
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn translation(v: Vec3) -> Self {
        let mut m = Matrix4::identity().m;
        m[12] = v.x;
        m[13] = v.y;
        m[14] = v.z;
        Matrix4 { m }
    }

    pub fn scale(v: Vec3) -> Self {
        let mut m = Matrix4::identity().m;
        m[0] = v.x;
        m[5] = v.y;
        m[10] = v.z;
        Matrix4 { m }
    }

    pub fn rotation_x(angle: f64) -> Self {
        let (s, c) = angle.sin_cos();
        let mut m = Matrix4::identity().m;
        m[5] = c;
        m[6] = s;
        m[9] = -s;
        m[10] = c;
        Matrix4 { m }
    }

    pub fn rotation_y(angle: f64) -> Self {
        let (s, c) = angle.sin_cos();
        let mut m = Matrix4::identity().m;
        m[0] = c;
        m[2] = -s;
        m[8] = s;
        m[10] = c;
        Matrix4 { m }
    }

    pub fn rotation_z(angle: f64) -> Self {
        let (s, c) = angle.sin_cos();
        let mut m = Matrix4::identity().m;
        m[0] = c;
        m[1] = s;
        m[4] = -s;
        m[5] = c;
        Matrix4 { m }
    }

    /// Right-handed perspective projection (OpenGL clip space, `z` in `-1..=1`).
    pub fn perspective(fov_y: f64, aspect: f64, near: f64, far: f64) -> Self {
        let f = 1.0 / (fov_y * 0.5).tan();
        let range = 1.0 / (near - far);
        let mut m = [0.0f64; 16];
        m[0] = f / aspect;
        m[5] = f;
        m[10] = (near + far) * range;
        m[11] = -1.0;
        m[14] = 2.0 * near * far * range;
        Matrix4 { m }
    }

    /// Right-handed orthographic projection (GL clip space, `z` in `-1..=1`).
    pub fn orthographic(left: f64, right: f64, bottom: f64, top: f64, near: f64, far: f64) -> Self {
        let mut m = [0.0f64; 16];
        m[0] = 2.0 / (right - left);
        m[5] = 2.0 / (top - bottom);
        m[10] = -2.0 / (far - near);
        m[12] = -(right + left) / (right - left);
        m[13] = -(top + bottom) / (top - bottom);
        m[14] = -(far + near) / (far - near);
        m[15] = 1.0;
        Matrix4 { m }
    }

    /// Right-handed view matrix looking from `eye` towards `target`.
    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let forward = (target - eye).normalized();
        let side = forward.cross(up).normalized();
        let up2 = side.cross(forward);
        let mut m = [0.0f64; 16];
        m[0] = side.x;
        m[4] = side.y;
        m[8] = side.z;
        m[1] = up2.x;
        m[5] = up2.y;
        m[9] = up2.z;
        m[2] = -forward.x;
        m[6] = -forward.y;
        m[10] = -forward.z;
        m[12] = -side.dot(eye);
        m[13] = -up2.dot(eye);
        m[14] = forward.dot(eye);
        m[15] = 1.0;
        Matrix4 { m }
    }

    /// Returns `self * rhs` (apply `rhs` first to a column vector).
    pub fn mul(&self, rhs: &Matrix4) -> Matrix4 {
        let mut out = [0.0f64; 16];
        for col in 0..4 {
            for row in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self.m[k * 4 + row] * rhs.m[col * 4 + k];
                }
                out[col * 4 + row] = sum;
            }
        }
        Matrix4 { m: out }
    }

    pub fn transform_vec4(&self, v: Vec4) -> Vec4 {
        Vec4::new(
            self.m[0] * v.x + self.m[4] * v.y + self.m[8] * v.z + self.m[12] * v.w,
            self.m[1] * v.x + self.m[5] * v.y + self.m[9] * v.z + self.m[13] * v.w,
            self.m[2] * v.x + self.m[6] * v.y + self.m[10] * v.z + self.m[14] * v.w,
            self.m[3] * v.x + self.m[7] * v.y + self.m[11] * v.z + self.m[15] * v.w,
        )
    }

    /// Transforms a point and performs the perspective divide, returning clip
    /// space in `-1..=1` and the original `w`.
    pub fn project(&self, point: Vec3) -> (Vec3, f64) {
        let clip = self.transform_vec4(Vec4::new(point.x, point.y, point.z, 1.0));
        if clip.w.abs() < f64::EPSILON {
            return (Vec3::new(clip.x, clip.y, clip.z), clip.w);
        }
        (
            Vec3::new(clip.x / clip.w, clip.y / clip.w, clip.z / clip.w),
            clip.w,
        )
    }

    /// Builds a rotation matrix from intrinsic X/Y/Z Euler angles in radians.
    pub fn from_euler_xyz(x: f64, y: f64, z: f64) -> Self {
        Matrix4::rotation_z(z)
            .mul(&Matrix4::rotation_y(y))
            .mul(&Matrix4::rotation_x(x))
    }

    pub fn to_radians(degrees: f64) -> f64 {
        MathHelper::degrees_to_radians(degrees)
    }
}

impl std::ops::Mul<Vec4> for Matrix4 {
    type Output = Vec4;
    fn mul(self, rhs: Vec4) -> Vec4 {
        self.transform_vec4(rhs)
    }
}

impl std::ops::Mul for Matrix4 {
    type Output = Matrix4;
    fn mul(self, rhs: Matrix4) -> Matrix4 {
        Matrix4::mul(&self, &rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_is_neutral() {
        let id = Matrix4::identity();
        let v = Vec4::new(1.0, 2.0, 3.0, 1.0);
        assert_eq!(id.transform_vec4(v), v);
    }

    #[test]
    fn translation_moves_point() {
        let t = Matrix4::translation(Vec3::new(10.0, 20.0, 30.0));
        let out = t.transform_vec4(Vec4::new(1.0, 1.0, 1.0, 1.0));
        assert_eq!(out, Vec4::new(11.0, 21.0, 31.0, 1.0));
    }

    #[test]
    fn perspective_projects_center_to_origin() {
        let p = Matrix4::perspective(MathHelper::degrees_to_radians(90.0), 1.0, 1.0, 100.0);
        let (ndc, w) = p.project(Vec3::new(0.0, 0.0, -5.0));
        assert!(w > 0.0);
        assert!(ndc.x.abs() < 1e-5 && ndc.y.abs() < 1e-5);
    }

    #[test]
    fn look_at_orients_forward() {
        let view = Matrix4::look_at(
            Vec3::new(0.0, 0.0, 5.0),
            Vec3::ZERO,
            Vec3::new(0.0, 1.0, 0.0),
        );
        let out = view.transform_vec4(Vec4::new(0.0, 0.0, 0.0, 1.0));
        assert!((out.z + 5.0).abs() < 1e-4);
    }
}
