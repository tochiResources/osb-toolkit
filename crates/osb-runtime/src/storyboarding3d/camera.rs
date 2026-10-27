use crate::animations::KeyframedValue;
use crate::math::{MathHelper, Matrix4, Vec2, Vec3, Vec4};

/// A perspective camera with keyframed position, target, up, field of view and
/// clip planes.
#[derive(Debug, Clone)]
pub struct PerspectiveCamera {
    pub resolution: Vec2,
    pub position_x: KeyframedValue<f64>,
    pub position_y: KeyframedValue<f64>,
    pub position_z: KeyframedValue<f64>,
    pub target_position: KeyframedValue<Vec3>,
    pub up: KeyframedValue<Vec3>,
    pub near_clip: KeyframedValue<f64>,
    pub near_fade: KeyframedValue<f64>,
    pub far_fade: KeyframedValue<f64>,
    pub far_clip: KeyframedValue<f64>,
    pub horizontal_fov: KeyframedValue<f64>,
    pub vertical_fov: KeyframedValue<f64>,
    /// Camera roll about its view direction, in radians.
    pub roll: KeyframedValue<f64>,
    /// Depth-of-field focus distance in world units (projected `w`). Empty keeps
    /// the sprite sharp.
    pub dof_focus: KeyframedValue<f64>,
    /// Half-width of the sharp band around the focus distance, in world units.
    /// Empty or non-positive disables DOF.
    pub dof_range: KeyframedValue<f64>,
    /// Blur radius in pixels at the edge of the DOF falloff.
    pub dof_max_blur: KeyframedValue<f64>,
}

impl Default for PerspectiveCamera {
    fn default() -> Self {
        PerspectiveCamera {
            resolution: Vec2::new(1366.0, 768.0),
            position_x: KeyframedValue::new(),
            position_y: KeyframedValue::new(),
            position_z: KeyframedValue::new(),
            target_position: KeyframedValue::new(),
            up: KeyframedValue::new(),
            near_clip: KeyframedValue::new(),
            near_fade: KeyframedValue::new(),
            far_fade: KeyframedValue::new(),
            far_clip: KeyframedValue::new(),
            horizontal_fov: KeyframedValue::new(),
            vertical_fov: KeyframedValue::new(),
            roll: KeyframedValue::new(),
            dof_focus: KeyframedValue::new(),
            dof_range: KeyframedValue::new(),
            dof_max_blur: KeyframedValue::new(),
        }
    }
}

/// The sampled state of a camera at one time.
#[derive(Debug, Clone, Copy)]
pub struct CameraState {
    pub view_projection: Matrix4,
    pub aspect_ratio: f64,
    pub focus_distance: f64,
    pub resolution_scale: f64,
    pub near_clip: f64,
    pub near_fade: f64,
    pub far_fade: f64,
    pub far_clip: f64,
    pub dof_focus: f64,
    pub dof_range: f64,
    pub dof_max_blur: f64,
}

/// The depth-of-field contribution for a sprite at a given camera distance.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DofSample {
    /// Opacity multiplier in `0..=1`.
    pub opacity: f64,
    /// Approximate blur radius in pixels.
    pub blur_px: f64,
}

impl PerspectiveCamera {
    pub fn aspect_ratio(&self) -> f64 {
        self.resolution.x / self.resolution.y
    }

    pub fn resolution_scale(&self) -> f64 {
        crate::storyboarding3d::STORYBOARD_SIZE.1 / self.resolution.y
    }

    /// Camera distance that yields a horizontal field of view of `fov` degrees.
    pub fn distance_for_horizontal_fov(&self, fov: f64) -> f64 {
        self.resolution.x * 0.5 / MathHelper::degrees_to_radians(fov * 0.5).tan()
    }

    /// Camera distance that yields a vertical field of view of `fov` degrees.
    pub fn distance_for_vertical_fov(&self, fov: f64) -> f64 {
        self.resolution.y * 0.5 / MathHelper::degrees_to_radians(fov * 0.5).tan()
    }

    /// Horizontal field of view in degrees for a focal length and sensor width.
    pub fn horizontal_fov_from_focal_length(&self, focal_length: f64, sensor_width: f64) -> f64 {
        (sensor_width / (2.0 * focal_length)).atan().to_degrees() * 2.0
    }

