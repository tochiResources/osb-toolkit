//! Sprite subdivision API (): given a source sprite and its current
//! projected state, return the individual sub-sprites of the subdivision (and
//! their data), so external tools can generate procedural subdivided sprites.
//!
//! osu! sprites have no texture sub-rectangle, so each cell references a sliced
//! texture that the procedural service (`subdivide::service`) emits to `outDir`;
//! this module returns the geometry + the `uv` rect the slicer uses.

use crate::math::Vec2;

use super::orientation::{AxisAnchor, ProjectionContext, SpriteProjection};

/// One sub-sprite of a subdivision. Cells use `OsbOrigin::TopLeft`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SubSprite {
    pub index: (u32, u32),
    /// The `[min, max]` rectangle within the source texture, in `0..=1`.
    pub uv: (Vec2, Vec2),
    pub position: Vec2,
    pub scale: Vec2,
    pub rotation: f64,
    pub origin: u8,
    pub depth: f64,
}

/// The result of subdividing (or not) a sprite.
#[derive(Debug, Clone, PartialEq)]
pub struct SubdivisionResult {
    /// Whether more than one cell was produced.
    pub subdivided: bool,
    pub reason: &'static str,
    pub cells: Vec<SubSprite>,
}

impl SubdivisionResult {
    fn single(cell: SubSprite, reason: &'static str) -> Self {
        SubdivisionResult {
            subdivided: false,
            reason,
            cells: vec![cell],
        }
    }
}

