//! The generic stateless math operation (M34): one op that dispatches to the full
//! `osb-math`/`osb-curve` helper surface by name, so the math SDK can expose
//! every vector/matrix/quaternion/box/color/curve helper without a wire op each.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A stateless math operation: a function name plus its positional arguments.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MathRequest {
    /// The function name, e.g. `vector.add`, `matrix.multiply`, `curve.length`.
    pub name: String,
    /// The positional arguments (numbers, vectors, matrices, quaternions, ...).
    #[serde(default)]
    pub args: Vec<serde_json::Value>,
}

/// The result of a math operation (a number, boolean, vector, matrix, ...).
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MathResult {
    /// The result value.
    pub value: serde_json::Value,
}
