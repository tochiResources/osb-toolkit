//! A 3D world and the unified storyboard bake.
//!
//! A [`World3d`] is a 3D root [`Node3d`] plus a [`PerspectiveCamera`] and a time
//! range; [`World3d::bake`] projects it and materializes the result into a
//! [`Storyboard`] (one sprite per baked sprite, written onto its osu! layer). A
//! node can carry a 2D subtree (`Node3d::scene2d`), so 2D sprites/segments/layers
//! embed in a 3D scene: the subtree is resolved with an identity camera and each
//! leaf becomes a 3D sprite in the node's local space, then the node's transform
//! projects it.
//!
//! [`StoryboardScene`] holds several 2D worlds and 3D worlds and bakes them into
//! **one** storyboard, so a generator can bake an entire storyboard in one call.

use crate::animations::Easing;
use crate::math::Vec3;
use crate::storyboarding::{
    Camera2d, Command, CommandType, CommandValue, OsbLayer, ParameterTypeValue, ResolvedChannels2d,
    Scene2d, Scene2dOptions, Storyboard,
};

use super::camera::PerspectiveCamera;
use super::node::{Node3d, Object3d, RotationMode, Sprite3d};
use super::optimize::{BakedCommand, BakedValue, CommandKind};
use super::scene::{bake, BakeOptions, BakedStoryboard};

/// A 3D world: a root node, a perspective camera and a time range.
#[derive(Debug, Clone)]
pub struct World3d {
    /// A label for the world.
    pub name: String,
    /// The world's root node.
    pub root: Node3d,
    /// The world's camera.
    pub camera: PerspectiveCamera,
    /// The world's start time (used when the bake options leave the range open).
    pub start_time: f64,
    /// The world's end time.
    pub end_time: f64,
}

impl Default for World3d {
    fn default() -> Self {
        World3d {
            name: String::new(),
            root: Node3d::default(),
            camera: PerspectiveCamera::default(),
            start_time: 0.0,
            end_time: 0.0,
        }
    }
}

impl World3d {
    /// A world rooted at `root` with `camera`.
    pub fn new(root: Node3d, camera: PerspectiveCamera) -> Self {
        World3d {
            root,
            camera,
            ..World3d::default()
        }
    }

    /// A named world.
    pub fn named(name: impl Into<String>) -> Self {
        World3d {
            name: name.into(),
            ..World3d::default()
        }
    }

    /// The raw projection: the baked per-sprite channels (2D subtrees embedded).
    pub fn baked(&self, options: &BakeOptions, options2d: &Scene2dOptions) -> BakedStoryboard {
        let mut root = self.root.clone();
        embed_scene2d(&mut root, options2d);
        bake(&root, &self.camera, options)
    }

    /// Bakes the world into a fresh [`Storyboard`].
    pub fn bake(&self, options: &BakeOptions, options2d: &Scene2dOptions) -> Storyboard {
        let mut storyboard = Storyboard::new();
        self.bake_into(&mut storyboard, options, options2d);
        storyboard
    }

    /// Bakes the world into an existing [`Storyboard`] (so several worlds compose).
    pub fn bake_into(
        &self,
        storyboard: &mut Storyboard,
        options: &BakeOptions,
        options2d: &Scene2dOptions,
    ) {
        let baked = self.baked(options, options2d);
        write_baked(storyboard, &baked);
    }
}

/// Bakes a raw [`BakedStoryboard`] into a fresh [`Storyboard`].
pub fn bake_to_storyboard(baked: &BakedStoryboard) -> Storyboard {
    let mut storyboard = Storyboard::new();
    write_baked(&mut storyboard, baked);
    storyboard
}

/// Materializes a baked result: one sprite per baked sprite, onto its osu! layer.
pub fn write_baked(storyboard: &mut Storyboard, baked: &BakedStoryboard) {
    for sprite in &baked.sprites {
        let layer = sprite
            .osb_layer
            .unwrap_or_else(|| OsbLayer::from_index(sprite.layer as usize));
        let handle = storyboard.create_sprite(layer, sprite.sprite_path.clone(), sprite.origin);
        let target = storyboard.sprite_mut(handle);
        target.additive = sprite.additive;
        let commands = target.commands_mut();
        for command in sprite.commands() {
            let (kind, command) = to_command(&command);
            commands.entry(kind).or_default().push(command);
        }
    }
}

