//! The 3D scene object hierarchy.
//!
//! `Object3d` is the base trait every scene object implements: it holds children,
//! contributes an optional local transform and inherited color/opacity/depth, and
//! projects zero or more sprite leaves. `Node3d` is the transform-carrying node;
//! `Sprite3d`, `Line3d`, `Triangle3d` and `Mesh3d` derive from it, adding the
//! visual. The scene tree is a `Vec<Box<dyn Object3d>>`, so the bake walks any mix
//! of objects polymorphically - the same shape as the reference toolkit's
//! `Object3d` / `Node3d` / `Sprite3d` class tree, expressed with a trait plus
//! composition (Rust has no classical inheritance).

use std::collections::BTreeMap;

use crate::animations::timeline::Timeline;
use crate::animations::KeyframedValue;
use crate::math::{Color4, Matrix4, Quaternion, Vec2, Vec3};
use crate::storyboarding::{Node2d, OsbLayer, OsbOrigin};

use super::camera::CameraState;
use super::mesh::{Face, LineSegment, Quad, Triangle};

/// How a sprite orients itself in screenspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RotationMode {
    /// No automatic rotation; the sprite keeps its `sprite_rotation`.
    Fixed,
    /// Aligns the sprite's X axis with the node's projected X axis.
    #[default]
    UnitX,
    /// Aligns the sprite's Y axis with the node's projected Y axis.
    UnitY,
}

/// The projected state of one sprite leaf at one time.
#[derive(Debug, Clone, Copy)]
pub struct SpriteState {
    pub position: Vec2,
    pub scale: Vec2,
    pub rotation: f64,
    pub color: Color4,
    pub opacity: f64,
    pub additive: bool,
    pub depth: f64,
    /// Accumulated `z_depth`, used for depth buckets and painter sorting.
    pub z_depth: f64,
    /// Approximate depth-of-field blur radius in pixels.
    pub blur: f64,
}

/// A sprite leaf's identity, stable across time (so the bake can group its
/// per-time samples into one output sprite).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeafMeta {
    pub sprite_path: String,
    pub origin: OsbOrigin,
    pub osb_layer: Option<OsbLayer>,
    pub additive: bool,
    /// Whether the leaf is drawn mirrored horizontally (mesh faces from behind).
    pub flip_h: bool,
}

/// The base of every 3D scene object.
pub trait Object3d: std::fmt::Debug + Send + Sync {
    /// A label for debugging.
    fn name(&self) -> &str;
    /// The osu! layer this object's sprites are written to, if any.
    fn layer(&self) -> Option<OsbLayer>;
    /// The object's children.
    fn children(&self) -> &[Box<dyn Object3d>];
    /// Mutable access to the children.
    fn children_mut(&mut self) -> &mut Vec<Box<dyn Object3d>>;
    /// A boxed clone (so `Box<dyn Object3d>: Clone`).
    fn clone_box(&self) -> Box<dyn Object3d>;

    /// Adds a child object.
    fn add(&mut self, child: Box<dyn Object3d>) {
        self.children_mut().push(child);
    }

    // -- transform contributions (identity by default; `Node3d` overrides) --

    /// This object's local transform at `time`.
    fn local_transform_at(&self, _time: f64) -> Matrix4 {
        Matrix4::identity()
    }
    /// This object's local tint at `time`.
    fn color_at(&self, _time: f64) -> Color4 {
        Color4::WHITE
    }
    /// This object's local opacity at `time`.
    fn opacity_at(&self, _time: f64) -> f64 {
        1.0
    }
    /// This object's local Euler rotation at `time` (radians).
    fn rotation_at(&self, _time: f64) -> Vec3 {
        Vec3::ZERO
    }
    /// This object's local painter's-algorithm depth offset at `time`.
    fn z_depth_at(&self, _time: f64) -> f64 {
        0.0
    }
    /// Whether children inherit this object's color.
    fn inherits_color(&self) -> bool {
        true
    }
    /// Whether children inherit this object's opacity.
    fn inherits_opacity(&self) -> bool {
        true
    }
    /// Whether children add this object's Euler rotation.
    fn inherits_rotation(&self) -> bool {
        true
    }
    /// Whether children add this object's `z_depth`.
    fn inherits_z_depth(&self) -> bool {
        true
    }

    /// A 2D subtree drawn in this object's local space, if any (3D is the superset).
    fn scene2d(&self) -> Option<&Node2d> {
        None
    }

    /// Mutable access to the 2D subtree slot, if this object has one.
    fn scene2d_mut(&mut self) -> Option<&mut Option<Box<Node2d>>> {
        None
    }

    /// Appends this object's projected sprite leaves at `time`. The order and
    /// count must match [`Object3d::leaves`]; a culled leaf is pushed with
    /// opacity `0` so the bake's channels stay aligned.
    fn sample(
        &self,
        _time: f64,
        _animation_start: f64,
        _camera: &CameraState,
        _state: &Object3dState,
        _out: &mut Vec<SpriteState>,
    ) {
    }

    /// Appends the identity of each leaf `sample` produces, in the same order.
    fn leaves(&self, _out: &mut Vec<LeafMeta>) {}
}

