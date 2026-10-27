//! The optimization library: storyboard optimization and `.osb` round-trip (M30).

use crate::error::Error;
use crate::models::{OptimizeOptionsInput, OptimizeResult};

/// Optimizes an `.osb` storyboard, returning the optimized text and stats.
pub fn optimize_osb(text: &str, options: OptimizeOptionsInput) -> Result<OptimizeResult, Error> {
    crate::Client::optimize_osb(text, options)
}
