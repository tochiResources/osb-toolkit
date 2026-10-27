//! Wire value types for the storyboard model. The math types live in
//! [`crate::math`].

use osb_runtime::storyboarding::AnimationMeta as RuntimeAnimationMeta;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::enums::{OsbLoopType, OsbOrigin};
use crate::math::{Color4, Vector2};

/// A sprite's sampled state at a point in time.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SpriteState {
    /// Sampled position.
    pub position: Vector2,
    /// Sampled scale (per axis).
    pub scale: Vector2,
    /// Sampled rotation in radians.
    pub rotation: f64,
    /// Sampled opacity in `0.0..=1.0`.
    pub opacity: f64,
    /// Whether the sprite is composited additively at this time.
    pub additive: bool,
    /// Sampled color.
    pub color: Color4,
    /// Sampled origin offset in texture pixels, added to the enum anchor.
    /// Zero when the origin is not animated.
    pub anchor: Vector2,
}

/// Frame-cycling metadata for an animation sprite.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AnimationMeta {
    /// Number of frames in the animation.
    pub frames: i32,
    /// Delay between frames, in milliseconds.
    pub frame_delay: f64,
    /// How the animation loops.
    pub loop_type: OsbLoopType,
}

impl From<RuntimeAnimationMeta> for AnimationMeta {
    fn from(value: RuntimeAnimationMeta) -> Self {
        AnimationMeta {
            frames: value.frame_count,
            frame_delay: value.frame_delay,
            loop_type: value.loop_type,
        }
    }
}

/// A sprite's initial state and metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SpriteInfo {
    /// The sprite's texture path (relative to the mapset).
    pub texture_path: String,
    /// The sprite anchor origin.
    pub origin: OsbOrigin,
    /// The sprite's initial position.
    pub position: Vector2,
    /// The sprite's initial scale.
    pub scale: Vector2,
    /// The sprite's initial rotation in radians.
    pub rotation: f64,
    /// The sprite's initial opacity.
    pub opacity: f64,
    /// The sprite's initial color.
    pub color: Color4,
    /// Whether the sprite starts additive.
    pub additive: bool,
    /// Whether the sprite starts horizontally flipped.
    pub flip_h: bool,
    /// Whether the sprite starts vertically flipped.
    pub flip_v: bool,
    /// Animation metadata, or `null` for a plain sprite.
    pub animation: Option<AnimationMeta>,
    /// The number of loop groups on the sprite.
    pub loop_count: usize,
    /// The number of trigger groups on the sprite.
    pub trigger_count: usize,
}

/// A stable handle to a sprite in a runtime's storyboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct Handle {
    /// The layer index the sprite lives on.
    pub layer: usize,
    /// The sprite index within its layer.
    pub sprite: usize,
}

/// A stable handle to a layer in a runtime's storyboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct LayerHandle {
    /// The layer index.
    pub layer: usize,
}
