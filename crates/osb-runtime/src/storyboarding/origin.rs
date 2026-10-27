//! Shared sprite-origin helpers (M32/M38).
//!
//! The anchor offset and the enum origin are one concept: a sprite's effective
//! pivot is the enum origin vector **plus** an animatable anchor offset (in
//! texture pixels). These helpers expose that resolution to every SDK, for both
//! 2D sprites and 3D nodes, and back the stateless `origin.*` bridge helpers.

use crate::math::{Vec2, Vec3};
use crate::storyboarding::OsbOrigin;
use crate::storyboarding::OsbSprite;

/// The anchor offset for an enum origin, in texture pixels
/// (`OsbSprite.GetOriginVector`).
pub fn origin_vector(origin: OsbOrigin, width: f64, height: f64) -> Vec2 {
    OsbSprite::get_origin_vector(origin, width, height)
}

/// The effective pivot: the enum origin vector plus an explicit anchor offset
/// (the 2D sprite anchor / 3D node pivot), in texture pixels.
pub fn resolve_anchor(origin: OsbOrigin, anchor: Vec2, width: f64, height: f64) -> Vec2 {
    origin_vector(origin, width, height) + anchor
}

/// The 3D effective pivot: the enum origin vector plus a `Vec3` anchor, where
/// the `z` component anchors the node's **depth** (its pivot z / z-index).
pub fn resolve_anchor_3d(origin: OsbOrigin, anchor: Vec3, width: f64, height: f64) -> Vec3 {
    let base = resolve_anchor(origin, Vec2::new(anchor.x, anchor.y), width, height);
    Vec3::new(base.x, base.y, anchor.z)
}

/// Whether a rotated sprite's bounds intersect the screen
/// (`OsbSprite.InScreenBounds`).
pub fn in_screen_bounds(position: Vec2, size: Vec2, rotation: f64, origin: Vec2) -> bool {
    OsbSprite::in_screen_bounds(position, size, rotation, origin)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_anchor_adds_the_offset() {
        let base = origin_vector(OsbOrigin::Centre, 100.0, 100.0);
        assert_eq!(base, Vec2::new(50.0, 50.0));
        let resolved = resolve_anchor(OsbOrigin::Centre, Vec2::new(10.0, -5.0), 100.0, 100.0);
        assert_eq!(resolved, Vec2::new(60.0, 45.0));
    }
}
