//! The 2D scene graph: a trait-based object hierarchy mirroring the 3D engine.
//!
//! `Object2d` is the base trait every 2D scene object implements: children, a
//! transform contribution (position/scale/rotation/opacity/color/anchor), and an
//! optional sprite leaf. `Node2d` is the transform-carrying node; `Sprite2d`
//! derives from it, adding the texture. The scene tree is a
//! `Vec<Box<dyn Object2d>>`, so [`Scene2d::resolve`] walks any mix of objects
//! polymorphically and samples the composed transforms into per-sprite `.osb`
//! command sets - the 2D counterpart of the 3D bake.

use crate::animations::keyframes::{Axis, Channel, FlattenMode};
use crate::animations::timeline::Timeline;
use crate::animations::{Easing, KeyframedValue};
use crate::math::{Color4, Vec2};

use super::helpers::apply_command_channel;
use super::{
    CommandType, KindedCommand, OsbLayer, OsbOrigin, ParameterTypeValue, SpriteTarget, Storyboard,
};

/// The base of every 2D scene object.
pub trait Object2d: std::fmt::Debug + Send + Sync {
    /// A label for debugging.
    fn name(&self) -> &str;
    /// The osu! layer this object's sprites are written to, if any.
    fn layer(&self) -> Option<OsbLayer>;
    /// The object's children.
    fn children(&self) -> &[Box<dyn Object2d>];
    /// Mutable access to the children.
    fn children_mut(&mut self) -> &mut Vec<Box<dyn Object2d>>;
    /// A boxed clone (so `Box<dyn Object2d>: Clone`).
    fn clone_box(&self) -> Box<dyn Object2d>;

    /// Adds a child object.
    fn add(&mut self, child: Box<dyn Object2d>) {
        self.children_mut().push(child);
    }

    /// The sprite leaf attached to this object, if any.
    fn sprite(&self) -> Option<&Sprite2d> {
        None
    }

    /// This object's transform node, if it has one (every built-in object does).
    fn node(&self) -> Option<&Node2d> {
        None
    }

    /// Mutable access to this object's transform node, if it has one.
    fn node_mut(&mut self) -> Option<&mut Node2d> {
        None
    }

    // -- transform contributions (identity by default; `Node2d` overrides) --

    /// The sampled translation in storyboard pixels.
    fn position_at(&self, _time: f64) -> Vec2 {
        Vec2::ZERO
    }
    /// The sampled per-axis scale.
    fn scale_at(&self, _time: f64) -> Vec2 {
        Vec2::ONE
    }
    /// The sampled rotation in radians.
    fn rotation_at(&self, _time: f64) -> f64 {
        0.0
    }
    /// The sampled opacity in `0..=1`.
    fn opacity_at(&self, _time: f64) -> f64 {
        1.0
    }
    /// The sampled tint.
    fn color_at(&self, _time: f64) -> Color4 {
        Color4::WHITE
    }
    /// The sampled rotation/scale pivot, in local pixels.
    fn anchor_at(&self, _time: f64) -> Vec2 {
        Vec2::ZERO
    }
    /// Whether children inherit this object's transform.
    fn inherits_transform(&self) -> bool {
        true
    }
    /// Whether children inherit this object's opacity.
    fn inherits_opacity(&self) -> bool {
        true
    }
    /// Whether children inherit this object's tint.
    fn inherits_color(&self) -> bool {
        true
    }

    /// The object's occupied `[start, end]` range (its transform, sprite and
    /// descendants), or `None` when empty.
    fn time_range(&self) -> Option<(f64, f64)> {
        let mut range = self.transform_range();
        if let Some(sprite) = self.sprite() {
            if let Some((start, end)) = sprite.transform_range() {
                range = Some(match range {
                    Some((lo, hi)) => (lo.min(start), hi.max(end)),
                    None => (start, end),
                });
            }
        }
        for child in self.children() {
            if let Some((start, end)) = child.time_range() {
                range = Some(match range {
                    Some((lo, hi)) => (lo.min(start), hi.max(end)),
                    None => (start, end),
                });
            }
        }
        range
    }

    /// This object's own transform range (without children).
    fn transform_range(&self) -> Option<(f64, f64)> {
        None
    }
}

impl Clone for Box<dyn Object2d> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// A node's per-field **timeline**: each 2D transform field is a [`Timeline`], so
/// a node's motion can be authored as sequenced clips (with nested groups) on top
/// of its static transform.
///
/// The timeline is an extra **local** transform composed after the node's own:
/// translation adds, scale multiplies, rotation adds, opacity/color multiply, and
/// a field with no active clip contributes identity.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TransformTimeline2d {
    /// Sequenced translation clips (2-vector channels).
    pub position: Timeline,
    /// Sequenced per-axis scale clips (2-vector channels).
    pub scale: Timeline,
    /// Sequenced rotation clips in radians (scalar channels).
    pub rotation: Timeline,
    /// Sequenced opacity clips (scalar channels).
    pub opacity: Timeline,
    /// Sequenced tint clips (color channels).
    pub color: Timeline,
    /// Sequenced pivot clips (2-vector channels).
    pub anchor: Timeline,
}

impl TransformTimeline2d {
    /// An empty transform timeline (every field contributes identity).
    pub fn new() -> Self {
        TransformTimeline2d::default()
    }

    /// The sampled translation at `time` (zero when no clip is active).
    pub fn position_at(&self, time: f64) -> Vec2 {
        self.position
            .channel_at(time)
            .and_then(|channel| channel.vec2_at(time))
            .unwrap_or(Vec2::ZERO)
    }

    /// The sampled scale at `time` (unit when no clip is active).
    pub fn scale_at(&self, time: f64) -> Vec2 {
        self.scale
            .channel_at(time)
            .and_then(|channel| channel.vec2_at(time))
            .unwrap_or(Vec2::ONE)
    }

    /// The sampled rotation at `time` in radians (zero when no clip is active).
    pub fn rotation_at(&self, time: f64) -> f64 {
        self.rotation
            .channel_at(time)
            .and_then(|channel| channel.scalar_at(time))
            .unwrap_or(0.0)
    }