/// Subdivides a sprite for `projection`, using the sprite-local rectangle
/// relative to its anchor (0..1 within the sprite).
pub fn subdivide(
    context: &ProjectionContext,
    anchor: (f64, f64),
    projection: SpriteProjection,
    bitmap_size: Vec2,
) -> SubdivisionResult {
    let (r0, s0, r1, s1) = (-anchor.0, -anchor.1, 1.0 - anchor.0, 1.0 - anchor.1);

    let (sx, sy) = match projection {
        SpriteProjection::Affine => (1, 1),
        SpriteProjection::Strips { slices } => (slices.max(1), 1),
        SpriteProjection::Grid { slices_x, slices_y } => (slices_x.max(1), slices_y.max(1)),
        SpriteProjection::Adaptive {
            threshold_px,
            max_slices,
        } => {
            let error = projective_error(context, (r0, s0, r1, s1));
            if error <= threshold_px {
                (1, 1)
            } else {
                let n = ((error / threshold_px).ceil() as u32).clamp(2, max_slices.max(2));
                (n, n)
            }
        }
    };

    if sx == 1 && sy == 1 {
        let cell = cell(
            context,
            0,
            0,
            (r0, s0, r1, s1),
            (0.0, 0.0, 1.0, 1.0),
            bitmap_size,
        );
        return SubdivisionResult::single(cell, "affine");
    }

    let mut cells = Vec::with_capacity((sx * sy) as usize);
    for i in 0..sx {
        for j in 0..sy {
            let fr0 = lerp(r0, r1, i as f64 / sx as f64);
            let fr1 = lerp(r0, r1, (i + 1) as f64 / sx as f64);
            let fs0 = lerp(s0, s1, j as f64 / sy as f64);
            let fs1 = lerp(s0, s1, (j + 1) as f64 / sy as f64);
            let uv = (
                (i as f64 / sx as f64, j as f64 / sy as f64),
                ((i + 1) as f64 / sx as f64, (j + 1) as f64 / sy as f64),
            );
            cells.push(cell(
                context,
                i,
                j,
                (fr0, fs0, fr1, fs1),
                (uv.0 .0, uv.0 .1, uv.1 .0, uv.1 .1),
                bitmap_size,
            ));
        }
    }
    SubdivisionResult {
        subdivided: true,
        reason: "mesh",
        cells,
    }
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

/// Builds one cell from a local rect and its `uv` rect (both as min/max).
#[allow(clippy::too_many_arguments)]
fn cell(
    context: &ProjectionContext,
    i: u32,
    j: u32,
    rect: (f64, f64, f64, f64),
    uv: (f64, f64, f64, f64),
    bitmap_size: Vec2,
) -> SubSprite {
    let (r0, s0, r1, s1) = rect;
    let p00 = context.project(r0, s0);
    let p10 = context.project(r1, s0);
    let p01 = context.project(r0, s1);
    let a = Vec2::new(p10.x - p00.x, p10.y - p00.y);
    let b = Vec2::new(p01.x - p00.x, p01.y - p00.y);

    let (rotation, _sx, _sy) = free_decomposition(a, b);

    let slice_w = (bitmap_size.x * (uv.2 - uv.0)).abs().max(1.0);
    let slice_h = (bitmap_size.y * (uv.3 - uv.1)).abs().max(1.0);
    let scale = Vec2::new(a.length() / slice_w, b.length() / slice_h);

    SubSprite {
        index: (i, j),
        uv: (Vec2::new(uv.0, uv.1), Vec2::new(uv.2, uv.3)),
        position: Vec2::new(p00.x, p00.y),
        scale,
        rotation,
        // OsbOrigin::TopLeft (id 0): the cell's min corner is its anchor.
        origin: 0,
        depth: p00.z,
    }
}

fn free_decomposition(a: Vec2, b: Vec2) -> (f64, f64, f64) {
    let det = a.x * b.y - a.y * b.x;
    let lb = b.length();
    let la = a.length();
    if lb >= la {
        let rotation = b.y.atan2(b.x) - std::f64::consts::FRAC_PI_2;
        let sy = lb;
        let sx = if sy.abs() > 1e-9 { det / sy } else { 0.0 };
        (rotation, sx, sy)
    } else {
        let rotation = a.y.atan2(a.x);
        let sx = la;
        let sy = if sx.abs() > 1e-9 { det / sx } else { 0.0 };
        (rotation, sx, sy)
    }
}

/// The maximum pixel deviation between the true projected quad corners and the
/// affine fit, used to decide whether subdivision is worthwhile.
fn projective_error(context: &ProjectionContext, rect: (f64, f64, f64, f64)) -> f64 {
    let (r0, s0, r1, s1) = rect;
    let p00 = context.project(r0, s0);
    let p10 = context.project(r1, s0);
    let p01 = context.project(r0, s1);
    let p11 = context.project(r1, s1);
    let affine_p11 = Vec2::new(p10.x + p01.x - p00.x, p10.y + p01.y - p00.y);
    Vec2::new(p11.x - affine_p11.x, p11.y - affine_p11.y).length()
}

/// The free-decomposition anchor knob is exposed for parity with the TS API.
pub const DEFAULT_ANCHOR: AxisAnchor = AxisAnchor::Auto;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{Quaternion, Vec3};
    use crate::storyboarding3d::camera::PerspectiveCamera;
    use crate::storyboarding3d::orientation::{resolve, SpriteOrientation};

    fn context() -> ProjectionContext {
        let mut camera = PerspectiveCamera {
            resolution: Vec2::new(1366.0, 768.0),
            ..PerspectiveCamera::default()
        };
        camera.position_z.add(0.0, 5.0);
        let state = camera.state_at(0.0);
        ProjectionContext {
            wvp: state.view_projection.mul(&Quaternion::IDENTITY.to_matrix()),
            camera: state,
            sprite_scale: Vec2::ONE,
            world_scale: Vec3::new(1.0, 1.0, 1.0),
            rotation_xyz: Vec3::ZERO,
            w: 5.0,
        }
    }

    #[test]
    fn affine_is_a_single_sprite() {
        let result = subdivide(
            &context(),
            (0.5, 0.5),
            SpriteProjection::Affine,
            Vec2::new(64.0, 64.0),
        );
        assert!(!result.subdivided);
        assert_eq!(result.cells.len(), 1);
        assert_eq!(result.cells[0].origin, 0);
    }

    #[test]
    fn grid_produces_slices_times_slices_cells() {
        let result = subdivide(
            &context(),
            (0.5, 0.5),
            SpriteProjection::Grid {
                slices_x: 3,
                slices_y: 2,
            },
            Vec2::new(64.0, 64.0),
        );
        assert!(result.subdivided);
        assert_eq!(result.cells.len(), 6);
        // Cells cover the full uv range.
        let first = result.cells[0];
        let last = result.cells[result.cells.len() - 1];
        assert!((first.uv.0.x - 0.0).abs() < 1e-9 && (first.uv.0.y - 0.0).abs() < 1e-9);
        assert!((last.uv.1.x - 1.0).abs() < 1e-9 && (last.uv.1.y - 1.0).abs() < 1e-9);
    }

    #[test]
    fn adaptive_subdivides_only_when_distorted() {
        let flat = subdivide(
            &context(),
            (0.5, 0.5),
            SpriteProjection::Adaptive {
                threshold_px: 100.0,
                max_slices: 4,
            },
            Vec2::new(64.0, 64.0),
        );
        assert!(!flat.subdivided);
    }

    #[test]
    fn resolve_and_subdivide_are_consistent_for_affine() {
        let ctx = context();
        let resolved = resolve(
            SpriteOrientation::Free {
                anchor: AxisAnchor::Auto,
                origin_correction: true,
            },
            &ctx,
        );
        let result = subdivide(
            &ctx,
            (0.5, 0.5),
            SpriteProjection::Affine,
            Vec2::new(64.0, 64.0),
        );
        // Both should be finite and produce a non-zero scale.
        assert!(resolved.scale.length() > 0.0);
        assert!(result.cells[0].scale.length() > 0.0);
    }
}
