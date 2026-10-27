//! Plain, serializable 3D input for the bake, shared by Rust, TypeScript and
//! C# via the golden-vector corpus (section 5.9 of the plan). It is the same
//! shape that the built-in 3D object parser (§5.12) and the procedural sprite
//! service consume.

use serde::{Deserialize, Serialize};

use crate::animations::keyframes::Channel;
use crate::animations::timeline::Timeline;
use crate::animations::{Easing, KeyframedValue};
use crate::math::{Color4, Vec2, Vec3};
use crate::storyboarding::OsbOrigin;

use super::camera::PerspectiveCamera;
use super::node::{Node3d, Object3d, RotationMode, Sprite3d, TransformTimeline3d};
use super::scene::{bake, BakeOptions, BakedStoryboard};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyframe3 {
    pub time: f64,
    pub value: [f64; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyframe2 {
    pub time: f64,
    pub value: [f64; 2],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyframe1 {
    pub time: f64,
    pub value: f64,
}

/// A sprite node in a plain 3D input tree.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NodeInput {
    pub sprite: String,
    #[serde(default)]
    pub origin: Option<u8>,
    #[serde(default)]
    pub additive: Option<bool>,
    #[serde(default)]
    pub position: Vec<Keyframe3>,
    #[serde(default)]
    pub scale: Vec<Keyframe3>,
    /// Euler rotation in radians (X/Y/Z).
    #[serde(default)]
    pub rotation: Vec<Keyframe3>,
    /// The local pivot rotation/scale act about.
    #[serde(default)]
    pub pivot: Vec<Keyframe3>,
    #[serde(default)]
    pub sprite_scale: Vec<Keyframe2>,
    #[serde(default)]
    pub sprite_rotation: Vec<Keyframe1>,
    #[serde(default)]
    pub rotation_mode: Option<String>,
    /// Frame sprite paths for an animation plane (empty = a still sprite).
    #[serde(default)]
    pub frames: Vec<String>,
    #[serde(default)]
    pub frame_delay: Option<f64>,
    /// Painter's-algorithm depth offset (accumulated down the tree).
    #[serde(default)]
    pub z_depth: Vec<Keyframe1>,
    #[serde(default)]
    pub inherits_rotation: Option<bool>,
    #[serde(default)]
    pub inherits_z_depth: Option<bool>,
    /// An optional per-field timeline, composed as an extra local transform.
    #[serde(default)]
    pub timeline: Option<TransformTimeline3dInput>,
    #[serde(default)]
    pub children: Vec<NodeInput>,
}

/// A plain keyframe in a timeline clip's channel: its component count (the
/// `value` length) selects the channel shape (1/2/3/4), and `easing` is the
/// [`Easing`] id of the segment from this key to the next (omitted = linear).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TimelineKeyInput {
    pub time: f64,
    #[serde(default)]
    pub value: Vec<f64>,
    #[serde(default)]
    pub easing: Option<u8>,
}

/// A plain timeline clip: a channel placed at `start` (keyframe times are
/// clip-relative).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TimelineClipInput {
    #[serde(default)]
    pub start: f64,
    #[serde(default)]
    pub keys: Vec<TimelineKeyInput>,
}

/// A plain timeline: clips plus nested groups (all sharing the time coordinate).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TimelineInput {
    #[serde(default)]
    pub clips: Vec<TimelineClipInput>,
    #[serde(default)]
    pub children: Vec<TimelineInput>,
}

/// A plain per-field 3D transform timeline (each field is a [`TimelineInput`]).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TransformTimeline3dInput {
    #[serde(default)]
    pub position: TimelineInput,
    #[serde(default)]
    pub scale: TimelineInput,
    #[serde(default)]
    pub rotation: TimelineInput,
    #[serde(default)]
    pub pivot: TimelineInput,
    #[serde(default)]
    pub z_depth: TimelineInput,
    #[serde(default)]
    pub color: TimelineInput,
    #[serde(default)]
    pub opacity: TimelineInput,
}

