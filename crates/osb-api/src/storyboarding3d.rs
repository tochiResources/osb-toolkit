//! Wire shapes for the 3D storyboard scene (`Scene3d`) and the 3D -> 2D bake,
//! mirroring `osb-runtime`'s `storyboarding3d::{input,scene,optimize}`.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::math::{Color4, Vector2};

/// A keyframed scalar.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Keyframe1 {
    /// The keyframe time in milliseconds.
    pub time: f64,
    /// The value.
    pub value: f64,
}

/// A keyframed 2D value.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Keyframe2 {
    /// The keyframe time in milliseconds.
    pub time: f64,
    /// The value `[x, y]`.
    pub value: [f64; 2],
}

/// A keyframed 3D value.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Keyframe3 {
    /// The keyframe time in milliseconds.
    pub time: f64,
    /// The value `[x, y, z]`.
    pub value: [f64; 3],
}

/// A node in a plain 3D scene tree.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Node3dInput {
    /// The sprite texture path.
    pub sprite: String,
    /// The sprite anchor origin id (`0..8`).
    #[serde(default)]
    pub origin: Option<u8>,
    /// Whether the sprite composites additively.
    #[serde(default)]
    pub additive: Option<bool>,
    /// World position keyframes.
    #[serde(default)]
    pub position: Vec<Keyframe3>,
    /// World scale keyframes.
    #[serde(default)]
    pub scale: Vec<Keyframe3>,
    /// Euler rotation keyframes in radians.
    #[serde(default)]
    pub rotation: Vec<Keyframe3>,
    /// The local pivot rotation/scale act about.
    #[serde(default)]
    pub pivot: Vec<Keyframe3>,
    /// Per-sprite scale keyframes.
    #[serde(default)]
    pub sprite_scale: Vec<Keyframe2>,
    /// Per-sprite rotation keyframes (radians).
    #[serde(default)]
    pub sprite_rotation: Vec<Keyframe1>,
    /// The legacy orientation axis (`fixed`, `unitX`, `unitY`).
    #[serde(default)]
    pub rotation_mode: Option<String>,
    /// Animation frame paths (empty = a still sprite).
    #[serde(default)]
    pub frames: Vec<String>,
    /// Delay between animation frames, in milliseconds.
    #[serde(default)]
    pub frame_delay: Option<f64>,
    /// Painter's-algorithm depth keyframes.
    #[serde(default)]
    pub z_depth: Vec<Keyframe1>,
    /// Whether the node inherits its parent's rotation.
    #[serde(default)]
    pub inherits_rotation: Option<bool>,
    /// Whether the node inherits its parent's depth.
    #[serde(default)]
    pub inherits_z_depth: Option<bool>,
    /// Child nodes.
    #[serde(default)]
    #[schema(no_recursion)]
    pub children: Vec<Node3dInput>,
}

/// A keyframed perspective camera.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct Camera3dInput {
    /// Camera position keyframes.
    #[serde(default)]
    pub position: Vec<Keyframe3>,
    /// Camera target keyframes.
    #[serde(default)]
    pub target: Vec<Keyframe3>,
    /// Camera up-vector keyframes.
    #[serde(default)]
    pub up: Vec<Keyframe3>,
    /// Vertical FOV keyframes (radians).
    #[serde(default)]
    pub vertical_fov: Vec<Keyframe1>,
    /// Horizontal FOV keyframes (radians).
    #[serde(default)]
    pub horizontal_fov: Vec<Keyframe1>,
    /// Near-clip keyframes.
    #[serde(default)]
    pub near_clip: Vec<Keyframe1>,
    /// Far-clip keyframes.
    #[serde(default)]
    pub far_clip: Vec<Keyframe1>,
    /// Camera roll keyframes (radians).
    #[serde(default)]
    pub roll: Vec<Keyframe1>,
    /// Depth-of-field focus keyframes.
    #[serde(default)]
    pub dof_focus: Vec<Keyframe1>,
    /// Depth-of-field range keyframes.
    #[serde(default)]
    pub dof_range: Vec<Keyframe1>,
    /// Depth-of-field max-blur keyframes.
    #[serde(default)]
    pub dof_max_blur: Vec<Keyframe1>,
    /// The output resolution `[width, height]`.
    #[serde(default)]
    pub resolution: Option<[f64; 2]>,
}

/// Bake options.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct BakeOptionsInput {
    /// Bake start time.
    #[serde(default)]
    pub start_time: Option<f64>,
    /// Bake end time.
    #[serde(default)]
    pub end_time: Option<f64>,
    /// Fixed time step (when adaptive sampling is off).
    #[serde(default)]
    pub time_step: Option<f64>,
    /// 2D simplification tolerance.
    #[serde(default)]
    pub tolerance_2d: Option<f64>,
    /// 1D simplification tolerance.
    #[serde(default)]
    pub tolerance_1d: Option<f64>,
    /// Decimal places in the output.
    #[serde(default)]
    pub decimals: Option<u32>,
    /// Bake with multiple threads.
    #[serde(default)]
    pub multithreaded: Option<bool>,
    /// Use adaptive time sampling (default).
    #[serde(default)]
    pub adaptive_sampling: Option<bool>,
    /// Adaptive-sampling pixel tolerance.
    #[serde(default)]
    pub adaptive_tolerance_px: Option<f64>,
    /// Sort sprites by depth.
    #[serde(default)]
    pub depth_sort: Option<bool>,
    /// Number of depth buckets.
    #[serde(default)]
    pub depth_buckets: Option<u8>,
}

/// A plain 3D scene: nodes, a camera and bake options.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema, Default)]
pub struct Scene3dInput {
    /// The root nodes.
    pub nodes: Vec<Node3dInput>,
    /// The camera.
    #[serde(default)]
    pub camera: Camera3dInput,
    /// The bake options.
    #[serde(default)]
    pub options: BakeOptionsInput,
}

/// The kind of a baked 2D command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum CommandKind {
    /// A move.
    Move,
    /// A per-axis scale.
    ScaleVec,
    /// A rotation.
    Rotate,
    /// A fade.
    Fade,
    /// A color.
    Color,
    /// An additive-blend toggle.
    Additive,
}

/// A baked command's endpoint value.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum BakedValue {
    /// A scalar value.
    Scalar(f64),
    /// A 2D value.
    Vector2(Vector2),
    /// A color.
    Color(Color4),
}

/// A linear command span produced by the bake.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BakedCommand {
    /// The command kind.
    pub kind: CommandKind,
    /// The span start time.
    pub start_time: f64,
    /// The span end time.
    pub end_time: f64,
    /// The start value.
    pub start: BakedValue,
    /// The end value.
    pub end: BakedValue,
}

/// A baked sprite: a texture plus its `.osb`-compatible command spans.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BakedSprite {
    /// The sprite texture path.
    pub sprite_path: String,
    /// The sprite anchor origin id (`0..8`).
    pub origin: u8,
    /// Whether the sprite is additive.
    pub additive: bool,
    /// The sprite's depth.
    pub depth: f64,
    /// The sprite's painter's-algorithm depth.
    pub z_depth: f64,
    /// The osu! layer bucket (`0..4`).
    pub layer: u8,
    /// The command spans.
    pub commands: Vec<BakedCommand>,
}

/// A baked storyboard: the 2D result of a 3D bake.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct BakedStoryboard {
    /// The baked sprites.
    pub sprites: Vec<BakedSprite>,
}
