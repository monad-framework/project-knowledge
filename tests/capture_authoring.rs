use std::fs;
use std::path::Path;
use std::process::Command;

use project_knowledge::authoring::plan::FieldOrigin;
use project_knowledge::model::{EvidenceResult, RECORD_SCHEMA, Record};
use project_knowledge::records::write_record;
use project_knowledge::{
    Error, EvidenceState, ResolutionOutcome, apply_capture_plan, build_capture_plan,
    compile_in_memory, load_records, parse_intent, save_plan,
};
use serde_json::json;
use tempfile::TempDir;
use uuid::Uuid;

const T0: &str = "2026-08-25T15:00:00Z";
const T1: &str = "2026-08-25T16:00:00Z";
const T2: &str = "2026-08-25T17:00:00Z";

#[test]
fn ca_a01_and_a18_df001_equivalence_without_structural_boilerplate() {
    let repo = git_repo();
    write_native(repo.path(), "adr.md", "status: proposed\n");
    write_native(repo.path(), "selected.md", "architecture accepted\n");
    write_native(
        repo.path(),
        "closure.md",
        "M0 did not falsify architecture\n",
    );
    commit_all(repo.path(), "native architecture state");

    let intent = parse_intent(
        &json!({
            "schema": "pk-authoring/v1",
            "subject": {"as": "architecture", "new": {"label": "ADR architecture"}},
            "representations": [
                {"as": "adr", "subject": "architecture", "path": "adr.md", "role": "decision_record"},
                {"as": "selected", "subject": "architecture", "path": "selected.md", "role": "current_architecture_definition"}
            ],
            "claims": [
                {"as": "proposed", "subject": "architecture", "concern": "decision_status", "value": "proposed"},
                {"as": "accepted", "subject": "architecture", "concern": "decision_status", "value": "accepted"}
            ],
            "assertions": [
                {"claim": "proposed", "representation": "adr", "valid_until": T1},
                {"claim": "accepted", "representation": "selected", "valid_from": T1}
            ],
            "authorities": [
                {"subject": "architecture", "concern": "decision_status", "representation": "adr", "basis": "proposal record", "valid_until": T1},
                {"subject": "architecture", "concern": "decision_status", "representation": "selected", "basis": "adoption decision", "valid_from": T1}
            ],
            "evidence_evaluations": [
                {"claim": "accepted", "method": "M0 falsification slice", "result": "pass", "inputs": ["closure.md"]}
            ]
        })
        .to_string(),
    )
    .unwrap();

    let plan = build_capture_plan(repo.path(), &intent).unwrap();
    assert!(!plan.has_blockers());
    assert_eq!(plan.operations.len(), 10);
    assert!(plan.operations.iter().all(|operation| {
        operation.field_origins.get("/id") == Some(&FieldOrigin::Generated)
            && operation.field_origins.get("/schema") == Some(&FieldOrigin::Generated)
    }));

    let subject = find_subject(&plan, "ADR architecture");
    let accepted = find_claim(&plan, "decision_status", json!("accepted"));
    let evidence = find_evidence_for_claim(&plan, accepted);
    let before_native = fs::read_to_string(repo.path().join("selected.md")).unwrap();

    apply_capture_plan(repo.path(), &plan).unwrap();
    let (model, _) = compile_in_memory(repo.path()).unwrap();
    let before = model
        .resolve_current(subject, "decision_status", Some(T0), None)
        .unwrap();
    let after = model
        .resolve_current(subject, "decision_status", Some(T2), None)
        .unwrap();
    assert_eq!(before.outcome, ResolutionOutcome::Resolved);
    assert_eq!(
        claim_value(repo.path(), before.claim_ids[0]),
        json!("proposed")
    );
    assert_eq!(after.outcome, ResolutionOutcome::Resolved);
    assert_eq!(after.claim_ids, vec![accepted]);
    assert_eq!(
        model.evidence_state(evidence).unwrap(),
        EvidenceState::Current
    );
    assert_eq!(
        fs::read_to_string(repo.path().join("selected.md")).unwrap(),
        before_native
    );
}

