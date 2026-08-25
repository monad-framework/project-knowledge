pub mod authoring;
pub mod compiler;
pub mod error;
pub mod git;
pub mod model;
pub mod records;
pub mod resolver;
pub mod store;

pub use authoring::{
    ApplyResult, AuthoringIntent, CapturePlan, FieldOrigin, RecordCatalog, apply_capture_plan,
    build_capture_plan, load_plan, parse_intent, render_plan, save_plan,
};
pub use compiler::{CompileReport, compile, compile_in_memory, default_db_path, rebuild};
pub use error::{Error, Result};
pub use model::*;
pub use records::{load_records, validate_cross_references, validate_records};
pub use resolver::{EvidenceState, Freshness, Resolution, ResolutionOutcome};
pub use store::ReadModel;
