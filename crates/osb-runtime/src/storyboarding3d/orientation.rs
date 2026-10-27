//! Sprite orientation & projection for the 3D -> 2D bake (section 5.10).
//!
//! Sprite 2D projection is split into two orthogonal knobs: [`SpriteOrientation`]
//! (how the sprite is oriented in screenspace - six algorithms) and
//! [`SpriteProjection`] (how perspective/trapezoid distortion is handled -
//! affine, strips, grid or adaptive). [`resolve`] returns the affine
//! rotation/scale for the non-mesh modes; [`super::subdivide`] produces the
//! per-cell sub-sprites for the mesh modes.

use std::f64::consts::FRAC_PI_2;

use crate::math::{Matrix4, Vec2, Vec3, Vec4};

use super::camera::CameraState;

const EPSILON: f64 = 1e-9;

/// The legacy screen-orientation axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LegacyAxis {
    Fixed,
    #[default]
    UnitX,
    UnitY,
}

/// Billboard taxonomy (Unity/Unreal/Godot/Blender-style).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BillboardKind {
    /// Always parallel to the screen (classic billboard).
    #[default]
    ScreenAligned,
    /// Faces the camera but keeps a world up.
    ViewAligned,
    /// World-Y locked (upright objects).
    AxisY,
    /// World-X locked.
    AxisX,
    /// Ignores node rotation; keeps the sprite's own screen rotation.
    Fixed,
}

/// Auto-flip controls for the auto-flip orientation mode.
#[derive(Debug, Clone, Copy)]
pub struct AutoFlipSettings {
    pub enabled: bool,
    /// Degrees past 90 in which the scale sign is flipped (the auto-flip threshold).
    pub threshold: f64,
}

impl Default for AutoFlipSettings {
    fn default() -> Self {
        AutoFlipSettings {
            enabled: true,
            threshold: 1.0,
        }
    }
}

/// Which projected local axis the free decomposition anchors on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AxisAnchor {
    #[default]
    Auto,
    X,
    Y,
}

/// How a sprite is oriented in screenspace - one of six selectable algorithms.
#[derive(Debug, Clone, Copy)]
pub enum SpriteOrientation {
    /// 1. The legacy behaviour (`atan2` on one projected axis + unsigned
    ///    `ExtractScale`), kept for backwards compatibility.
    Legacy(LegacyAxis),
    /// 2. Auto-flip: signed scale + foreshortening + automatic sign flip.
    AutoFlip {
        axis: LegacyAxis,
        auto_flip: AutoFlipSettings,
    },
    /// 3. Signed QR/Gram-Schmidt decomposition (no axis lockout, no reverse).
    Free {
        anchor: AxisAnchor,
        origin_correction: bool,
    },
    /// 4. Subdivision (`super::subdivide`).
    Mesh,
    /// 5. Billboard taxonomy.
    Billboard(BillboardKind),
    /// 6. Free with automatic adaptive upgrade to a mesh.
    Hybrid {
        upgrade_threshold_px: f64,
        max_slices: u32,
    },
}

impl Default for SpriteOrientation {
    fn default() -> Self {
        SpriteOrientation::Legacy(LegacyAxis::UnitX)
    }
}

/// How perspective/trapezoid distortion is handled.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum SpriteProjection {
    /// One sprite (affine fit).
    #[default]
    Affine,
    /// 1-D subdivision along one axis (cheap trapezoid for single-axis rotation).
    Strips { slices: u32 },
    /// NxM subdivision.
    Grid { slices_x: u32, slices_y: u32 },
    /// Quadtree subdivision driven by per-cell corner distortion.
    Adaptive { threshold_px: f64, max_slices: u32 },
}

/// Everything the orientation/projection algorithms need: the world-view-
/// projection matrix (which already includes the node's world transform, so it
/// maps a *local* point to the screen), the sprite's own scale and the camera.
#[derive(Debug, Clone, Copy)]
pub struct ProjectionContext {
    pub wvp: Matrix4,
    pub camera: CameraState,
    pub sprite_scale: Vec2,
    pub world_scale: Vec3,
    /// The node's Euler rotation in radians (for the auto-flip foreshortening).
    pub rotation_xyz: Vec3,
    /// The projected `w` at the sprite origin (camera distance).
    pub w: f64,
}