impl Clone for Box<dyn Object3d> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// The inherited transform/color/opacity of a node at one time.
#[derive(Debug, Clone, Copy)]
pub struct Object3dState {
    pub world_transform: Matrix4,
    pub color: Color4,
    pub opacity: f64,
    /// Accumulated Euler rotation in radians (X/Y/Z).
    pub rotation_xyz: Vec3,
    /// Accumulated painter's-algorithm depth offset.
    pub z_depth: f64,
    /// Determinant of the world transform's 3x3 part; negative means mirrored
    /// (which flips face winding).
    pub determinant: f64,
}

impl Object3dState {
    pub fn initial() -> Self {
        Object3dState {
            world_transform: Matrix4::identity(),
            color: Color4::WHITE,
            opacity: 1.0,
            rotation_xyz: Vec3::ZERO,
            z_depth: 0.0,
            determinant: 1.0,
        }
    }
}

/// Composes a child's inherited state from its parent's.
pub fn state_of(object: &dyn Object3d, time: f64, parent: &Object3dState) -> Object3dState {
    let local = object.local_transform_at(time);
    let color = object.color_at(time);
    let opacity = object.opacity_at(time);
    let rotation = object.rotation_at(time);
    let z_depth = object.z_depth_at(time);
    Object3dState {
        world_transform: parent.world_transform.mul(&local),
        color: if object.inherits_color() {
            multiply_color(color, parent.color)
        } else {
            color
        },
        opacity: if object.inherits_opacity() {
            opacity * parent.opacity
        } else {
            opacity
        },
        rotation_xyz: if object.inherits_rotation() {
            rotation + parent.rotation_xyz
        } else {
            rotation
        },
        z_depth: if object.inherits_z_depth() {
            z_depth + parent.z_depth
        } else {
            z_depth
        },
        determinant: parent.determinant * determinant3(&local),
    }
}

/// A node's per-field **timeline**: each 3D transform field is a [`Timeline`], so
/// a node's motion can be authored as sequenced clips (with nested groups) on top
/// of its static transform.
///
/// The timeline is an extra **local** transform composed after the node's own:
/// position adds, scale multiplies, rotation adds, opacity/color multiply,
/// `z_depth` adds, and a field with no active clip contributes identity.
#[derive(Debug, Clone, Default)]
pub struct TransformTimeline3d {
    /// Sequenced translation clips (3-vector channels).
    pub position: Timeline,
    /// Sequenced per-axis scale clips (3-vector channels).
    pub scale: Timeline,
    /// Sequenced Euler rotation clips in radians (3-vector channels).
    pub rotation: Timeline,
    /// Sequenced pivot clips (3-vector channels).
    pub pivot: Timeline,
    /// Sequenced `z_depth` clips (scalar channels).
    pub z_depth: Timeline,
    /// Sequenced tint clips (color channels).
    pub color: Timeline,
    /// Sequenced opacity clips (scalar channels).
    pub opacity: Timeline,
}

impl TransformTimeline3d {
    /// An empty transform timeline (every field contributes identity).
    pub fn new() -> Self {
        TransformTimeline3d::default()
    }

    /// The sampled translation at `time` (zero when no clip is active).
    pub fn position_at(&self, time: f64) -> Vec3 {
        self.position
            .channel_at(time)
            .and_then(|channel| channel.vec3_at(time))
            .unwrap_or(Vec3::ZERO)
    }

    /// The sampled scale at `time` (unit when no clip is active).
    pub fn scale_at(&self, time: f64) -> Vec3 {
        self.scale
            .channel_at(time)
            .and_then(|channel| channel.vec3_at(time))
            .unwrap_or(Vec3::ONE)
    }

    /// The sampled Euler rotation at `time` in radians (zero when no clip is active).
    pub fn rotation_at(&self, time: f64) -> Vec3 {
        self.rotation
            .channel_at(time)
            .and_then(|channel| channel.vec3_at(time))
            .unwrap_or(Vec3::ZERO)
    }

    /// The sampled pivot at `time` (zero when no clip is active).
    pub fn pivot_at(&self, time: f64) -> Vec3 {
        self.pivot
            .channel_at(time)
            .and_then(|channel| channel.vec3_at(time))
            .unwrap_or(Vec3::ZERO)
    }

    /// The sampled `z_depth` at `time` (zero when no clip is active).
    pub fn z_depth_at(&self, time: f64) -> f64 {
        self.z_depth
            .channel_at(time)
            .and_then(|channel| channel.scalar_at(time))
            .unwrap_or(0.0)
    }

    /// The sampled tint at `time` (white when no clip is active).
    pub fn color_at(&self, time: f64) -> Color4 {
        self.color
            .channel_at(time)
            .and_then(|channel| channel.color_at(time))
            .unwrap_or(Color4::WHITE)
    }

    /// The sampled opacity at `time` (one when no clip is active).
    pub fn opacity_at(&self, time: f64) -> f64 {
        self.opacity
            .channel_at(time)
            .and_then(|channel| channel.scalar_at(time))
            .unwrap_or(1.0)
    }

    /// The extra local transform at `time`: `T(pos) * R * S * T(-pivot)`.
    pub fn local_at(&self, time: f64) -> Matrix4 {
        let rotation = self.rotation_at(time);
        Matrix4::translation(self.position_at(time))
            .mul(&Quaternion::from_euler_angles(rotation.x, rotation.y, rotation.z).to_matrix())
            .mul(&Matrix4::scale(self.scale_at(time)))
            .mul(&Matrix4::translation(-self.pivot_at(time)))
    }