    /// Samples the camera at `time` into a [`CameraState`].
    pub fn state_at(&self, time: f64) -> CameraState {
        let aspect = self.aspect_ratio();
        let camera_position = Vec3::new(
            self.position_x.value_at(time).unwrap_or(0.0),
            self.position_y.value_at(time).unwrap_or(0.0),
            self.position_z.value_at(time).unwrap_or(0.0),
        );
        let target_position = self.target_position.value_at(time).unwrap_or(Vec3::ZERO);
        let up = self
            .up
            .value_at(time)
            .unwrap_or(Vec3::new(0.0, 1.0, 0.0))
            .normalized();

        let fov_y = if !self.horizontal_fov.is_empty() {
            let fov_x =
                MathHelper::degrees_to_radians(self.horizontal_fov.value_at(time).unwrap_or(0.0));
            2.0 * (fov_x * 0.5).tan().atan2(aspect)
        } else if !self.vertical_fov.is_empty() {
            MathHelper::degrees_to_radians(self.vertical_fov.value_at(time).unwrap_or(0.0))
        } else {
            let distance = (camera_position - target_position).length().max(0.0001);
            2.0 * (self.resolution.y * 0.5 / distance).atan()
        };

        let focus_distance = self.resolution.y * 0.5 / (fov_y * 0.5).tan();
        let near_clip = if !self.near_clip.is_empty() {
            self.near_clip.value_at(time).unwrap_or(0.0)
        } else {
            (focus_distance * 0.5).min(1.0)
        };
        let far_clip = if !self.far_clip.is_empty() {
            self.far_clip.value_at(time).unwrap_or(0.0)
        } else {
            focus_distance * 1.5
        };
        let near_fade = if !self.near_fade.is_empty() {
            self.near_fade.value_at(time).unwrap_or(near_clip)
        } else {
            near_clip
        };
        let far_fade = if !self.far_fade.is_empty() {
            self.far_fade.value_at(time).unwrap_or(far_clip)
        } else {
            far_clip
        };

        let view = Matrix4::look_at(camera_position, target_position, up);
        // Roll rotates the camera about its view axis (camera space), so it is
        // applied after `look_at` (`roll.mul(view)` in the column convention).
        let roll = self.roll.value_at(time).unwrap_or(0.0);
        let view = if roll == 0.0 {
            view
        } else {
            Matrix4::rotation_z(roll).mul(&view)
        };
        let projection = Matrix4::perspective(fov_y, aspect, near_clip, far_clip);
        CameraState {
            // `X.mul(Y)` applies `Y` first (column convention), so the OpenTK
            // `view * projection` is `projection.mul(view)` here.
            view_projection: projection.mul(&view),
            aspect_ratio: aspect,
            focus_distance,
            resolution_scale: self.resolution_scale(),
            near_clip,
            near_fade,
            far_fade,
            far_clip,
            dof_focus: self.dof_focus.value_at(time).unwrap_or(focus_distance),
            dof_range: self.dof_range.value_at(time).unwrap_or(0.0),
            dof_max_blur: self.dof_max_blur.value_at(time).unwrap_or(0.0),
        }
    }
}

/// An orthographic (axonometric) camera: keyframed position/target/up plus a
/// keyframed view height in world units.
#[derive(Debug, Clone)]
pub struct OrthographicCamera {
    pub resolution: Vec2,
    pub position_x: KeyframedValue<f64>,
    pub position_y: KeyframedValue<f64>,
    pub position_z: KeyframedValue<f64>,
    pub target_position: KeyframedValue<Vec3>,
    pub up: KeyframedValue<Vec3>,
    /// The vertical extent of the view in world units.
    pub view_height: KeyframedValue<f64>,
    pub near_clip: KeyframedValue<f64>,
    pub far_clip: KeyframedValue<f64>,
    /// Camera roll about its view direction, in radians.
    pub roll: KeyframedValue<f64>,
    pub dof_focus: KeyframedValue<f64>,
    pub dof_range: KeyframedValue<f64>,
    pub dof_max_blur: KeyframedValue<f64>,
}

