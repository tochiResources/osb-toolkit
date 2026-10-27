//! The `curve` library wire types: the curve families and their control
//! points. The math is canonical in `crates/osb-curve`; these types are the
//! stable contract the SDKs are generated from.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::math::Vector2;

pub use osb_runtime::curve::CurveKind;

/// A curve: the interpolation plus its control points.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CurveSpec {
    /// The curve family.
    pub kind: CurveKind,
    /// The control points.
    pub points: Vec<Vector2>,
}

/// A circular-arc sampling request.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ArcRequest {
    /// The arc centre.
    pub center: Vector2,
    /// The radius.
    pub radius: f64,
    /// The start angle in degrees.
    pub start_angle: f64,
    /// The sweep angle in degrees.
    pub sweep_angle: f64,
    /// The number of points (default `64`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
}

/// An ellipse sampling request.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct EllipseRequest {
    /// The ellipse centre.
    pub center: Vector2,
    /// The horizontal radius.
    pub radius_x: f64,
    /// The vertical radius.
    pub radius_y: f64,
    /// The number of points (default `64`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
}

/// A line-segment intersection request.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
pub struct LineIntersectRequest {
    /// The first segment's start.
    pub a1: Vector2,
    /// The first segment's end.
    pub a2: Vector2,
    /// The second segment's start.
    pub b1: Vector2,
    /// The second segment's end.
    pub b2: Vector2,
}

/// An optional point result.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, ToSchema)]
pub struct OptionalPointResult {
    /// The point, or `null` when there is none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point: Option<Vector2>,
}
