//! The generic motion-graphics helper operations.
//!
//! Two bridge ops - `storyboarding3dHelper` and `storyboardHelper` - dispatch by
//! `name` to the canonical Rust helper libraries
//! (`osb_runtime::storyboarding3d::helpers` and
//! `osb_runtime::storyboarding::helpers`), so the whole set can grow without a
//! new wire op per helper. `input` and `value` are shape-typed JSON: a keyframe
//! `channel` (`{ keys: [{ time, value: [...] }] }`) for the 3D helpers, and a
//! `commands` list (`StoryboardCommand[]`) for the 2D helpers.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A motion-graphics helper call: a helper `name` plus its input.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HelperRequest {
    /// The helper name, e.g. `flatten`, `orbit`, `selectCommands`.
    pub name: String,
    /// The helper's input (a keyframe channel, a command list, or its parameters).
    #[serde(default)]
    pub input: serde_json::Value,
}

/// The result of a motion-graphics helper (a keyframe channel, a command list,
/// a list of vectors, or a scalar).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HelperResult {
    /// The result value.
    pub value: serde_json::Value,
}
