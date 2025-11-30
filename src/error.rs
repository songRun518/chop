use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("{0}")]
    Io(#[from] io::Error),

    #[error("{0}")]
    Serde(#[from] serde_json::Error),

    #[error("Scoop is not found.")]
    ScoopNotFound,
}