#[test]
fn ca_a02_df002_equivalence_preserves_unknown_alternatives_provenance_and_evidence_scope() {
    let repo = git_repo();
    write_native(repo.path(), "open.md", "Which serialization?\n");
    write_native(
        repo.path(),
        "decision.md",
        "Select JSON over YAML, TOML, custom.\n",
    );
    write_native(repo.path(), "schema.json", "{}\n");
    commit_all(repo.path(), "serialization artifacts");

    let intent = parse_intent(
        &json!({
            "schema": "pk-authoring/v1",
            "subject": {"as": "serialization", "new": {"label": "portable serialization choice"}},
            "representations": [
                {"as": "open", "subject": "serialization", "path": "open.md", "role": "open_question"},
                {"as": "decision", "subject": "serialization", "path": "decision.md", "role": "technology_decision"}
            ],
            "claims": [
                {"as": "json", "subject": "serialization", "concern": "serialization.format", "value": "json"},
                {"as": "yaml", "subject": "serialization", "concern": "serialization.format", "value": "yaml"},
                {"as": "toml", "subject": "serialization", "concern": "serialization.format", "value": "toml"},
                {"as": "custom", "subject": "serialization", "concern": "serialization.format", "value": "custom"}
            ],
            "activities": [
                {"as": "selection", "activity_type": "technology_selection", "used": ["open.md"], "generated_representations": ["decision"]}
            ],
            "assertions": [
                {"claim": "json", "representation": "decision", "valid_from": T1}
            ],
            "authorities": [
                {"subject": "serialization", "concern": "serialization.format", "representation": "decision", "basis": "M0 technology decision", "valid_from": T1}
            ],
            "relationships": [
                {"from": {"kind": "claim", "ref": "yaml"}, "relation": "alternative_considered_in", "to": {"kind": "representation", "ref": "decision"}},
                {"from": {"kind": "claim", "ref": "toml"}, "relation": "alternative_considered_in", "to": {"kind": "representation", "ref": "decision"}},
                {"from": {"kind": "claim", "ref": "custom"}, "relation": "alternative_considered_in", "to": {"kind": "representation", "ref": "decision"}}
            ],
            "evidence_evaluations": [
                {"claim": "json", "method": "implementation inspection", "result": "pass", "inputs": ["decision.md", "schema.json"]}
            ]
        })
        .to_string(),
    )
    .unwrap();

    let plan = build_capture_plan(repo.path(), &intent).unwrap();
    assert_eq!(plan.operations.len(), 14);
    let subject = find_subject(&plan, "portable serialization choice");
    let json_claim = find_claim(&plan, "serialization.format", json!("json"));
    let evidence = find_evidence_for_claim(&plan, json_claim);
    apply_capture_plan(repo.path(), &plan).unwrap();

    let (model, _) = compile_in_memory(repo.path()).unwrap();
    let before = model
        .resolve_current(subject, "serialization.format", Some(T0), None)
        .unwrap();
    let after = model
        .resolve_current(subject, "serialization.format", Some(T2), None)
        .unwrap();
    assert_eq!(before.outcome, ResolutionOutcome::Unknown);
    assert_eq!(after.outcome, ResolutionOutcome::Resolved);
    assert_eq!(after.claim_ids, vec![json_claim]);
    assert_eq!(
        model.evidence_state(evidence).unwrap(),
        EvidenceState::Current
    );

    let records = load_records(repo.path()).unwrap();
    assert_eq!(
        records
            .iter()
            .filter(|record| matches!(record, Record::Relationship { relation, .. } if relation == "alternative_considered_in"))
            .count(),
        3
    );
    assert_eq!(
        records
            .iter()
            .filter(|record| matches!(record, Record::EvidenceEvaluation { .. }))
            .count(),
        1
    );
}

