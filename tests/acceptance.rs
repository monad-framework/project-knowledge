use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::Command;

use project_knowledge::model::{
    EvidenceResult, NativeReference, ObservationStatus, Record, RECORD_SCHEMA,
};
use project_knowledge::records::write_record;
use project_knowledge::{
    EvidenceState, Freshness, ReadModel, ResolutionOutcome, compile, compile_in_memory,
    default_db_path, rebuild,
};
use tempfile::TempDir;
use uuid::Uuid;

const T0: &str = "2026-01-01T00:00:00Z";

#[test]
fn s1_minimal_markdown_git_requires_no_semantic_records() {
    let repo = git_repo();
    fs::write(repo.path().join("README.md"), "# Minimal project\n").unwrap();
    commit_all(repo.path(), "initial");

    let (model, report) = compile_in_memory(repo.path()).unwrap();
    assert!(!report.enriched);
    assert_eq!(report.record_count, 0);
    assert!(!repo.path().join(".pk").exists());
    let observations = model.observations().unwrap();
    assert!(observations.iter().any(|observation| {
        observation.source_system == "git"
            && observation.object_type == "repository"
            && observation.status == ObservationStatus::Available
            && observation.state.is_some()
    }));
}

#[test]
fn s2_subject_identity_survives_representation_relocation() {
    let repo = git_repo();
    fs::create_dir_all(repo.path().join("docs/decisions")).unwrap();
    fs::write(repo.path().join("docs/decisions/ADR-0001.md"), "decision\n").unwrap();
    commit_all(repo.path(), "add adr");

    let subject = Uuid::new_v4();
    write_record(
        repo.path(),
        &Record::Subject {
            schema: RECORD_SCHEMA.into(),
            id: subject,
            label: Some("ADR-0001".into()),
        },
    )
    .unwrap();

    let old_rep = Uuid::new_v4();
    write_record(
        repo.path(),
        &representation(old_rep, Some(subject), "docs/decisions/ADR-0001.md", "decision_record"),
    )
    .unwrap();

    fs::create_dir_all(repo.path().join("architecture/decisions")).unwrap();
    git(
        repo.path(),
        &[
            "mv",
            "docs/decisions/ADR-0001.md",
            "architecture/decisions/ADR-0001.md",
        ],
    );
    commit_all(repo.path(), "move adr");

    let new_rep = Uuid::new_v4();
    write_record(
        repo.path(),
        &representation(
            new_rep,
            Some(subject),
            "architecture/decisions/ADR-0001.md",
            "decision_record",
        ),
    )
    .unwrap();

    let (model, _) = compile_in_memory(repo.path()).unwrap();
    let representations = model.representations_for_subject(subject).unwrap();
    assert_eq!(representations.len(), 2);
    assert!(representations.contains(&old_rep));
    assert!(representations.contains(&new_rep));
}

#[test]
fn s3_scoped_authority_beats_stale_projection_without_using_repetition() {
    let repo = git_repo();
    fs::write(repo.path().join("state.json"), "{}\n").unwrap();
    fs::write(repo.path().join("status.md"), "stale\n").unwrap();
    commit_all(repo.path(), "sources");

    let subject = Uuid::new_v4();
    let canonical_rep = Uuid::new_v4();
    let projection_rep = Uuid::new_v4();
    let current_claim = Uuid::new_v4();
    let stale_claim = Uuid::new_v4();

    for record in [
        Record::Subject {
            schema: RECORD_SCHEMA.into(),
            id: subject,
            label: Some("work item".into()),
        },
        representation(canonical_rep, Some(subject), "state.json", "canonical_source"),
        representation(projection_rep, Some(subject), "status.md", "projection"),
        claim(current_claim, subject, "lifecycle.status", "closed"),
        claim(stale_claim, subject, "lifecycle.status", "ready"),
        assertion(Uuid::new_v4(), current_claim, canonical_rep, None, None),
        assertion(Uuid::new_v4(), stale_claim, projection_rep, None, None),
        authority(
            Uuid::new_v4(),
            subject,
            "lifecycle.status",
            canonical_rep,
            "project lifecycle policy",
        ),
    ] {
        write_record(repo.path(), &record).unwrap();
    }

    let (model, _) = compile_in_memory(repo.path()).unwrap();
    let resolution = model
        .resolve_current(subject, "lifecycle.status", None, None)
        .unwrap();
    assert_eq!(resolution.outcome, ResolutionOutcome::Resolved);
    assert_eq!(resolution.claim_ids, vec![current_claim]);
}

