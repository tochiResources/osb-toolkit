//! The optimization library wire types (M30): the knobs for an optimization
//! pass, the resulting stats, and the optimized `.osb` text. `OptimizeStats`
//! lives in `osb-runtime` (the source of truth) and is re-exported here.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub use osb_runtime::optimize::OptimizeStats;

/// An `.osb` optimization request.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OptimizeRequest {
    /// The `.osb` storyboard text.
    pub text: String,
    /// The optimization options.
    pub options: OptimizeOptionsInput,
}

/// The knobs controlling an optimization pass (all default to `false`/none).
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OptimizeOptionsInput {
    /// Remove zero-duration commands whose value does not change.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_redundant: Option<bool>,
    /// Remove commands identical to an earlier command of the same type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deduplicate: Option<bool>,
    /// Merge a command with the next when they are continuous.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_adjacent: Option<bool>,
    /// Split commands longer than this many milliseconds (into linear pieces).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_threshold: Option<f64>,
}

/// The optimized `.osb` text plus the stats.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct OptimizeResult {
    /// The optimized `.osb` storyboard text.
    pub osb: String,
    /// The effect of the pass.
    pub stats: OptimizeStats,
}
