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

    #[error("authoring input is incomplete or ambiguous: {0}")]
    AuthoringInput(String),

    #[error("capture plan is blocked: {0}")]
    BlockedPlan(String),

    #[error("capture plan is stale: {0}")]
    StalePlan(String),

    #[error("capture plan conflicts with existing state: {0}")]
    Conflict(String),
}

impl Error {
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::Json(_) | Self::AuthoringInput(_) => 3,
            Self::StalePlan(_) => 4,
            Self::Schema { .. } | Self::InvalidRecord { .. } | Self::CrossReference(_) => 5,
            Self::Conflict(_) => 6,
            Self::BlockedPlan(_) => 3,
            _ => 1,
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