/// A keyframed perspective camera in a plain 3D input.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CameraInput {
    #[serde(default)]
    pub position: Vec<Keyframe3>,
    #[serde(default)]
    pub target: Vec<Keyframe3>,
    #[serde(default)]
    pub up: Vec<Keyframe3>,
    #[serde(default)]
    pub vertical_fov: Vec<Keyframe1>,
    #[serde(default)]
    pub horizontal_fov: Vec<Keyframe1>,
    #[serde(default)]
    pub near_clip: Vec<Keyframe1>,
    #[serde(default)]
    pub far_clip: Vec<Keyframe1>,
    /// Camera roll about the view direction, in radians.
    #[serde(default)]
    pub roll: Vec<Keyframe1>,
    #[serde(default)]
    pub dof_focus: Vec<Keyframe1>,
    #[serde(default)]
    pub dof_range: Vec<Keyframe1>,
    #[serde(default)]
    pub dof_max_blur: Vec<Keyframe1>,
    #[serde(default)]
    pub resolution: Option<[f64; 2]>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct OptionsInput {
    #[serde(default)]
    pub start_time: Option<f64>,
    #[serde(default)]
    pub end_time: Option<f64>,
    #[serde(default)]
    pub time_step: Option<f64>,
    #[serde(default)]
    pub tolerance_2d: Option<f64>,
    #[serde(default)]
    pub tolerance_1d: Option<f64>,
    #[serde(default)]
    pub decimals: Option<u32>,
    #[serde(default)]
    pub multithreaded: Option<bool>,
    #[serde(default)]
    pub adaptive_sampling: Option<bool>,
    #[serde(default)]
    pub adaptive_tolerance_px: Option<f64>,
    #[serde(default)]
    pub depth_sort: Option<bool>,
    #[serde(default)]
    pub depth_buckets: Option<u8>,
}

/// Plain 3D input for the bake.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Bake3dInput {
    pub nodes: Vec<NodeInput>,
    #[serde(default)]
    pub camera: CameraInput,
    #[serde(default)]
    pub options: OptionsInput,
}

/// Builds the scene tree, camera and options from plain input.
pub fn build_scene(input: &Bake3dInput) -> (Node3d, PerspectiveCamera, BakeOptions) {
    let mut camera = PerspectiveCamera::default();
    let c = &input.camera;
    if let Some(resolution) = c.resolution {
        camera.resolution = Vec2::new(resolution[0], resolution[1]);
    }
    for kf in &c.position {
        camera.position_x.add(kf.time, kf.value[0]);
        camera.position_y.add(kf.time, kf.value[1]);
        camera.position_z.add(kf.time, kf.value[2]);
    }
    for kf in &c.target {
        camera
            .target_position
            .add(kf.time, Vec3::new(kf.value[0], kf.value[1], kf.value[2]));
    }
    for kf in &c.up {
        camera
            .up
            .add(kf.time, Vec3::new(kf.value[0], kf.value[1], kf.value[2]));
    }
    for kf in &c.vertical_fov {
        camera.vertical_fov.add(kf.time, kf.value);
    }
    for kf in &c.horizontal_fov {
        camera.horizontal_fov.add(kf.time, kf.value);
    }
    for kf in &c.near_clip {
        camera.near_clip.add(kf.time, kf.value);
    }
    for kf in &c.far_clip {
        camera.far_clip.add(kf.time, kf.value);
    }
    for kf in &c.roll {
        camera.roll.add(kf.time, kf.value);
    }
    for kf in &c.dof_focus {
        camera.dof_focus.add(kf.time, kf.value);
    }
    for kf in &c.dof_range {
        camera.dof_range.add(kf.time, kf.value);
    }
    for kf in &c.dof_max_blur {
        camera.dof_max_blur.add(kf.time, kf.value);
    }

    let o = &input.options;
    let mut options = BakeOptions::default();
    if let Some(v) = o.start_time {
        options.start_time = v;
    }
    if let Some(v) = o.end_time {
        options.end_time = v;
    }
    if let Some(v) = o.time_step {
        options.time_step = v;
    }
    if let Some(v) = o.tolerance_2d {
        options.tolerance_2d = v;
    }
    if let Some(v) = o.tolerance_1d {
        options.tolerance_1d = v;
    }
    if let Some(v) = o.decimals {
        options.decimals = v;
    }
    if let Some(v) = o.multithreaded {
        options.multithreaded = v;
    }
    if let Some(v) = o.adaptive_sampling {
        options.adaptive_sampling = v;
    }
    if let Some(v) = o.adaptive_tolerance_px {
        options.adaptive_tolerance_px = v;
    }
    if let Some(v) = o.depth_sort {
        options.depth_sort = v;
    }
    if let Some(v) = o.depth_buckets {
        options.depth_buckets = v;
    }

    let mut root = Node3d::default();
    for node in &input.nodes {
        root.add(build_node(node));
    }
    (root, camera, options)
}

