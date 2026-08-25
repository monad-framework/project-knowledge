use std::collections::{BTreeMap, HashMap};
use std::path::{Component, Path, PathBuf};

use jiff::Timestamp;
use uuid::Uuid;

use crate::authoring::catalog::{CatalogMatch, RecordCatalog, record_kind, record_relative_path};
use crate::authoring::intent::{AUTHORING_SCHEMA, AuthoringIntent, ReferenceInput};
use crate::authoring::plan::{
    CAPTURE_PLAN_SCHEMA, CapturePlan, FieldOrigin, PlanMessage, PlanOperation, PlanPrecondition,
};
use crate::error::{Error, Result};
use crate::git::GitAdapter;
use crate::model::{EntityKind, EntityRef, NativeReference, RECORD_SCHEMA, Record, RelationshipOrigin};
use crate::records::validate_records;

pub fn parse_intent(text: &str) -> Result<AuthoringIntent> {
    let intent: AuthoringIntent = serde_json::from_str(text)?;
    if intent.schema != AUTHORING_SCHEMA {
        return Err(Error::AuthoringInput(format!(
            "unsupported authoring schema {}",
            intent.schema
        )));
    }
    Ok(intent)
}

pub fn build_capture_plan(root: &Path, intent: &AuthoringIntent) -> Result<CapturePlan> {
    if intent.schema != AUTHORING_SCHEMA {
        return Err(Error::AuthoringInput(format!(
            "unsupported authoring schema {}",
            intent.schema
        )));
    }

    let created_at = Timestamp::now().to_string();
    let mut builder = PlanBuilder::new(root, created_at.clone())?;
    builder.consume(intent)?;
    builder.finish(created_at)
}

#[derive(Clone, Copy)]
struct AliasEntry {
    kind: EntityKind,
    id: Uuid,
}

struct PlanBuilder<'a> {
    root: &'a Path,
    capture_time: String,
    catalog: RecordCatalog,
    aliases: HashMap<String, AliasEntry>,
    records: HashMap<Uuid, Record>,
    operations: Vec<PlanOperation>,
    preconditions: Vec<PlanPrecondition>,
    existing_states: BTreeMap<PathBuf, String>,
    native_states: BTreeMap<PathBuf, String>,
    warnings: Vec<PlanMessage>,
    blockers: Vec<PlanMessage>,
}

impl<'a> PlanBuilder<'a> {
    fn new(root: &'a Path, capture_time: String) -> Result<Self> {
        let catalog = RecordCatalog::load(root)?;
        let records = catalog
            .records()
            .iter()
            .map(|record| (record.id(), record.clone()))
            .collect();
        Ok(Self {
            root,
            capture_time,
            catalog,
            aliases: HashMap::new(),
            records,
            operations: Vec::new(),
            preconditions: Vec::new(),
            existing_states: BTreeMap::new(),
            native_states: BTreeMap::new(),
            warnings: Vec::new(),
            blockers: Vec::new(),
        })
    }

