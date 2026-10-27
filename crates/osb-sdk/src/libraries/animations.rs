//! The animations library: easing evaluation.

use crate::error::Error;
use crate::models::Easing;

/// Applies a named easing to a normalized time in `0..1`.
pub fn apply_easing(easing: Easing, t: f64) -> Result<f64, Error> {
    crate::Client::apply_easing(easing, t)
}

/// Evaluates a CSS cubic-bezier easing (`CustomEasing`).
pub fn apply_cubic_bezier(x1: f64, y1: f64, x2: f64, y2: f64, t: f64) -> Result<f64, Error> {
    crate::Client::apply_cubic_bezier(x1, y1, x2, y2, t)
}
