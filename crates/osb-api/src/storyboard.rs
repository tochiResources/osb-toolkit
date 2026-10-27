//! The full storyboard model, read back from a live runtime (M37).
//!
//! The runtime `Effect::generate` returns a [`Storyboard`](osb_runtime) model;
//! this module is the wire form of that model so the SDK `Generator` can return
//! the authored storyboard, matching the runtime shape. Commands are emitted in
//! a stable order (time, then kind) so the JSON is deterministic.

use std::collections::HashMap;

use osb_runtime::storyboarding::{
    Command as RuntimeCommand, CommandType as RuntimeCommandType,
    CommandValue as RuntimeCommandValue, Layer as RuntimeLayer, Loop as RuntimeLoop,
    OsbSprite as RuntimeSprite, ParameterTypeValue as RuntimeParameterValue,
    Storyboard as RuntimeStoryboard, Trigger as RuntimeTrigger,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::enums::{Easing, OsbLayer, OsbLoopType, OsbOrigin};
use crate::math::{Color4, Vector2};
use crate::model::AnimationMeta;

/// A parameter command's numeric payload.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum ParameterValue {
    /// Horizontal-flip amount.
    FlipHorizontal(f64),
    /// Vertical-flip amount.
    FlipVertical(f64),
    /// Additive-blend amount.
    Additive(f64),
}

/// A command's endpoint value.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum CommandValue {
    /// A scalar value.
    Scalar(f64),
    /// A 2D value.
    Vector2(Vector2),
    /// A color value.
    Color(Color4),
    /// A parameter value.
    Parameter(ParameterValue),
    /// An absent value.
    None,
}

/// The kind of a storyboard command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum StoryboardCommandKind {
    /// A move.
    Move,
    /// A horizontal move.
    MoveX,
    /// A vertical move.
    MoveY,
    /// A uniform scale.
    Scale,
    /// A per-axis scale.
    ScaleVec,
    /// A rotation.
    Rotate,
    /// A fade.
    Fade,
    /// A color.
    Color,
    /// A hue/saturation/brightness color.
    ColorHsb,
    /// A parameter (`P`).
    Parameter,
    /// An animatable origin offset.
    Anchor,
}

/// A single time-based command on a sprite timeline.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StoryboardCommand {
    /// The command kind.
    pub kind: StoryboardCommandKind,
    /// The start time in milliseconds.
    pub start_time: f64,
    /// The end time in milliseconds.
    pub end_time: f64,
    /// The easing function.
    pub easing: Easing,
    /// The start value.
    pub start: CommandValue,
    /// The end value.
    pub end: CommandValue,
}

/// A loop group on a sprite.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StoryboardLoop {
    /// The loop start time in milliseconds.
    pub start_time: f64,
    /// The loop end time in milliseconds.
    pub end_time: f64,
    /// How the loop repeats.
    pub loop_type: OsbLoopType,
    /// The number of iterations.
    pub count: i32,
    /// The commands inside the loop.
    pub commands: Vec<StoryboardCommand>,
}

/// A trigger group on a sprite.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StoryboardTrigger {
    /// The trigger name (matches a hit object).
    pub name: String,
    /// The trigger start time in milliseconds.
    pub start_time: f64,
    /// The trigger end time in milliseconds.
    pub end_time: f64,
    /// The trigger group index.
    pub group: i32,
    /// The commands inside the trigger.
    pub commands: Vec<StoryboardCommand>,
}

