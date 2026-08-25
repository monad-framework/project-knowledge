use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::model::{EntityKind, EvidenceResult, RelationshipOrigin};

pub const AUTHORING_SCHEMA: &str = "pk-authoring/v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthoringIntent {
    pub schema: String,
    #[serde(default)]
    pub subject: Option<SubjectIntent>,
    #[serde(default)]
    pub representations: Vec<RepresentationIntent>,
    #[serde(default)]
    pub claims: Vec<ClaimIntent>,
    #[serde(default)]
    pub contexts: Vec<ContextIntent>,
    #[serde(default)]
    pub activities: Vec<ActivityIntent>,
    #[serde(default)]
    pub assertions: Vec<AssertionIntent>,
    #[serde(default)]
    pub authorities: Vec<AuthorityIntent>,
    #[serde(default)]
    pub relationships: Vec<RelationshipIntent>,
    #[serde(default)]
    pub evidence_evaluations: Vec<EvidenceIntent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SubjectIntent {
    #[serde(rename = "as")]
    pub alias: String,
    #[serde(default)]
    pub new: Option<NewSubjectIntent>,
    #[serde(default)]
    pub existing: Option<ExistingSelector>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NewSubjectIntent {
    #[serde(default)]
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExistingSelector {
    pub kind: EntityKind,
    #[serde(default)]
    pub id: Option<uuid::Uuid>,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub locator: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReferenceInput {
    Alias(String),
    Existing { existing: ExistingSelector },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RepresentationIntent {
    #[serde(rename = "as")]
    pub alias: String,
    pub subject: ReferenceInput,
    pub path: String,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClaimIntent {
    #[serde(rename = "as")]
    pub alias: String,
    pub subject: ReferenceInput,
    pub concern: String,
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContextIntent {
    #[serde(rename = "as")]
    pub alias: String,
    #[serde(default)]
    pub dimensions: BTreeMap<String, String>,
    #[serde(default)]
    pub source_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActivityIntent {
    #[serde(rename = "as")]
    pub alias: String,
    pub activity_type: String,
    #[serde(default)]
    pub recorded_at: Option<String>,
    #[serde(default)]
    pub used: Vec<String>,
    #[serde(default)]
    pub generated_representations: Vec<ReferenceInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AssertionIntent {
    #[serde(rename = "as", default)]
    pub alias: Option<String>,
    pub claim: ReferenceInput,
    pub representation: ReferenceInput,
    #[serde(default)]
    pub recorded_at: Option<String>,
    #[serde(default)]
    pub valid_from: Option<String>,
    #[serde(default)]
    pub valid_until: Option<String>,
    #[serde(default)]
    pub context: Option<ReferenceInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorityIntent {
    #[serde(rename = "as", default)]
    pub alias: Option<String>,
    pub subject: ReferenceInput,
    #[serde(default)]
    pub concern: Option<String>,
    pub representation: ReferenceInput,
    #[serde(default)]
    pub basis: Option<String>,
    #[serde(default)]
    pub recorded_at: Option<String>,
    #[serde(default)]
    pub valid_from: Option<String>,
    #[serde(default)]
    pub valid_until: Option<String>,
    #[serde(default)]
    pub context: Option<ReferenceInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EntityReferenceIntent {
    pub kind: EntityKind,
    #[serde(rename = "ref")]
    pub reference: ReferenceInput,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RelationshipIntent {
    #[serde(rename = "as", default)]
    pub alias: Option<String>,
    pub from: EntityReferenceIntent,
    pub relation: String,
    pub to: EntityReferenceIntent,
    #[serde(default)]
    pub origin: Option<RelationshipOrigin>,
    #[serde(default)]
    pub activity: Option<ReferenceInput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceIntent {
    #[serde(rename = "as", default)]
    pub alias: Option<String>,
    pub claim: ReferenceInput,
    pub method: String,
    pub result: EvidenceResult,
    #[serde(default)]
    pub recorded_at: Option<String>,
    #[serde(default)]
    pub inputs: Vec<String>,
    #[serde(default)]
    pub context: Option<ReferenceInput>,
    #[serde(default)]
    pub notes: Option<String>,
}