    /// The timeline's occupied `[start, end]` range across its fields.
    pub fn time_range(&self) -> Option<(f64, f64)> {
        let mut range: Option<(f64, f64)> = None;
        for timeline in [
            &self.position,
            &self.scale,
            &self.rotation,
            &self.pivot,
            &self.z_depth,
            &self.color,
            &self.opacity,
        ] {
            if let Some((start, end)) = timeline.time_range() {
                range = Some(match range {
                    Some((lo, hi)) => (lo.min(start), hi.max(end)),
                    None => (start, end),
                });
            }
        }
        range
    }

    /// The timeline's natural duration (last time minus first time).
    pub fn duration(&self) -> f64 {
        self.time_range().map(|(a, b)| b - a).unwrap_or(0.0)
    }

    /// Shifts every field in time.
    pub fn shift(&mut self, delta: f64) -> &mut Self {
        self.position.shift(delta);
        self.scale.shift(delta);
        self.rotation.shift(delta);
        self.pivot.shift(delta);
        self.z_depth.shift(delta);
        self.color.shift(delta);
        self.opacity.shift(delta);
        self
    }

    /// Scales every field in time about `anchor`.
    pub fn scale_time(&mut self, factor: f64, anchor: f64) -> &mut Self {
        self.position.scale(factor, anchor);
        self.scale.scale(factor, anchor);
        self.rotation.scale(factor, anchor);
        self.pivot.scale(factor, anchor);
        self.z_depth.scale(factor, anchor);
        self.color.scale(factor, anchor);
        self.opacity.scale(factor, anchor);
        self
    }

    /// Reverses every field in time about `[start, end]`.
    pub fn reverse(&mut self, start: f64, end: f64) -> &mut Self {
        self.position.reverse(start, end);
        self.scale.reverse(start, end);
        self.rotation.reverse(start, end);
        self.pivot.reverse(start, end);
        self.z_depth.reverse(start, end);
        self.color.reverse(start, end);
        self.opacity.reverse(start, end);
        self
    }
}

/// A transform-carrying node: position/scale/rotation/pivot keyframes plus
/// inherited color/opacity/depth and children (the base of every visual object).
#[derive(Debug, Clone)]
pub struct Node3d {
    /// A label for debugging.
    pub name: String,
    /// The osu! layer this node's sprites are written to; `None` uses the bake's
    /// depth bucket.
    pub layer: Option<OsbLayer>,
    pub position: KeyframedValue<Vec3>,
    pub scale: KeyframedValue<Vec3>,
    /// Euler rotation in radians (X/Y/Z).
    pub rotation: KeyframedValue<Vec3>,
    /// The pivot the node's rotation/scale act about, in local space.
    pub pivot: KeyframedValue<Vec3>,
    /// Painter's-algorithm depth offset, accumulated down the tree.
    pub z_depth: KeyframedValue<f64>,
    pub color: KeyframedValue<Color4>,
    pub opacity: KeyframedValue<f64>,
    /// The node's per-field timeline, composed as an extra local transform.
    pub timeline: Option<TransformTimeline3d>,
    /// The node's children (any mix of object types).
    pub children: Vec<Box<dyn Object3d>>,
    pub inherits_color: bool,
    pub inherits_opacity: bool,
    pub inherits_rotation: bool,
    pub inherits_z_depth: bool,
    /// A 2D subtree drawn in this node's local space (2D content in a 3D scene).
    pub scene2d: Option<Box<Node2d>>,
}

impl Default for Node3d {
    fn default() -> Self {
        Node3d {
            name: String::new(),
            layer: None,
            position: KeyframedValue::new(),
            scale: KeyframedValue::new(),
            rotation: KeyframedValue::new(),
            pivot: KeyframedValue::new(),
            z_depth: KeyframedValue::new(),
            color: KeyframedValue::new(),
            opacity: KeyframedValue::new(),
            timeline: None,
            children: Vec::new(),
            inherits_color: true,
            inherits_opacity: true,
            inherits_rotation: true,
            inherits_z_depth: true,
            scene2d: None,
        }
    }
}

impl Node3d {
    /// A named empty node.
    pub fn new(name: impl Into<String>) -> Self {
        Node3d {
            name: name.into(),
            ..Node3d::default()
        }
    }

    /// The pivot at `time`, or the local origin when unset.
    pub fn pivot_at(&self, time: f64) -> Vec3 {
        self.pivot.value_at(time).unwrap_or(Vec3::ZERO)
    }

    /// The node's local transform at `time` (its keyframes composed with its
    /// timeline): `T(pos) * R * S * T(-pivot)`.
    pub fn transform_at(&self, time: f64) -> Matrix4 {
        let scale = self.scale.value_at(time).unwrap_or(Vec3::ONE);
        let rotation = self.rotation.value_at(time).unwrap_or(Vec3::ZERO);
        let position = self.position.value_at(time).unwrap_or(Vec3::ZERO);
        let pivot = self.pivot_at(time);
        let base = Matrix4::translation(position)
            .mul(&Quaternion::from_euler_angles(rotation.x, rotation.y, rotation.z).to_matrix())
            .mul(&Matrix4::scale(scale))
            .mul(&Matrix4::translation(-pivot));
        match &self.timeline {
            Some(timeline) => base.mul(&timeline.local_at(time)),
            None => base,
        }
    }