    /// The sampled opacity at `time` (one when no clip is active).
    pub fn opacity_at(&self, time: f64) -> f64 {
        self.opacity
            .channel_at(time)
            .and_then(|channel| channel.scalar_at(time))
            .unwrap_or(1.0)
    }

    /// The sampled tint at `time` (white when no clip is active).
    pub fn color_at(&self, time: f64) -> Color4 {
        self.color
            .channel_at(time)
            .and_then(|channel| channel.color_at(time))
            .unwrap_or(Color4::WHITE)
    }

    /// The sampled pivot at `time` (zero when no clip is active).
    pub fn anchor_at(&self, time: f64) -> Vec2 {
        self.anchor
            .channel_at(time)
            .and_then(|channel| channel.vec2_at(time))
            .unwrap_or(Vec2::ZERO)
    }

    /// The extra local affine at `time`: `T(pos) * R * S * T(-anchor)`.
    pub fn local_at(&self, time: f64) -> Affine2 {
        Affine2::translation(self.position_at(time))
            .mul(&Affine2::rotation(self.rotation_at(time)))
            .mul(&Affine2::scaling(self.scale_at(time)))
            .mul(&Affine2::translation(-self.anchor_at(time)))
    }

    /// The timeline's occupied `[start, end]` range across its fields.
    pub fn time_range(&self) -> Option<(f64, f64)> {
        let mut range: Option<(f64, f64)> = None;
        for timeline in [
            &self.position,
            &self.scale,
            &self.rotation,
            &self.opacity,
            &self.color,
            &self.anchor,
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

    /// Whether every field is empty.
    pub fn is_empty(&self) -> bool {
        self.position.clip_count() == 0
            && self.scale.clip_count() == 0
            && self.rotation.clip_count() == 0
            && self.opacity.clip_count() == 0
            && self.color.clip_count() == 0
            && self.anchor.clip_count() == 0
    }

    /// Shifts every field in time.
    pub fn shift(&mut self, delta: f64) -> &mut Self {
        self.position.shift(delta);
        self.scale.shift(delta);
        self.rotation.shift(delta);
        self.opacity.shift(delta);
        self.color.shift(delta);
        self.anchor.shift(delta);
        self
    }

    /// Scales every field in time about `anchor`.
    pub fn scale_time(&mut self, factor: f64, anchor: f64) -> &mut Self {
        self.position.scale(factor, anchor);
        self.scale.scale(factor, anchor);
        self.rotation.scale(factor, anchor);
        self.opacity.scale(factor, anchor);
        self.color.scale(factor, anchor);
        self.anchor.scale(factor, anchor);
        self
    }

    /// Reverses every field in time about `[start, end]`.
    pub fn reverse(&mut self, start: f64, end: f64) -> &mut Self {
        self.position.reverse(start, end);
        self.scale.reverse(start, end);
        self.rotation.reverse(start, end);
        self.opacity.reverse(start, end);
        self.color.reverse(start, end);
        self.anchor.reverse(start, end);
        self
    }
}

/// A transform-carrying node: keyframe channels for position/scale/rotation/
/// opacity/color/anchor plus children (the base of every visual object).
#[derive(Debug, Clone)]
pub struct Node2d {
    /// A label for debugging.
    pub name: String,
    /// The osu! layer this node's sprites are written to when the scene is baked;
    /// `None` inherits the nearest ancestor's layer (or `Pass`).
    pub layer: Option<OsbLayer>,
    /// Translation in storyboard pixels (a 2-vector channel).
    pub position: Channel,
    /// Per-axis scale (a 2-vector channel).
    pub scale: Channel,
    /// Rotation in radians (a scalar channel).
    pub rotation: Channel,
    /// Opacity in `0..=1` (a scalar channel).
    pub opacity: Channel,
    /// Tint color (a color channel).
    pub color: Channel,
    /// The rotation/scale pivot, in local pixels (a 2-vector channel).
    pub anchor: Channel,
    /// The node's per-field timeline, composed as an extra local transform.
    pub timeline: Option<TransformTimeline2d>,
    /// The node's children (any mix of object types).
    pub children: Vec<Box<dyn Object2d>>,
    /// Whether children inherit this node's transform.
    pub inherits_transform: bool,
    /// Whether children inherit this node's opacity.
    pub inherits_opacity: bool,
    /// Whether children inherit this node's tint.
    pub inherits_color: bool,
}

impl Default for Node2d {
    fn default() -> Self {
        Node2d {
            name: String::new(),
            layer: None,
            position: constant_vec2(Vec2::ZERO),
            scale: constant_vec2(Vec2::ONE),
            rotation: constant_scalar(0.0),
            opacity: constant_scalar(1.0),
            color: constant_color(Color4::WHITE),
            anchor: constant_vec2(Vec2::ZERO),
            timeline: None,
            children: Vec::new(),
            inherits_transform: true,
            inherits_opacity: true,
            inherits_color: true,
        }
    }
}

impl Node2d {
    /// A named empty node.
    pub fn new(name: impl Into<String>) -> Self {
        Node2d {
            name: name.into(),
            ..Node2d::default()
        }
    }

    /// Sets this node's per-field timeline and returns it.
    pub fn with_timeline(mut self, timeline: TransformTimeline2d) -> Self {
        self.timeline = Some(timeline);
        self
    }

    /// Mutable access to this node's timeline, inserting an empty one if absent.
    pub fn timeline_mut(&mut self) -> &mut TransformTimeline2d {
        self.timeline.get_or_insert_with(TransformTimeline2d::new)
    }

    // -- top-down value ops (cascade through a node) ------------------------

    /// Translates the node by `delta` pixels.
    pub fn translate(&mut self, delta: Vec2) -> &mut Self {
        self.position = self.position.translate(delta);
        self
    }

    /// Scales the node about its origin by `factor` (position, scale and anchor
    /// all scale, so a group spreads out).
    pub fn scale_by(&mut self, factor: f64) -> &mut Self {
        self.scale = self.scale.scale(factor);
        self.position = self.position.scale(factor);
        self.anchor = self.anchor.scale(factor);
        self
    }

    /// Adds `radians` to the rotation.
    pub fn rotate(&mut self, radians: f64) -> &mut Self {
        self.rotation = self.rotation.offset(radians);
        self
    }

    /// Sets a constant opacity.
    pub fn set_opacity(&mut self, value: f64) -> &mut Self {
        self.opacity = constant_scalar(value);
        self
    }

    /// Sets a constant tint color.
    pub fn set_color(&mut self, color: Color4) -> &mut Self {
        self.color = constant_color(color);
        self
    }

    /// Flattens every field (or one axis) to a constant, and every descendant.
    pub fn flatten(&mut self, axis: Option<Axis>, mode: FlattenMode) -> &mut Self {
        self.position = self.position.flatten(axis, mode);
        self.scale = self.scale.flatten(axis, mode);
        self.rotation = self.rotation.flatten(axis, mode);
        self.opacity = self.opacity.flatten(axis, mode);
        self.color = self.color.flatten(axis, mode);
        self.anchor = self.anchor.flatten(axis, mode);
        for child in &mut self.children {
            child_flatten(child.as_mut(), axis, mode);
        }
        self
    }

    /// Re-eases every field, and every descendant.
    pub fn ease(&mut self, easing: Easing, samples: usize) -> &mut Self {
        self.position = self.position.ease(easing, samples);
        self.scale = self.scale.ease(easing, samples);
        self.rotation = self.rotation.ease(easing, samples);
        self.opacity = self.opacity.ease(easing, samples);
        self.color = self.color.ease(easing, samples);
        self.anchor = self.anchor.ease(easing, samples);
        for child in &mut self.children {
            child_ease(child.as_mut(), easing, samples);
        }
        self
    }

    /// Seeded jitter of every field.
    pub fn randomize(&mut self, seed: u64, min: f64, max: f64) -> &mut Self {
        let jitter = |channel: &Channel| channel.randomize(None, min, max, seed, 0.0);
        self.position = jitter(&self.position);
        self.scale = jitter(&self.scale);
        self.rotation = jitter(&self.rotation);
        self.opacity = jitter(&self.opacity);
        self.color = jitter(&self.color);
        self.anchor = jitter(&self.anchor);
        self
    }

    // -- time ops -----------------------------------------------------------

    /// Shifts this node and every descendant in time.
    pub fn shift(&mut self, delta: f64) -> &mut Self {
        self.position = self.position.shift(delta);
        self.scale = self.scale.shift(delta);
        self.rotation = self.rotation.shift(delta);
        self.opacity = self.opacity.shift(delta);
        self.color = self.color.shift(delta);
        self.anchor = self.anchor.shift(delta);
        if let Some(timeline) = &mut self.timeline {
            timeline.shift(delta);
        }
        for child in &mut self.children {
            child_shift(child.as_mut(), delta);
        }
        self
    }

    /// Scales this node and every descendant in time.
    pub fn scale_time(&mut self, factor: f64, anchor: f64) -> &mut Self {
        self.position = self.position.scale_time(factor, anchor);
        self.scale = self.scale.scale_time(factor, anchor);
        self.rotation = self.rotation.scale_time(factor, anchor);
        self.opacity = self.opacity.scale_time(factor, anchor);
        self.color = self.color.scale_time(factor, anchor);
        self.anchor = self.anchor.scale_time(factor, anchor);
        if let Some(timeline) = &mut self.timeline {
            timeline.scale_time(factor, anchor);
        }
        for child in &mut self.children {
            child_scale_time(child.as_mut(), factor, anchor);
        }
        self
    }

    /// Reverses this node and every descendant in time.
    pub fn reverse(&mut self, start: f64, end: f64) -> &mut Self {
        self.position = self.position.reverse(start, end);
        self.scale = self.scale.reverse(start, end);
        self.rotation = self.rotation.reverse(start, end);
        self.opacity = self.opacity.reverse(start, end);
        self.color = self.color.reverse(start, end);
        self.anchor = self.anchor.reverse(start, end);
        if let Some(timeline) = &mut self.timeline {
            timeline.reverse(start, end);
        }
        for child in &mut self.children {
            child_reverse(child.as_mut(), start, end);
        }
        self
    }

    /// The node's local affine at `time` (its channels composed with its timeline).
    pub fn local_at(&self, time: f64) -> Affine2 {
        let base = Affine2::translation(self.position_at(time))
            .mul(&Affine2::rotation(self.rotation_at(time)))
            .mul(&Affine2::scaling(self.scale_at(time)))
            .mul(&Affine2::translation(-self.anchor_at(time)));
        match &self.timeline {
            Some(timeline) => base.mul(&timeline.local_at(time)),
            None => base,
        }
    }

    /// The node's own transform range (without children).
    pub fn own_time_range(&self) -> Option<(f64, f64)> {
        let mut range: Option<(f64, f64)> = None;
        for channel in [
            &self.position,
            &self.scale,
            &self.rotation,
            &self.opacity,
            &self.color,
            &self.anchor,
        ] {
            if let Some((start, end)) = channel.time_range() {
                range = Some(match range {
                    Some((lo, hi)) => (lo.min(start), hi.max(end)),
                    None => (start, end),
                });
            }
        }
        if let Some(timeline) = &self.timeline {
            if let Some((start, end)) = timeline.time_range() {
                range = Some(match range {
                    Some((lo, hi)) => (lo.min(start), hi.max(end)),
                    None => (start, end),
                });
            }
        }
        range
    }
}

impl Object2d for Node2d {
    fn name(&self) -> &str {
        &self.name
    }
    fn layer(&self) -> Option<OsbLayer> {
        self.layer
    }
    fn children(&self) -> &[Box<dyn Object2d>] {
        &self.children
    }
    fn children_mut(&mut self) -> &mut Vec<Box<dyn Object2d>> {
        &mut self.children
    }
    fn clone_box(&self) -> Box<dyn Object2d> {
        Box::new(self.clone())
    }
    fn node(&self) -> Option<&Node2d> {
        Some(self)
    }
    fn node_mut(&mut self) -> Option<&mut Node2d> {
        Some(self)
    }
    fn position_at(&self, time: f64) -> Vec2 {
        self.position.vec2_at(time).unwrap_or(Vec2::ZERO)
    }
    fn scale_at(&self, time: f64) -> Vec2 {
        self.scale.vec2_at(time).unwrap_or(Vec2::ONE)
    }
    fn rotation_at(&self, time: f64) -> f64 {
        self.rotation.scalar_at(time).unwrap_or(0.0)
    }
    fn opacity_at(&self, time: f64) -> f64 {
        let opacity = self.opacity.scalar_at(time).unwrap_or(1.0);
        match &self.timeline {
            Some(timeline) => opacity * timeline.opacity_at(time),
            None => opacity,
        }
    }
    fn color_at(&self, time: f64) -> Color4 {
        let color = self.color.color_at(time).unwrap_or(Color4::WHITE);
        match &self.timeline {
            Some(timeline) => multiply_color(color, timeline.color_at(time)),
            None => color,
        }
    }
    fn anchor_at(&self, time: f64) -> Vec2 {
        self.anchor.vec2_at(time).unwrap_or(Vec2::ZERO)
    }
    fn inherits_transform(&self) -> bool {
        self.inherits_transform
    }
    fn inherits_opacity(&self) -> bool {
        self.inherits_opacity
    }
    fn inherits_color(&self) -> bool {
        self.inherits_color
    }
    fn transform_range(&self) -> Option<(f64, f64)> {
        self.own_time_range()
    }
}

/// A sprite leaf: a [`Node2d`] with a texture path, origin and additive flag.
#[derive(Debug, Clone)]
pub struct Sprite2d {
    /// The sprite's transform node.
    pub node: Node2d,
    /// The sprite texture path (relative to the mapset).
    pub texture_path: String,
    /// The sprite anchor origin.
    pub origin: OsbOrigin,
    /// Whether the sprite starts additive.
    pub additive: bool,
    /// Extra non-transform commands appended verbatim to the resolved output.
    pub commands: Vec<KindedCommand>,
}

impl Sprite2d {
    /// A sprite at the identity transform.
    pub fn new(texture_path: impl Into<String>, origin: OsbOrigin) -> Self {
        Sprite2d {
            node: Node2d::default(),
            texture_path: texture_path.into(),
            origin,
            additive: false,
            commands: Vec::new(),
        }
    }
}

impl Object2d for Sprite2d {
    fn name(&self) -> &str {
        &self.node.name
    }
    fn layer(&self) -> Option<OsbLayer> {
        self.node.layer
    }
    fn children(&self) -> &[Box<dyn Object2d>] {
        &self.node.children
    }
    fn children_mut(&mut self) -> &mut Vec<Box<dyn Object2d>> {
        &mut self.node.children
    }
    fn clone_box(&self) -> Box<dyn Object2d> {
        Box::new(self.clone())
    }
    fn sprite(&self) -> Option<&Sprite2d> {
        Some(self)
    }
    fn node(&self) -> Option<&Node2d> {
        Some(&self.node)
    }
    fn node_mut(&mut self) -> Option<&mut Node2d> {
        Some(&mut self.node)
    }
    fn position_at(&self, time: f64) -> Vec2 {
        self.node.position_at(time)
    }
    fn scale_at(&self, time: f64) -> Vec2 {
        self.node.scale_at(time)
    }
    fn rotation_at(&self, time: f64) -> f64 {
        self.node.rotation_at(time)
    }
    fn opacity_at(&self, time: f64) -> f64 {
        self.node.opacity_at(time)
    }
    fn color_at(&self, time: f64) -> Color4 {
        self.node.color_at(time)
    }
    fn anchor_at(&self, time: f64) -> Vec2 {
        self.node.anchor_at(time)
    }
    fn inherits_transform(&self) -> bool {
        self.node.inherits_transform
    }
    fn inherits_opacity(&self) -> bool {
        self.node.inherits_opacity
    }
    fn inherits_color(&self) -> bool {
        self.node.inherits_color
    }
    fn transform_range(&self) -> Option<(f64, f64)> {
        self.node.own_time_range()
    }
}

fn child_flatten(object: &mut dyn Object2d, axis: Option<Axis>, mode: FlattenMode) {
    if let Some(node) = object.node_mut() {
        node.flatten(axis, mode);
    }
}

fn child_ease(object: &mut dyn Object2d, easing: Easing, samples: usize) {
    if let Some(node) = object.node_mut() {
        node.ease(easing, samples);
    }
}

fn child_shift(object: &mut dyn Object2d, delta: f64) {
    if let Some(node) = object.node_mut() {
        node.shift(delta);
    }
}

fn child_scale_time(object: &mut dyn Object2d, factor: f64, anchor: f64) {
    if let Some(node) = object.node_mut() {
        node.scale_time(factor, anchor);
    }
}

fn child_reverse(object: &mut dyn Object2d, start: f64, end: f64) {
    if let Some(node) = object.node_mut() {
        node.reverse(start, end);
    }
}

/// A 2D camera: a keyframed view transform mapping world coordinates to screen.
///
/// The view at `time` is `screen = anchor + R(rotation) * (zoom * (world -
/// position))`, so a sprite's resolved transform is the camera view composed with
/// the scene's node transforms (the camera is the root of the scene).
#[derive(Debug, Clone, PartialEq)]
pub struct Camera2d {
    /// The world point that maps to `anchor`.
    pub position: Channel,
    /// Per-axis zoom (a scalar broadcasts).
    pub zoom: Channel,
    /// Rotation in radians.
    pub rotation: Channel,
    /// The screen point the camera position maps to.
    pub anchor: Channel,
}

impl Default for Camera2d {
    fn default() -> Self {
        Camera2d::identity()
    }
}

impl Camera2d {
    /// A camera at the origin with unit zoom and no rotation.
    pub fn identity() -> Self {
        Camera2d {
            position: constant_vec2(Vec2::ZERO),
            zoom: constant_vec2(Vec2::ONE),
            rotation: constant_scalar(0.0),
            anchor: constant_vec2(Vec2::ZERO),
        }
    }

    /// The world -> screen view transform at `time`.
    pub fn view_at(&self, time: f64) -> Affine2 {
        let position = self.position.vec2_at(time).unwrap_or(Vec2::ZERO);
        let zoom = self.zoom.vec2_at(time).unwrap_or(Vec2::ONE);
        let rotation = self.rotation.scalar_at(time).unwrap_or(0.0);
        let anchor = self.anchor.vec2_at(time).unwrap_or(Vec2::ZERO);
        Affine2::translation(anchor)
            .mul(&Affine2::rotation(rotation))
            .mul(&Affine2::scaling(zoom))
            .mul(&Affine2::translation(-position))
    }

    /// The camera's occupied `[start, end]` range across its channels.
    pub fn time_range(&self) -> Option<(f64, f64)> {
        let mut range: Option<(f64, f64)> = None;
        for channel in [&self.position, &self.zoom, &self.rotation, &self.anchor] {
            if let Some((start, end)) = channel.time_range() {
                range = Some(match range {
                    Some((lo, hi)) => (lo.min(start), hi.max(end)),
                    None => (start, end),
                });
            }
        }
        range
    }
}

/// Options controlling the 2D scene resolution.
#[derive(Debug, Clone)]
pub struct Scene2dOptions {
    /// The sample range start; when `end_time <= start_time` the scene's natural
    /// range is used instead.
    pub start_time: f64,
    /// The sample range end.
    pub end_time: f64,
    /// The fixed sampling step in milliseconds.
    pub time_step: f64,
    /// Douglas-Peucker tolerance for position/scale, in pixels.
    pub tolerance_2d: f64,
    /// Tolerance for rotation/opacity.
    pub tolerance_1d: f64,
    /// Decimal places each simplified value is rounded to.
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

/// A sprite resolved from the scene: its texture, origin, layer and command set.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedSprite2d {
    /// The sprite texture path.
    pub texture_path: String,
    /// The sprite anchor origin.
    pub origin: OsbOrigin,
    /// The osu! layer the sprite is written to.
    pub layer: OsbLayer,
    /// Whether the sprite is additive.
    pub additive: bool,
    /// The resolved commands, in time order.
    pub commands: Vec<KindedCommand>,
    /// The resolved `[start, end]` time range.
    pub time_range: (f64, f64),
}

/// A sprite resolved from the scene as keyframe **channels** (before they are
/// flattened to commands). This is what the 3D embed consumes, so a 2D sprite can
/// be placed in a 3D scene with its motion intact.
#[derive(Debug, Clone)]
pub struct ResolvedChannels2d {
    /// The sprite texture path.
    pub texture_path: String,
    /// The sprite anchor origin.
    pub origin: OsbOrigin,
    /// The osu! layer the sprite is written to.
    pub layer: OsbLayer,
    /// Whether the sprite is additive.
    pub additive: bool,
    /// Resolved position.
    pub position: KeyframedValue<Vec2>,
    /// Resolved scale.
    pub scale: KeyframedValue<Vec2>,
    /// Resolved rotation (radians).
    pub rotation: KeyframedValue<f64>,
    /// Resolved opacity.
    pub opacity: KeyframedValue<f64>,
    /// Resolved tint.
    pub color: KeyframedValue<Color4>,
    /// Extra non-transform commands (e.g. additive parameters).
    pub extra_commands: Vec<KindedCommand>,
    /// The resolved `[start, end]` time range.
    pub time_range: (f64, f64),
}

/// A 2D scene graph (a "world"): a root [`Node2d`], a [`Camera2d`] and a time range.
#[derive(Debug, Clone, Default)]
pub struct Scene2d {
    /// A label for the scene/world.
    pub name: String,
    /// The scene's root node.
    pub root: Node2d,
    /// The scene's camera (the view transform applied to every sprite).
    pub camera: Camera2d,
    /// The scene's start time (used when the options leave the range open).
    pub start_time: f64,
    /// The scene's end time.
    pub end_time: f64,
}

/// A 2D world is a [`Scene2d`] (the 2D counterpart of [`crate::storyboarding3d::World3d`]).
pub type World2d = Scene2d;

impl Scene2d {
    /// An empty scene with an identity root and camera.
    pub fn new() -> Self {
        Scene2d {
            name: String::new(),
            root: Node2d::default(),
            camera: Camera2d::identity(),
            start_time: 0.0,
            end_time: 0.0,
        }
    }

    /// A scene rooted at `root`.
    pub fn from_root(root: Node2d) -> Self {
        Scene2d {
            root,
            ..Scene2d::new()
        }
    }

    /// A named scene/world.
    pub fn named(name: impl Into<String>) -> Self {
        Scene2d {
            name: name.into(),
            ..Scene2d::new()
        }
    }

    /// Adds a child to the root.
    pub fn add(&mut self, child: Box<dyn Object2d>) -> &mut Self {
        self.root.add(child);
        self
    }

    /// The scene's occupied `[start, end]` range (nodes + camera).
    pub fn time_range(&self) -> Option<(f64, f64)> {
        let mut range = self.root.time_range();
        if let Some((start, end)) = self.camera.time_range() {
            range = Some(match range {
                Some((lo, hi)) => (lo.min(start), hi.max(end)),
                None => (start, end),
            });
        }
        range
    }

    /// Resolves the scene into per-sprite command sets by sampling the composed
    /// transforms (camera + nodes) over the sample range.
    pub fn resolve(&self, options: &Scene2dOptions) -> Vec<ResolvedSprite2d> {
        self.resolve_channels(options)
            .iter()
            .map(channels_to_resolved_sprite)
            .collect()
    }

    /// Resolves the scene into per-sprite keyframe channels (camera applied).
    pub fn resolve_channels(&self, options: &Scene2dOptions) -> Vec<ResolvedChannels2d> {
        self.resolve_channels_with(&self.camera, options)
    }

    /// Resolves the scene into per-sprite keyframe channels with an explicit
    /// camera (the 3D embed uses an identity camera to get local transforms).
    pub fn resolve_channels_with(
        &self,
        camera: &Camera2d,
        options: &Scene2dOptions,
    ) -> Vec<ResolvedChannels2d> {
        let times = self.sample_times(options);
        let mut leaves = Vec::new();
        collect_leaves(&self.root, None, &mut Vec::new(), &mut leaves);
        leaves
            .into_iter()
            .map(|(chain, sprite, layer)| {
                resolve_leaf(camera, &chain, sprite, layer, &times, options)
            })
            .collect()
    }

    /// The sample times for `options` (the scene's range when the options are open).
    pub fn sample_times(&self, options: &Scene2dOptions) -> Vec<f64> {
        let (start, end) = if options.end_time > options.start_time {
            (options.start_time, options.end_time)
        } else if self.end_time > self.start_time {
            (self.start_time, self.end_time)
        } else {
            self.time_range().unwrap_or((0.0, 0.0))
        };
        let step = options.time_step.max(1.0);
        let mut times = Vec::new();
        if end > start {
            let mut t = start;
            while t < end {
                times.push(t);
                t += step;
            }
            times.push(end);
        } else {
            times.push(start);
            times.push(start + step);
        }
        times
    }

    /// Bakes the scene into a fresh [`Storyboard`], writing each sprite onto the
    /// osu! layer of the node it came from.
    pub fn bake(&self, options: &Scene2dOptions) -> Storyboard {
        let mut storyboard = Storyboard::new();
        self.bake_into(&mut storyboard, options);
        storyboard
    }

    /// Bakes the scene into an existing [`Storyboard`] (so several worlds compose).
    pub fn bake_into(&self, storyboard: &mut Storyboard, options: &Scene2dOptions) {
        for resolved in self.resolve(options) {
            let handle =
                storyboard.create_sprite(resolved.layer, resolved.texture_path, resolved.origin);
            let sprite = storyboard.sprite_mut(handle);
            sprite.additive = resolved.additive;
            let commands = sprite.commands_mut();
            for (kind, command) in resolved.commands {
                commands.entry(kind).or_default().push(command);
            }
        }
    }

    /// Resolves the scene and writes each sprite onto `target` in `storyboard`,
    /// overriding the per-node layers.
    pub fn write_to<S: SpriteTarget + Copy>(
        &self,
        storyboard: &mut Storyboard,
        target: S,
        options: &Scene2dOptions,
    ) {
        for resolved in self.resolve(options) {
            let handle = storyboard.create_sprite(target, resolved.texture_path, resolved.origin);
            let sprite = storyboard.sprite_mut(handle);
            sprite.additive = resolved.additive;
            let commands = sprite.commands_mut();
            for (kind, command) in resolved.commands {
                commands.entry(kind).or_default().push(command);
            }
        }
    }
}

/// A leaf's chain of objects, its sprite and its inherited osu! layer.
type Leaf<'a> = (Vec<&'a dyn Object2d>, &'a Sprite2d, OsbLayer);

fn collect_leaves<'a>(
    object: &'a dyn Object2d,
    inherited_layer: Option<OsbLayer>,
    chain: &mut Vec<&'a dyn Object2d>,
    out: &mut Vec<Leaf<'a>>,
) {
    let layer = object.layer().or(inherited_layer);
    chain.push(object);
    if let Some(sprite) = object.sprite() {
        out.push((chain.clone(), sprite, layer.unwrap_or_default()));
    }
    for child in object.children() {
        collect_leaves(child.as_ref(), layer, chain, out);
    }
    chain.pop();
}

fn resolve_leaf(
    camera: &Camera2d,
    chain: &[&dyn Object2d],
    sprite: &Sprite2d,
    layer: OsbLayer,
    times: &[f64],
    options: &Scene2dOptions,
) -> ResolvedChannels2d {
    let mut position = KeyframedValue::new();
    let mut scale = KeyframedValue::new();
    let mut rotation = KeyframedValue::new();
    let mut opacity = KeyframedValue::new();
    let mut color = KeyframedValue::new();

    for &time in times {
        // The camera is the root of the scene; then the chain of node transforms
        // (each composed with its timeline), ending at the sprite's own node.
        let mut world = camera.view_at(time);
        let mut accumulated_opacity = 1.0;
        let mut accumulated_color = Color4::WHITE;
        for object in chain {
            world = world.mul(&local_of(*object, time));
            accumulated_opacity *= object.opacity_at(time);
            accumulated_color = multiply_color(accumulated_color, object.color_at(time));
        }

        // The sprite's local origin (its pivot) is at the sprite's `anchor`, which
        // the renderer resolves as `origin + anchor`; the resolved position is that
        // pivot in world space.
        let sprite_anchor = sprite.node.anchor_at(time);
        let (sprite_rotation, sprite_scale) = decompose(world.m);
        position.add(time, world.apply(sprite_anchor));
        scale.add(time, sprite_scale);
        rotation.add(time, sprite_rotation);
        opacity.add(time, accumulated_opacity);
        color.add(time, accumulated_color);
    }

    position.simplify_2d(options.tolerance_2d, options.decimals);
    scale.simplify_2d(options.tolerance_2d, options.decimals);
    rotation.simplify_1d(options.tolerance_1d, options.decimals);
    opacity.simplify_1d(options.tolerance_1d, options.decimals);
    color.simplify_color(options.tolerance_1d, options.decimals);

    let mut extra_commands: Vec<KindedCommand> = Vec::new();
    if sprite.additive {
        extra_commands.push((
            CommandType::Parameter,
            super::Command::new(
                times.first().copied().unwrap_or(0.0),
                times.last().copied().unwrap_or(0.0),
                Easing::None,
                super::CommandValue::Parameter(ParameterTypeValue::Additive(1.0)),
                super::CommandValue::Parameter(ParameterTypeValue::Additive(1.0)),
            ),
        ));
    }
    extra_commands.extend(sprite.commands.iter().copied());

    let time_range = (
        times.first().copied().unwrap_or(0.0),
        times.last().copied().unwrap_or(0.0),
    );
    ResolvedChannels2d {
        texture_path: sprite.texture_path.clone(),
        origin: sprite.origin,
        layer,
        additive: sprite.additive,
        position,
        scale,
        rotation,
        opacity,
        color,
        extra_commands,
        time_range,
    }
}

/// The local affine of an object at `time` (a node's channels + timeline).
fn local_of(object: &dyn Object2d, time: f64) -> Affine2 {
    let base = Affine2::translation(object.position_at(time))
        .mul(&Affine2::rotation(object.rotation_at(time)))
        .mul(&Affine2::scaling(object.scale_at(time)))
        .mul(&Affine2::translation(-object.anchor_at(time)));
    match object.node() {
        Some(node) => match &node.timeline {
            Some(timeline) => base.mul(&timeline.local_at(time)),
            None => base,
        },
        None => base,
    }
}

/// Flattens a resolved channel set into `.osb`-compatible commands.
pub fn channels_to_resolved_sprite(resolved: &ResolvedChannels2d) -> ResolvedSprite2d {
    let mut commands: Vec<KindedCommand> = Vec::new();
    commands.extend(apply_command_channel(
        &[],
        CommandType::Move,
        &Channel::Vector2(resolved.position.clone()),
    ));
    commands.extend(apply_command_channel(
        &[],
        CommandType::ScaleVec,
        &Channel::Vector2(resolved.scale.clone()),
    ));
    commands.extend(apply_command_channel(
        &[],
        CommandType::Rotate,
        &Channel::Scalar(resolved.rotation.clone()),
    ));
    commands.extend(apply_command_channel(
        &[],
        CommandType::Fade,
        &Channel::Scalar(resolved.opacity.clone()),
    ));
    commands.extend(apply_command_channel(
        &[],
        CommandType::Color,
        &Channel::Color(resolved.color.clone()),
    ));
    commands.extend(resolved.extra_commands.iter().copied());
    commands.sort_by(|a, b| a.1.start_time.total_cmp(&b.1.start_time));
    ResolvedSprite2d {
        texture_path: resolved.texture_path.clone(),
        origin: resolved.origin,
        layer: resolved.layer,
        additive: resolved.additive,
        commands,
        time_range: resolved.time_range,
    }
}

/// A 2D affine transform: a 2x2 matrix `m` (column-major: `[m00, m01, m10, m11]`)
/// plus a translation `t`. `apply(p) = m * p + t`.
#[derive(Debug, Clone, Copy)]
pub struct Affine2 {
    m: [f64; 4],
    t: Vec2,
}

impl Affine2 {
    fn translation(t: Vec2) -> Self {
        Affine2 {
            m: [1.0, 0.0, 0.0, 1.0],
            t,
        }
    }

