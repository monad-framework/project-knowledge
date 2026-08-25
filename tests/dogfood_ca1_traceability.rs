use std::collections::BTreeSet;
use std::path::Path;

use project_knowledge::{EvidenceState, Record, ResolutionOutcome, compile_in_memory, load_records};
use uuid::Uuid;

const SUBJECT: Uuid = Uuid::from_u128(0xf8adff3e11ad43a992757d30fc9d1973);
const CLAIM_COMPLETE: Uuid = Uuid::from_u128(0xa033a49d167541d0aacc00cae1f2d724);
const ACTIVITY: Uuid = Uuid::from_u128(0xbc2c61c6df444a8a9bc97711bc3505d6);
const EVIDENCE: Uuid = Uuid::from_u128(0x91cc9483aa064f13b7e3209d30cdb30b);

#[test]
fn df_003_recovers_evidence_driven_ca1_delivery_trace() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let records = load_records(root).expect("dogfood records should validate");
    assert!(records.len() >= 39);

    let (model, report) = compile_in_memory(root).expect("repository should compile itself");
    assert_eq!(report.record_count, records.len());

    let status = model
        .resolve_current(SUBJECT, "capability_status", None, None)
        .expect("CA-1 capability status should resolve");
    assert_eq!(status.outcome, ResolutionOutcome::Resolved);
    assert_eq!(status.claim_ids, vec![CLAIM_COMPLETE]);

    assert_eq!(
        model
            .evidence_state(EVIDENCE)
            .expect("CA-1 evidence should resolve"),
        EvidenceState::Current
    );

    let representation_roles: BTreeSet<_> = records
        .iter()
        .filter_map(|record| match record {
            Record::Representation {
                subject_id: Some(subject_id),
                role,
                ..
            } if *subject_id == SUBJECT => Some(role.as_str()),
            _ => None,
        })
        .collect();
    assert_eq!(
        representation_roles,
        BTreeSet::from([
            "architecture_decision",
            "capability_design",
            "motivating_evidence",
            "verification_record",
        ])
    );

    let activity = records
        .iter()
        .find_map(|record| match record {
            Record::Activity {
                id,
                activity_type,
                used,
                generated_representation_ids,
                ..
            } if *id == ACTIVITY => Some((activity_type, used, generated_representation_ids)),
            _ => None,
        })
        .expect("DF-003 delivery Activity should exist");
    assert_eq!(activity.0, "evidence_driven_capability_delivery");
    assert_eq!(activity.1.len(), 2);
    assert_eq!(activity.2.len(), 3);

    let relations: Vec<_> = records
        .iter()
        .filter_map(|record| match record {
            Record::Relationship {
                relation,
                activity_id: Some(activity_id),
                ..
            } if *activity_id == ACTIVITY => Some(relation.as_str()),
            _ => None,
        })
        .collect();
    assert_eq!(relations.len(), 4);
    assert_eq!(relations.iter().filter(|value| **value == "motivates").count(), 2);
    assert!(relations.contains(&"governs_design_of"));
    assert!(relations.contains(&"verifies"));
}
