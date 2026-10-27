//! The 3D scene engine and the 3D -> 2D bake.
//!
//! Projects a [`scene::Scene3d`]-style tree (nodes/sprites with position, scale
//! and rotation keyframes) through a [`camera::PerspectiveCamera`] and
//! simplifies the resulting per-sprite screenspace channels into `.osb`-compatible
//! command spans. This is the canonical implementation, kept in parity across
//! the SDKs by shared golden vectors (of the plan).
//!
//! # Numerical conventions
//!
//! These hold everywhere in this module tree and are relied on for cross-language
//! parity:
//!
//! - **Column-major, `A.mul(B)` applies `B` first.** The runtime `Matrix4` is the
//!   transpose of OpenTK's row-vector convention, so `view * projection` becomes
//!   `projection.mul(view)` and `translation * rotation * scale` becomes
//!   `translation.mul(rotation).mul(scale)`.
//! - **Signed scales, no unsigned `ExtractScale`.** Orientation decomposition uses
//!   `sx = det / sy` (signed), so a plane seen from behind mirrors rather than
//!   reversing. Signed `world_scale` feeds the sprite scale directly.
//! - **`w < 0` culls.** A point at or behind the camera (`w <= near`) is clipped
//!   (`clip`) or culled (sprites set opacity 0); `clip_to_screen` divides by
//!   `|w|` so a behind-camera point never flips the image.
//! - **One 2x2 decomposition** (`free_decomposition`): anchor on the longer
//!   projected axis, `rotation = atan2(b.y, b.x) - 90deg` for the Y anchor, and
//!   drop the residual shear. Every orientation/mesh/subdivision path uses it.
//! - **Shortest-path angles.** Sprite rotation is accumulated through
//!   `DoubleAngle` semantics; the bake stores raw radians and relies on the
//!   simplifier rather than re-wrapping.
//! - **Deterministic ordering.** Sampling is deterministic and the depth sort is
//!   stable, so single-threaded and multi-threaded bakes produce identical bytes.

pub mod camera;
pub mod clip;
pub mod gltf;
pub mod helpers;
pub mod input;
pub mod r#loop;
pub mod mesh;
pub mod meshopt;
pub mod node;
pub mod optimize;
pub mod orientation;
pub mod parser;
pub mod procedural;
pub mod scene;
pub mod subdivide;
pub mod world;

pub use camera::{CameraState, DofSample, OrthographicCamera, PerspectiveCamera};
pub use clip::{clip_near, in_front, to_clip};
pub use gltf::parse_gltf;
pub use helpers::{
    orbit, path_follow, pulse, scatter, spin, stagger, tumble, Axis, Channel, FlattenMode,
};
pub use input::{
    bake_input, build_scene, Bake3dInput, CameraInput, NodeInput, OptionsInput, TimelineClipInput,
    TimelineInput, TimelineKeyInput, TransformTimeline3dInput,
};
pub use mesh::{Face, LineSegment, MeshData, MeshSprite, Quad, Triangle};
pub use meshopt::merge_triangles_to_quads;
pub use node::{
    state_of, LeafMeta, Line3d, Mesh3d, Node3d, Object3d, Object3dState, RotationMode, Sprite3d,
    SpriteState, TransformTimeline3d, Triangle3d,
};
pub use optimize::{BakedCommand, BakedValue, CommandKind};
pub use orientation::{
    AxisAnchor, BillboardKind, LegacyAxis, ProjectionContext, ResolvedSprite, SpriteOrientation,
    SpriteProjection,
};
pub use parser::{parse_obj, parse_stl, ObjParseOptions, ParsedMesh, UpAxis};
pub use procedural::{cache_key, generate, GeneratedSprite, SpriteGenOptions, SpriteManifest};
pub use r#loop::{detect_loop, solve_loop_end, LoopDetection, LoopHelper, LoopItem};
pub use scene::{bake, BakeOptions, BakedSprite, BakedStoryboard};
pub use subdivide::{subdivide, SubSprite, SubdivisionResult};
pub use world::{bake_to_storyboard, StoryboardScene, World3d};

/// The osu! storyboard canvas size (`OsuHitObject.StoryboardSize`).
pub const STORYBOARD_SIZE: (f64, f64) = (854.0, 480.0);
