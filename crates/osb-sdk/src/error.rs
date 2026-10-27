use thiserror::Error;

/// Errors returned by the SDK.
#[derive(Debug, Error)]
pub enum Error {
    /// The native bridge returned an error envelope.
    #[error("osbtk bridge error: {0}")]
    Bridge(String),
    /// A request or response could not be (de)serialized.
    #[error("osbtk json error: {0}")]
    Json(#[from] serde_json::Error),
}
