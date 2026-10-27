//! The image-generation library wire types (M25): drawing primitive shapes
//! (rect/rounded-rect/ellipse/circle/line/polygon) with fill and stroke onto a
//! new raster image, written to a file.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::math::Vector2;

/// An 8-bit RGBA colour.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
pub struct RgbaColor {
    /// Red channel.
    pub r: u8,
    /// Green channel.
    pub g: u8,
    /// Blue channel.
    pub b: u8,
    /// Alpha channel.
    pub a: u8,
}

/// A primitive shape kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum ShapeKind {
    /// An axis-aligned rectangle.
    Rect,
    /// A rectangle with rounded corners.
    RoundedRect,
    /// An ellipse.
    Ellipse,
    /// A circle.
    Circle,
    /// A straight line segment.
    Line,
    /// A closed polygon.
    Polygon,
}

/// A shape to draw.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Shape {
    /// The shape kind.
    pub kind: ShapeKind,
    /// The centre (rect/rounded-rect/ellipse/circle).
    pub center: Vector2,
    /// The size (width and height; a circle's diameter is `size.x`).
    pub size: Vector2,
    /// The corner radius for a rounded rectangle.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_radius: Option<f64>,
    /// Explicit points (line endpoints, polygon vertices).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub points: Option<Vec<Vector2>>,
    /// The fill colour.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<RgbaColor>,
    /// The stroke colour.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<RgbaColor>,
    /// The stroke width in pixels (default `1`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<f64>,
}

/// A shape-rendering request: a canvas, a background and a shape list.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RenderShapesRequest {
    /// The canvas width in pixels.
    pub width: u32,
    /// The canvas height in pixels.
    pub height: u32,
    /// The background colour (default transparent).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<RgbaColor>,
    /// The shapes to draw, in order.
    pub shapes: Vec<Shape>,
    /// The output file path (its extension selects the format).
    pub output: String,
}