#[test]
fn s4_historical_correction_preserves_old_and_new_valid_truth() {
    let repo = git_repo();
    fs::write(repo.path().join("status.md"), "status\n").unwrap();
    commit_all(repo.path(), "source");

    let subject = Uuid::new_v4();
    let rep = Uuid::new_v4();
    let old_claim = Uuid::new_v4();
    let new_claim = Uuid::new_v4();

    for record in [
        Record::Subject {
            schema: RECORD_SCHEMA.into(),
            id: subject,
            label: None,
        },
        representation(rep, Some(subject), "status.md", "status_source"),
        claim(old_claim, subject, "phase", "requirements"),
        claim(new_claim, subject, "phase", "domain_modeling"),
        assertion(
            Uuid::new_v4(),
            old_claim,
            rep,
            Some("2026-01-01T00:00:00Z"),
            Some("2026-02-01T00:00:00Z"),
        ),
        assertion(
            Uuid::new_v4(),
            new_claim,
            rep,
            Some("2026-02-01T00:00:00Z"),
            None,
        ),
        authority(Uuid::new_v4(), subject, "phase", rep, "root status policy"),
    ] {
        write_record(repo.path(), &record).unwrap();
    }

    let (model, _) = compile_in_memory(repo.path()).unwrap();
    let historical = model
        .resolve_current(subject, "phase", Some("2026-01-15T00:00:00Z"), None)
        .unwrap();
    let current = model
        .resolve_current(subject, "phase", Some("2026-03-01T00:00:00Z"), None)
        .unwrap();
    assert_eq!(historical.claim_ids, vec![old_claim]);
    assert_eq!(current.claim_ids, vec![new_claim]);
}

#[test]
fn s5_context_preserves_observed_state_distinct_from_current_state() {
    let repo = git_repo();
    fs::write(repo.path().join("file.txt"), "one\n").unwrap();
    commit_all(repo.path(), "one");
    let old_head = git_output(repo.path(), &["rev-parse", "HEAD"]);

    fs::write(repo.path().join("file.txt"), "two\n").unwrap();
    commit_all(repo.path(), "two");
    let new_head = git_output(repo.path(), &["rev-parse", "HEAD"]);
    assert_ne!(old_head, new_head);

    let context_id = Uuid::new_v4();
    let mut dimensions = BTreeMap::new();
    dimensions.insert("checkout".into(), "historical-worktree".into());
    write_record(
        repo.path(),
        &Record::Context {
            schema: RECORD_SCHEMA.into(),
            id: context_id,
            dimensions,
            source_state: Some(NativeReference {
                source_system: "git".into(),
                object_type: "repository".into(),
                locator: ".".into(),
                state: Some(old_head.clone()),
            }),
        },
    )
    .unwrap();

    let (model, _) = compile_in_memory(repo.path()).unwrap();
    let Record::Context { source_state, .. } = model.record(context_id).unwrap().unwrap() else {
        panic!("expected context");
    };
    assert_eq!(source_state.unwrap().state.as_deref(), Some(old_head.as_str()));
    let current = model.observation("git", "repository", ".").unwrap().unwrap();
    assert_eq!(current.state.as_deref(), Some(new_head.as_str()));
}

