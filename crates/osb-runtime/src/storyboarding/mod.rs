//! Storyboard model: layers, sprites, commands and the effect runtime.

mod command;
pub mod helpers;
mod import;
mod layer;
pub mod origin;
mod runtime;
mod scene;
mod sprite;
mod types;

pub use command::{Command, CommandType, CommandValue, ParameterTypeValue};
pub use helpers::{
    align_commands, apply_command_channel, apply_command_channel_eased, command_channel,
    command_easings, duplicate_commands, filter_commands, flatten_commands, map_commands,
    merge_commands, quantize_commands, randomize_commands, remove_commands, retime_bezier,
    retime_easing, reverse_commands, scale_time_commands, select_commands, set_easing,
    shift_commands, split_commands, timeline_commands, transform_command_channel,
    transform_command_values, KindedCommand,
};
pub use import::import_osb;
pub use layer::{Layer, OsbSample};
pub use origin::{in_screen_bounds, origin_vector, resolve_anchor, resolve_anchor_3d};
pub use runtime::{Effect, ExportItem, LayerKey, Runtime, SpriteTarget, Storyboard};
pub use scene::{
    Affine2, Camera2d, Node2d, Object2d, ResolvedChannels2d, ResolvedSprite2d, Scene2d,
    Scene2dOptions, Sprite2d, TransformTimeline2d, World2d,
};
pub use sprite::{AnimationMeta, Loop, OsbSprite, Trigger};
pub use types::{LayerHandle, OsbLayer, OsbLoopType, OsbOrigin, ParameterType, SpriteHandle};