    /// The node's local tint at `time` (keyframes multiplied by its timeline).
    pub fn color_with_timeline(&self, time: f64) -> Color4 {
        let color = self.color.value_at(time).unwrap_or(Color4::WHITE);
        match &self.timeline {
            Some(timeline) => multiply_color(color, timeline.color_at(time)),
            None => color,
        }
    }

    /// The node's local opacity at `time` (keyframes times its timeline).
    pub fn opacity_with_timeline(&self, time: f64) -> f64 {
        let opacity = self.opacity.value_at(time).unwrap_or(1.0);
        match &self.timeline {
            Some(timeline) => opacity * timeline.opacity_at(time),
            None => opacity,
        }
    }

    /// The node's local `z_depth` at `time` (keyframes plus its timeline).
    pub fn z_depth_with_timeline(&self, time: f64) -> f64 {
        let z = self.z_depth.value_at(time).unwrap_or(0.0);
        match &self.timeline {
            Some(timeline) => z + timeline.z_depth_at(time),
            None => z,
        }
    }
}

impl Object3d for Node3d {
    fn name(&self) -> &str {
        &self.name
    }

    fn layer(&self) -> Option<OsbLayer> {
        self.layer
    }

    fn children(&self) -> &[Box<dyn Object3d>] {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<Box<dyn Object3d>> {
        &mut self.children
    }

    fn clone_box(&self) -> Box<dyn Object3d> {
        Box::new(self.clone())
    }

    fn local_transform_at(&self, time: f64) -> Matrix4 {
        self.transform_at(time)
    }

    fn color_at(&self, time: f64) -> Color4 {
        self.color_with_timeline(time)
    }

    fn opacity_at(&self, time: f64) -> f64 {
        self.opacity_with_timeline(time)
    }

    fn rotation_at(&self, time: f64) -> Vec3 {
        self.rotation.value_at(time).unwrap_or(Vec3::ZERO)
    }

    fn z_depth_at(&self, time: f64) -> f64 {
        self.z_depth_with_timeline(time)
    }

    fn inherits_color(&self) -> bool {
        self.inherits_color
    }

    fn inherits_opacity(&self) -> bool {
        self.inherits_opacity
    }

    fn inherits_rotation(&self) -> bool {
        self.inherits_rotation
    }

    fn inherits_z_depth(&self) -> bool {
        self.inherits_z_depth
    }

    fn scene2d(&self) -> Option<&Node2d> {
        self.scene2d.as_deref()
    }

    fn scene2d_mut(&mut self) -> Option<&mut Option<Box<Node2d>>> {
        Some(&mut self.scene2d)
    }
}

/// A sprite: a [`Node3d`] with a texture path, origin and screen-space scale/
/// rotation.
#[derive(Debug, Clone)]
pub struct Sprite3d {
    pub node: Node3d,
    pub sprite_path: String,
    pub origin: OsbOrigin,
    pub additive: bool,
    pub rotation_mode: RotationMode,
    pub sprite_scale: KeyframedValue<Vec2>,
    /// Additional screen-space rotation in radians.
    pub sprite_rotation: KeyframedValue<f64>,
    pub use_distance_fade: bool,
    /// Frame sprite paths for an animation plane (empty = a still sprite).
    pub frames: Vec<String>,
    /// Frame delay in ms; required (> 0) for `frames` to animate.
    pub frame_delay: f64,
}

impl Sprite3d {
    pub fn new(sprite_path: impl Into<String>, origin: OsbOrigin) -> Self {
        Sprite3d {
            node: Node3d::default(),
            sprite_path: sprite_path.into(),
            origin,
            additive: false,
            rotation_mode: RotationMode::UnitX,
            sprite_scale: KeyframedValue::new(),
            sprite_rotation: KeyframedValue::new(),
            use_distance_fade: true,
            frames: Vec::new(),
            frame_delay: 0.0,
        }
    }

    /// Whether this sprite plays a frame sequence.
    pub fn is_animated(&self) -> bool {
        self.frames.len() > 1 && self.frame_delay > 0.0
    }

    /// The active frame index at `time`, anchored on `start`.
    pub fn frame_index_at(&self, time: f64, start: f64) -> usize {
        if !self.is_animated() {
            return 0;
        }
        let elapsed = (time - start).max(0.0);
        ((elapsed / self.frame_delay).floor() as usize) % self.frames.len()
    }

    fn leaf_meta(&self, sprite_path: &str) -> LeafMeta {
        LeafMeta {
            sprite_path: sprite_path.to_string(),
            origin: self.origin,
            osb_layer: self.node.layer,
            additive: self.additive,
            flip_h: false,
        }
    }