/// Converts a baked command into a runtime command.
fn to_command(command: &BakedCommand) -> (CommandType, Command) {
    if command.kind == CommandKind::FlipH {
        let start = scalar(command.start);
        let end = scalar(command.end);
        return (
            CommandType::Parameter,
            Command::new(
                command.start_time,
                command.end_time,
                Easing::None,
                CommandValue::Parameter(ParameterTypeValue::FlipHorizontal(start)),
                CommandValue::Parameter(ParameterTypeValue::FlipHorizontal(end)),
            ),
        );
    }
    let kind = match command.kind {
        CommandKind::Move => CommandType::Move,
        CommandKind::ScaleVec => CommandType::ScaleVec,
        CommandKind::Rotate => CommandType::Rotate,
        CommandKind::Fade => CommandType::Fade,
        CommandKind::Color => CommandType::Color,
        CommandKind::Additive => CommandType::Parameter,
        CommandKind::FlipH => unreachable!(),
    };
    (
        kind,
        Command::new(
            command.start_time,
            command.end_time,
            Easing::None,
            to_value(command.start, kind),
            to_value(command.end, kind),
        ),
    )
}

fn scalar(value: BakedValue) -> f64 {
    match value {
        BakedValue::Scalar(v) => v,
        BakedValue::Vector2(v) => v.x,
        BakedValue::Color(c) => c.r,
    }
}

fn to_value(value: BakedValue, kind: CommandType) -> CommandValue {
    match value {
        BakedValue::Scalar(v) => {
            if kind == CommandType::Parameter {
                CommandValue::Parameter(ParameterTypeValue::Additive(v))
            } else {
                CommandValue::Scalar(v)
            }
        }
        BakedValue::Vector2(v) => CommandValue::Vector2(v),
        BakedValue::Color(c) => CommandValue::Color(c),
    }
}

/// Embeds every `scene2d` subtree as 3D sprites in its node's local space.
fn embed_scene2d(object: &mut dyn Object3d, options: &Scene2dOptions) {
    if let Some(slot) = object.scene2d_mut() {
        if let Some(subtree) = slot.take() {
            let scene = Scene2d {
                root: *subtree,
                ..Scene2d::new()
            };
            let resolved = scene.resolve_channels_with(&Camera2d::identity(), options);
            for sprite in resolved {
                object.add(Box::new(sprite_to_object(sprite)));
            }
        }
    }
    for child in object.children_mut() {
        embed_scene2d(child.as_mut(), options);
    }
}

/// Converts a resolved 2D sprite into a 3D sprite in the node's local space: the
/// 2D position becomes a local `Vec3(x, y, 0)`, the 2D scale/rotation become the
/// sprite's screen scale/rotation, and opacity/color are inherited.
fn sprite_to_object(sprite: ResolvedChannels2d) -> Sprite3d {
    let mut sprite3d = Sprite3d::new(sprite.texture_path.clone(), sprite.origin);
    sprite3d.additive = sprite.additive;
    // Keep the 2D orientation: no automatic axis alignment, so the resolved 2D
    // rotation (in screen space) is used as-is.
    sprite3d.rotation_mode = RotationMode::Fixed;
    sprite3d.sprite_scale = sprite.scale;
    sprite3d.sprite_rotation = sprite.rotation;
    sprite3d.use_distance_fade = false;
    sprite3d.node.layer = Some(sprite.layer);
    for keyframe in sprite.position.keyframes() {
        sprite3d.node.position.add_eased(
            keyframe.time,
            Vec3::new(keyframe.value.x, keyframe.value.y, 0.0),
            keyframe.easing,
        );
    }
    for keyframe in sprite.opacity.keyframes() {
        sprite3d
            .node
            .opacity
            .add_eased(keyframe.time, keyframe.value, keyframe.easing);
    }
    for keyframe in sprite.color.keyframes() {
        sprite3d
            .node
            .color
            .add_eased(keyframe.time, keyframe.value, keyframe.easing);
    }
    sprite3d
}

/// A set of 2D worlds and 3D worlds baked into **one** storyboard.
#[derive(Debug, Clone, Default)]
pub struct StoryboardScene {
    /// The 2D worlds.
    pub worlds2d: Vec<Scene2d>,
    /// The 3D worlds.
    pub worlds3d: Vec<World3d>,
}

impl StoryboardScene {
    /// An empty storyboard scene.
    pub fn new() -> Self {
        StoryboardScene::default()
    }

    /// Adds a 2D world.
    pub fn add_world2d(&mut self, world: Scene2d) -> &mut Self {
        self.worlds2d.push(world);
        self
    }