impl ProjectionContext {
    /// Projects a local `(r, s)` offset (in sprite-local units) to the screen.
    pub fn project(&self, r: f64, s: f64) -> Vec4 {
        self.camera.to_screen(
            &self.wvp,
            Vec3::new(self.sprite_scale.x * r, self.sprite_scale.y * s, 0.0),
        )
    }

    /// Projects a local `(r, s)` without the sprite's own scale.
    fn project_raw(&self, r: f64, s: f64) -> Vec4 {
        self.camera.to_screen(&self.wvp, Vec3::new(r, s, 0.0))
    }

    pub fn origin(&self) -> Vec2 {
        let p = self.project_raw(0.0, 0.0);
        Vec2::new(p.x, p.y)
    }

    /// The projected local X axis, sprite-scaled (screen px per unit cell).
    pub fn scaled_x(&self) -> Vec2 {
        difference(self.project(1.0, 0.0), self.project(0.0, 0.0))
    }

    pub fn scaled_y(&self) -> Vec2 {
        difference(self.project(0.0, 1.0), self.project(0.0, 0.0))
    }

    fn raw_x(&self) -> Vec2 {
        difference(self.project_raw(1.0, 0.0), self.project_raw(0.0, 0.0))
    }

    fn raw_y(&self) -> Vec2 {
        difference(self.project_raw(0.0, 1.0), self.project_raw(0.0, 0.0))
    }
}

fn difference(a: Vec4, b: Vec4) -> Vec2 {
    Vec2::new(a.x - b.x, a.y - b.y)
}

/// A resolved affine sprite state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResolvedSprite {
    pub rotation: f64,
    pub scale: Vec2,
}

/// Resolves the affine rotation/scale for the non-mesh orientation modes.
pub fn resolve(orientation: SpriteOrientation, context: &ProjectionContext) -> ResolvedSprite {
    match orientation {
        SpriteOrientation::Legacy(axis) => legacy(axis, context),
        SpriteOrientation::AutoFlip {
            axis,
            auto_flip: settings,
        } => auto_flip(axis, settings, context),
        SpriteOrientation::Free { anchor, .. } => free(anchor, context),
        SpriteOrientation::Mesh => free(AxisAnchor::Auto, context),
        SpriteOrientation::Billboard(kind) => billboard(kind, context),
        SpriteOrientation::Hybrid { .. } => free(AxisAnchor::Auto, context),
    }
}

fn legacy(axis: LegacyAxis, context: &ProjectionContext) -> ResolvedSprite {
    let raw_x = context.raw_x();
    let raw_y = context.raw_y();
    let rotation = match axis {
        LegacyAxis::Fixed => 0.0,
        LegacyAxis::UnitX => raw_x.y.atan2(raw_x.x),
        LegacyAxis::UnitY => raw_y.y.atan2(raw_y.x) - FRAC_PI_2,
    };
    let perspective = if context.w.abs() < EPSILON {
        0.0
    } else {
        context.camera.focus_distance / context.w
    };
    let scale = Vec2::new(
        context.sprite_scale.x
            * context.world_scale.x
            * perspective
            * context.camera.resolution_scale,
        context.sprite_scale.y
            * context.world_scale.y
            * perspective
            * context.camera.resolution_scale,
    );
    ResolvedSprite { rotation, scale }
}

fn auto_flip(
    axis: LegacyAxis,
    settings: AutoFlipSettings,
    context: &ProjectionContext,
) -> ResolvedSprite {
    let mut resolved = legacy(axis, context);

    // Foreshortening: a plane turned edge-on shrinks toward zero.
    let f = |degrees: f64| -> f64 {
        let a = degrees.rem_euclid(180.0);
        (((90.0 - a).abs()) / 90.0).clamp(0.0, 1.0)
    };
    let rot_y = context.rotation_xyz.y.to_degrees();
    let rot_x = context.rotation_xyz.x.to_degrees();
    resolved.scale.x *= f(rot_y);
    resolved.scale.y *= f(rot_x);

    if settings.enabled {
        let a = rot_y.rem_euclid(360.0);
        if (90.0 - settings.threshold..=270.0 + settings.threshold).contains(&a) {
            resolved.scale.x = -resolved.scale.x.abs();
        }
        let b = rot_x.rem_euclid(360.0);
        if (90.0 - settings.threshold..=270.0 + settings.threshold).contains(&b) {
            resolved.scale.y = -resolved.scale.y.abs();
        }
    }
    resolved
}