    /// Projects the sprite (and its animation frames) at `time`.
    fn sprite_state(
        &self,
        time: f64,
        camera: &CameraState,
        object_state: &Object3dState,
    ) -> SpriteState {
        let wvp = camera.view_projection.mul(&object_state.world_transform);
        let screen = camera.to_screen(&wvp, Vec3::ZERO);

        let base_angle = match self.rotation_mode {
            RotationMode::Fixed => 0.0,
            RotationMode::UnitX => {
                let unit = camera.to_screen(&wvp, Vec3::new(1.0, 0.0, 0.0));
                (unit.y - screen.y).atan2(unit.x - screen.x)
            }
            RotationMode::UnitY => {
                let unit = camera.to_screen(&wvp, Vec3::new(0.0, 1.0, 0.0));
                (unit.y - screen.y).atan2(unit.x - screen.x) - std::f64::consts::FRAC_PI_2
            }
        };
        let rotation = base_angle + self.sprite_rotation.value_at(time).unwrap_or(0.0);

        let world_scale = extract_scale(&object_state.world_transform);
        let sprite_scale = self.sprite_scale.value_at(time).unwrap_or(Vec2::ONE);
        let perspective = if screen.w.abs() < f64::EPSILON {
            0.0
        } else {
            camera.focus_distance / screen.w
        };
        let scale = Vec2::new(
            sprite_scale.x * world_scale.x * perspective * camera.resolution_scale,
            sprite_scale.y * world_scale.y * perspective * camera.resolution_scale,
        );

        let mut opacity = if screen.w < 0.0 {
            0.0
        } else {
            object_state.opacity
        };
        if self.use_distance_fade {
            opacity *= camera.opacity_at(screen.w);
        }
        let dof = if screen.w < 0.0 {
            crate::storyboarding3d::camera::DofSample {
                opacity: 1.0,
                blur_px: 0.0,
            }
        } else {
            camera.dof_at(screen.w)
        };
        opacity *= dof.opacity;

        SpriteState {
            position: Vec2::new(screen.x, screen.y),
            scale,
            rotation,
            color: object_state.color,
            opacity,
            additive: self.additive,
            depth: screen.z,
            z_depth: object_state.z_depth,
            blur: dof.blur_px,
        }
    }
}

impl Object3d for Sprite3d {
    fn name(&self) -> &str {
        &self.node.name
    }

    fn layer(&self) -> Option<OsbLayer> {
        self.node.layer
    }

    fn children(&self) -> &[Box<dyn Object3d>] {
        &self.node.children
    }

    fn children_mut(&mut self) -> &mut Vec<Box<dyn Object3d>> {
        &mut self.node.children
    }

    fn clone_box(&self) -> Box<dyn Object3d> {
        Box::new(self.clone())
    }

    fn local_transform_at(&self, time: f64) -> Matrix4 {
        self.node.transform_at(time)
    }

    fn color_at(&self, time: f64) -> Color4 {
        self.node.color_with_timeline(time)
    }

    fn opacity_at(&self, time: f64) -> f64 {
        self.node.opacity_with_timeline(time)
    }

    fn rotation_at(&self, time: f64) -> Vec3 {
        self.node.rotation.value_at(time).unwrap_or(Vec3::ZERO)
    }

    fn z_depth_at(&self, time: f64) -> f64 {
        self.node.z_depth_with_timeline(time)
    }

    fn inherits_color(&self) -> bool {
        self.node.inherits_color
    }

    fn inherits_opacity(&self) -> bool {
        self.node.inherits_opacity
    }

    fn inherits_rotation(&self) -> bool {
        self.node.inherits_rotation
    }

    fn inherits_z_depth(&self) -> bool {
        self.node.inherits_z_depth
    }

    fn scene2d(&self) -> Option<&Node2d> {
        self.node.scene2d.as_deref()
    }

    fn scene2d_mut(&mut self) -> Option<&mut Option<Box<Node2d>>> {
        Some(&mut self.node.scene2d)
    }

    fn leaves(&self, out: &mut Vec<LeafMeta>) {
        if self.is_animated() {
            for frame in &self.frames {
                out.push(self.leaf_meta(frame));
            }
        } else {
            out.push(self.leaf_meta(&self.sprite_path));
        }
    }

