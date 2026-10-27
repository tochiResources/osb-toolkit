//! Wire responses for the operations that do not return a richer model.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::math::Vector2;

/// The native runtime version.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct VersionResult {
    /// The runtime version string.
    pub version: String,
}

/// The result of importing a `.osb` storyboard.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ImportOsbResult {
    /// The sprite count after import.
    pub sprite_count: usize,
}

/// A summary of a parsed `.osb` storyboard.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ParseOsbResult {
    /// The number of layers.
    pub layer_count: usize,
    /// The number of sprites.
    pub sprite_count: usize,
}

/// A summary of a loaded beatmap.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoadBeatmapResult {
    /// The number of hit objects.
    pub hit_objects: usize,
    /// The number of control points.
    pub timing_points: usize,
}

/// The storyboard sprite count.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SpriteCountResult {
    /// The number of sprites.
    pub sprite_count: usize,
}

/// The loaded audio duration.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AudioDurationResult {
    /// The audio duration in milliseconds (`0` when no audio is loaded).
    pub audio_duration: f64,
}

/// A deterministic random value.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
pub struct RandomResult {
    /// The sampled value in `[min, max)`.
    pub value: f64,
}

/// An easing function applied to a normalized time.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
pub struct EasingResult {
    /// The eased value.
    pub value: f64,
}

/// A generic acknowledgement.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
pub struct OkResult {
    /// Always `true` on success.
    pub ok: bool,
}

/// FFT magnitudes.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct FftResult {
    /// The magnitude of each frequency bin.
    pub magnitudes: Vec<f64>,
}

/// A single scalar value.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
pub struct ScalarResult {
    /// The value.
    pub value: f64,
}

/// A single boolean value.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
pub struct BoolResult {
    /// The value.
    pub value: bool,
}

/// A segment's depth indexes and bounds.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SegmentIndexesResult {
    /// The recorded indexes, in insertion order.
    pub indexes: Vec<f64>,
    /// The lowest recorded index.
    pub lowest: f64,
    /// The highest recorded index.
    pub highest: f64,
}

/// The result of cleaning a segment.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SegmentCleanResult {
    /// The number of zero-duration sprites removed.
    pub removed: usize,
}

/// The result of discarding a segment.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SegmentDiscardResult {
    /// The number of remaining top-level layers.
    pub layer_count: usize,
}

/// A list of points.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PointListResult {
    /// The points.
    pub points: Vec<Vector2>,
}

/// The error envelope returned when an operation fails.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ErrorResult {
    /// A human-readable error message.
    pub error: String,
}