#[test]
fn ca_a03_authority_requires_explicit_scope_and_basis() {
    let repo = git_repo();
    write_native(repo.path(), "source.md", "state\n");
    commit_all(repo.path(), "source");
    let intent = parse_intent(
        &json!({
            "schema": "pk-authoring/v1",
            "subject": {"as": "s", "new": {"label": "subject"}},
            "representations": [{"as": "r", "subject": "s", "path": "source.md", "role": "source"}],
            "authorities": [{"subject": "s", "representation": "r"}]
        })
        .to_string(),
    )
    .unwrap();
    let plan = build_capture_plan(repo.path(), &intent).unwrap();
    assert!(plan.has_blockers());
    assert!(matches!(
        apply_capture_plan(repo.path(), &plan),
        Err(Error::BlockedPlan(_))
    ));
}

#[test]
fn ca_a04_ambiguous_existing_identity_fails_closed() {
    let repo = git_repo();
    for _ in 0..2 {
        write_record(
            repo.path(),
            &Record::Subject {
                schema: RECORD_SCHEMA.into(),
                id: Uuid::new_v4(),
                label: Some("duplicate".into()),
            },
        )
        .unwrap();
    }
    let intent = parse_intent(
        &json!({
            "schema": "pk-authoring/v1",
            "subject": {"as": "s", "existing": {"kind": "subject", "label": "duplicate"}}
        })
        .to_string(),
    )
    .unwrap();
    assert!(matches!(
        build_capture_plan(repo.path(), &intent),
        Err(Error::AuthoringInput(message)) if message.contains("ambiguous")
    ));
}

#[test]
fn ca_a05_relevant_source_change_stales_plan() {
    let repo = git_repo();
    write_native(repo.path(), "source.md", "v1\n");
    commit_all(repo.path(), "v1");
    let plan = simple_representation_plan(repo.path(), "source.md");
    write_native(repo.path(), "source.md", "v2\n");
    assert!(matches!(
        apply_capture_plan(repo.path(), &plan),
        Err(Error::StalePlan(_))
    ));
}

#[test]
fn ca_a06_unrelated_change_does_not_stale_plan() {
    let repo = git_repo();
    write_native(repo.path(), "source.md", "stable\n");
    write_native(repo.path(), "other.md", "v1\n");
    commit_all(repo.path(), "sources");
    let plan = simple_representation_plan(repo.path(), "source.md");
    write_native(repo.path(), "other.md", "v2\n");
    let result = apply_capture_plan(repo.path(), &plan).unwrap();
    assert!(result.valid);
}

#[test]
fn ca_a07_field_origins_are_inspectable_and_suggestions_fail_closed() {
    let repo = git_repo();
    write_native(repo.path(), "source.md", "stable\n");
    commit_all(repo.path(), "source");
    let mut plan = simple_representation_plan(repo.path(), "source.md");
    let representation = plan
        .operations
        .iter()
        .find(|operation| matches!(operation.record, Record::Representation { .. }))
        .unwrap();
    assert_eq!(
        representation.field_origins.get("/role"),
        Some(&FieldOrigin::Authored)
    );
    assert_eq!(
        representation.field_origins.get("/native/state"),
        Some(&FieldOrigin::Observed)
    );
    plan.operations[0]
        .field_origins
        .insert("/label".into(), FieldOrigin::Suggested);
    assert!(matches!(
        apply_capture_plan(repo.path(), &plan),
        Err(Error::BlockedPlan(_))
    ));
}

#[test]
fn ca_a08_prospective_validation_fails_before_persistent_writes() {
    let repo = git_repo();
    let intent = parse_intent(
        &json!({
            "schema": "pk-authoring/v1",
            "subject": {"as": "s", "new": {"label": "subject"}},
            "claims": [{"as": "c", "subject": "s", "concern": "x", "value": 1}]
        })
        .to_string(),
    )
    .unwrap();
    let mut plan = build_capture_plan(repo.path(), &intent).unwrap();
    let claim = plan
        .operations
        .iter_mut()
        .find(|operation| matches!(operation.record, Record::Claim { .. }))
        .unwrap();
    if let Record::Claim { subject_id, .. } = &mut claim.record {
        *subject_id = Uuid::new_v4();
    }
    assert!(apply_capture_plan(repo.path(), &plan).is_err());
    assert!(!repo.path().join(".pk/records").exists());
}

