//! Wire requests: the bodies each bridge operation accepts. The command op's
//! `start`/`end` are free-form (`Object`) because their shape depends on `kind`
//! (a scalar, a point, or a color).

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::curve::CurveSpec;
use crate::enums::{Easing, OsbLayer, OsbLoopType, OsbOrigin, ParameterType};
use crate::math::Vector2;
use crate::storyboarding3d::Scene3dInput;

/// A `bake3d` request: a plain 3D scene.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Bake3dRequest {
    /// The 3D scene.
    pub input: Scene3dInput,
}

/// A request carrying a block of text (an effect script, `.osu` or `.osb`).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TextRequest {
    /// The text payload.
    pub text: String,
}

/// An FFT request against a `.wav` file.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FftRequest {
    /// Path to the audio file.
    pub path: String,
    /// Sample time in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<f64>,
    /// Number of frequency bins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub magnitudes: Option<usize>,
}

/// Resolve (or create) a layer by name.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct LayerHandleRequest {
    /// The layer name.
    pub name: String,
}

/// Create a sprite.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateSpriteRequest {
    /// The target layer name (omitted when `layerHandle` is set).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
    /// The target segment handle (takes precedence over `layer`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_handle: Option<usize>,
    /// The texture path (relative to the mapset).
    pub path: String,
    /// The sprite anchor origin.
    pub origin: OsbOrigin,
    /// The initial position; omitted means the sprite's default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Vector2>,
}

/// Create an animation sprite.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateAnimationRequest {
    /// The target layer name (omitted when `layerHandle` is set).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
    /// The target segment handle (takes precedence over `layer`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_handle: Option<usize>,
    /// The texture path (relative to the mapset).
    pub path: String,
    /// The number of frames.
    pub frames: i32,
    /// Delay between frames, in milliseconds.
    pub frame_delay: f64,
    /// How the animation loops.
    pub loop_type: OsbLoopType,
    /// The sprite anchor origin.
    pub origin: OsbOrigin,
    /// The initial position.
    pub position: Vector2,
}

/// A sprite command. `start`/`end` are a scalar, a `Vector2`, or a `Color4`
/// depending on `kind`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CommandRequest {
    /// The command kind (`move`, `moveX`, `moveY`, `scale`, `scaleVec`,
    /// `rotate`, `fade`, `color`, `colorHsb`, `anchor`, `parameter`, `flipH`,
    /// `flipV`, `additive`).
    pub kind: String,
    /// The layer handle.
    pub layer: usize,
    /// The sprite handle.
    pub sprite: usize,
    /// The command start time in milliseconds.
    pub start_time: f64,
    /// The command end time in milliseconds.
    pub end_time: f64,
    /// The easing function.
    pub easing: Easing,
    /// The start value (scalar, point or color).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Object)]
    pub start: Option<serde_json::Value>,
    /// The end value (scalar, point or color).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Object)]
    pub end: Option<serde_json::Value>,
    /// The parameter kind for a `parameter` command.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter: Option<ParameterType>,
}

/// Start a loop group.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StartLoopGroupRequest {
    /// The layer handle.
    pub layer: usize,
    /// The sprite handle.
    pub sprite: usize,
    /// The loop start time in milliseconds.
    pub start_time: f64,
    /// The number of iterations.
    pub count: f64,
}

/// Start a trigger group.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct StartTriggerGroupRequest {
    /// The layer handle.
    pub layer: usize,
    /// The sprite handle.
    pub sprite: usize,
    /// The trigger name (matches a hit object).
    pub name: String,
    /// The trigger start time in milliseconds.
    pub start_time: f64,
    /// The trigger end time in milliseconds.
    pub end_time: f64,
    /// The trigger group index.
    pub group: i32,
}

/// A sprite handle pair.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
pub struct HandleRequest {
    /// The layer handle.
    pub layer: usize,
    /// The sprite handle.
    pub sprite: usize,
}

/// A sprite handle pair plus a sample time.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
pub struct SpriteTimeRequest {
    /// The layer handle.
    pub layer: usize,
    /// The sprite handle.
    pub sprite: usize,
    /// The sample time in milliseconds.
    pub time: f64,
}

/// A time query.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
pub struct TimeRequest {
    /// The time in milliseconds.
    pub time: f64,
}

