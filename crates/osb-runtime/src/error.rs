use thiserror::Error;

/// Errors produced while building or rendering a storyboard.
#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("i/o error: {0}")]
    IO(#[from] std::io::Error),

    #[error("failed to parse .osu: {0}")]
    Mapset(String),

    #[error("unsupported effect construct: {0}")]
    Unsupported(String),

    #[error("asset error: {0}")]
    Asset(String),

    #[error("render error: {0}")]
    Render(String),
}

impl From<osb_core::io::Error> for RuntimeError {
    fn from(error: osb_core::io::Error) -> Self {
        RuntimeError::IO(error.into())
    }
}

pub type RuntimeResult<T> = Result<T, RuntimeError>;
