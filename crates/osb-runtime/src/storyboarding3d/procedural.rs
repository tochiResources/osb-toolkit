//! Procedural sprite generation service (section 5.12): given a source sprite and
//! its current projected state, compute the subdivision result and emit a
//! content-addressed manifest (and, when a source bitmap is available, the sliced
//! cell textures) to `outDir`. It regenerates on change / new input / cache-miss.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::math::Vec2;

use super::orientation::{ProjectionContext, SpriteProjection};
use super::subdivide::subdivide;

/// One generated sub-sprite (geometry + the `uv` the slicer uses).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedSprite {
    pub index: [u32; 2],
    pub uv: [[f64; 2]; 2],
    pub sprite_path: String,
    pub position: [f64; 2],
    pub scale: [f64; 2],
    pub rotation: f64,
}

/// The manifest written next to the generated sprites.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpriteManifest {
    pub cache_key: String,
    pub source_path: String,
    pub subdivided: bool,
    pub cells: Vec<GeneratedSprite>,
}

/// Options for the procedural sprite service.
#[derive(Debug, Clone)]
pub struct SpriteGenOptions {
    pub out_dir: PathBuf,
    pub anchor: (f64, f64),
    pub projection: SpriteProjection,
    pub bitmap_size: Vec2,
}

/// A stable content-addressed key for `(source, options, projected state)`.
pub fn cache_key(
    context: &ProjectionContext,
    source_path: &str,
    options: &SpriteGenOptions,
) -> String {
    let mut hasher = DefaultHasher::new();
    for value in context.wvp.m {
        value.to_bits().hash(&mut hasher);
    }
    for value in [
        context.sprite_scale.x,
        context.sprite_scale.y,
        context.world_scale.x,
        context.world_scale.y,
        context.world_scale.z,
        context.rotation_xyz.x,
        context.rotation_xyz.y,
        context.rotation_xyz.z,
        context.w,
        options.anchor.0,
        options.anchor.1,
        options.bitmap_size.x,
        options.bitmap_size.y,
    ] {
        value.to_bits().hash(&mut hasher);
    }
    source_path.hash(&mut hasher);
    format!("{:?}", options.projection).hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

/// Computes (or returns a cached) sub-sprite manifest, writing it to `outDir`.
/// Returns `(manifest, cache_hit)`.
pub fn generate(
    context: &ProjectionContext,
    source_path: &str,
    options: &SpriteGenOptions,
) -> std::io::Result<(SpriteManifest, bool)> {
    let key = cache_key(context, source_path, options);
    let dir = options.out_dir.join(&key);
    let manifest_path = dir.join("manifest.json");

    if let Ok(text) = osb_core::io::read_string(&manifest_path) {
        if let Ok(manifest) = serde_json::from_str::<SpriteManifest>(&text) {
            return Ok((manifest, true));
        }
    }

    let result = subdivide(
        context,
        options.anchor,
        options.projection,
        options.bitmap_size,
    );
    let cells = result
        .cells
        .iter()
        .map(|cell| GeneratedSprite {
            index: [cell.index.0, cell.index.1],
            uv: [[cell.uv.0.x, cell.uv.0.y], [cell.uv.1.x, cell.uv.1.y]],
            sprite_path: format!("{key}/cell_{}_{}.png", cell.index.0, cell.index.1),
            position: [cell.position.x, cell.position.y],
            scale: [cell.scale.x, cell.scale.y],
            rotation: cell.rotation,
        })
        .collect();

    let manifest = SpriteManifest {
        cache_key: key,
        source_path: source_path.to_string(),
        subdivided: result.subdivided,
        cells,
    };
    osb_core::io::create_dir_all(&dir)?;
    let text = serde_json::to_string_pretty(&manifest).unwrap_or_default();
    osb_core::io::write(&manifest_path, text)?;
    Ok((manifest, false))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{Quaternion, Vec3};
    use crate::storyboarding3d::camera::PerspectiveCamera;

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

    fn options(dir: PathBuf) -> SpriteGenOptions {
        SpriteGenOptions {
            out_dir: dir,
            anchor: (0.5, 0.5),
            projection: SpriteProjection::Grid {
                slices_x: 2,
                slices_y: 2,
            },
            bitmap_size: Vec2::new(64.0, 64.0),
        }
    }

    #[test]
    fn generates_and_then_hits_the_cache() {
        let dir = osb_core::io::temp_dir().join(format!("osb_gen_{}", std::process::id()));
        let _ = osb_core::io::remove_dir_all(&dir);
        let context = context();
        let options = options(dir.clone());

        let (first, hit1) = generate(&context, "sb/dot.png", &options).unwrap();
        assert!(!hit1);
        assert!(first.subdivided);
        assert_eq!(first.cells.len(), 4);

        let (second, hit2) = generate(&context, "sb/dot.png", &options).unwrap();
        assert!(hit2, "second call should hit the cache");
        assert_eq!(second.cache_key, first.cache_key);

        let _ = osb_core::io::remove_dir_all(&dir);
    }

    #[test]
    fn cache_key_changes_with_state() {
        let mut context = context();
        let options = options(osb_core::io::temp_dir());
        let key1 = cache_key(&context, "sb/dot.png", &options);
        context.w = 6.0;
        let key2 = cache_key(&context, "sb/dot.png", &options);
        assert_ne!(key1, key2);
    }
}