    fn consume(&mut self, intent: &AuthoringIntent) -> Result<()> {
        if let Some(subject) = &intent.subject {
            match (&subject.new, &subject.existing) {
                (Some(new), None) => {
                    let id = Uuid::new_v4();
                    let record = Record::Subject {
                        schema: RECORD_SCHEMA.to_string(),
                        id,
                        label: new.label.clone(),
                    };
                    let mut origins = generated_origins();
                    origins.insert("/label".to_string(), FieldOrigin::Authored);
                    self.add_new(&subject.alias, EntityKind::Subject, record, origins)?;
                }
                (None, Some(selector)) => {
                    let selected = self.catalog.resolve(selector)?;
                    if record_kind(&selected.record) != EntityKind::Subject {
                        return Err(Error::AuthoringInput(
                            "subject selector did not resolve a Subject".to_string(),
                        ));
                    }
                    let id = selected.record.id();
                    self.track_existing(&selected)?;
                    self.add_alias(&subject.alias, EntityKind::Subject, id)?;
                }
                _ => {
                    return Err(Error::AuthoringInput(
                        "subject must specify exactly one of new or existing".to_string(),
                    ));
                }
            }
        }

        for item in &intent.representations {
            let subject_id = self.resolve(&item.subject, EntityKind::Subject)?;
            let native = self.observe_path(&item.path)?;
            let id = Uuid::new_v4();
            let record = Record::Representation {
                schema: RECORD_SCHEMA.to_string(),
                id,
                subject_id: Some(subject_id),
                role: item.role.clone(),
                native,
            };
            let mut origins = generated_origins();
            authored(&mut origins, &["/subject_id", "/role", "/native/locator"]);
            generated(&mut origins, &["/native/source_system", "/native/object_type"]);
            observed(&mut origins, &["/native/state"]);
            self.add_new(&item.alias, EntityKind::Representation, record, origins)?;
        }

        for item in &intent.claims {
            let subject_id = self.resolve(&item.subject, EntityKind::Subject)?;
            let id = Uuid::new_v4();
            let record = Record::Claim {
                schema: RECORD_SCHEMA.to_string(),
                id,
                subject_id,
                concern: item.concern.clone(),
                value: item.value.clone(),
            };
            let mut origins = generated_origins();
            authored(&mut origins, &["/subject_id", "/concern", "/value"]);
            self.add_new(&item.alias, EntityKind::Claim, record, origins)?;
        }

        for item in &intent.contexts {
            let id = Uuid::new_v4();
            let source_state = item
                .source_path
                .as_deref()
                .map(|path| self.observe_path(path))
                .transpose()?;
            let record = Record::Context {
                schema: RECORD_SCHEMA.to_string(),
                id,
                dimensions: item.dimensions.clone(),
                source_state,
            };
            let mut origins = generated_origins();
            origins.insert("/dimensions".to_string(), FieldOrigin::Authored);
            if item.source_path.is_some() {
                authored(&mut origins, &["/source_state/locator"]);
                generated(
                    &mut origins,
                    &["/source_state/source_system", "/source_state/object_type"],
                );
                observed(&mut origins, &["/source_state/state"]);
            }
            self.add_new(&item.alias, EntityKind::Context, record, origins)?;
        }

        for item in &intent.activities {
            let mut used = Vec::new();
            for path in &item.used {
                used.push(self.observe_path(path)?);
            }
            let mut generated_representation_ids = Vec::new();
            for reference in &item.generated_representations {
                generated_representation_ids.push(self.resolve(reference, EntityKind::Representation)?);
            }
            let id = Uuid::new_v4();
            let recorded_at = item
                .recorded_at
                .clone()
                .unwrap_or_else(|| self.capture_time.clone());
            let record = Record::Activity {
                schema: RECORD_SCHEMA.to_string(),
                id,
                activity_type: item.activity_type.clone(),
                recorded_at,
                used,
                generated_representation_ids,
            };
            let mut origins = generated_origins();
            authored(
                &mut origins,
                &["/activity_type", "/used", "/generated_representation_ids"],
            );
            origins.insert(
                "/recorded_at".to_string(),
                if item.recorded_at.is_some() {
                    FieldOrigin::Authored
                } else {
                    FieldOrigin::Generated
                },
            );
            if !item.used.is_empty() {
                observed(&mut origins, &["/used/*/state"]);
            }
            self.add_new(&item.alias, EntityKind::Activity, record, origins)?;
        }

        for item in &intent.assertions {
            let claim_id = self.resolve(&item.claim, EntityKind::Claim)?;
            let representation_id = self.resolve(&item.representation, EntityKind::Representation)?;
            let context_id = item
                .context
                .as_ref()
                .map(|reference| self.resolve(reference, EntityKind::Context))
                .transpose()?;
            let source_state = self
                .records
                .get(&representation_id)
                .and_then(|record| match record {
                    Record::Representation { native, .. } => Some(native.clone()),
                    _ => None,
                });
            let id = Uuid::new_v4();
            let record = Record::Assertion {
                schema: RECORD_SCHEMA.to_string(),
                id,
                claim_id,
                representation_id,
                recorded_at: item
                    .recorded_at
                    .clone()
                    .unwrap_or_else(|| self.capture_time.clone()),
                valid_from: item.valid_from.clone(),
                valid_until: item.valid_until.clone(),
                source_state,
                context_id,
            };
            let mut origins = generated_origins();
            authored(&mut origins, &["/claim_id", "/representation_id"]);
            origins.insert(
                "/recorded_at".to_string(),
                if item.recorded_at.is_some() {
                    FieldOrigin::Authored
                } else {
                    FieldOrigin::Generated
                },
            );
            if item.valid_from.is_some() {
                origins.insert("/valid_from".to_string(), FieldOrigin::Authored);
            }
            if item.valid_until.is_some() {
                origins.insert("/valid_until".to_string(), FieldOrigin::Authored);
            }
            if item.context.is_some() {
                origins.insert("/context_id".to_string(), FieldOrigin::Authored);
            }
            if matches!(record, Record::Assertion { source_state: Some(_), .. }) {
                observed(&mut origins, &["/source_state"]);
            }
            self.add_new_optional(item.alias.as_deref(), EntityKind::Assertion, record, origins)?;
        }

        for item in &intent.authorities {
            let Some(concern) = item.concern.as_ref().filter(|value| !value.trim().is_empty()) else {
                self.blockers.push(PlanMessage::new(
                    "authority_concern_required",
                    "Authority requires an explicit concern/scope",
                ));
                continue;
            };
            let Some(basis) = item.basis.as_ref().filter(|value| !value.trim().is_empty()) else {
                self.blockers.push(PlanMessage::new(
                    "authority_basis_required",
                    "Authority requires an explicit basis",
                ));
                continue;
            };
            let subject_id = self.resolve(&item.subject, EntityKind::Subject)?;
            let representation_id = self.resolve(&item.representation, EntityKind::Representation)?;
            let context_id = item
                .context
                .as_ref()
                .map(|reference| self.resolve(reference, EntityKind::Context))
                .transpose()?;
            let id = Uuid::new_v4();
            let record = Record::Authority {
                schema: RECORD_SCHEMA.to_string(),
                id,
                subject_id,
                concern: concern.clone(),
                representation_id,
                basis: basis.clone(),
                recorded_at: item
                    .recorded_at
                    .clone()
                    .unwrap_or_else(|| self.capture_time.clone()),
                valid_from: item.valid_from.clone(),
                valid_until: item.valid_until.clone(),
                context_id,
            };
            let mut origins = generated_origins();
            authored(
                &mut origins,
                &["/subject_id", "/concern", "/representation_id", "/basis"],
            );
            origins.insert(
                "/recorded_at".to_string(),
                if item.recorded_at.is_some() {
                    FieldOrigin::Authored
                } else {
                    FieldOrigin::Generated
                },
            );
            if item.valid_from.is_some() {
                origins.insert("/valid_from".to_string(), FieldOrigin::Authored);
            }
            if item.valid_until.is_some() {
                origins.insert("/valid_until".to_string(), FieldOrigin::Authored);
            }
            if item.context.is_some() {
                origins.insert("/context_id".to_string(), FieldOrigin::Authored);
            }
            self.add_new_optional(item.alias.as_deref(), EntityKind::Authority, record, origins)?;
        }

        for item in &intent.relationships {
            let from = EntityRef {
                kind: item.from.kind,
                id: self.resolve(&item.from.reference, item.from.kind)?,
            };
            let to = EntityRef {
                kind: item.to.kind,
                id: self.resolve(&item.to.reference, item.to.kind)?,
            };
            let activity_id = item
                .activity
                .as_ref()
                .map(|reference| self.resolve(reference, EntityKind::Activity))
                .transpose()?;
            let id = Uuid::new_v4();
            let record = Record::Relationship {
                schema: RECORD_SCHEMA.to_string(),
                id,
                from,
                relation: item.relation.clone(),
                to,
                origin: item.origin.unwrap_or(RelationshipOrigin::Authored),
                activity_id,
            };
            let mut origins = generated_origins();
            authored(&mut origins, &["/from", "/relation", "/to"]);
            origins.insert(
                "/origin".to_string(),
                if item.origin.is_some() {
                    FieldOrigin::Authored
                } else {
                    FieldOrigin::Generated
                },
            );
            if item.activity.is_some() {
                origins.insert("/activity_id".to_string(), FieldOrigin::Authored);
            }
            self.add_new_optional(item.alias.as_deref(), EntityKind::Relationship, record, origins)?;
        }

        for item in &intent.evidence_evaluations {
            let claim_id = self.resolve(&item.claim, EntityKind::Claim)?;
            let context_id = item
                .context
                .as_ref()
                .map(|reference| self.resolve(reference, EntityKind::Context))
                .transpose()?;
            let mut inputs = Vec::new();
            for path in &item.inputs {
                inputs.push(self.observe_path(path)?);
            }
            let id = Uuid::new_v4();
            let record = Record::EvidenceEvaluation {
                schema: RECORD_SCHEMA.to_string(),
                id,
                claim_id,
                method: item.method.clone(),
                result: item.result,
                recorded_at: item
                    .recorded_at
                    .clone()
                    .unwrap_or_else(|| self.capture_time.clone()),
                inputs,
                context_id,
                notes: item.notes.clone(),
            };
            let mut origins = generated_origins();
            authored(&mut origins, &["/claim_id", "/method", "/result", "/inputs"]);
            origins.insert(
                "/recorded_at".to_string(),
                if item.recorded_at.is_some() {
                    FieldOrigin::Authored
                } else {
                    FieldOrigin::Generated
                },
            );
            if item.context.is_some() {
                origins.insert("/context_id".to_string(), FieldOrigin::Authored);
            }
            if item.notes.is_some() {
                origins.insert("/notes".to_string(), FieldOrigin::Authored);
            }
            if !item.inputs.is_empty() {
                observed(&mut origins, &["/inputs/*/state"]);
            }
            self.add_new_optional(
                item.alias.as_deref(),
                EntityKind::EvidenceEvaluation,
                record,
                origins,
            )?;
        }

        Ok(())
    }

