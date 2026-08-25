use std::path::Path;

use project_knowledge::{
    EvidenceState, Record, ResolutionOutcome, compile_in_memory, load_records,
};
use serde_json::Value;
use uuid::Uuid;

const SUBJECT: Uuid = Uuid::from_u128(0x4d67972333fb4fdaad639e514a6f11c4);
const CLAIM_PROPOSED: Uuid = Uuid::from_u128(0xde11b61aae7f410d9484ccb4aa2553d5);
const CLAIM_ACCEPTED: Uuid = Uuid::from_u128(0xbcb240ef0b9d4e6bbe79043c6d07b644);
const EVIDENCE_ACCEPTED: Uuid = Uuid::from_u128(0xd5e30f0c5b524c6ba1f00ccd50c61ae2);

fn claim_value(records: &[Record], id: Uuid) -> Option<&Value> {
    records.iter().find_map(|record| match record {
        Record::Claim {
            id: claim_id,
            value,
            ..
        } if *claim_id == id => Some(value),
        _ => None,
    })
}

#[test]
fn df_001_recovers_adr_status_across_adoption_boundary() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let records = load_records(root).expect("dogfood records should validate");
    assert_eq!(records.len(), 10);

    let (model, report) = compile_in_memory(root).expect("repository should compile itself");
    assert!(report.enriched);
    assert_eq!(report.record_count, 10);

    let before = model
        .resolve_current(
            SUBJECT,
            "decision_status",
            Some("2026-08-25T15:23:00Z"),
            None,
        )
        .expect("pre-adoption status should resolve");
    assert_eq!(before.outcome, ResolutionOutcome::Resolved);
    assert_eq!(before.claim_ids, vec![CLAIM_PROPOSED]);
    assert_eq!(
        claim_value(&records, CLAIM_PROPOSED),
        Some(&Value::String("proposed".to_string()))
    );

    let after = model
        .resolve_current(
            SUBJECT,
            "decision_status",
            Some("2026-08-25T16:00:00Z"),
            None,
        )
        .expect("post-adoption status should resolve");
    assert_eq!(after.outcome, ResolutionOutcome::Resolved);
    assert_eq!(after.claim_ids, vec![CLAIM_ACCEPTED]);
    assert_eq!(
        claim_value(&records, CLAIM_ACCEPTED),
        Some(&Value::String("accepted".to_string()))
    );

    assert_eq!(
        model
            .evidence_state(EVIDENCE_ACCEPTED)
            .expect("accepted-claim evidence should resolve"),
        EvidenceState::Current
    );

    let representations = model
        .representations_for_subject(SUBJECT)
        .expect("subject representations should resolve");
    assert_eq!(representations.len(), 2);
}
