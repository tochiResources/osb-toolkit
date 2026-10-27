//! The curve library: Bezier/Catmull-Rom curves, sampling, arc length, tangents,
//! bounding boxes and Douglas-Peucker simplification.

use crate::error::Error;
use crate::models::{Box2, CurveSpec, PointListResult, Vector2};

/// The arc length of a curve.
pub fn length(curve: CurveSpec) -> Result<f64, Error> {
    crate::Client::curve_length(curve)
}

/// The axis-aligned bounding box of a curve.
pub fn bounding_box(curve: CurveSpec) -> Result<Box2, Error> {
    crate::Client::curve_bounding_box(curve)
}

/// The point at `t`, parametrically or by arc length.
pub fn point(curve: CurveSpec, t: f64, by_arc_length: bool) -> Result<Vector2, Error> {
    crate::Client::curve_point(curve, t, by_arc_length)
}

/// The unit tangent at `t`.
pub fn tangent(curve: CurveSpec, t: f64) -> Result<Vector2, Error> {
    crate::Client::curve_tangent(curve, t)
}

/// `count` points along the curve, parametrically or by arc length.
pub fn sample(
    curve: CurveSpec,
    count: usize,
    by_arc_length: bool,
) -> Result<PointListResult, Error> {
    crate::Client::curve_sample(curve, count, by_arc_length)
}

/// Simplifies a polyline with the Douglas-Peucker algorithm.
pub fn simplify(points: Vec<Vector2>, tolerance: f64) -> Result<PointListResult, Error> {
    crate::Client::simplify(points, tolerance)
}

/// The unit normal at `t`.
pub fn normal(curve: CurveSpec, t: f64) -> Result<Vector2, Error> {
    crate::Client::curve_normal(curve, t)
}

/// The curvature at `t`.
pub fn curvature(curve: CurveSpec, t: f64) -> Result<f64, Error> {
    crate::Client::curve_curvature(curve, t)
}

/// The distance from a point to the curve.
pub fn distance_to(curve: CurveSpec, point: Vector2) -> Result<f64, Error> {
    crate::Client::curve_distance_to(curve, point)
}

/// The polyline offset along the curve's local normal.
pub fn offset(curve: CurveSpec, distance: f64) -> Result<PointListResult, Error> {
    crate::Client::curve_offset(curve, distance)
}

/// Points along a circular arc.
pub fn arc(
    center: Vector2,
    radius: f64,
    start_angle: f64,
    sweep_angle: f64,
    count: usize,
) -> Result<PointListResult, Error> {
    crate::Client::curve_arc(center, radius, start_angle, sweep_angle, count)
}

/// Points around an ellipse.
pub fn ellipse(
    center: Vector2,
    radius_x: f64,
    radius_y: f64,
    count: usize,
) -> Result<PointListResult, Error> {
    crate::Client::curve_ellipse(center, radius_x, radius_y, count)
}

/// The intersection of two line segments, if any.
pub fn line_intersect(
    a1: Vector2,
    a2: Vector2,
    b1: Vector2,
    b2: Vector2,
) -> Result<crate::models::OptionalPointResult, Error> {
    crate::Client::line_intersect(a1, a2, b1, b2)
}