fn free(anchor: AxisAnchor, context: &ProjectionContext) -> ResolvedSprite {
    let a = context.scaled_x();
    let b = context.scaled_y();
    let det = a.x * b.y - a.y * b.x;
    let use_y = match anchor {
        AxisAnchor::X => false,
        AxisAnchor::Y => true,
        AxisAnchor::Auto => b.length() >= a.length(),
    };
    if use_y {
        let rotation = b.y.atan2(b.x) - FRAC_PI_2;
        let sy = b.length();
        let sx = if sy.abs() > EPSILON { det / sy } else { 0.0 };
        ResolvedSprite {
            rotation,
            scale: Vec2::new(sx, sy),
        }
    } else {
        let rotation = a.y.atan2(a.x);
        let sx = a.length();
        let sy = if sx.abs() > EPSILON { det / sx } else { 0.0 };
        ResolvedSprite {
            rotation,
            scale: Vec2::new(sx, sy),
        }
    }
}

fn billboard(kind: BillboardKind, context: &ProjectionContext) -> ResolvedSprite {
    match kind {
        BillboardKind::ScreenAligned => ResolvedSprite {
            rotation: 0.0,
            scale: free(AxisAnchor::Auto, context).scale,
        },
        BillboardKind::ViewAligned => ResolvedSprite {
            rotation: 0.0,
            scale: free(AxisAnchor::Auto, context).scale,
        },
        BillboardKind::AxisY => legacy(LegacyAxis::UnitX, context),
        BillboardKind::AxisX => legacy(LegacyAxis::UnitY, context),
        BillboardKind::Fixed => legacy(LegacyAxis::Fixed, context),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn context(rotation_y: f64) -> ProjectionContext {
        let camera = {
            let mut camera = super::super::camera::PerspectiveCamera {
                resolution: Vec2::new(1366.0, 768.0),
                ..super::super::camera::PerspectiveCamera::default()
            };
            camera.position_z.add(0.0, 5.0);
            camera.state_at(0.0)
        };
        let world = crate::math::Quaternion::from_euler_angles(0.0, rotation_y, 0.0).to_matrix();
        ProjectionContext {
            wvp: camera.view_projection.mul(&world),
            camera,
            sprite_scale: Vec2::ONE,
            world_scale: Vec3::new(1.0, 1.0, 1.0),
            rotation_xyz: Vec3::new(0.0, rotation_y, 0.0),
            w: 5.0,
        }
    }

    #[test]
    fn legacy_unitx_does_not_foreshorten_at_90_degrees() {
        let r0 = resolve(SpriteOrientation::Legacy(LegacyAxis::UnitX), &context(0.0));
        let r90 = resolve(
            SpriteOrientation::Legacy(LegacyAxis::UnitX),
            &context(FRAC_PI_2),
        );
        // Legacy scale is rotation-independent (the axis-lockout limitation).
        assert!((r90.scale.x - r0.scale.x).abs() < 1e-6);
        assert!(r90.scale.x.abs() > 0.1);
    }

    #[test]
    fn free_does_not_reverse_and_mirrors() {
        let r180 = resolve(
            SpriteOrientation::Free {
                anchor: AxisAnchor::Auto,
                origin_correction: true,
            },
            &context(std::f64::consts::PI),
        );
        // A plane seen from behind is a 180-degree rotation with positive scale.
        assert!((r180.rotation.abs() - std::f64::consts::PI).abs() < 0.05 || r180.scale.x < 0.0);
    }

    #[test]
    fn auto_flip_foreshortens_at_90_degrees() {
        let at_edge = resolve(
            SpriteOrientation::AutoFlip {
                axis: LegacyAxis::UnitX,
                auto_flip: AutoFlipSettings::default(),
            },
            &context(FRAC_PI_2),
        );
        assert!(at_edge.scale.x.abs() < 0.05);
    }
}
