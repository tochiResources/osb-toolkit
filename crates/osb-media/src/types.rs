//! The media library wire types: media metadata, frame extraction/encoding and
//! audio extraction. Media crosses the bridge by file path and is processed by
//! the `ffmpeg`/`ffprobe` binaries.
//!
//! These live here - the media library is the source of truth - and the wire
//! contract (`osb-api`) re-exports them.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Media metadata.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MediaInfo {
    /// The duration in seconds (`0` when unknown).
    pub duration: f64,
    /// The video width in pixels (`0` for audio-only).
    pub width: u32,
    /// The video height in pixels (`0` for audio-only).
    pub height: u32,
    /// The container/format name.
    pub format: String,
    /// The video codec name, when present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_codec: Option<String>,
    /// The audio codec name, when present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_codec: Option<String>,
}

/// A frame-extraction request (video -> image sequence).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ExtractFramesRequest {
    /// The input media path.
    pub input: String,
    /// The directory to write frames into.
    pub output_dir: String,
    /// The sampling rate (frames per second); defaults to the source rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fps: Option<f64>,
    /// The start time in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
    /// The maximum number of frames to extract.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    /// Scale frames to this width.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    /// Scale frames to this height.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
}

/// The result of a frame extraction.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FrameResult {
    /// The number of frames written.
    pub count: u32,
    /// The directory the frames were written into.
    pub output_dir: String,
    /// The frame filename pattern (`frame_%04d.png`).
    pub pattern: String,
}

/// An image-sequence encoding request (frames -> video).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EncodeVideoRequest {
    /// The input filename pattern (e.g. `frames/frame_%04d.png`).
    pub input_pattern: String,
    /// The output video path.
    pub output: String,
    /// The frame rate.
    pub fps: f64,
    /// The video codec (default `libx264`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    /// The constant-rate-factor quality (`0`-`51`; lower is better).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crf: Option<u32>,
}

/// An audio-extraction request (media -> audio file).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ExtractAudioRequest {
    /// The input media path.
    pub input: String,
    /// The output audio path (its extension selects the format).
    pub output: String,
}