/// A sprite with its full command surface.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StoryboardSprite {
    /// The sprite's texture path (relative to the mapset).
    pub texture_path: String,
    /// The sprite anchor origin.
    pub origin: OsbOrigin,
    /// The initial origin offset in texture pixels.
    pub anchor: Vector2,
    /// The initial position.
    pub position: Vector2,
    /// The initial scale.
    pub scale: Vector2,
    /// The initial rotation in radians.
    pub rotation: f64,
    /// The initial opacity.
    pub opacity: f64,
    /// The initial color.
    pub color: Color4,
    /// Whether the sprite starts additive.
    pub additive: bool,
    /// Whether the sprite starts horizontally flipped.
    pub flip_h: bool,
    /// Whether the sprite starts vertically flipped.
    pub flip_v: bool,
    /// Whether the sprite is excluded from output (`DiscardSprite`).
    pub exclude: bool,
    /// Animation metadata, or `null` for a plain sprite.
    pub animation: Option<AnimationMeta>,
    /// The sprite's top-level commands.
    pub commands: Vec<StoryboardCommand>,
    /// The sprite's loop groups.
    pub loops: Vec<StoryboardLoop>,
    /// The sprite's trigger groups.
    pub triggers: Vec<StoryboardTrigger>,
}

/// A sound sample on a layer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StoryboardSample {
    /// The sample path (relative to the mapset).
    pub path: String,
    /// The sample time in milliseconds.
    pub time: f64,
    /// The sample volume (`0..=100`).
    pub volume: f64,
}

/// A layer/segment with its sprites and samples.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StoryboardLayer {
    /// The layer/segment identifier (`""` for an anonymous segment).
    pub identifier: String,
    /// The osu! layer this segment belongs to.
    pub osb_layer: OsbLayer,
    /// Whether the segment's children are emitted in reverse order.
    pub reverse_depth: bool,
    /// The layer's sprites, in creation order.
    pub sprites: Vec<StoryboardSprite>,
    /// The layer's sound samples.
    pub samples: Vec<StoryboardSample>,
}

/// The whole storyboard, in draw order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Storyboard {
    /// The layers/segments in draw order.
    pub layers: Vec<StoryboardLayer>,
}

/// The stable order commands are listed in.
const COMMAND_ORDER: [RuntimeCommandType; 11] = [
    RuntimeCommandType::Move,
    RuntimeCommandType::MoveX,
    RuntimeCommandType::MoveY,
    RuntimeCommandType::Scale,
    RuntimeCommandType::ScaleVec,
    RuntimeCommandType::Rotate,
    RuntimeCommandType::Fade,
    RuntimeCommandType::Color,
    RuntimeCommandType::ColorHsb,
    RuntimeCommandType::Parameter,
    RuntimeCommandType::Anchor,
];

impl From<RuntimeParameterValue> for ParameterValue {
    fn from(value: RuntimeParameterValue) -> Self {
        match value {
            RuntimeParameterValue::FlipHorizontal(v) => ParameterValue::FlipHorizontal(v),
            RuntimeParameterValue::FlipVertical(v) => ParameterValue::FlipVertical(v),
            RuntimeParameterValue::Additive(v) => ParameterValue::Additive(v),
        }
    }
}

impl From<RuntimeCommandValue> for CommandValue {
    fn from(value: RuntimeCommandValue) -> Self {
        match value {
            RuntimeCommandValue::Scalar(v) => CommandValue::Scalar(v),
            RuntimeCommandValue::Vector2(v) => CommandValue::Vector2(v),
            RuntimeCommandValue::Color(v) => CommandValue::Color(v),
            RuntimeCommandValue::Parameter(v) => CommandValue::Parameter(v.into()),
            RuntimeCommandValue::None => CommandValue::None,
        }
    }
}

