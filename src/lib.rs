pub mod compiler;
pub mod error;
pub mod git;
pub mod model;
pub mod records;
pub mod resolver;
pub mod store;

pub use compiler::{CompileReport, compile, compile_in_memory, default_db_path, rebuild};
pub use error::{Error, Result};
pub use model::*;
pub use records::{load_records, validate_cross_references};
pub use resolver::{EvidenceState, Freshness, Resolution, ResolutionOutcome};
pub use store::ReadModel;