    fn finish(mut self, created_at: String) -> Result<CapturePlan> {
        for operation in &self.operations {
            self.preconditions.push(PlanPrecondition::OutputPath {
                path: operation.path.clone(),
            });
        }
        for (path, state) in self.existing_states {
            self.preconditions.push(PlanPrecondition::ExistingRecordState {
                path: relative_string(&path),
                state,
            });
        }
        for (path, state) in self.native_states {
            self.preconditions.push(PlanPrecondition::NativeBlobState {
                path: relative_string(&path),
                state,
            });
        }

        if self.blockers.is_empty() {
            let mut prospective = self.catalog.records().to_vec();
            prospective.extend(self.operations.iter().map(|operation| operation.record.clone()));
            if let Err(error) = validate_records(&prospective) {
                self.blockers.push(PlanMessage::new(
                    "prospective_validation_failed",
                    error.to_string(),
                ));
            }
        }

        Ok(CapturePlan {
            schema: CAPTURE_PLAN_SCHEMA.to_string(),
            plan_id: Uuid::new_v4(),
            created_at,
            operations: self.operations,
            preconditions: self.preconditions,
            warnings: self.warnings,
            blockers: self.blockers,
        })
    }

    fn add_new(
        &mut self,
        alias: &str,
        kind: EntityKind,
        record: Record,
        origins: BTreeMap<String, FieldOrigin>,
    ) -> Result<()> {
        let id = record.id();
        self.add_alias(alias, kind, id)?;
        self.add_operation(record, origins);
        Ok(())
    }

