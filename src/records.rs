use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use uuid::Uuid;

use crate::error::{Error, Result};
use crate::model::{EntityKind, Record};

const RECORD_SCHEMA_JSON: &str = include_str!("../schemas/v1/record.schema.json");

pub fn load_records(root: &Path) -> Result<Vec<Record>> {
    let records_root = root.join(".pk").join("records");
    if !records_root.exists() {
        return Ok(Vec::new());
    }

    let schema: serde_json::Value = serde_json::from_str(RECORD_SCHEMA_JSON)?;
    let validator = jsonschema::validator_for(&schema).map_err(|error| Error::Schema {
        path: PathBuf::from("schemas/v1/record.schema.json"),
        message: error.to_string(),
    })?;

    let mut files = Vec::new();
    collect_json_files(&records_root, &mut files)?;
    files.sort();

    let mut records = Vec::with_capacity(files.len());
    for path in files {
        let text = fs::read_to_string(&path)?;
        let value: serde_json::Value = serde_json::from_str(&text)?;
        if let Err(error) = validator.validate(&value) {
            return Err(Error::Schema {
                path: path.clone(),
                message: error.to_string(),
            });
        }
        let record: Record = serde_json::from_value(value)?;
        record.semantic_validate()?;
        records.push(record);
    }

    validate_cross_references(&records)?;
    Ok(records)
}

pub fn write_record(root: &Path, record: &Record) -> Result<PathBuf> {
    record.semantic_validate()?;
    let dir = root.join(".pk").join("records").join(record.kind_name());
    fs::create_dir_all(&dir)?;
    let path = dir.join(format!("{}.json", record.id()));
    let mut text = serde_json::to_string_pretty(record)?;
    text.push('\n');
    fs::write(&path, text)?;
    Ok(path)
}

pub fn validate_cross_references(records: &[Record]) -> Result<()> {
    let mut ids = HashSet::new();
    let mut subjects = HashSet::new();
    let mut representations = HashSet::new();
    let mut claims = HashSet::new();
    let mut activities = HashSet::new();
    let mut contexts = HashSet::new();

    for record in records {
        if !ids.insert(record.id()) {
            return Err(Error::CrossReference(format!(
                "duplicate record id {}",
                record.id()
            )));
        }
        match record {
            Record::Subject { id, .. } => {
                subjects.insert(*id);
            }
            Record::Representation { id, .. } => {
                representations.insert(*id);
            }
            Record::Claim { id, .. } => {
                claims.insert(*id);
            }
            Record::Activity { id, .. } => {
                activities.insert(*id);
            }
            Record::Context { id, .. } => {
                contexts.insert(*id);
            }
            _ => {}
        }
    }

    for record in records {
        match record {
            Record::Representation {
                id,
                subject_id: Some(subject_id),
                ..
            } => require_id(*id, "subject", *subject_id, &subjects)?,
            Record::Claim { id, subject_id, .. } => {
                require_id(*id, "subject", *subject_id, &subjects)?
            }
            Record::Assertion {
                id,
                claim_id,
                representation_id,
                context_id,
                ..
            } => {
                require_id(*id, "claim", *claim_id, &claims)?;
                require_id(*id, "representation", *representation_id, &representations)?;
                if let Some(context_id) = context_id {
                    require_id(*id, "context", *context_id, &contexts)?;
                }
            }
            Record::Authority {
                id,
                subject_id,
                representation_id,
                context_id,
                ..
            } => {
                require_id(*id, "subject", *subject_id, &subjects)?;
                require_id(*id, "representation", *representation_id, &representations)?;
                if let Some(context_id) = context_id {
                    require_id(*id, "context", *context_id, &contexts)?;
                }
            }
            Record::Relationship {
                id,
                from,
                to,
                activity_id,
                ..
            } => {
                require_entity(*id, from.kind, from.id, records)?;
                require_entity(*id, to.kind, to.id, records)?;
                if let Some(activity_id) = activity_id {
                    require_id(*id, "activity", *activity_id, &activities)?;
                }
            }
            Record::Activity {
                id,
                generated_representation_ids,
                ..
            } => {
                for representation_id in generated_representation_ids {
                    require_id(*id, "representation", *representation_id, &representations)?;
                }
            }
            Record::EvidenceEvaluation {
                id,
                claim_id,
                context_id,
                ..
            } => {
                require_id(*id, "claim", *claim_id, &claims)?;
                if let Some(context_id) = context_id {
                    require_id(*id, "context", *context_id, &contexts)?;
                }
            }
            _ => {}
        }
    }

    Ok(())
}

fn collect_json_files(dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_json_files(&path, files)?;
        } else if path.extension().is_some_and(|ext| ext == "json") {
            files.push(path);
        }
    }
    Ok(())
}

fn require_id(owner: Uuid, label: &str, target: Uuid, set: &HashSet<Uuid>) -> Result<()> {
    if !set.contains(&target) {
        return Err(Error::CrossReference(format!(
            "record {owner} references missing {label} {target}"
        )));
    }
    Ok(())
}

fn require_entity(owner: Uuid, kind: EntityKind, target: Uuid, records: &[Record]) -> Result<()> {
    let exists = records.iter().any(|record| {
        record.id() == target
            && matches!(
                (kind, record),
                (EntityKind::Subject, Record::Subject { .. })
                    | (EntityKind::Representation, Record::Representation { .. })
                    | (EntityKind::Claim, Record::Claim { .. })
                    | (EntityKind::Assertion, Record::Assertion { .. })
                    | (EntityKind::Authority, Record::Authority { .. })
                    | (EntityKind::Relationship, Record::Relationship { .. })
                    | (EntityKind::Activity, Record::Activity { .. })
                    | (EntityKind::Context, Record::Context { .. })
                    | (
                        EntityKind::EvidenceEvaluation,
                        Record::EvidenceEvaluation { .. }
                    )
            )
    });
    if !exists {
        return Err(Error::CrossReference(format!(
            "record {owner} references missing {kind:?} {target}"
        )));
    }
    Ok(())
}
