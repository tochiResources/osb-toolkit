use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::call;
use crate::error::Error;
use crate::models::{Color4, OsbOrigin, Storyboard, StoryboardCommand};

/// A 2D transform: each field is an optional keyframe channel (omitted fields
/// default to identity).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Scene2dTransform {
    /// Translation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<super::Channel>,
    /// Per-axis scale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale: Option<super::Channel>,
    /// Rotation in radians.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<super::Channel>,
    /// Opacity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<super::Channel>,
    /// Tint color.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<super::Channel>,
    /// Rotation/scale pivot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<super::Channel>,
}

/// A sprite leaf in the scene.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scene2dSprite {
    /// The texture path.
    pub texture_path: String,
    /// The sprite anchor origin.
    pub origin: OsbOrigin,
    /// Whether the sprite starts additive.
    #[serde(default)]
    pub additive: bool,
    /// The sprite's local transform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transform: Option<Scene2dTransform>,
    /// Extra non-transform commands.
    #[serde(default)]
    pub commands: Vec<StoryboardCommand>,
}

/// A node in the 2D scene tree.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scene2dNode {
    /// A label.
    #[serde(default)]
    pub name: String,
    /// The node's transform.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transform: Option<Scene2dTransform>,
    /// The sprite attached to this node.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sprite: Option<Scene2dSprite>,
    /// The node's children.
    #[serde(default)]
    pub children: Vec<Scene2dNode>,
    /// Whether children inherit this node's opacity.
    #[serde(default = "default_true")]
    pub inherits_opacity: bool,
    /// Whether children inherit this node's tint.
    #[serde(default = "default_true")]
    pub inherits_color: bool,
    /// Whether children inherit this node's transform.
    #[serde(default = "default_true")]
    pub inherits_transform: bool,
}

fn default_true() -> bool {
    true
}

/// A 2D scene graph.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Scene2d {
    /// The scene's root node.
    pub root: Scene2dNode,
}

/// Resolution options.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Scene2dOptions {
    /// Sample range start (0 with `endTime <= startTime` uses the natural range).
    pub start_time: f64,
    /// Sample range end.
    pub end_time: f64,
    /// Fixed sampling step in milliseconds.
    pub time_step: f64,
    /// Douglas-Peucker tolerance for position/scale.
    pub tolerance_2d: f64,
    /// Tolerance for rotation/opacity.
    pub tolerance_1d: f64,
    /// Decimal places.
    pub decimals: u32,
}

impl Default for Scene2dOptions {
    fn default() -> Self {
        Scene2dOptions {
            start_time: 0.0,
            end_time: 0.0,
            time_step: 16.0,
            tolerance_2d: 0.1,
            tolerance_1d: 0.001,
            decimals: 3,
        }
    }
}

/// A sprite resolved from the scene.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedSprite2d {
    /// The texture path.
    pub texture_path: String,
    /// The sprite anchor origin.
    pub origin: OsbOrigin,
    /// Whether the sprite is additive.
    pub additive: bool,
    /// The resolved commands.
    pub commands: Vec<StoryboardCommand>,
    /// The resolved time range.
    pub time_range: TimeRange,
}

/// A `[start, end]` time range.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TimeRange {
    /// The start time.
    pub start: f64,
    /// The end time.
    pub end: f64,
}

fn run_scene(name: &str, scene: &Scene2d, extra: Value) -> Result<Scene2d, Error> {
    let mut input = if extra.is_object() { extra } else { json!({}) };
    input
        .as_object_mut()
        .expect("object")
        .insert("scene".to_string(), serde_json::to_value(scene)?);
    Ok(serde_json::from_value(call(name, input)?)?)
}

/// Resolves the scene into per-sprite command sets.
pub fn resolve(scene: &Scene2d, options: &Scene2dOptions) -> Result<Vec<ResolvedSprite2d>, Error> {
    Ok(serde_json::from_value(call(
        "scene2d.resolve",
        json!({ "scene": scene, "options": options }),
    )?)?)
}

/// The scene's occupied `[start, end]` range.
pub fn time_range(scene: &Scene2d) -> Result<Option<(f64, f64)>, Error> {
    let value: Value = call("scene2d.timeRange", json!({ "scene": scene }))?;
    if value.is_null() {
        return Ok(None);
    }
    Ok(Some((
        value.get("start").and_then(Value::as_f64).unwrap_or(0.0),
        value.get("end").and_then(Value::as_f64).unwrap_or(0.0),
    )))
}

/// Translates the whole scene (children inherit).
pub fn translate(scene: &Scene2d, delta: [f64; 2]) -> Result<Scene2d, Error> {
    run_scene("scene2d.translate", scene, json!({ "delta": delta }))
}

/// Scales the whole scene about its origin.
pub fn scale(scene: &Scene2d, factor: f64) -> Result<Scene2d, Error> {
    run_scene("scene2d.scale", scene, json!({ "factor": factor }))
}

/// Rotates the whole scene (radians).
pub fn rotate(scene: &Scene2d, radians: f64) -> Result<Scene2d, Error> {
    run_scene("scene2d.rotate", scene, json!({ "radians": radians }))
}

/// Sets a constant opacity on the root (children inherit).
pub fn set_opacity(scene: &Scene2d, value: f64) -> Result<Scene2d, Error> {
    run_scene("scene2d.setOpacity", scene, json!({ "value": value }))
}

/// Sets a constant tint on the root (children inherit).
pub fn set_color(scene: &Scene2d, color: Color4) -> Result<Scene2d, Error> {
    run_scene("scene2d.setColor", scene, json!({ "color": color }))
}

/// Flattens the whole scene.
pub fn flatten(scene: &Scene2d, axis: Option<&str>, mode: &str) -> Result<Scene2d, Error> {
    run_scene(
        "scene2d.flatten",
        scene,
        json!({ "axis": axis, "mode": mode }),
    )
}

/// Re-eases the whole scene.
pub fn ease(scene: &Scene2d, easing: &str, samples: usize) -> Result<Scene2d, Error> {
    run_scene(
        "scene2d.ease",
        scene,
        json!({ "easing": easing, "samples": samples }),
    )
}

/// Shifts the whole scene in time.
pub fn shift(scene: &Scene2d, delta: f64) -> Result<Scene2d, Error> {
    run_scene("scene2d.shift", scene, json!({ "delta": delta }))
}

/// Scales the whole scene in time.
pub fn scale_time(scene: &Scene2d, factor: f64, anchor: f64) -> Result<Scene2d, Error> {
    run_scene(
        "scene2d.scaleTime",
        scene,
        json!({ "factor": factor, "anchor": anchor }),
    )
}

/// Reverses the whole scene in time.
pub fn reverse(scene: &Scene2d, start: f64, end: f64) -> Result<Scene2d, Error> {
    run_scene(
        "scene2d.reverse",
        scene,
        json!({ "start": start, "end": end }),
    )
}

/// Bakes the scene into a full storyboard model (writing each sprite onto the
/// osu! layer of the node it came from).
pub fn bake(scene: &Scene2d, options: &Scene2dOptions) -> Result<Storyboard, Error> {
    Ok(serde_json::from_value(call(
        "scene2d.bake",
        json!({ "scene": scene, "options": options }),
    )?)?)
}
