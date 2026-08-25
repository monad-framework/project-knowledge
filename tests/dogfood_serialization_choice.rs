use std::collections::HashSet;
use std::path::Path;

use project_knowledge::{
    EntityKind, EvidenceState, Record, ResolutionOutcome, compile_in_memory, load_records,
};
use serde_json::Value;
use uuid::Uuid;

const SUBJECT: Uuid = Uuid::from_u128(0x7a4e2d611c3b4f82a6d59e71b2c4d8f0);
const REP_DECISION: Uuid = Uuid::from_u128(0xa3d57c182e644b9f8c217f0a5d3e9b62);
const CLAIM_JSON: Uuid = Uuid::from_u128(0xc4f1298e7a3d4b619e521d8c6f0a4b73);
const CLAIM_YAML: Uuid = Uuid::from_u128(0xd5a23b9f8c4e4d72a1632e9f7b1c5d84);
const CLAIM_TOML: Uuid = Uuid::from_u128(0xe6b34ca09d5f4e83b2743fa08c2d6e95);
const CLAIM_CUSTOM: Uuid = Uuid::from_u128(0xf7c45db1ae604f9483654ab19d3e7f06);
const ACTIVITY_SELECTION: Uuid = Uuid::from_u128(0x7e3bc42815d7406bbadcb12804a5e67d);
const EVIDENCE_JSON: Uuid = Uuid::from_u128(0x6d2ab31704c64f5aa9cba017f394d56c);

fn claim_value(records: &[Record], id: Uuid) -> Option<&Value> {
    records.iter().find_map(|record| match record {
        Record::Claim {
            id: claim_id, value, ..
        } if *claim_id == id => Some(value),
        _ => None,
    })
}

#[test]
fn df_002_preserves_uncertainty_alternatives_and_selection() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let records = load_records(root).expect("dogfood records should validate");
    assert!(records.len() >= 24);

    let (model, report) = compile_in_memory(root).expect("repository should compile itself");
    assert!(report.enriched);

    let before = model
        .resolve_current(
            SUBJECT,
            "serialization.format",
            Some("2026-08-25T15:30:00Z"),
            None,
        )
        .expect("pre-selection state should resolve safely");
    assert_eq!(before.outcome, ResolutionOutcome::Unknown);
    assert!(before.claim_ids.is_empty());

    let after = model
        .resolve_current(
            SUBJECT,
            "serialization.format",
            Some("2026-08-25T16:00:00Z"),
            None,
        )
        .expect("post-selection state should resolve");
    assert_eq!(after.outcome, ResolutionOutcome::Resolved);
    assert_eq!(after.claim_ids, vec![CLAIM_JSON]);
    assert_eq!(
        claim_value(&records, CLAIM_JSON),
        Some(&Value::String("json".to_string()))
    );

    let alternatives: HashSet<Uuid> = records
        .iter()
        .filter_map(|record| match record {
            Record::Relationship {
                from,
                relation,
                to,
                ..
            } if relation == "alternative_considered_in"
                && from.kind == EntityKind::Claim
                && to.kind == EntityKind::Representation
                && to.id == REP_DECISION => Some(from.id),
            _ => None,
        })
        .collect();
    assert_eq!(
        alternatives,
        HashSet::from([CLAIM_YAML, CLAIM_TOML, CLAIM_CUSTOM])
    );
    assert_eq!(
        claim_value(&records, CLAIM_YAML),
        Some(&Value::String("yaml".to_string()))
    );
    assert_eq!(
        claim_value(&records, CLAIM_TOML),
        Some(&Value::String("toml".to_string()))
    );
    assert_eq!(
        claim_value(&records, CLAIM_CUSTOM),
        Some(&Value::String("custom".to_string()))
    );

    assert!(records.iter().any(|record| matches!(
        record,
        Record::Activity {
            id,
            generated_representation_ids,
            ..
        } if *id == ACTIVITY_SELECTION && generated_representation_ids == &vec![REP_DECISION]
    )));

    assert_eq!(
        model
            .evidence_state(EVIDENCE_JSON)
            .expect("implementation evidence should resolve"),
        EvidenceState::Current
    );
}
