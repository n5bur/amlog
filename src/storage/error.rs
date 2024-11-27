use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("ADIF error: {0}")]
    Adif(String),

    #[error("Data validation error: {0}")]
    Validation(String),

    #[error("Storage backend error: {0}")]
    Backend(String),
}
