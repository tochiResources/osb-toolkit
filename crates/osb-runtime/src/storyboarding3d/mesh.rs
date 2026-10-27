//! Mesh primitives (section 5.11): line, triangle, quad/plane and indexed mesh,
//! projected to screenspace sprite states with near-plane clipping and backface
//! culling. osu! can only draw textured sprites, so a triangle is filled by two
//! affine sprites, and a mesh face is one affine sprite per
//! face.

use crate::math::{Matrix4, Vec2, Vec3, Vec4};

use super::camera::CameraState;
use super::clip::{in_front, to_clip};

const ORIGIN_TOP_LEFT: u8 = 0;
const ORIGIN_CENTRE_LEFT: u8 = 3;

/// A projected primitive, ready to become an `OsbSprite` command set.
#[derive(Debug, Clone, PartialEq)]
pub struct MeshSprite {
    pub origin: u8,
    pub flip_h: bool,
    pub position: Vec2,
    pub scale: Vec2,
    pub rotation: f64,
    pub opacity: f64,
    pub depth: f64,
    /// The sprite to render the face with (from its material), if known.
    pub sprite_path: Option<String>,
}

fn lerp4(a: Vec4, b: Vec4, t: f64) -> Vec4 {
    Vec4::new(
        a.x + (b.x - a.x) * t,
        a.y + (b.y - a.y) * t,
        a.z + (b.z - a.z) * t,
        a.w + (b.w - a.w) * t,
    )
}

/// The maximum-priority clip: keeps a point at or in front of the near plane.
fn clip_point(a: Vec4, b: Vec4, near: f64) -> (Vec4, Vec4) {
    let (mut a, mut b) = (a, b);
    if !in_front(&a, near) {
        let t = (near - a.w) / (b.w - a.w);
        a = lerp4(a, b, t);
    }
    if !in_front(&b, near) {
        let t = (near - b.w) / (a.w - b.w);
        b = lerp4(b, a, t);
    }
    (a, b)
}

/// A projected line/capsule primitive.
#[derive(Debug, Clone, Copy)]
pub struct LineSegment {
    pub start: Vec3,
    pub end: Vec3,
    pub thickness: f64,
}

impl LineSegment {
    /// Projects the line to a single sprite anchored at the start point.
    pub fn project(
        &self,
        camera: &CameraState,
        wvp: &Matrix4,
        bitmap_width: f64,
        opacity: f64,
    ) -> Option<MeshSprite> {
        let a = to_clip(wvp, self.start);
        let b = to_clip(wvp, self.end);
        let near = camera.near_clip;
        if !in_front(&a, near) && !in_front(&b, near) {
            return None;
        }
        let (a, b) = clip_point(a, b, near);
        let sa = camera.clip_to_screen(a);
        let sb = camera.clip_to_screen(b);
        let delta = Vec2::new(sb.x - sa.x, sb.y - sa.y);
        let length = delta.length();
        if length <= f64::EPSILON {
            return None;
        }
        Some(MeshSprite {
            origin: ORIGIN_CENTRE_LEFT,
            flip_h: false,
            position: Vec2::new(sa.x, sa.y),
            scale: Vec2::new(length / bitmap_width.max(1.0), self.thickness),
            rotation: delta.y.atan2(delta.x),
            opacity,
            depth: sa.z,
            sprite_path: None,
        })
    }
}

/// A projected quad (four corners in order), approximated by one affine sprite
/// anchored at the first corner.
#[derive(Debug, Clone, Copy)]
pub struct Quad {
    pub corners: [Vec3; 4],
}

impl Quad {
    pub fn project(
        &self,
        camera: &CameraState,
        wvp: &Matrix4,
        bitmap_size: Vec2,
        opacity: f64,
    ) -> Option<MeshSprite> {
        let clips: Vec<Vec4> = self.corners.iter().map(|c| to_clip(wvp, *c)).collect();
        if !clips.iter().any(|c| in_front(c, camera.near_clip)) {
            return None;
        }
        let s0 = camera.clip_to_screen(clips[0]);
        let s1 = camera.clip_to_screen(clips[1]);
        let s3 = camera.clip_to_screen(clips[3]);
        let a = Vec2::new(s1.x - s0.x, s1.y - s0.y);
        let b = Vec2::new(s3.x - s0.x, s3.y - s0.y);
        let (rotation, _sx, _sy) = free_decomposition(a, b);
        Some(MeshSprite {
            origin: ORIGIN_TOP_LEFT,
            flip_h: false,
            position: Vec2::new(s0.x, s0.y),
            scale: Vec2::new(
                a.length() / bitmap_size.x.max(1.0),
                b.length() / bitmap_size.y.max(1.0),
            ),
            rotation,
            opacity,
            depth: s0.z,
            sprite_path: None,
        })
    }
}