#[test]
fn ca_a09_same_plan_reapply_is_idempotent() {
    let repo = git_repo();
    let plan = subject_only_plan(repo.path(), "idempotent");
    let first = apply_capture_plan(repo.path(), &plan).unwrap();
    let second = apply_capture_plan(repo.path(), &plan).unwrap();
    assert_eq!(first.created.len(), 1);
    assert_eq!(second.no_op.len(), 1);
}

#[test]
fn ca_a10_divergent_existing_output_is_not_overwritten() {
    let repo = git_repo();
    let plan = subject_only_plan(repo.path(), "planned");
    let operation = &plan.operations[0];
    let path = repo.path().join(&operation.path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    let divergent = match &operation.record {
        Record::Subject { id, .. } => Record::Subject {
            schema: RECORD_SCHEMA.into(),
            id: *id,
            label: Some("divergent".into()),
        },
        _ => unreachable!(),
    };
    write_record(repo.path(), &divergent).unwrap();
    let before = fs::read_to_string(&path).unwrap();
    assert!(matches!(
        apply_capture_plan(repo.path(), &plan),
        Err(Error::Conflict(_))
    ));
    assert_eq!(fs::read_to_string(path).unwrap(), before);
}

#[test]
fn ca_a12_recorded_time_never_implies_valid_time() {
    let repo = git_repo();
    write_native(repo.path(), "source.md", "source\n");
    commit_all(repo.path(), "source");
    let intent = parse_intent(
        &json!({
            "schema": "pk-authoring/v1",
            "subject": {"as": "s", "new": {"label": "subject"}},
            "representations": [{"as": "r", "subject": "s", "path": "source.md", "role": "source"}],
            "claims": [{"as": "c", "subject": "s", "concern": "state", "value": "current"}],
            "assertions": [{"claim": "c", "representation": "r"}]
        })
        .to_string(),
    )
    .unwrap();
    let plan = build_capture_plan(repo.path(), &intent).unwrap();
    let assertion = plan
        .operations
        .iter()
        .find_map(|operation| match &operation.record {
            Record::Assertion {
                recorded_at,
                valid_from,
                valid_until,
                ..
            } => Some((recorded_at, valid_from, valid_until)),
            _ => None,
        })
        .unwrap();
    assert!(!assertion.0.is_empty());
    assert!(assertion.1.is_none());
    assert!(assertion.2.is_none());
}

#[test]
fn ca_a13_evidence_breadth_remains_exactly_claim_scoped() {
    let repo = git_repo();
    write_native(repo.path(), "evidence.md", "evidence\n");
    commit_all(repo.path(), "evidence");
    let intent = parse_intent(
        &json!({
            "schema": "pk-authoring/v1",
            "subject": {"as": "s", "new": {"label": "subject"}},
            "claims": [
                {"as": "c1", "subject": "s", "concern": "one", "value": true},
                {"as": "c2", "subject": "s", "concern": "two", "value": true}
            ],
            "evidence_evaluations": [
                {"claim": "c1", "method": "inspection", "result": "pass", "inputs": ["evidence.md"]}
            ]
        })
        .to_string(),
    )
    .unwrap();
    let plan = build_capture_plan(repo.path(), &intent).unwrap();
    let c1 = find_claim(&plan, "one", json!(true));
    let c2 = find_claim(&plan, "two", json!(true));
    let evidence_targets: Vec<_> = plan
        .operations
        .iter()
        .filter_map(|operation| match &operation.record {
            Record::EvidenceEvaluation { claim_id, .. } => Some(*claim_id),
            _ => None,
        })
        .collect();
    assert_eq!(evidence_targets, vec![c1]);
    assert!(!evidence_targets.contains(&c2));
}

#[test]
fn ca_a14_minimal_project_remains_zero_ceremony() {
    let repo = git_repo();
    write_native(repo.path(), "README.md", "# minimal\n");
    commit_all(repo.path(), "minimal");
    let (_, report) = compile_in_memory(repo.path()).unwrap();
    assert_eq!(report.record_count, 0);
    assert!(!repo.path().join(".pk").exists());
}

#[test]
fn ca_a15_saved_plan_is_noncanonical_and_deletable() {
    let repo = git_repo();
    let plan = subject_only_plan(repo.path(), "portable");
    let plan_path = repo.path().join("capture-plan.json");
    save_plan(&plan_path, &plan).unwrap();
    apply_capture_plan(repo.path(), &plan).unwrap();
    let before = load_records(repo.path()).unwrap();
    fs::remove_file(plan_path).unwrap();
    let after = load_records(repo.path()).unwrap();
    assert_eq!(before, after);
}

#[test]
fn ca_a16_capture_bundles_compose_without_fixed_global_size() {
    let repo = git_repo();
    let first = subject_only_plan(repo.path(), "first");
    apply_capture_plan(repo.path(), &first).unwrap();
    let second = subject_only_plan(repo.path(), "second");
    apply_capture_plan(repo.path(), &second).unwrap();
    let records = load_records(repo.path()).unwrap();
    assert_eq!(records.len(), 2);
    assert!(records.iter().any(
        |record| matches!(record, Record::Subject { label: Some(label), .. } if label == "first")
    ));
    assert!(records.iter().any(
        |record| matches!(record, Record::Subject { label: Some(label), .. } if label == "second")
    ));
}

fn subject_only_plan(root: &Path, label: &str) -> project_knowledge::CapturePlan {
    let intent = parse_intent(
        &json!({
            "schema": "pk-authoring/v1",
            "subject": {"as": "s", "new": {"label": label}}
        })
        .to_string(),
    )
    .unwrap();
    build_capture_plan(root, &intent).unwrap()
}

fn simple_representation_plan(root: &Path, path: &str) -> project_knowledge::CapturePlan {
    let intent = parse_intent(
        &json!({
            "schema": "pk-authoring/v1",
            "subject": {"as": "s", "new": {"label": "subject"}},
            "representations": [{"as": "r", "subject": "s", "path": path, "role": "source"}]
        })
        .to_string(),
    )
    .unwrap();
    build_capture_plan(root, &intent).unwrap()
}

fn find_subject(plan: &project_knowledge::CapturePlan, label: &str) -> Uuid {
    plan.operations
        .iter()
        .find_map(|operation| match &operation.record {
            Record::Subject {
                id,
                label: Some(value),
                ..
            } if value == label => Some(*id),
            _ => None,
        })
        .unwrap()
}

fn find_claim(
    plan: &project_knowledge::CapturePlan,
    concern: &str,
    value: serde_json::Value,
) -> Uuid {
    plan.operations
        .iter()
        .find_map(|operation| match &operation.record {
            Record::Claim {
                id,
                concern: record_concern,
                value: record_value,
                ..
            } if record_concern == concern && *record_value == value => Some(*id),
            _ => None,
        })
        .unwrap()
}

fn find_evidence_for_claim(plan: &project_knowledge::CapturePlan, claim: Uuid) -> Uuid {
    plan.operations
        .iter()
        .find_map(|operation| match &operation.record {
            Record::EvidenceEvaluation { id, claim_id, .. } if *claim_id == claim => Some(*id),
            _ => None,
        })
        .unwrap()
}

fn claim_value(root: &Path, id: Uuid) -> serde_json::Value {
    load_records(root)
        .unwrap()
        .into_iter()
        .find_map(|record| match record {
            Record::Claim {
                id: claim_id,
                value,
                ..
            } if claim_id == id => Some(value),
            _ => None,
        })
        .unwrap()
}

fn git_repo() -> TempDir {
    let repo = tempfile::tempdir().unwrap();
    git(repo.path(), &["init", "-q"]);
    git(repo.path(), &["config", "user.email", "test@example.com"]);
    git(repo.path(), &["config", "user.name", "Test"]);
    repo
}

fn write_native(root: &Path, relative: &str, text: &str) {
    let path = root.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, text).unwrap();
}

fn commit_all(root: &Path, message: &str) {
    git(root, &["add", "."]);
    git(root, &["commit", "-q", "-m", message]);
}

fn git(root: &Path, args: &[&str]) {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {:?} failed: {}",
        args,
        String::from_utf8_lossy(&output.stderr)
    );
}

#[allow(dead_code)]
fn evidence_result_is_copy() -> EvidenceResult {
    EvidenceResult::Pass
}
