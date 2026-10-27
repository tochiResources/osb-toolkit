//! Effect-script runtime.
//!
//! This crate reimplements the parts of the effect-scripting API that effect
//! scripts rely on: the beatmap (`.osu`) model, hit objects and slider curves,
//! the storyboard sprite/command model, easing and `KeyframedValue`
//! interpolation, FFT, and a software sprite compositor. It is host-agnostic:
//! the After Effects, Premiere Pro and DaVinci Resolve adapters supply the pixel
//! buffer, time, parameter values and asset resolution.

pub mod animations;
pub mod assets;
pub mod audio;
pub mod common;
pub mod context;
pub mod diagnostics;
pub mod error;
pub mod host;
pub mod instance;
pub mod mapset;
pub mod optimize;
pub mod params;

pub mod preview;
pub mod render;
pub mod storyboarding;
pub mod storyboarding3d;

/// The shared math types (`osb-core::math`), re-exported so `crate::math::*` and
/// `osb_runtime::math::*` paths stay stable.
pub use osb_core::math;

/// The canonical curve math (`osb-core::curve`), re-exported for consumers that
/// only depend on the runtime.
pub use osb_core::curve;

pub use error::{RuntimeError, RuntimeResult};

/// Types generated effect code imports with a single `use osb_runtime::prelude::*;`.
pub mod prelude {
    pub use crate::animations::{Easing, Interpolate, Keyframe, KeyframedValue};
    pub use crate::assets::{Bitmap, Texture, TextureStore};
    pub use crate::audio::AudioBuffer;
    pub use crate::context::Context;
    pub use crate::instance::EffectInstance;
    pub use crate::mapset::{Beatmap, ControlPoint, HitObject, HitObjectKind, SliderPath};
    pub use crate::math::{Box2, Color4, MathHelper, Matrix4, Quaternion, Vec2, Vec3, Vec4};
    pub use crate::params::{Control, ParameterDescriptor, ParameterValue};
    pub use crate::preview::{
        approach_time, build_preview, circle_radius, ElementToggles, GameMode, OsuPreviewEffect,
        OsuPreviewOptions, PreviewStats, SkinPaths,
    };
    pub use crate::render::{render, Concurrency, RenderOptions};
    pub use crate::storyboarding::{
        AnimationMeta, Command, CommandType, Effect, ExportItem, Layer, LayerHandle, LayerKey,
        Loop, OsbLayer, OsbLoopType, OsbOrigin, OsbSample, OsbSprite, ParameterType, Runtime,
        SpriteHandle, SpriteTarget, Storyboard, Trigger,
    };
}