    /// Adds a 3D world.
    pub fn add_world3d(&mut self, world: World3d) -> &mut Self {
        self.worlds3d.push(world);
        self
    }

    /// Bakes only the 2D worlds.
    pub fn bake2d(&self, options: &Scene2dOptions) -> Storyboard {
        let mut storyboard = Storyboard::new();
        for world in &self.worlds2d {
            world.bake_into(&mut storyboard, options);
        }
        storyboard
    }

    /// Bakes only the 3D worlds.
    pub fn bake3d(&self, options: &BakeOptions, options2d: &Scene2dOptions) -> Storyboard {
        let mut storyboard = Storyboard::new();
        for world in &self.worlds3d {
            world.bake_into(&mut storyboard, options, options2d);
        }
        storyboard
    }

    /// Bakes every world (2D and 3D) into one storyboard.
    pub fn bake(&self, options2d: &Scene2dOptions, options3d: &BakeOptions) -> Storyboard {
        let mut storyboard = Storyboard::new();
        self.bake_into(&mut storyboard, options2d, options3d);
        storyboard
    }

    /// Bakes every world (2D and 3D) into an existing storyboard, so a generator
    /// can compose baked worlds with hand-authored sprites.
    pub fn bake_into(
        &self,
        storyboard: &mut Storyboard,
        options2d: &Scene2dOptions,
        options3d: &BakeOptions,
    ) {
        for world in &self.worlds2d {
            world.bake_into(storyboard, options2d);
        }
        for world in &self.worlds3d {
            world.bake_into(storyboard, options3d, options2d);
        }
    }
}

/// A 3D scene is a [`World3d`] (the 2D counterpart is [`Scene2d`]).
pub type Scene3d = World3d;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storyboarding::{Node2d, Object2d, OsbOrigin, Scene2d, Sprite2d};

    fn facing_camera() -> PerspectiveCamera {
        let mut camera = PerspectiveCamera::default();
        camera.position_z.add(0.0, 5.0);
        camera
    }

    #[test]
    fn bakes_a_world_onto_its_layer() {
        let mut root = Node3d::default();
        let mut sprite = Sprite3d::new("sb/dot.png", OsbOrigin::Centre);
        sprite.node.layer = Some(OsbLayer::Foreground);
        root.add(Box::new(sprite));
        let world = World3d::new(root, facing_camera());
        let mut storyboard = world.bake(&BakeOptions::default(), &Scene2dOptions::default());
        // The sprite is on the Foreground layer.
        let foreground = storyboard.layer(OsbLayer::Foreground);
        assert_eq!(foreground.len(), 1);
        assert_eq!(foreground.sprites()[0].texture_path, "sb/dot.png");
    }

    #[test]
    fn embeds_a_2d_sprite_in_a_3d_scene() {
        // A 3D node with a 2D sprite at local (100, 0).
        let mut node = Node3d::default();
        let mut leaf = Node2d::new("leaf");
        leaf.layer = Some(OsbLayer::Overlay);
        let mut sprite = Sprite2d::new("sb/dot2d.png", OsbOrigin::Centre);
        let mut position = crate::animations::KeyframedValue::new();
        position.add(0.0, crate::math::Vec2::new(100.0, 0.0));
        sprite.node.position = crate::animations::keyframes::Channel::Vector2(position);
        leaf.add(Box::new(sprite));
        node.scene2d = Some(Box::new(leaf));

        let world = World3d::new(node, facing_camera());
        let mut storyboard = world.bake(&BakeOptions::default(), &Scene2dOptions::default());
        let overlay = storyboard.layer(OsbLayer::Overlay);
        assert_eq!(overlay.len(), 1);
        assert_eq!(overlay.sprites()[0].texture_path, "sb/dot2d.png");
    }

    #[test]
    fn bakes_2d_and_3d_worlds_into_one_storyboard() {
        let mut root2d = Node2d::default();
        root2d.add(Box::new(Sprite2d::new("sb/2d.png", OsbOrigin::Centre)));
        let mut world2d = Scene2d::from_root(root2d);
        world2d.name = "flat".to_string();
        let mut root = Node3d::default();
        root.add(Box::new(Sprite3d::new("sb/3d.png", OsbOrigin::Centre)));
        let world3d = World3d::new(root, facing_camera());

        let scene = StoryboardScene {
            worlds2d: vec![world2d],
            worlds3d: vec![world3d],
        };
        let storyboard = scene.bake(&Scene2dOptions::default(), &BakeOptions::default());
        assert_eq!(storyboard.sprite_count(), 2);
    }
}
