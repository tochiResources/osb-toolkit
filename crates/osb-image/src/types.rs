//! The image library wire types: image metadata and the fine-grained processing
//! options (resize/crop/rotate/flip/colour/filter). Images cross the bridge by
//! file path so the JSON stays small.
//!
//! These live here - the image library is the source of truth - and the wire
//! contract (`osb-api`) re-exports them.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Image metadata.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ImageInfo {
    /// The image width in pixels.
    pub width: u32,
    /// The image height in pixels.
    pub height: u32,
    /// The format (e.g. `PNG`), inferred from the path.
    pub format: String,
}

/// A crop rectangle.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ImageCrop {
    /// The left edge.
    pub x: u32,
    /// The top edge.
    pub y: u32,
    /// The width.
    pub width: u32,
    /// The height.
    pub height: u32,
}

/// Fine-grained image processing options (all optional; applied in order).
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ImageOptions {
    /// Resize to this width (the height is derived when omitted).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_width: Option<u32>,
    /// Resize to this height (the width is derived when omitted).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_height: Option<u32>,
    /// Crop to a rectangle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crop: Option<ImageCrop>,
    /// Rotate by degrees (rounded to a multiple of 90).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate: Option<f64>,
    /// Mirror horizontally.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flip_horizontal: Option<bool>,
    /// Mirror vertically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flip_vertical: Option<bool>,
    /// Convert to grayscale.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grayscale: Option<bool>,
    /// Invert the colours.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert: Option<bool>,
    /// Brightness in `-1.0..=1.0`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness: Option<f64>,
    /// Contrast in `-1.0..=1.0`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contrast: Option<f64>,
    /// Gaussian blur sigma.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blur: Option<f64>,
}

/// An image processing request: read `path`, apply `options`, write `output`.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ImageProcessRequest {
    /// The input file path.
    pub path: String,
    /// The output file path (its extension selects the format).
    pub output: String,
    /// The processing options.
    pub options: ImageOptions,
}