/// A filled triangle, drawn as two affine sprites.
#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub points: [Vec3; 3],
}

impl Triangle {
    /// Projects the triangle to two affine sprites. `mirrored` flips the
    /// backface-cull winding when the world transform has a negative determinant.
    pub fn project(
        &self,
        camera: &CameraState,
        wvp: &Matrix4,
        bitmap_size: Vec2,
        opacity: f64,
        mirrored: bool,
    ) -> Option<[MeshSprite; 2]> {
        let clips: Vec<Vec4> = self.points.iter().map(|c| to_clip(wvp, *c)).collect();
        if !clips.iter().all(|c| in_front(c, camera.near_clip)) {
            return None;
        }
        let s: Vec<Vec4> = clips.iter().map(|c| camera.clip_to_screen(*c)).collect();
        // Backface cull: a clockwise triangle faces away; a mirrored parent
        // reverses the winding.
        let cross = (s[2].x - s[0].x) * (s[1].y - s[0].y) - (s[2].y - s[0].y) * (s[1].x - s[0].x);
        let cull = if mirrored { cross < 0.0 } else { cross > 0.0 };
        if cull {
            return None;
        }
        // Base edge p0->p2, apex p1 projected onto it.
        let base = Vec2::new(s[2].x - s[0].x, s[2].y - s[0].y);
        let base_len = base.length();
        if base_len <= f64::EPSILON {
            return None;
        }
        let dir = Vec2::new(base.x / base_len, base.y / base_len);
        let to_apex = Vec2::new(s[1].x - s[0].x, s[1].y - s[0].y);
        let dot = dir.x * to_apex.x + dir.y * to_apex.y;
        let foot = Vec2::new(s[0].x + dir.x * dot, s[0].y + dir.y * dot);
        let apex_h = Vec2::new(s[1].x - foot.x, s[1].y - foot.y).length();
        let rotation = base.y.atan2(base.x);

        let sprite = |scale: Vec2, flip_h: bool| MeshSprite {
            origin: ORIGIN_TOP_LEFT,
            flip_h,
            position: foot,
            scale,
            rotation,
            opacity,
            depth: s[0].z,
            sprite_path: None,
        };
        Some([
            sprite(
                Vec2::new(
                    (base_len - dot) / bitmap_size.x.max(1.0),
                    apex_h / bitmap_size.y.max(1.0),
                ),
                false,
            ),
            sprite(
                Vec2::new(
                    dot / bitmap_size.x.max(1.0),
                    apex_h / bitmap_size.y.max(1.0),
                ),
                true,
            ),
        ])
    }
}

/// A face of a [`MeshData`] (3 or 4 vertex indices) and its optional material.
#[derive(Debug, Clone)]
pub struct Face {
    pub indices: Vec<u32>,
    /// Material name, resolved to a sprite path via [`MeshData::materials`].
    pub material: Option<String>,
}

impl Face {
    pub fn new(indices: Vec<u32>) -> Self {
        Face {
            indices,
            material: None,
        }
    }
}

/// An indexed mesh; each face projects to one affine sprite (triangles cull).
#[derive(Debug, Clone)]
pub struct MeshData {
    pub vertices: Vec<Vec3>,
    pub faces: Vec<Face>,
    pub bitmap_size: Vec2,
    /// Material name -> sprite path, used to resolve each face's `material`.
    pub materials: std::collections::BTreeMap<String, String>,
}

impl MeshData {
    pub fn project(
        &self,
        camera: &CameraState,
        wvp: &Matrix4,
        opacity: f64,
        mirrored: bool,
    ) -> Vec<MeshSprite> {
        let mut sprites = Vec::new();
        for face in &self.faces {
            let sprite_path = face
                .material
                .as_ref()
                .and_then(|name| self.materials.get(name))
                .cloned();
            let points: Vec<Vec3> = face
                .indices
                .iter()
                .filter_map(|i| self.vertices.get(*i as usize).copied())
                .collect();
            if points.len() == 3 {
                let triangle = Triangle {
                    points: [points[0], points[1], points[2]],
                };
                if let Some(pair) =
                    triangle.project(camera, wvp, self.bitmap_size, opacity, mirrored)
                {
                    for mut sprite in pair {
                        sprite.sprite_path = sprite_path.clone();
                        sprites.push(sprite);
                    }
                }
            } else if points.len() >= 4 {
                let quad = Quad {
                    corners: [points[0], points[1], points[2], points[3]],
                };
                if let Some(mut sprite) = quad.project(camera, wvp, self.bitmap_size, opacity) {
                    sprite.sprite_path = sprite_path;
                    sprites.push(sprite);
                }
            }
        }
        sprites
    }
}

