//! The shared origin/anchor helpers.

use serde_json::json;

use crate::error::Error;
use crate::models::{OsbOrigin, Vector2, Vector3};

/// The anchor offset for an enum origin, in texture pixels.
pub fn origin_vector(origin: OsbOrigin, width: f64, height: f64) -> Result<Vector2, Error> {
    Ok(serde_json::from_value(super::call(
        "origin.getOriginVector",
        json!({ "origin": origin, "width": width, "height": height }),
    )?)?)
}

/// The effective pivot: the enum origin vector plus an explicit anchor offset.
pub fn resolve_anchor(
    origin: OsbOrigin,
    anchor: Vector2,
    width: f64,
    height: f64,
) -> Result<Vector2, Error> {
    Ok(serde_json::from_value(super::call(
        "origin.resolveAnchor",
        json!({ "origin": origin, "anchor": anchor, "width": width, "height": height }),
    )?)?)
}

/// The 3D effective pivot: a `Vector3` anchor whose `z` anchors the node's
/// **depth** (z-index) as well as its rotation/scale pivot.
pub fn resolve_anchor_3d(
    origin: OsbOrigin,
    anchor: Vector3,
    width: f64,
    height: f64,
) -> Result<Vector3, Error> {
    Ok(serde_json::from_value(super::call(
        "origin.resolveAnchor3d",
        json!({ "origin": origin, "anchor": anchor, "width": width, "height": height }),
    )?)?)
}

/// Whether a rotated sprite's bounds intersect the screen.
pub fn in_screen_bounds(
    position: Vector2,
    size: Vector2,
    rotation: f64,
    origin: Vector2,
) -> Result<bool, Error> {
    Ok(serde_json::from_value(super::call(
        "origin.inScreenBounds",
        json!({ "position": position, "size": size, "rotation": rotation, "origin": origin }),
    )?)?)
}