    fn sample(
        &self,
        time: f64,
        animation_start: f64,
        camera: &CameraState,
        state: &Object3dState,
        out: &mut Vec<SpriteState>,
    ) {
        let base = self.sprite_state(time, camera, state);
        if self.is_animated() {
            let current = self.frame_index_at(time, animation_start);
            for index in 0..self.frames.len() {
                let mut frame_state = base;
                if index != current {
                    frame_state.opacity = 0.0;
                }
                out.push(frame_state);
            }
        } else {
            out.push(base);
        }
    }
}

/// A line/capsule object: a [`Node3d`] plus animated start/end/thickness.
#[derive(Debug, Clone)]
pub struct Line3d {
    pub node: Node3d,
    pub sprite_path: String,
    pub origin: OsbOrigin,
    pub start: KeyframedValue<Vec3>,
    pub end: KeyframedValue<Vec3>,
    pub thickness: KeyframedValue<f64>,
    pub use_distance_fade: bool,
    /// The texture size the line's length/thickness scale against.
    pub bitmap_size: Vec2,
}

impl Line3d {
    pub fn new(sprite_path: impl Into<String>, origin: OsbOrigin) -> Self {
        Line3d {
            node: Node3d::default(),
            sprite_path: sprite_path.into(),
            origin,
            start: KeyframedValue::new(),
            end: KeyframedValue::new(),
            thickness: KeyframedValue::new(),
            use_distance_fade: true,
            bitmap_size: Vec2::new(1.0, 1.0),
        }
    }
}

impl Object3d for Line3d {
    fn name(&self) -> &str {
        &self.node.name
    }
    fn layer(&self) -> Option<OsbLayer> {
        self.node.layer
    }
    fn children(&self) -> &[Box<dyn Object3d>] {
        &self.node.children
    }
    fn children_mut(&mut self) -> &mut Vec<Box<dyn Object3d>> {
        &mut self.node.children
    }
    fn clone_box(&self) -> Box<dyn Object3d> {
        Box::new(self.clone())
    }
    fn local_transform_at(&self, time: f64) -> Matrix4 {
        self.node.transform_at(time)
    }
    fn color_at(&self, time: f64) -> Color4 {
        self.node.color_with_timeline(time)
    }
    fn opacity_at(&self, time: f64) -> f64 {
        self.node.opacity_with_timeline(time)
    }
    fn rotation_at(&self, time: f64) -> Vec3 {
        self.node.rotation.value_at(time).unwrap_or(Vec3::ZERO)
    }
    fn z_depth_at(&self, time: f64) -> f64 {
        self.node.z_depth_with_timeline(time)
    }
    fn inherits_color(&self) -> bool {
        self.node.inherits_color
    }
    fn inherits_opacity(&self) -> bool {
        self.node.inherits_opacity
    }
    fn inherits_rotation(&self) -> bool {
        self.node.inherits_rotation
    }
    fn inherits_z_depth(&self) -> bool {
        self.node.inherits_z_depth
    }
    fn scene2d(&self) -> Option<&Node2d> {
        self.node.scene2d.as_deref()
    }

    fn scene2d_mut(&mut self) -> Option<&mut Option<Box<Node2d>>> {
        Some(&mut self.node.scene2d)
    }

    fn leaves(&self, out: &mut Vec<LeafMeta>) {
        out.push(LeafMeta {
            sprite_path: self.sprite_path.clone(),
            origin: OsbOrigin::from_index(ORIGIN_CENTRE_LEFT as usize),
            osb_layer: self.node.layer,
            additive: false,
            flip_h: false,
        });
    }

    fn sample(
        &self,
        time: f64,
        _animation_start: f64,
        camera: &CameraState,
        state: &Object3dState,
        out: &mut Vec<SpriteState>,
    ) {
        let wvp = camera.view_projection.mul(&state.world_transform);
        let mut opacity = state.opacity;
        if self.use_distance_fade {
            let origin = wvp.transform_vec4(crate::math::Vec4::new(0.0, 0.0, 0.0, 1.0));
            opacity *= camera.opacity_at(origin.w);
        }
        let segment = LineSegment {
            start: self.start.value_at(time).unwrap_or(Vec3::ZERO),
            end: self.end.value_at(time).unwrap_or(Vec3::ZERO),
            thickness: self.thickness.value_at(time).unwrap_or(1.0),
        };
        match segment.project(camera, &wvp, self.bitmap_size.x, opacity) {
            Some(sprite) => out.push(mesh_sprite_state(sprite, state, false)),
            None => out.push(culled_state(state)),
        }
    }
}

/// A filled triangle object: a [`Node3d`] plus three animated points.
#[derive(Debug, Clone)]
pub struct Triangle3d {
    pub node: Node3d,
    pub sprite_path: String,
    pub origin: OsbOrigin,
    pub points: [KeyframedValue<Vec3>; 3],
    /// The texture size the triangle scales against.
    pub bitmap_size: Vec2,
}

impl Triangle3d {
    pub fn new(sprite_path: impl Into<String>, origin: OsbOrigin) -> Self {
        Triangle3d {
            node: Node3d::default(),
            sprite_path: sprite_path.into(),
            origin,
            points: [
                KeyframedValue::new(),
                KeyframedValue::new(),
                KeyframedValue::new(),
            ],
            bitmap_size: Vec2::new(1.0, 1.0),
        }
    }
}

impl Object3d for Triangle3d {
    fn name(&self) -> &str {
        &self.node.name
    }
    fn layer(&self) -> Option<OsbLayer> {
        self.node.layer
    }
    fn children(&self) -> &[Box<dyn Object3d>] {
        &self.node.children
    }
    fn children_mut(&mut self) -> &mut Vec<Box<dyn Object3d>> {
        &mut self.node.children
    }
    fn clone_box(&self) -> Box<dyn Object3d> {
        Box::new(self.clone())
    }
    fn local_transform_at(&self, time: f64) -> Matrix4 {
        self.node.transform_at(time)
    }
    fn color_at(&self, time: f64) -> Color4 {
        self.node.color_with_timeline(time)
    }
    fn opacity_at(&self, time: f64) -> f64 {
        self.node.opacity_with_timeline(time)
    }
    fn rotation_at(&self, time: f64) -> Vec3 {
        self.node.rotation.value_at(time).unwrap_or(Vec3::ZERO)
    }
    fn z_depth_at(&self, time: f64) -> f64 {
        self.node.z_depth_with_timeline(time)
    }
    fn inherits_color(&self) -> bool {
        self.node.inherits_color
    }
    fn inherits_opacity(&self) -> bool {
        self.node.inherits_opacity
    }
    fn inherits_rotation(&self) -> bool {
        self.node.inherits_rotation
    }
    fn inherits_z_depth(&self) -> bool {
        self.node.inherits_z_depth
    }
    fn scene2d(&self) -> Option<&Node2d> {
        self.node.scene2d.as_deref()
    }