fn build_node(input: &NodeInput) -> Box<dyn Object3d> {
    let origin = OsbOrigin::from_index(input.origin.unwrap_or(4) as usize);
    let mut sprite = Sprite3d::new(input.sprite.clone(), origin);
    sprite.additive = input.additive.unwrap_or(false);
    sprite.rotation_mode = parse_rotation_mode(input.rotation_mode.as_deref());
    sprite.frames = input.frames.clone();
    sprite.frame_delay = input.frame_delay.unwrap_or(0.0);
    for kf in &input.sprite_scale {
        sprite
            .sprite_scale
            .add(kf.time, Vec2::new(kf.value[0], kf.value[1]));
    }
    for kf in &input.sprite_rotation {
        sprite.sprite_rotation.add(kf.time, kf.value);
    }

    let mut node = Node3d::default();
    for kf in &input.position {
        node.position
            .add(kf.time, Vec3::new(kf.value[0], kf.value[1], kf.value[2]));
    }
    for kf in &input.scale {
        node.scale
            .add(kf.time, Vec3::new(kf.value[0], kf.value[1], kf.value[2]));
    }
    for kf in &input.rotation {
        node.rotation
            .add(kf.time, Vec3::new(kf.value[0], kf.value[1], kf.value[2]));
    }
    for kf in &input.pivot {
        node.pivot
            .add(kf.time, Vec3::new(kf.value[0], kf.value[1], kf.value[2]));
    }
    for kf in &input.z_depth {
        node.z_depth.add(kf.time, kf.value);
    }
    node.inherits_rotation = input.inherits_rotation.unwrap_or(true);
    node.inherits_z_depth = input.inherits_z_depth.unwrap_or(true);
    if let Some(timeline) = &input.timeline {
        node.timeline = Some(build_transform_timeline3d(timeline));
    }
    sprite.node = node;
    for child in &input.children {
        sprite.node.add(build_node(child));
    }
    Box::new(sprite)
}

/// Builds a runtime [`Channel`] from plain timeline keys; the first key's value
/// length selects the shape (1/2/3/4 components).
fn build_timeline_channel(keys: &[TimelineKeyInput]) -> Channel {
    let dimension = keys.first().map(|key| key.value.len()).unwrap_or(1);
    let easing =
        |key: &TimelineKeyInput| key.easing.and_then(Easing::from_id).unwrap_or(Easing::None);
    let component =
        |key: &TimelineKeyInput, index: usize| key.value.get(index).copied().unwrap_or(0.0);
    match dimension {
        2 => {
            let mut kv = KeyframedValue::new();
            for key in keys {
                kv.add_eased(
                    key.time,
                    Vec2::new(component(key, 0), component(key, 1)),
                    easing(key),
                );
            }
            Channel::Vector2(kv)
        }
        3 => {
            let mut kv = KeyframedValue::new();
            for key in keys {
                kv.add_eased(
                    key.time,
                    Vec3::new(component(key, 0), component(key, 1), component(key, 2)),
                    easing(key),
                );
            }
            Channel::Vector3(kv)
        }
        4 => {
            let mut kv = KeyframedValue::new();
            for key in keys {
                kv.add_eased(
                    key.time,
                    Color4::new(
                        component(key, 0),
                        component(key, 1),
                        component(key, 2),
                        component(key, 3),
                    ),
                    easing(key),
                );
            }
            Channel::Color(kv)
        }
        _ => {
            let mut kv = KeyframedValue::new();
            for key in keys {
                kv.add_eased(key.time, component(key, 0), easing(key));
            }
            Channel::Scalar(kv)
        }
    }
}