#[test]
fn s6_evidence_is_claim_relative_and_ignores_unrelated_change() {
    let repo = git_repo();
    fs::create_dir_all(repo.path().join("src")).unwrap();
    fs::write(repo.path().join("src/a.txt"), "a1\n").unwrap();
    fs::write(repo.path().join("src/unrelated.txt"), "u1\n").unwrap();
    commit_all(repo.path(), "inputs");
    let a_blob = git_output(repo.path(), &["rev-parse", "HEAD:src/a.txt"]);

    let subject = Uuid::new_v4();
    let c1 = Uuid::new_v4();
    let c2 = Uuid::new_v4();
    let evaluation = Uuid::new_v4();
    for record in [
        Record::Subject {
            schema: RECORD_SCHEMA.into(),
            id: subject,
            label: None,
        },
        claim(c1, subject, "validator.claim", "C1"),
        claim(c2, subject, "validator.claim", "C2 broader"),
        Record::EvidenceEvaluation {
            schema: RECORD_SCHEMA.into(),
            id: evaluation,
            claim_id: c1,
            method: "exact fixture validator".into(),
            result: EvidenceResult::Pass,
            recorded_at: T0.into(),
            inputs: vec![NativeReference {
                source_system: "git".into(),
                object_type: "blob".into(),
                locator: "src/a.txt".into(),
                state: Some(a_blob),
            }],
            context_id: None,
            notes: Some("supports C1 only".into()),
        },
    ] {
        write_record(repo.path(), &record).unwrap();
    }

    let (model, _) = compile_in_memory(repo.path()).unwrap();
    assert_eq!(model.evidence_state(evaluation).unwrap(), EvidenceState::Current);
    assert!(model.evidence_for_claim(c2).unwrap().is_empty());

    fs::write(repo.path().join("src/unrelated.txt"), "u2\n").unwrap();
    commit_all(repo.path(), "unrelated change");
    let (model, _) = compile_in_memory(repo.path()).unwrap();
    assert_eq!(model.evidence_state(evaluation).unwrap(), EvidenceState::Current);

    fs::write(repo.path().join("src/a.txt"), "a2\n").unwrap();
    commit_all(repo.path(), "relevant change");
    let (model, _) = compile_in_memory(repo.path()).unwrap();
    assert_eq!(model.evidence_state(evaluation).unwrap(), EvidenceState::Stale);
}

#[test]
fn s7_derived_projection_freshness_tracks_declared_inputs() {
    let repo = git_repo();
    fs::write(repo.path().join("source.md"), "source v1\n").unwrap();
    fs::write(repo.path().join("summary.md"), "summary\n").unwrap();
    commit_all(repo.path(), "source and projection");
    let source_blob = git_output(repo.path(), &["rev-parse", "HEAD:source.md"]);

    let representation_id = Uuid::new_v4();
    write_record(
        repo.path(),
        &representation(representation_id, None, "summary.md", "projection"),
    )
    .unwrap();
    write_record(
        repo.path(),
        &Record::Activity {
            schema: RECORD_SCHEMA.into(),
            id: Uuid::new_v4(),
            activity_type: "generate_summary".into(),
            recorded_at: T0.into(),
            used: vec![NativeReference {
                source_system: "git".into(),
                object_type: "blob".into(),
                locator: "source.md".into(),
                state: Some(source_blob),
            }],
            generated_representation_ids: vec![representation_id],
        },
    )
    .unwrap();

    let (model, _) = compile_in_memory(repo.path()).unwrap();
    assert_eq!(
        model.representation_freshness(representation_id).unwrap(),
        Freshness::Current
    );

    fs::write(repo.path().join("source.md"), "source v2\n").unwrap();
    commit_all(repo.path(), "change source");
    let (model, _) = compile_in_memory(repo.path()).unwrap();
    assert_eq!(
        model.representation_freshness(representation_id).unwrap(),
        Freshness::Stale
    );
}