    fn rotation(theta: f64) -> Self {
        let (sin, cos) = theta.sin_cos();
        Affine2 {
            m: [cos, -sin, sin, cos],
            t: Vec2::ZERO,
        }
    }

    fn scaling(scale: Vec2) -> Self {
        Affine2 {
            m: [scale.x, 0.0, 0.0, scale.y],
            t: Vec2::ZERO,
        }
    }

    /// `self ∘ other` (apply `other` first, then `self`).
    fn mul(self, other: &Affine2) -> Self {
        let a = self.m;
        let b = other.m;
        let m = [
            a[0] * b[0] + a[1] * b[2],
            a[0] * b[1] + a[1] * b[3],
            a[2] * b[0] + a[3] * b[2],
            a[2] * b[1] + a[3] * b[3],
        ];
        let t = Vec2::new(
            a[0] * other.t.x + a[1] * other.t.y + self.t.x,
            a[2] * other.t.x + a[3] * other.t.y + self.t.y,
        );
        Affine2 { m, t }
    }

    fn apply(&self, point: Vec2) -> Vec2 {
        Vec2::new(
            self.m[0] * point.x + self.m[1] * point.y + self.t.x,
            self.m[2] * point.x + self.m[3] * point.y + self.t.y,
        )
    }
}

/// Decomposes a 2x2 matrix into `(rotation, scale)`, anchoring on the longer
/// projected axis and dropping any residual shear.
fn decompose(m: [f64; 4]) -> (f64, Vec2) {
    let column_x = Vec2::new(m[0], m[2]);
    let column_y = Vec2::new(m[1], m[3]);
    let len_x = column_x.length();
    let len_y = column_y.length();
    let determinant = m[0] * m[3] - m[1] * m[2];
    if len_x >= len_y && len_x > 1e-12 {
        let rotation = column_x.y.atan2(column_x.x);
        (rotation, Vec2::new(len_x, determinant / len_x))
    } else if len_y > 1e-12 {
        let rotation = column_y.y.atan2(column_y.x) - std::f64::consts::FRAC_PI_2;
        (rotation, Vec2::new(determinant / len_y, len_y))
    } else {
        (0.0, Vec2::new(len_x, len_y))
    }
}

/// Component-wise color multiply (inherited tint).
fn multiply_color(a: Color4, b: Color4) -> Color4 {
    Color4::new(a.r * b.r, a.g * b.g, a.b * b.b, a.a * b.a)
}

fn constant_scalar(value: f64) -> Channel {
    let mut kv = KeyframedValue::new();
    kv.add(0.0, value);
    Channel::Scalar(kv)
}

fn constant_vec2(value: Vec2) -> Channel {
    let mut kv = KeyframedValue::new();
    kv.add(0.0, value);
    Channel::Vector2(kv)
}

fn constant_color(value: Color4) -> Channel {
    let mut kv = KeyframedValue::new();
    kv.add(0.0, value);
    Channel::Color(kv)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ramp_position(from: Vec2, to: Vec2, end: f64) -> Channel {
        let mut kv = KeyframedValue::new();
        kv.add(0.0, from);
        kv.add(end, to);
        Channel::Vector2(kv)
    }

    fn sample_move(commands: &[KindedCommand], time: f64) -> Vec2 {
        let command = commands
            .iter()
            .find(|(kind, _)| *kind == CommandType::Move)
            .map(|(_, command)| command)
            .expect("a move command");
        command.value_at(time).as_vec2()
    }

    fn sprite_with_position(texture: &str, from: Vec2, to: Vec2, end: f64) -> Box<dyn Object2d> {
        let mut sprite = Sprite2d::new(texture, OsbOrigin::Centre);
        sprite.node.position = ramp_position(from, to, end);
        Box::new(sprite)
    }

    #[test]
    fn a_leaf_sprite_resolves_to_move_scale_and_fade() {
        let mut root = Node2d::default();
        root.add(sprite_with_position(
            "sb/dot.png",
            Vec2::ZERO,
            Vec2::new(100.0, 0.0),
            100.0,
        ));
        let scene = Scene2d::from_root(root);
        let resolved = scene.resolve(&Scene2dOptions::default());
        assert_eq!(resolved.len(), 1);
        let start = sample_move(&resolved[0].commands, 0.0);
        let end = sample_move(&resolved[0].commands, 100.0);
        assert!((end.x - 100.0).abs() < 1e-6, "end = {end:?}");
        assert!(start.x.abs() < 1e-6);
    }

    #[test]
    fn a_parent_translate_cascades_into_children() {
        let mut root = Node2d::new("root");
        root.position = ramp_position(Vec2::ZERO, Vec2::new(0.0, 50.0), 100.0);
        root.add(sprite_with_position(
            "sb/dot.png",
            Vec2::ZERO,
            Vec2::new(100.0, 0.0),
            100.0,
        ));

        let scene = Scene2d::from_root(root);
        let resolved = scene.resolve(&Scene2dOptions::default());
        let end = sample_move(&resolved[0].commands, 100.0);
        // Child moves +100x locally; the parent adds +50y.
        assert!((end.x - 100.0).abs() < 1e-4, "end = {end:?}");
        assert!((end.y - 50.0).abs() < 1e-4, "end = {end:?}");
    }

    #[test]
    fn parent_scale_composes_into_child_scale() {
        let mut root = Node2d::new("root");
        {
            let mut kv = KeyframedValue::new();
            kv.add(0.0, Vec2::new(1.0, 1.0));
            kv.add(100.0, Vec2::new(2.0, 2.0));
            root.scale = Channel::Vector2(kv);
        }
        root.add(Box::new(Sprite2d::new("sb/dot.png", OsbOrigin::Centre)));
        let scene = Scene2d::from_root(root);
        let resolved = scene.resolve(&Scene2dOptions::default());
        let command = resolved[0]
            .commands
            .iter()
            .find(|(kind, _)| *kind == CommandType::ScaleVec)
            .map(|(_, command)| command)
            .unwrap();
        assert!((command.value_at(100.0).as_vec2().x - 2.0).abs() < 1e-4);
    }

    #[test]
    fn top_down_control_reaches_every_descendant() {
        let mut root = Node2d::new("root");
        root.add(Box::new(Sprite2d::new("sb/a.png", OsbOrigin::Centre)));
        let mut child = Node2d::new("child");
        child.add(Box::new(Sprite2d::new("sb/b.png", OsbOrigin::Centre)));
        root.add(Box::new(child));
        // Translating the root shifts both leaves' resolved positions.
        root.translate(Vec2::new(20.0, 10.0));
        let scene = Scene2d::from_root(root);
        let resolved = scene.resolve(&Scene2dOptions::default());
        assert_eq!(resolved.len(), 2);
        for sprite in &resolved {
            let start = sample_move(&sprite.commands, 0.0);
            assert!((start.x - 20.0).abs() < 1e-6);
            assert!((start.y - 10.0).abs() < 1e-6);
        }
    }

    #[test]
    fn flatten_collapses_the_whole_scene() {
        let mut root = Node2d::default();
        root.add(sprite_with_position(
            "sb/dot.png",
            Vec2::ZERO,
            Vec2::new(100.0, 0.0),
            100.0,
        ));
        root.flatten(None, FlattenMode::Last);
        let scene = Scene2d::from_root(root);
        let resolved = scene.resolve(&Scene2dOptions::default());
        let moves = resolved[0]
            .commands
            .iter()
            .filter(|(kind, _)| *kind == CommandType::Move)
            .count();
        assert_eq!(moves, 1);
    }

    fn ramp_timeline(from: Vec2, to: Vec2, end: f64) -> Timeline {
        let mut timeline = Timeline::new();
        timeline.add(ramp_position(from, to, end), 0.0);
        timeline
    }

    #[test]
    fn a_node_timeline_composes_an_extra_local_transform() {
        let mut root = Node2d {
            position: ramp_position(Vec2::ZERO, Vec2::new(50.0, 0.0), 100.0),
            ..Node2d::default()
        };
        let mut timeline = TransformTimeline2d::new();
        timeline.position = ramp_timeline(Vec2::ZERO, Vec2::new(30.0, 0.0), 100.0);
        root.timeline = Some(timeline);
        root.add(Box::new(Sprite2d::new("sb/dot.png", OsbOrigin::Centre)));

        let scene = Scene2d::from_root(root);
        let resolved = scene.resolve(&Scene2dOptions::default());
        // The node moves +50 and its timeline adds +30 in the node's local space.
        let end = sample_move(&resolved[0].commands, 100.0);
        assert!((end.x - 80.0).abs() < 1e-4, "end = {end:?}");
    }

    #[test]
    fn a_node_timeline_multiplies_opacity() {
        let mut root = Node2d::default();
        root.set_opacity(0.5);
        let mut timeline = TransformTimeline2d::new();
        let mut opacity = KeyframedValue::new();
        opacity.add(0.0, 0.5);
        timeline.opacity.add(Channel::Scalar(opacity), 0.0);
        root.timeline = Some(timeline);
        root.add(Box::new(Sprite2d::new("sb/dot.png", OsbOrigin::Centre)));

        let scene = Scene2d::from_root(root);
        let resolved = scene.resolve(&Scene2dOptions::default());
        let fade = resolved[0]
            .commands
            .iter()
            .find(|(kind, _)| *kind == CommandType::Fade)
            .map(|(_, command)| command)
            .expect("a fade command");
        assert!((fade.value_at(0.0).as_scalar() - 0.25).abs() < 1e-6);
    }
}