/// A deterministic random request.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
pub struct RandomRequest {
    /// The inclusive lower bound.
    pub min: f64,
    /// The exclusive upper bound.
    pub max: f64,
}

/// An easing evaluation request.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
pub struct ApplyEasingRequest {
    /// The easing function.
    pub easing: Easing,
    /// The normalized time in `0.0..=1.0`.
    pub t: f64,
}

/// A named-or-custom easing evaluation request.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EasingValueRequest {
    /// A named easing (used when `cubicBezier` is absent).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub easing: Option<Easing>,
    /// A CSS-style cubic-bezier's control points `[x1, y1, x2, y2]`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cubic_bezier: Option<[f64; 4]>,
    /// The normalized time in `0.0..=1.0`.
    pub t: f64,
}

/// Create a sound sample on a layer.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateSampleRequest {
    /// The target layer name (omitted when `layerHandle` is set).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
    /// The target segment handle (takes precedence over `layer`).
    #[serde(rename = "layerHandle", skip_serializing_if = "Option::is_none")]
    pub layer_handle: Option<usize>,
    /// The sample path (relative to the mapset).
    pub path: String,
    /// The sample time in milliseconds.
    pub time: f64,
    /// The sample volume (`0..=100`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<f64>,
}

/// A segment handle request.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
pub struct SegmentRequest {
    /// The segment handle.
    pub layer: usize,
}

/// Set a segment's reverse-depth flag.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
pub struct SegmentReverseDepthRequest {
    /// The segment handle.
    pub layer: usize,
    /// Whether children are emitted in reverse order.
    pub value: bool,
}

/// Record a depth index on a segment.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
pub struct SegmentAddIndexRequest {
    /// The segment handle.
    pub layer: usize,
    /// The index to record.
    pub index: f64,
}

/// Move a segment to another osu! layer (`ChangePosition`).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SegmentChangePositionRequest {
    /// The segment handle.
    pub layer: usize,
    /// The destination osu! layer.
    pub osb_layer: OsbLayer,
}

/// A curve query.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CurveRequest {
    /// The curve.
    pub curve: CurveSpec,
}

/// A curve query at a parametric or arc-length position.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CurvePointRequest {
    /// The curve.
    pub curve: CurveSpec,
    /// The position in `0.0..=1.0`.
    pub t: f64,
    /// Sample by arc length instead of the raw parameter (default `false`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_arc_length: Option<bool>,
}

/// A curve sampling request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CurveSampleRequest {
    /// The curve.
    pub curve: CurveSpec,
    /// The number of points to sample.
    pub count: usize,
    /// Sample by arc length instead of the raw parameter (default `false`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_arc_length: Option<bool>,
}

/// A Douglas-Peucker simplification request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct SimplifyRequest {
    /// The polyline to simplify.
    pub points: Vec<Vector2>,
    /// The maximum deviation to allow.
    pub tolerance: f64,
}

/// A curve offset request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CurveOffsetRequest {
    /// The curve.
    pub curve: CurveSpec,
    /// The signed offset along the curve's local normal.
    pub distance: f64,
}

/// A curve hit-test request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CurveDistanceRequest {
    /// The curve.
    pub curve: CurveSpec,
    /// The query point.
    pub point: Vector2,
}

/// An FFT request against the runtime's loaded audio.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetFftRequest {
    /// Sample time in milliseconds.
    pub time: f64,
    /// Number of frequency bins.
    pub magnitudes: usize,
    /// The easing applied across the bins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub easing: Option<Easing>,
    /// A frequency cut-off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cutoff: Option<f64>,
}

/// The anchor offset for an origin.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
pub struct GetOriginVectorRequest {
    /// The sprite anchor origin.
    pub origin: OsbOrigin,
    /// The sprite width.
    pub width: f64,
    /// The sprite height.
    pub height: f64,
}

/// Whether a rotated sprite's bounding box intersects the widescreen
/// storyboard area.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
pub struct InScreenBoundsRequest {
    /// The sprite centre position.
    pub position: Vector2,
    /// The sprite size.
    pub size: Vector2,
    /// The rotation in radians.
    pub rotation: f64,
    /// The anchor offset (from `GetOriginVector`).
    pub origin: Vector2,
}