    fn scene2d_mut(&mut self) -> Option<&mut Option<Box<Node2d>>> {
        Some(&mut self.node.scene2d)
    }

    fn leaves(&self, out: &mut Vec<LeafMeta>) {
        for flip_h in [false, true] {
            out.push(LeafMeta {
                sprite_path: self.sprite_path.clone(),
                origin: OsbOrigin::from_index(ORIGIN_TOP_LEFT as usize),
                osb_layer: self.node.layer,
                additive: false,
                flip_h,
            });
        }
    }

    fn sample(
        &self,
        time: f64,
        _animation_start: f64,
        camera: &CameraState,
        state: &Object3dState,
        out: &mut Vec<SpriteState>,
    ) {
        let wvp = camera.view_projection.mul(&state.world_transform);
        let triangle = Triangle {
            points: [
                self.points[0].value_at(time).unwrap_or(Vec3::ZERO),
                self.points[1].value_at(time).unwrap_or(Vec3::ZERO),
                self.points[2].value_at(time).unwrap_or(Vec3::ZERO),
            ],
        };
        match triangle.project(
            camera,
            &wvp,
            self.bitmap_size,
            state.opacity,
            state.determinant < 0.0,
        ) {
            Some(pair) => {
                for sprite in pair {
                    out.push(mesh_sprite_state(sprite, state, false));
                }
            }
            None => {
                out.push(culled_state(state));
                out.push(culled_state(state));
            }
        }
    }
}

/// An indexed mesh object: a [`Node3d`] plus faces, vertices and materials.
#[derive(Debug, Clone)]
pub struct Mesh3d {
    pub node: Node3d,
    pub vertices: Vec<Vec3>,
    pub faces: Vec<Face>,
    pub bitmap_size: Vec2,
    /// Material name -> sprite path, used to resolve each face's `material`.
    pub materials: BTreeMap<String, String>,
}

impl Mesh3d {
    pub fn new() -> Self {
        Mesh3d {
            node: Node3d::default(),
            vertices: Vec::new(),
            faces: Vec::new(),
            bitmap_size: Vec2::new(1.0, 1.0),
            materials: BTreeMap::new(),
        }
    }
}

impl Default for Mesh3d {
    fn default() -> Self {
        Mesh3d::new()
    }
}

impl Object3d for Mesh3d {
    fn name(&self) -> &str {
        &self.node.name
    }
    fn layer(&self) -> Option<OsbLayer> {
        self.node.layer
    }
    fn children(&self) -> &[Box<dyn Object3d>] {
        &self.node.children
    }
    fn children_mut(&mut self) -> &mut Vec<Box<dyn Object3d>> {
        &mut self.node.children
    }
    fn clone_box(&self) -> Box<dyn Object3d> {
        Box::new(self.clone())
    }
    fn local_transform_at(&self, time: f64) -> Matrix4 {
        self.node.transform_at(time)
    }
    fn color_at(&self, time: f64) -> Color4 {
        self.node.color_with_timeline(time)
    }
    fn opacity_at(&self, time: f64) -> f64 {
        self.node.opacity_with_timeline(time)
    }
    fn rotation_at(&self, time: f64) -> Vec3 {
        self.node.rotation.value_at(time).unwrap_or(Vec3::ZERO)
    }
    fn z_depth_at(&self, time: f64) -> f64 {
        self.node.z_depth_with_timeline(time)
    }
    fn inherits_color(&self) -> bool {
        self.node.inherits_color
    }
    fn inherits_opacity(&self) -> bool {
        self.node.inherits_opacity
    }
    fn inherits_rotation(&self) -> bool {
        self.node.inherits_rotation
    }
    fn inherits_z_depth(&self) -> bool {
        self.node.inherits_z_depth
    }
    fn scene2d(&self) -> Option<&Node2d> {
        self.node.scene2d.as_deref()
    }

    fn scene2d_mut(&mut self) -> Option<&mut Option<Box<Node2d>>> {
        Some(&mut self.node.scene2d)
    }

    fn leaves(&self, out: &mut Vec<LeafMeta>) {
        for face in &self.faces {
            let path = face
                .material
                .as_ref()
                .and_then(|name| self.materials.get(name))
                .cloned()
                .unwrap_or_default();
            let count = if face.indices.len() == 3 { 2 } else { 1 };
            for index in 0..count {
                out.push(LeafMeta {
                    sprite_path: path.clone(),
                    origin: OsbOrigin::from_index(ORIGIN_TOP_LEFT as usize),
                    osb_layer: self.node.layer,
                    additive: false,
                    flip_h: index == 1,
                });
            }
        }
    }