impl Default for OrthographicCamera {
    fn default() -> Self {
        OrthographicCamera {
            resolution: Vec2::new(1366.0, 768.0),
            position_x: KeyframedValue::new(),
            position_y: KeyframedValue::new(),
            position_z: KeyframedValue::new(),
            target_position: KeyframedValue::new(),
            up: KeyframedValue::new(),
            view_height: KeyframedValue::new(),
            near_clip: KeyframedValue::new(),
            far_clip: KeyframedValue::new(),
            roll: KeyframedValue::new(),
            dof_focus: KeyframedValue::new(),
            dof_range: KeyframedValue::new(),
            dof_max_blur: KeyframedValue::new(),
        }
    }
}

impl OrthographicCamera {
    pub fn resolution_scale(&self) -> f64 {
        crate::storyboarding3d::STORYBOARD_SIZE.1 / self.resolution.y
    }

    /// Camera distance that yields a horizontal field of view of `fov` degrees.
    pub fn distance_for_horizontal_fov(&self, fov: f64) -> f64 {
        self.resolution.x * 0.5 / MathHelper::degrees_to_radians(fov * 0.5).tan()
    }

    /// Camera distance that yields a vertical field of view of `fov` degrees.
    pub fn distance_for_vertical_fov(&self, fov: f64) -> f64 {
        self.resolution.y * 0.5 / MathHelper::degrees_to_radians(fov * 0.5).tan()
    }

    pub fn state_at(&self, time: f64) -> CameraState {
        let aspect = self.resolution.x / self.resolution.y;
        let view_height = self.view_height.value_at(time).unwrap_or(self.resolution.y);
        let half_h = view_height * 0.5;
        let half_w = half_h * aspect;
        let camera_position = Vec3::new(
            self.position_x.value_at(time).unwrap_or(0.0),
            self.position_y.value_at(time).unwrap_or(0.0),
            self.position_z.value_at(time).unwrap_or(0.0),
        );
        let target_position = self.target_position.value_at(time).unwrap_or(Vec3::ZERO);
        let up = self
            .up
            .value_at(time)
            .unwrap_or(Vec3::new(0.0, 1.0, 0.0))
            .normalized();
        let near = self.near_clip.value_at(time).unwrap_or(-1000.0);
        let far = self.far_clip.value_at(time).unwrap_or(1000.0);

        let view = Matrix4::look_at(camera_position, target_position, up);
        let roll = self.roll.value_at(time).unwrap_or(0.0);
        let view = if roll == 0.0 {
            view
        } else {
            Matrix4::rotation_z(roll).mul(&view)
        };
        let projection = Matrix4::orthographic(-half_w, half_w, -half_h, half_h, near, far);
        CameraState {
            view_projection: projection.mul(&view),
            aspect_ratio: aspect,
            // Screen px per world unit for the sprite scale at `w = 1`.
            focus_distance: self.resolution.y / view_height.max(1e-6),
            resolution_scale: self.resolution_scale(),
            near_clip: near,
            near_fade: near,
            far_fade: far,
            far_clip: far,
            dof_focus: self.dof_focus.value_at(time).unwrap_or(0.0),
            dof_range: self.dof_range.value_at(time).unwrap_or(0.0),
            dof_max_blur: self.dof_max_blur.value_at(time).unwrap_or(0.0),
        }
    }
}

impl CameraState {
    /// Projects `point` through `transform` to screenspace: returns
    /// `(x, y, depth, w)`.
    pub fn to_screen(&self, transform: &Matrix4, point: Vec3) -> Vec4 {
        self.clip_to_screen(transform.transform_vec4(Vec4::new(point.x, point.y, point.z, 1.0)))
    }

    /// Converts a clip-space point (the result of `wvp * (point, 1)`) to
    /// screenspace. Used by the mesh primitives after near-plane clipping.
    pub fn clip_to_screen(&self, transformed: Vec4) -> Vec4 {
        let storyboard = Vec2::new(
            crate::storyboarding3d::STORYBOARD_SIZE.0,
            crate::storyboarding3d::STORYBOARD_SIZE.1,
        );
        let scale = Vec2::new(storyboard.y * self.aspect_ratio, storyboard.y);
        let offset = (scale.x - storyboard.x) * 0.5;

        let w_abs = transformed.w.abs().max(f64::EPSILON);
        let ndc = Vec2::new(transformed.x / w_abs, transformed.y / w_abs);
        let screen = Vec2::new((ndc.x + 1.0) * 0.5 * scale.x, (ndc.y + 1.0) * 0.5 * scale.y);
        let depth = transformed.z / transformed.w;
        Vec4::new(screen.x - offset, screen.y, depth, transformed.w)
    }

