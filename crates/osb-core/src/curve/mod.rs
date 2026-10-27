//! Canonical bezier / curve / points library.
//!
//! One implementation of the path math effect-script consumers need -
//! linear/quadratic/cubic/general Bezier and Catmull-Rom curves, uniform and
//! arc-length sampling, length, tangents, bounding boxes and Douglas-Peucker
//! simplification - exposed through the C ABI, the OpenAPI contract and every
//! SDK so no language duplicates the algorithm.

pub mod family;
pub mod primitives;
pub mod sample;
pub mod simplify;
pub mod slider;

pub use family::{Curve, CurveKind};
pub use primitives::{arc_points, ellipse_points, line_intersection};
pub use sample::SAMPLES_PER_SEGMENT;
pub use simplify::simplify;
pub use slider::{CurveType, SliderPath};

/// The shared math types, re-exported so consumers need only depend on
/// `osb-core`.
pub use crate::math::{Box2, Vec2};
