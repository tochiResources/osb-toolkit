use thiserror::Error;

/// Errors produced while parsing an After Effects project.
#[derive(Debug, Error)]
pub enum AeError {
    /// The file does not start with the `RIFX` magic.
    #[error("not an After Effects project file")]
    NotRifx,

    /// The RIFX structure was present but the project tree was not.
    #[error("malformed After Effects project: {0}")]
    Malformed(String),

    /// Reading the file failed.
    #[error("i/o error: {0}")]
    IO(#[from] std::io::Error),
}

/// Convenience alias for parser results.
pub type AeResult<T> = Result<T, AeError>;