    /// Distance-based fade.
    pub fn opacity_at(&self, distance: f64) -> f64 {
        if distance < self.near_fade {
            ((distance - self.near_clip) / (self.near_fade - self.near_clip)).clamp(0.0, 1.0)
        } else if distance > self.far_fade {
            ((self.far_clip - distance) / (self.far_clip - self.far_fade)).clamp(0.0, 1.0)
        } else {
            1.0
        }
    }

    /// Depth-of-field sample for a sprite at `distance` (world units, the
    /// projected `w`): sharp inside `focus +/- range`, then an opacity falloff and
    /// a blur radius that both grow to their maximum at `distance` one `range`
    /// beyond the band. Disabled (`range <= 0`) keeps the sprite sharp.
    pub fn dof_at(&self, distance: f64) -> DofSample {
        if self.dof_range <= 0.0 {
            return DofSample {
                opacity: 1.0,
                blur_px: 0.0,
            };
        }
        let t = ((distance - self.dof_focus).abs() / self.dof_range).clamp(0.0, 1.0);
        DofSample {
            opacity: 1.0 - t,
            blur_px: self.dof_max_blur * t,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Vec3;

    #[test]
    fn orthographic_projects_center_and_right_edge() {
        let mut camera = OrthographicCamera::default();
        camera.position_z.add(0.0, 5.0);
        let state = camera.state_at(0.0);
        let center = state.to_screen(&state.view_projection, Vec3::ZERO);
        assert!((center.x - 427.0).abs() < 0.5, "center.x = {}", center.x);
        assert!((center.y - 240.0).abs() < 0.5, "center.y = {}", center.y);

        let half_w = camera.resolution.x / camera.resolution.y * (camera.resolution.y * 0.5);
        let edge = state.to_screen(&state.view_projection, Vec3::new(half_w, 0.0, 0.0));
        assert!((edge.x - 854.0).abs() < 1.0, "edge.x = {}", edge.x);
    }

    #[test]
    fn roll_rotates_the_projected_axes() {
        let mut camera = PerspectiveCamera::default();
        camera.position_z.add(0.0, 5.0);
        camera.roll.add(0.0, std::f64::consts::FRAC_PI_2);
        let state = camera.state_at(0.0);
        // A point on the world +X axis rolls onto the screen Y axis.
        let x = state.to_screen(&state.view_projection, Vec3::new(100.0, 0.0, 0.0));
        let center = state.to_screen(&state.view_projection, Vec3::ZERO);
        assert!((x.x - center.x).abs() < 0.5, "x stays at centre: {}", x.x);
        assert!(
            (x.y - center.y).abs() > 50.0,
            "rolled x moves off centre: {} vs {}",
            x.y,
            center.y
        );
    }

    #[test]
    fn dof_disabled_is_sharp_and_enabled_falls_off() {
        let mut camera = PerspectiveCamera::default();
        camera.position_z.add(0.0, 5.0);
        let sharp = camera.state_at(0.0);
        assert_eq!(sharp.dof_at(5.0).opacity, 1.0);
        assert_eq!(sharp.dof_at(500.0).blur_px, 0.0);

        camera.dof_focus.add(0.0, 5.0);
        camera.dof_range.add(0.0, 2.0);
        camera.dof_max_blur.add(0.0, 8.0);
        let dof = camera.state_at(0.0);
        assert_eq!(dof.dof_at(5.0).opacity, 1.0);
        let edge = dof.dof_at(7.0);
        assert!(edge.opacity < 1.0 && edge.blur_px > 0.0);
    }

    #[test]
    fn fov_distance_and_focal_length_helpers_are_consistent() {
        let camera = PerspectiveCamera::default();
        // A 90 degree vertical FOV at 768px is a distance of half the height.
        let distance = camera.distance_for_vertical_fov(90.0);
        assert!((distance - 384.0).abs() < 1e-6, "distance = {}", distance);
        // 50mm on a 36mm sensor is ~39.6 degrees horizontal.
        let fov = camera.horizontal_fov_from_focal_length(50.0, 36.0);
        assert!((fov - 39.6).abs() < 0.1, "fov = {}", fov);
    }
}