    fn add_new_optional(
        &mut self,
        alias: Option<&str>,
        kind: EntityKind,
        record: Record,
        origins: BTreeMap<String, FieldOrigin>,
    ) -> Result<()> {
        if let Some(alias) = alias {
            self.add_alias(alias, kind, record.id())?;
        }
        self.add_operation(record, origins);
        Ok(())
    }

    fn add_operation(&mut self, record: Record, origins: BTreeMap<String, FieldOrigin>) {
        let path = record_relative_path(&record);
        self.records.insert(record.id(), record.clone());
        self.operations.push(PlanOperation::create_record(
            relative_string(&path),
            record,
            origins,
        ));
    }

    fn add_alias(&mut self, alias: &str, kind: EntityKind, id: Uuid) -> Result<()> {
        if alias.trim().is_empty() {
            return Err(Error::AuthoringInput("local alias must not be empty".to_string()));
        }
        if self.aliases.contains_key(alias) {
            return Err(Error::AuthoringInput(format!(
                "duplicate local alias {alias}"
            )));
        }
        self.aliases.insert(alias.to_string(), AliasEntry { kind, id });
        Ok(())
    }

    fn resolve(&mut self, reference: &ReferenceInput, expected: EntityKind) -> Result<Uuid> {
        match reference {
            ReferenceInput::Alias(alias) => {
                if let Ok(id) = Uuid::parse_str(alias) {
                    let record = self.records.get(&id).ok_or_else(|| {
                        Error::AuthoringInput(format!("record {id} does not exist"))
                    })?;
                    if record_kind(record) != expected {
                        return Err(Error::AuthoringInput(format!(
                            "record {id} is {:?}, expected {expected:?}",
                            record_kind(record)
                        )));
                    }
                    return Ok(id);
                }
                let entry = self.aliases.get(alias).copied().ok_or_else(|| {
                    Error::AuthoringInput(format!("unknown local alias {alias}"))
                })?;
                if entry.kind != expected {
                    return Err(Error::AuthoringInput(format!(
                        "alias {alias} is {:?}, expected {expected:?}",
                        entry.kind
                    )));
                }
                Ok(entry.id)
            }
            ReferenceInput::Existing { existing } => {
                let selected = self.catalog.resolve(existing)?;
                let kind = record_kind(&selected.record);
                if kind != expected {
                    return Err(Error::AuthoringInput(format!(
                        "existing selector resolved {kind:?}, expected {expected:?}"
                    )));
                }
                let id = selected.record.id();
                self.track_existing(&selected)?;
                Ok(id)
            }
        }
    }