/// Builds a runtime [`Timeline`] from plain clips and nested groups.
fn build_timeline(input: &TimelineInput) -> Timeline {
    let mut timeline = Timeline::new();
    for clip in &input.clips {
        timeline.add(build_timeline_channel(&clip.keys), clip.start);
    }
    for child in &input.children {
        timeline.add_child(build_timeline(child));
    }
    timeline
}

/// Builds a runtime [`TransformTimeline3d`] from its plain input.
fn build_transform_timeline3d(input: &TransformTimeline3dInput) -> TransformTimeline3d {
    TransformTimeline3d {
        position: build_timeline(&input.position),
        scale: build_timeline(&input.scale),
        rotation: build_timeline(&input.rotation),
        pivot: build_timeline(&input.pivot),
        z_depth: build_timeline(&input.z_depth),
        color: build_timeline(&input.color),
        opacity: build_timeline(&input.opacity),
    }
}

fn parse_rotation_mode(value: Option<&str>) -> RotationMode {
    match value.map(str::to_ascii_lowercase).as_deref() {
        Some("fixed") => RotationMode::Fixed,
        Some("unitx") => RotationMode::UnitX,
        Some("unity") => RotationMode::UnitY,
        _ => RotationMode::UnitX,
    }
}

/// Builds and bakes plain 3D input.
pub fn bake_input(input: &Bake3dInput) -> BakedStoryboard {
    let (root, camera, options) = build_scene(input);
    bake(&root, &camera, &options)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_and_bakes_plain_input() {
        let input: Bake3dInput = serde_json::from_str(
            r#"{
                "nodes": [{ "sprite": "sb/dot.png" }],
                "camera": { "position": [{ "time": 0, "value": [0, 0, 5] }] },
                "options": { "endTime": 0 }
            }"#,
        )
        .unwrap();
        let baked = bake_input(&input);
        assert_eq!(baked.sprites.len(), 1);
        let position = baked.sprites[0].position.value_at(0.0).unwrap();
        assert!((position.x - 427.0).abs() < 0.5);
        assert!((position.y - 240.0).abs() < 0.5);
    }

    #[test]
    fn a_node_timeline_matches_a_static_position() {
        let with_timeline: Bake3dInput = serde_json::from_str(
            r#"{
                "nodes": [{
                    "sprite": "sb/dot.png",
                    "timeline": {
                        "position": { "clips": [{ "start": 0, "keys": [
                            { "time": 0, "value": [2, 0, 0] },
                            { "time": 1000, "value": [2, 0, 0] }
                        ] }] }
                    }
                }],
                "camera": { "position": [{ "time": 0, "value": [0, 0, 5] }] }
            }"#,
        )
        .unwrap();
        let with_static: Bake3dInput = serde_json::from_str(
            r#"{
                "nodes": [{ "sprite": "sb/dot.png", "position": [{ "time": 0, "value": [2, 0, 0] }] }],
                "camera": { "position": [{ "time": 0, "value": [0, 0, 5] }] }
            }"#,
        )
        .unwrap();
        let a = bake_input(&with_timeline);
        let b = bake_input(&with_static);
        assert_eq!(a.sprites.len(), 1);
        let pa = a.sprites[0].position.value_at(0.0).unwrap();
        let pb = b.sprites[0].position.value_at(0.0).unwrap();
        assert!((pa.x - pb.x).abs() < 1e-6, "{pa:?} vs {pb:?}");
        assert!((pa.y - pb.y).abs() < 1e-6, "{pa:?} vs {pb:?}");
    }
}
