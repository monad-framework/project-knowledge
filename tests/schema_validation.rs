use std::fs;

#[test]
fn checked_in_schema_is_valid_json_schema_and_rejects_unknown_kind() {
    let schema: serde_json::Value =
        serde_json::from_str(include_str!("../schemas/v1/record.schema.json")).unwrap();
    let validator = jsonschema::validator_for(&schema).unwrap();
    let invalid = serde_json::json!({
        "schema": "pk/v1",
        "kind": "magic_truth",
        "id": "aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa"
    });
    assert!(!validator.is_valid(&invalid));
}

#[test]
fn schema_file_is_stable_checked_in_source() {
    let text = fs::read_to_string("schemas/v1/record.schema.json").unwrap();
    assert!(text.contains("Project Knowledge Portable Record v1"));
    assert!(text.contains("evidence_evaluation"));
}