    fn track_existing(&mut self, selected: &CatalogMatch) -> Result<()> {
        let path = selected.path.clone();
        if self.existing_states.contains_key(&path) {
            return Ok(());
        }
        let state = GitAdapter.hash_path(self.root, &path)?;
        self.existing_states.insert(path, state);
        Ok(())
    }

    fn observe_path(&mut self, value: &str) -> Result<NativeReference> {
        let path = safe_relative_path(value)?;
        let native = GitAdapter.native_blob_for_path(self.root, &path)?;
        if let Some(state) = native.state.clone() {
            self.native_states.entry(path).or_insert(state);
        }
        Ok(native)
    }
}

fn generated_origins() -> BTreeMap<String, FieldOrigin> {
    BTreeMap::from([
        ("/schema".to_string(), FieldOrigin::Generated),
        ("/kind".to_string(), FieldOrigin::Generated),
        ("/id".to_string(), FieldOrigin::Generated),
    ])
}

fn authored(origins: &mut BTreeMap<String, FieldOrigin>, paths: &[&str]) {
    for path in paths {
        origins.insert((*path).to_string(), FieldOrigin::Authored);
    }
}

fn generated(origins: &mut BTreeMap<String, FieldOrigin>, paths: &[&str]) {
    for path in paths {
        origins.insert((*path).to_string(), FieldOrigin::Generated);
    }
}

fn observed(origins: &mut BTreeMap<String, FieldOrigin>, paths: &[&str]) {
    for path in paths {
        origins.insert((*path).to_string(), FieldOrigin::Observed);
    }
}

fn safe_relative_path(value: &str) -> Result<PathBuf> {
    let path = PathBuf::from(value);
    if path.as_os_str().is_empty() || path.is_absolute() {
        return Err(Error::AuthoringInput(format!(
            "native path must be repository-relative: {value}"
        )));
    }
    if path
        .components()
        .any(|component| matches!(component, Component::ParentDir | Component::RootDir | Component::Prefix(_)))
    {
        return Err(Error::AuthoringInput(format!(
            "native path escapes repository root: {value}"
        )));
    }
    Ok(path)
}

fn relative_string(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