impl From<RuntimeCommandType> for StoryboardCommandKind {
    fn from(kind: RuntimeCommandType) -> Self {
        match kind {
            RuntimeCommandType::Move => StoryboardCommandKind::Move,
            RuntimeCommandType::MoveX => StoryboardCommandKind::MoveX,
            RuntimeCommandType::MoveY => StoryboardCommandKind::MoveY,
            RuntimeCommandType::Scale => StoryboardCommandKind::Scale,
            RuntimeCommandType::ScaleVec => StoryboardCommandKind::ScaleVec,
            RuntimeCommandType::Rotate => StoryboardCommandKind::Rotate,
            RuntimeCommandType::Fade => StoryboardCommandKind::Fade,
            RuntimeCommandType::Color => StoryboardCommandKind::Color,
            RuntimeCommandType::ColorHsb => StoryboardCommandKind::ColorHsb,
            RuntimeCommandType::Parameter => StoryboardCommandKind::Parameter,
            RuntimeCommandType::Anchor => StoryboardCommandKind::Anchor,
            // Loop/Trigger are containers, not leaf commands.
            RuntimeCommandType::Loop | RuntimeCommandType::Trigger => {
                StoryboardCommandKind::Parameter
            }
        }
    }
}

/// Flattens a command map into a stable, time-ordered list.
fn commands(map: &HashMap<RuntimeCommandType, Vec<RuntimeCommand>>) -> Vec<StoryboardCommand> {
    let mut out = Vec::new();
    for kind in COMMAND_ORDER {
        if let Some(list) = map.get(&kind) {
            for command in list {
                out.push(StoryboardCommand {
                    kind: kind.into(),
                    start_time: command.start_time,
                    end_time: command.end_time,
                    easing: command.easing,
                    start: command.start_value.into(),
                    end: command.end_value.into(),
                });
            }
        }
    }
    out.sort_by(|a, b| {
        a.start_time
            .total_cmp(&b.start_time)
            .then(a.end_time.total_cmp(&b.end_time))
    });
    out
}

impl From<&RuntimeLoop> for StoryboardLoop {
    fn from(looped: &RuntimeLoop) -> Self {
        StoryboardLoop {
            start_time: looped.start_time,
            end_time: looped.end_time,
            loop_type: looped.loop_type,
            count: looped.count,
            commands: commands(looped.commands()),
        }
    }
}

impl From<&RuntimeTrigger> for StoryboardTrigger {
    fn from(trigger: &RuntimeTrigger) -> Self {
        StoryboardTrigger {
            name: trigger.trigger_name.clone(),
            start_time: trigger.start_time,
            end_time: trigger.end_time,
            group: trigger.group,
            commands: commands(trigger.commands()),
        }
    }
}

impl From<&RuntimeSprite> for StoryboardSprite {
    fn from(sprite: &RuntimeSprite) -> Self {
        StoryboardSprite {
            texture_path: sprite.texture_path.clone(),
            origin: sprite.origin,
            anchor: sprite.anchor_point,
            position: sprite.initial_position,
            scale: sprite.initial_scale,
            rotation: sprite.initial_rotation,
            opacity: sprite.initial_opacity,
            color: sprite.initial_color,
            additive: sprite.additive,
            flip_h: sprite.flip_h,
            flip_v: sprite.flip_v,
            exclude: sprite.exclude,
            animation: sprite.animation.map(AnimationMeta::from),
            commands: commands(sprite.commands()),
            loops: sprite.loops().iter().map(StoryboardLoop::from).collect(),
            triggers: sprite
                .triggers()
                .iter()
                .map(StoryboardTrigger::from)
                .collect(),
        }
    }
}

impl From<&RuntimeLayer> for StoryboardLayer {
    fn from(layer: &RuntimeLayer) -> Self {
        StoryboardLayer {
            identifier: layer.identifier().to_string(),
            osb_layer: layer.osb_layer(),
            reverse_depth: layer.reverse_depth(),
            sprites: layer.sprites().iter().map(StoryboardSprite::from).collect(),
            samples: layer
                .samples()
                .iter()
                .map(|sample| StoryboardSample {
                    path: sample.path.clone(),
                    time: sample.time,
                    volume: sample.volume,
                })
                .collect(),
        }
    }
}

impl From<&RuntimeStoryboard> for Storyboard {
    fn from(storyboard: &RuntimeStoryboard) -> Self {
        Storyboard {
            layers: storyboard.layers().map(StoryboardLayer::from).collect(),
        }
    }
}
