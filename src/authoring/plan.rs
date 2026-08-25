use std::collections::BTreeMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::model::Record;

pub const CAPTURE_PLAN_SCHEMA: &str = "pk-capture-plan/v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldOrigin {
    Authored,
    Generated,
    Observed,
    Suggested,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlanOperation {
    pub op: String,
    pub path: String,
    pub record: Record,
    #[serde(default)]
    pub field_origins: BTreeMap<String, FieldOrigin>,
}

impl PlanOperation {
    pub fn create_record(
        path: String,
        record: Record,
        field_origins: BTreeMap<String, FieldOrigin>,
    ) -> Self {
        Self {
            op: "create_record".to_string(),
            path,
            record,
            field_origins,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PlanPrecondition {
    OutputPath { path: String },
    ExistingRecordState { path: String, state: String },
    NativeBlobState { path: String, state: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlanMessage {
    pub code: String,
    pub message: String,
}

impl PlanMessage {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapturePlan {
    pub schema: String,
    pub plan_id: Uuid,
    pub created_at: String,
    #[serde(default)]
    pub operations: Vec<PlanOperation>,
    #[serde(default)]
    pub preconditions: Vec<PlanPrecondition>,
    #[serde(default)]
    pub warnings: Vec<PlanMessage>,
    #[serde(default)]
    pub blockers: Vec<PlanMessage>,
}

impl CapturePlan {
    pub fn has_blockers(&self) -> bool {
        !self.blockers.is_empty()
    }

    pub fn validate_envelope(&self) -> Result<()> {
        if self.schema != CAPTURE_PLAN_SCHEMA {
            return Err(Error::AuthoringInput(format!(
                "unsupported capture plan schema {}",
                self.schema
            )));
        }
        for operation in &self.operations {
            if operation.op != "create_record" {
                return Err(Error::AuthoringInput(format!(
                    "unsupported capture operation {}",
                    operation.op
                )));
            }
        }
        Ok(())
    }

    pub fn contains_unconfirmed_suggestion(&self) -> bool {
        self.operations.iter().any(|operation| {
            operation
                .field_origins
                .values()
                .any(|origin| *origin == FieldOrigin::Suggested)
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyResult {
    pub plan_id: Uuid,
    pub created: Vec<String>,
    pub no_op: Vec<String>,
    pub valid: bool,
}

pub fn load_plan(path: &Path) -> Result<CapturePlan> {
    let text = std::fs::read_to_string(path)?;
    let plan: CapturePlan = serde_json::from_str(&text)?;
    plan.validate_envelope()?;
    Ok(plan)
}

pub fn save_plan(path: &Path, plan: &CapturePlan) -> Result<()> {
    plan.validate_envelope()?;
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut text = serde_json::to_string_pretty(plan)?;
    text.push('\n');
    std::fs::write(path, text)?;
    Ok(())
}