#[test]
fn s8_unknown_is_first_class_when_authority_is_absent() {
    let repo = git_repo();
    fs::write(repo.path().join("note.md"), "note\n").unwrap();
    commit_all(repo.path(), "source");

    let subject = Uuid::new_v4();
    let rep = Uuid::new_v4();
    let claim_id = Uuid::new_v4();
    for record in [
        Record::Subject {
            schema: RECORD_SCHEMA.into(),
            id: subject,
            label: None,
        },
        representation(rep, Some(subject), "note.md", "note"),
        claim(claim_id, subject, "status", "maybe"),
        assertion(Uuid::new_v4(), claim_id, rep, None, None),
    ] {
        write_record(repo.path(), &record).unwrap();
    }

    let (model, _) = compile_in_memory(repo.path()).unwrap();
    let resolution = model.resolve_current(subject, "status", None, None).unwrap();
    assert_eq!(resolution.outcome, ResolutionOutcome::Unknown);
}

#[test]
fn clean_room_rebuild_preserves_semantic_results() {
    let repo = git_repo();
    fs::write(repo.path().join("state.json"), "{}\n").unwrap();
    commit_all(repo.path(), "source");

    let subject = Uuid::new_v4();
    let rep = Uuid::new_v4();
    let claim_id = Uuid::new_v4();
    for record in [
        Record::Subject {
            schema: RECORD_SCHEMA.into(),
            id: subject,
            label: None,
        },
        representation(rep, Some(subject), "state.json", "canonical"),
        claim(claim_id, subject, "status", "closed"),
        assertion(Uuid::new_v4(), claim_id, rep, None, None),
        authority(Uuid::new_v4(), subject, "status", rep, "test policy"),
    ] {
        write_record(repo.path(), &record).unwrap();
    }

    compile(repo.path()).unwrap();
    let before = ReadModel::open(&default_db_path(repo.path()))
        .unwrap()
        .resolve_current(subject, "status", None, None)
        .unwrap();

    rebuild(repo.path()).unwrap();
    let after = ReadModel::open(&default_db_path(repo.path()))
        .unwrap()
        .resolve_current(subject, "status", None, None)
        .unwrap();

    assert_eq!(before.outcome, after.outcome);
    assert_eq!(before.claim_ids, after.claim_ids);
    assert_eq!(before.authority_assignment_ids, after.authority_assignment_ids);
}

fn git_repo() -> TempDir {
    let dir = tempfile::tempdir().unwrap();
    git(dir.path(), &["init", "-q"]);
    git(
        dir.path(),
        &["config", "user.email", "pk-test@example.invalid"],
    );
    git(
        dir.path(),
        &["config", "user.name", "Project Knowledge Test"],
    );
    dir
}

fn commit_all(root: &Path, message: &str) {
    git(root, &["add", "-A"]);
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

fn git_output(root: &Path, args: &[&str]) -> String {
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
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn representation(id: Uuid, subject_id: Option<Uuid>, locator: &str, role: &str) -> Record {
    Record::Representation {
        schema: RECORD_SCHEMA.into(),
        id,
        subject_id,
        role: role.into(),
        native: NativeReference {
            source_system: "git".into(),
            object_type: "blob".into(),
            locator: locator.into(),
            state: None,
        },
    }
}

fn claim(id: Uuid, subject_id: Uuid, concern: &str, value: &str) -> Record {
    Record::Claim {
        schema: RECORD_SCHEMA.into(),
        id,
        subject_id,
        concern: concern.into(),
        value: serde_json::Value::String(value.into()),
    }
}

fn assertion(
    id: Uuid,
    claim_id: Uuid,
    representation_id: Uuid,
    valid_from: Option<&str>,
    valid_until: Option<&str>,
) -> Record {
    Record::Assertion {
        schema: RECORD_SCHEMA.into(),
        id,
        claim_id,
        representation_id,
        recorded_at: T0.into(),
        valid_from: valid_from.map(str::to_string),
        valid_until: valid_until.map(str::to_string),
        source_state: None,
        context_id: None,
    }
}

fn authority(
    id: Uuid,
    subject_id: Uuid,
    concern: &str,
    representation_id: Uuid,
    basis: &str,
) -> Record {
    Record::Authority {
        schema: RECORD_SCHEMA.into(),
        id,
        subject_id,
        concern: concern.into(),
        representation_id,
        basis: basis.into(),
        recorded_at: T0.into(),
        valid_from: None,
        valid_until: None,
        context_id: None,
    }
}