fn free_decomposition(a: Vec2, b: Vec2) -> (f64, f64, f64) {
    let det = a.x * b.y - a.y * b.x;
    if b.length() >= a.length() {
        let rotation = b.y.atan2(b.x) - std::f64::consts::FRAC_PI_2;
        let sy = b.length();
        let sx = if sy.abs() > 1e-9 { det / sy } else { 0.0 };
        (rotation, sx, sy)
    } else {
        let rotation = a.y.atan2(a.x);
        let sx = a.length();
        let sy = if sx.abs() > 1e-9 { det / sx } else { 0.0 };
        (rotation, sx, sy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storyboarding3d::camera::PerspectiveCamera;

    fn camera() -> (CameraState, Matrix4) {
        let mut camera = PerspectiveCamera {
            resolution: Vec2::new(1366.0, 768.0),
            ..PerspectiveCamera::default()
        };
        camera.position_z.add(0.0, 5.0);
        let state = camera.state_at(0.0);
        (state, state.view_projection)
    }

    #[test]
    fn line_projects_to_a_rotated_sprite() {
        let (camera, wvp) = camera();
        let line = LineSegment {
            start: Vec3::new(-1.0, 0.0, 0.0),
            end: Vec3::new(1.0, 0.0, 0.0),
            thickness: 2.0,
        };
        let sprite = line.project(&camera, &wvp, 32.0, 1.0).unwrap();
        assert!(sprite.rotation.abs() < 1e-6);
        assert!(sprite.scale.x > 0.0);
        assert_eq!(sprite.origin, ORIGIN_CENTRE_LEFT);
    }

    #[test]
    fn line_behind_camera_is_culled() {
        let (camera, wvp) = camera();
        // The camera is at z=5 looking toward -z, so z=+10 is behind it.
        let line = LineSegment {
            start: Vec3::new(0.0, 0.0, 10.0),
            end: Vec3::new(1.0, 0.0, 10.0),
            thickness: 1.0,
        };
        assert!(line.project(&camera, &wvp, 32.0, 1.0).is_none());
    }

    #[test]
    fn quad_projects_and_mesh_culls_backfaces() {
        let (camera, wvp) = camera();
        let quad = Quad {
            corners: [
                Vec3::new(-1.0, -1.0, 0.0),
                Vec3::new(1.0, -1.0, 0.0),
                Vec3::new(1.0, 1.0, 0.0),
                Vec3::new(-1.0, 1.0, 0.0),
            ],
        };
        assert!(quad
            .project(&camera, &wvp, Vec2::new(64.0, 64.0), 1.0)
            .is_some());

        // A triangle facing away (clockwise) is culled.
        let triangle = Triangle {
            points: [
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
            ],
        };
        let front = triangle.project(&camera, &wvp, Vec2::new(64.0, 64.0), 1.0, false);
        let back = Triangle {
            points: [triangle.points[0], triangle.points[2], triangle.points[1]],
        }
        .project(&camera, &wvp, Vec2::new(64.0, 64.0), 1.0, false);
        assert!(front.is_some() ^ back.is_some(), "one winding is culled");
    }

    #[test]
    fn mirrored_parent_flips_the_cull_winding() {
        let (camera, wvp) = camera();
        let triangle = Triangle {
            points: [
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(1.0, 0.0, 0.0),
            ],
        };
        let normal = triangle.project(&camera, &wvp, Vec2::new(64.0, 64.0), 1.0, false);
        let mirrored = triangle.project(&camera, &wvp, Vec2::new(64.0, 64.0), 1.0, true);
        assert!(
            normal.is_some() ^ mirrored.is_some(),
            "mirror flips culling"
        );
    }

    #[test]
    fn material_resolves_to_a_sprite_path() {
        let (camera, wvp) = camera();
        let mut materials = std::collections::BTreeMap::new();
        materials.insert("skin".to_string(), "sb/skin.png".to_string());
        let mesh = MeshData {
            vertices: vec![
                Vec3::new(-1.0, -1.0, 0.0),
                Vec3::new(1.0, -1.0, 0.0),
                Vec3::new(1.0, 1.0, 0.0),
                Vec3::new(-1.0, 1.0, 0.0),
            ],
            faces: vec![Face {
                indices: vec![0, 1, 2, 3],
                material: Some("skin".into()),
            }],
            bitmap_size: Vec2::new(64.0, 64.0),
            materials,
        };
        let sprites = mesh.project(&camera, &wvp, 1.0, false);
        assert_eq!(sprites.len(), 1);
        assert_eq!(sprites[0].sprite_path.as_deref(), Some("sb/skin.png"));
    }
}
