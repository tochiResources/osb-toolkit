//! The strictly 3D constructors (`orbit`/`spin`/`tumble`/`pathFollow`) and the
//! 3D world / unified storyboard bake.

use serde_json::{json, Value};

use super::{call, channel_call, Channel};
use crate::error::Error;
use crate::models::Storyboard;

/// A circular orbit position channel (`Vec3`).
#[allow(clippy::too_many_arguments)]
pub fn orbit(
    center: [f64; 3],
    radius_x: f64,
    radius_y: f64,
    start_angle: f64,
    sweep: f64,
    count: usize,
) -> Result<Channel, Error> {
    channel_call(
        "scene3d.orbit",
        json!({ "center": center, "radiusX": radius_x, "radiusY": radius_y,
                "startAngle": start_angle, "sweep": sweep, "count": count }),
    )
}

/// A 2D circular orbit (`z = 0`).
pub fn orbit_2d(
    center: [f64; 2],
    radius_x: f64,
    radius_y: f64,
    start_angle: f64,
    sweep: f64,
    count: usize,
) -> Result<Channel, Error> {
    orbit(
        [center[0], center[1], 0.0],
        radius_x,
        radius_y,
        start_angle,
        sweep,
        count,
    )
}

/// A procedural spin about `axis`.
pub fn spin(axis: &str, turns: f64, duration: f64, count: usize) -> Result<Channel, Error> {
    channel_call(
        "scene3d.spin",
        json!({ "axis": axis, "turns": turns, "duration": duration, "count": count }),
    )
}

/// Independent spins about X, Y and Z.
pub fn tumble(turns: [f64; 3], duration: f64, count: usize) -> Result<Channel, Error> {
    channel_call(
        "scene3d.tumble",
        json!({ "turns": turns, "duration": duration, "count": count }),
    )
}

/// A position channel sampled along a polyline.
pub fn path_follow(
    points: Vec<[f64; 3]>,
    count: usize,
    by_arc_length: bool,
) -> Result<Channel, Error> {
    channel_call(
        "scene3d.pathFollow",
        json!({ "points": points, "count": count, "byArcLength": by_arc_length }),
    )
}

/// Bakes a 3D world (a `Bake3dInput`-shaped object: `{ nodes, camera, options }`)
/// into a full storyboard model.
pub fn bake_world(world: Value, options2d: Value) -> Result<Storyboard, Error> {
    Ok(serde_json::from_value(call(
        "scene3d.bakeWorld",
        json!({ "world": world, "options2d": options2d }),
    )?)?)
}

/// Bakes a set of 2D and 3D worlds into one storyboard model. `input` carries
/// `{ worlds2d, worlds3d, options, options2d }`.
pub fn bake_scene(input: Value) -> Result<Storyboard, Error> {
    Ok(serde_json::from_value(call("scene3d.bakeScene", input)?)?)
}
