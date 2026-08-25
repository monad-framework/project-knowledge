use std::collections::BTreeMap;

use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::error::{Error, Result};

pub const RECORD_SCHEMA: &str = "pk/v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct NativeReference {
    pub source_system: String,
    pub object_type: String,
    pub locator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl NativeReference {
    pub fn key(&self) -> (String, String, String) {
        (
            self.source_system.clone(),
            self.object_type.clone(),
            self.locator.clone(),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationshipOrigin {
    Authored,
    Imported,
    Derived,
    Inferred,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityKind {
    Subject,
    Representation,
    Claim,
    Assertion,
    Authority,
    Relationship,
    Activity,
    Context,
    EvidenceEvaluation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntityRef {
    pub kind: EntityKind,
    pub id: Uuid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceResult {
    Pass,
    Fail,
    Inconclusive,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObservationStatus {
    Available,
    Missing,
    NotRepository,
    Unavailable,
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceObservation {
    pub source_system: String,
    pub object_type: String,
    pub locator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    pub status: ObservationStatus,
    pub observed_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl SourceObservation {
    pub fn key(&self) -> (String, String, String) {
        (
            self.source_system.clone(),
            self.object_type.clone(),
            self.locator.clone(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Record {
    Subject {
        schema: String,
        id: Uuid,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        label: Option<String>,
    },
    Representation {
        schema: String,
        id: Uuid,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        subject_id: Option<Uuid>,
        role: String,
        native: NativeReference,
    },
    Claim {
        schema: String,
        id: Uuid,
        subject_id: Uuid,
        concern: String,
        value: Value,
    },
    Assertion {
        schema: String,
        id: Uuid,
        claim_id: Uuid,
        representation_id: Uuid,
        recorded_at: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        valid_from: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        valid_until: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        source_state: Option<NativeReference>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        context_id: Option<Uuid>,
    },
    Authority {
        schema: String,
        id: Uuid,
        subject_id: Uuid,
        concern: String,
        representation_id: Uuid,
        basis: String,
        recorded_at: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        valid_from: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        valid_until: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        context_id: Option<Uuid>,
    },
    Relationship {
        schema: String,
        id: Uuid,
        from: EntityRef,
        relation: String,
        to: EntityRef,
        origin: RelationshipOrigin,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        activity_id: Option<Uuid>,
    },
    Activity {
        schema: String,
        id: Uuid,
        activity_type: String,
        recorded_at: String,
        #[serde(default)]
        used: Vec<NativeReference>,
        #[serde(default)]
        generated_representation_ids: Vec<Uuid>,
    },
    Context {
        schema: String,
        id: Uuid,
        #[serde(default)]
        dimensions: BTreeMap<String, String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        source_state: Option<NativeReference>,
    },
    EvidenceEvaluation {
        schema: String,
        id: Uuid,
        claim_id: Uuid,
        method: String,
        result: EvidenceResult,
        recorded_at: String,
        #[serde(default)]
        inputs: Vec<NativeReference>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        context_id: Option<Uuid>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        notes: Option<String>,
    },
}

impl Record {
    pub fn id(&self) -> Uuid {
        match self {
            Self::Subject { id, .. }
            | Self::Representation { id, .. }
            | Self::Claim { id, .. }
            | Self::Assertion { id, .. }
            | Self::Authority { id, .. }
            | Self::Relationship { id, .. }
            | Self::Activity { id, .. }
            | Self::Context { id, .. }
            | Self::EvidenceEvaluation { id, .. } => *id,
        }
    }

    pub fn kind_name(&self) -> &'static str {
        match self {
            Self::Subject { .. } => "subject",
            Self::Representation { .. } => "representation",
            Self::Claim { .. } => "claim",
            Self::Assertion { .. } => "assertion",
            Self::Authority { .. } => "authority",
            Self::Relationship { .. } => "relationship",
            Self::Activity { .. } => "activity",
            Self::Context { .. } => "context",
            Self::EvidenceEvaluation { .. } => "evidence_evaluation",
        }
    }

    pub fn schema(&self) -> &str {
        match self {
            Self::Subject { schema, .. }
            | Self::Representation { schema, .. }
            | Self::Claim { schema, .. }
            | Self::Assertion { schema, .. }
            | Self::Authority { schema, .. }
            | Self::Relationship { schema, .. }
            | Self::Activity { schema, .. }
            | Self::Context { schema, .. }
            | Self::EvidenceEvaluation { schema, .. } => schema,
        }
    }

    pub fn native_references(&self) -> Vec<&NativeReference> {
        match self {
            Self::Representation { native, .. } => vec![native],
            Self::Assertion { source_state, .. } => source_state.iter().collect(),
            Self::Activity { used, .. } => used.iter().collect(),
            Self::Context { source_state, .. } => source_state.iter().collect(),
            Self::EvidenceEvaluation { inputs, .. } => inputs.iter().collect(),
            _ => Vec::new(),
        }
    }

    pub fn semantic_validate(&self) -> Result<()> {
        if self.schema() != RECORD_SCHEMA {
            return Err(Error::InvalidRecord {
                id: self.id().to_string(),
                message: format!("unsupported schema {}", self.schema()),
            });
        }

        match self {
            Self::Representation { role, native, .. } => {
                require_nonempty(self.id(), "role", role)?;
                validate_native(self.id(), native)?;
            }
            Self::Claim { concern, .. } => {
                require_nonempty(self.id(), "concern", concern)?;
            }
            Self::Assertion {
                recorded_at,
                valid_from,
                valid_until,
                source_state,
                ..
            } => {
                parse_timestamp(self.id(), "recorded_at", recorded_at)?;
                validate_window(self.id(), valid_from.as_deref(), valid_until.as_deref())?;
                if let Some(native) = source_state {
                    validate_native(self.id(), native)?;
                }
            }
            Self::Authority {
                concern,
                basis,
                recorded_at,
                valid_from,
                valid_until,
                ..
            } => {
                require_nonempty(self.id(), "concern", concern)?;
                require_nonempty(self.id(), "basis", basis)?;
                parse_timestamp(self.id(), "recorded_at", recorded_at)?;
                validate_window(self.id(), valid_from.as_deref(), valid_until.as_deref())?;
            }
            Self::Relationship { relation, .. } => {
                require_nonempty(self.id(), "relation", relation)?;
            }
            Self::Activity {
                activity_type,
                recorded_at,
                used,
                ..
            } => {
                require_nonempty(self.id(), "activity_type", activity_type)?;
                parse_timestamp(self.id(), "recorded_at", recorded_at)?;
                for native in used {
                    validate_native(self.id(), native)?;
                }
            }
            Self::Context { source_state, .. } => {
                if let Some(native) = source_state {
                    validate_native(self.id(), native)?;
                }
            }
            Self::EvidenceEvaluation {
                method,
                recorded_at,
                inputs,
                ..
            } => {
                require_nonempty(self.id(), "method", method)?;
                parse_timestamp(self.id(), "recorded_at", recorded_at)?;
                for native in inputs {
                    validate_native(self.id(), native)?;
                }
            }
            Self::Subject { .. } => {}
        }

        Ok(())
    }
}

fn require_nonempty(id: Uuid, field: &str, value: &str) -> Result<()> {
    if value.trim().is_empty() {
        return Err(Error::InvalidRecord {
            id: id.to_string(),
            message: format!("{field} must not be empty"),
        });
    }
    Ok(())
}

fn validate_native(id: Uuid, native: &NativeReference) -> Result<()> {
    require_nonempty(id, "native.source_system", &native.source_system)?;
    require_nonempty(id, "native.object_type", &native.object_type)?;
    require_nonempty(id, "native.locator", &native.locator)?;
    Ok(())
}

fn parse_timestamp(id: Uuid, field: &str, value: &str) -> Result<Timestamp> {
    value
        .parse::<Timestamp>()
        .map_err(|error| Error::InvalidRecord {
            id: id.to_string(),
            message: format!("{field} is not a valid RFC3339 timestamp: {error}"),
        })
}

fn validate_window(id: Uuid, from: Option<&str>, until: Option<&str>) -> Result<()> {
    let from = from
        .map(|value| parse_timestamp(id, "valid_from", value))
        .transpose()?;
    let until = until
        .map(|value| parse_timestamp(id, "valid_until", value))
        .transpose()?;
    if let (Some(from), Some(until)) = (from, until)
        && from >= until
    {
        return Err(Error::InvalidRecord {
            id: id.to_string(),
            message: "valid_from must be earlier than valid_until".to_string(),
        });
    }
    Ok(())
}

pub fn window_contains(
    valid_from: Option<&str>,
    valid_until: Option<&str>,
    at: &Timestamp,
) -> Result<bool> {
    let from = valid_from.map(str::parse::<Timestamp>).transpose()?;
    let until = valid_until.map(str::parse::<Timestamp>).transpose()?;
    Ok(from.as_ref().is_none_or(|value| value <= at)
        && until.as_ref().is_none_or(|value| at < value))
}
