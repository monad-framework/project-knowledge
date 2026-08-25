use std::collections::HashSet;
use std::fs;
use std::path::{Component, Path, PathBuf};

use crate::authoring::catalog::record_relative_path;
use crate::authoring::plan::{ApplyResult, CapturePlan, PlanPrecondition};
use crate::error::{Error, Result};
use crate::git::GitAdapter;
use crate::records::{load_records, record_text, validate_records, write_record};

pub fn apply_capture_plan(root: &Path, plan: &CapturePlan) -> Result<ApplyResult> {
    plan.validate_envelope()?;
    if plan.has_blockers() {
        return Err(Error::BlockedPlan(
            plan.blockers
                .iter()
                .map(|item| item.message.as_str())
                .collect::<Vec<_>>()
                .join("; "),
        ));
    }
    if plan.contains_unconfirmed_suggestion() {
        return Err(Error::BlockedPlan(
            "plan contains unconfirmed suggested semantic fields".to_string(),
        ));
    }

    check_preconditions(root, plan)?;

    let current = load_records(root)?;
    let mut current_ids: HashSet<_> = current.iter().map(|record| record.id()).collect();
    let mut prospective = current.clone();
    let mut created = Vec::new();
    let mut no_op = Vec::new();

    for operation in &plan.operations {
        let relative = safe_plan_path(&operation.path)?;
        let canonical = record_relative_path(&operation.record);
        if relative != canonical {
            return Err(Error::Conflict(format!(
                "planned path {} is not canonical for record {} (expected {})",
                operation.path,
                operation.record.id(),
                canonical.display()
            )));
        }
        let target = root.join(&relative);
        let expected = record_text(&operation.record)?;
        if target.exists() {
            let actual = fs::read_to_string(&target)?;
            if actual == expected {
                no_op.push(operation.path.clone());
                continue;
            }
            return Err(Error::Conflict(format!(
                "planned output already exists with divergent content: {}",
                operation.path
            )));
        }
        if !current_ids.insert(operation.record.id()) {
            return Err(Error::Conflict(format!(
                "record id {} already exists at another path",
                operation.record.id()
            )));
        }
        prospective.push(operation.record.clone());
        created.push(operation.path.clone());
    }

    validate_records(&prospective)?;

    for operation in &plan.operations {
        if no_op.contains(&operation.path) {
            continue;
        }
        let written = write_record(root, &operation.record)?;
        let expected = root.join(safe_plan_path(&operation.path)?);
        if written != expected {
            return Err(Error::Conflict(format!(
                "record serializer wrote {} instead of planned {}",
                written.display(),
                expected.display()
            )));
        }
    }

    let final_records = load_records(root)?;
    validate_records(&final_records)?;

    Ok(ApplyResult {
        plan_id: plan.plan_id,
        created,
        no_op,
        valid: true,
    })
}

fn check_preconditions(root: &Path, plan: &CapturePlan) -> Result<()> {
    for precondition in &plan.preconditions {
        match precondition {
            PlanPrecondition::OutputPath { .. } => {
                // Output paths are checked against exact planned bytes below so completed
                // plans can be re-applied idempotently.
            }
            PlanPrecondition::ExistingRecordState { path, state }
            | PlanPrecondition::NativeBlobState { path, state } => {
                let relative = safe_plan_path(path)?;
                let current = GitAdapter.hash_path(root, &relative).map_err(|error| {
                    Error::StalePlan(format!("relevant path {path} is unavailable: {error}"))
                })?;
                if &current != state {
                    return Err(Error::StalePlan(format!(
                        "relevant path {path} changed: planned {state}, current {current}"
                    )));
                }
            }
        }
    }
    Ok(())
}

fn safe_plan_path(value: &str) -> Result<PathBuf> {
    let path = PathBuf::from(value);
    if path.as_os_str().is_empty() || path.is_absolute() {
        return Err(Error::Conflict(format!("unsafe plan path {value}")));
    }
    if path.components().any(|component| {
        matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )
    }) {
        return Err(Error::Conflict(format!(
            "plan path escapes repository root: {value}"
        )));
    }
    Ok(path)
}
