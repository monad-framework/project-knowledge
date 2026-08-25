use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("time parse error: {0}")]
    Time(#[from] jiff::Error),

    #[error("Git command failed: {0}")]
    Git(String),

    #[error("schema validation failed for {path}: {message}")]
    Schema { path: PathBuf, message: String },

    #[error("invalid Project Knowledge record {id}: {message}")]
    InvalidRecord { id: String, message: String },

    #[error("record reference validation failed: {0}")]
    CrossReference(String),

    #[error("record not found: {0}")]
    NotFound(String),
}

pub type Result<T> = std::result::Result<T, Error>;