    fn sample(
        &self,
        _time: f64,
        _animation_start: f64,
        camera: &CameraState,
        state: &Object3dState,
        out: &mut Vec<SpriteState>,
    ) {
        let wvp = camera.view_projection.mul(&state.world_transform);
        let mirrored = state.determinant < 0.0;
        for face in &self.faces {
            let points: Vec<Vec3> = face
                .indices
                .iter()
                .filter_map(|index| self.vertices.get(*index as usize).copied())
                .collect();
            if points.len() == 3 {
                let triangle = Triangle {
                    points: [points[0], points[1], points[2]],
                };
                match triangle.project(camera, &wvp, self.bitmap_size, state.opacity, mirrored) {
                    Some(pair) => {
                        for sprite in pair {
                            out.push(mesh_sprite_state(sprite, state, false));
                        }
                    }
                    None => {
                        out.push(culled_state(state));
                        out.push(culled_state(state));
                    }
                }
            } else if points.len() >= 4 {
                let quad = Quad {
                    corners: [points[0], points[1], points[2], points[3]],
                };
                match quad.project(camera, &wvp, self.bitmap_size, state.opacity) {
                    Some(sprite) => out.push(mesh_sprite_state(sprite, state, false)),
                    None => out.push(culled_state(state)),
                }
            }
        }
    }
}

/// A zero-opacity state for a culled leaf (keeps the bake's channels aligned).
fn culled_state(state: &Object3dState) -> SpriteState {
    SpriteState {
        position: Vec2::ZERO,
        scale: Vec2::ZERO,
        rotation: 0.0,
        color: state.color,
        opacity: 0.0,
        additive: false,
        depth: 0.0,
        z_depth: state.z_depth,
        blur: 0.0,
    }
}

/// Converts a projected [`super::mesh::MeshSprite`] to a [`SpriteState`].
fn mesh_sprite_state(
    sprite: super::mesh::MeshSprite,
    state: &Object3dState,
    additive: bool,
) -> SpriteState {
    SpriteState {
        position: sprite.position,
        scale: sprite.scale,
        rotation: sprite.rotation,
        color: state.color,
        opacity: sprite.opacity,
        additive,
        depth: sprite.depth,
        z_depth: state.z_depth,
        blur: 0.0,
    }
}

const ORIGIN_TOP_LEFT: u8 = 0;
const ORIGIN_CENTRE_LEFT: u8 = 3;

/// The signed determinant of the transform's upper-left 3x3.
fn determinant3(m: &Matrix4) -> f64 {
    let a = m.m[0];
    let b = m.m[4];
    let c = m.m[8];
    let d = m.m[1];
    let e = m.m[5];
    let f = m.m[9];
    let g = m.m[2];
    let h = m.m[6];
    let i = m.m[10];
    a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g)
}

/// The per-axis scale of an affine transform (column lengths).
pub(crate) fn extract_scale(m: &Matrix4) -> Vec3 {
    let column = |c: usize| {
        let (x, y, z) = (m.m[c * 4], m.m[c * 4 + 1], m.m[c * 4 + 2]);
        (x * x + y * y + z * z).sqrt()
    };
    Vec3::new(column(0), column(1), column(2))
}

/// Component-wise color multiply (inherited color), matching `CommandColor *`.
fn multiply_color(a: Color4, b: Color4) -> Color4 {
    Color4::new(a.r * b.r, a.g * b.g, a.b * b.b, a.a * b.a)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::Vec4;

    fn point_at(node: &Node3d, point: Vec3) -> Vec3 {
        let v = node
            .transform_at(0.0)
            .transform_vec4(Vec4::new(point.x, point.y, point.z, 1.0));
        Vec3::new(v.x, v.y, v.z)
    }

    #[test]
    fn pivot_scales_about_a_local_point() {
        let mut node = Node3d::default();
        node.scale.add(0.0, Vec3::new(2.0, 2.0, 2.0));
        // Without a pivot, the local origin maps to the node position (origin).
        assert!(point_at(&node, Vec3::ZERO).length() < 1e-9);
        node.pivot.add(0.0, Vec3::new(1.0, 0.0, 0.0));
        // The pivot point itself is invariant under rotation/scale.
        assert!(point_at(&node, Vec3::new(1.0, 0.0, 0.0)).length() < 1e-9);
        // The origin now maps to -(pivot * scale).
        assert!((point_at(&node, Vec3::ZERO).x + 2.0).abs() < 1e-9);
    }

    #[test]
    fn timeline_composes_an_extra_local_transform() {
        let mut node = Node3d::default();
        let mut timeline = TransformTimeline3d::new();
        let mut position = KeyframedValue::new();
        position.add(0.0, Vec3::new(2.0, 0.0, 0.0));
        timeline.position.add(
            crate::animations::keyframes::Channel::Vector3(position),
            0.0,
        );
        let mut opacity = KeyframedValue::new();
        opacity.add(0.0, 0.5);
        timeline
            .opacity
            .add(crate::animations::keyframes::Channel::Scalar(opacity), 0.0);
        node.timeline = Some(timeline);

        let state = state_of(&node, 0.0, &Object3dState::initial());
        let origin = state
            .world_transform
            .transform_vec4(Vec4::new(0.0, 0.0, 0.0, 1.0));
        assert!((origin.x - 2.0).abs() < 1e-9, "origin = {origin:?}");
        assert!((state.opacity - 0.5).abs() < 1e-9);
    }
}
