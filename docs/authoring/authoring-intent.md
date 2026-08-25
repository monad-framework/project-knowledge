# Authoring Intent

## Purpose

Authoring Intent is a compact input format for expressing semantic choices without manually constructing portable S2 records.

It is **not canonical project memory** and does not replace `pk/v1`. Its only normative purpose is to provide a machine-readable input to the Capture Planner.

Interactive capture produces the same logical intent in memory without requiring the user to create this file.

## Version

The first implementation should use an independently versioned envelope such as:

```json
{
  "schema": "pk-authoring/v1"
}
```

Authoring schema evolution must not imply S2 schema evolution.

## Local aliases

Intent documents use local aliases to refer to records within one capture bundle.

Example:

```json
{
  "schema": "pk-authoring/v1",
  "subject": {
    "as": "architecture",
    "new": { "label": "ADR-0001 architecture decision" }
  },
  "representations": [
    {
      "as": "adr",
      "subject": "architecture",
      "path": "docs/decisions/ADR-0001-federated-portable-core.md",
      "role": "decision_record"
    },
    {
      "as": "selected",
      "subject": "architecture",
      "path": "docs/architecture/selected-architecture.md",
      "role": "current_architecture_definition"
    }
  ],
  "claims": [
    {
      "as": "accepted",
      "subject": "architecture",
      "concern": "decision_status",
      "value": "accepted"
    }
  ],
  "assertions": [
    {
      "claim": "accepted",
      "representation": "selected",
      "valid_from": "2026-08-25T15:24:13Z"
    }
  ],
  "authorities": [
    {
      "subject": "architecture",
      "concern": "decision_status",
      "representation": "selected",
      "basis": "Architecture PR #8 adoption decision",
      "valid_from": "2026-08-25T15:24:13Z"
    }
  ]
}
```

The author does not provide UUIDs, record paths, `schema`, `kind`, or Git blob hashes.

## Existing-record references

A capture may select an existing record by stable ID or through an interactive lookup result.

For non-interactive input, explicit UUID reference is always allowed. Convenience selectors may also be supported when unambiguous, for example:

```json
{
  "existing": {
    "kind": "representation",
    "id": "92157a02-4841-4880-a69e-50fd2e459bca"
  }
}
```

The first implementation should not define fuzzy matching as authoritative identity resolution. Label/path lookup may produce candidates, but multiple matches require explicit selection.

## Semantic fields remain semantic

Authoring Intent must still require explicit values for meaning-bearing fields such as:

- Representation role;
- Claim concern and value;
- relationship type;
- relationship origin when authored versus imported/derived matters;
- authority concern and basis;
- valid-time boundaries when the author intends to assert them;
- evidence target Claim;
- evidence method/result; and
- activity type where an Activity is intentionally captured.

## Time defaults

`recorded_at` may default to the capture event time because it records when Project Knowledge records the assertion/evaluation.

`valid_from` and `valid_until` must **not** silently default to `recorded_at`. They describe domain validity, not merely capture time.

Interactive mode may offer explicit conveniences such as “valid from now” or “valid from selected Git commit time,” but choosing one converts it into authored intent and must be visible in review.

## Native Git references

For a repository path selected as a native Representation or evidence input, the author normally supplies the path and semantic role. The Planner may mechanically obtain:

- `source_system = git`;
- `object_type = blob`;
- `locator = <selected path>`; and
- the current Git blob/object state.

The observed state must remain visible in the plan.

## Suggestions

The first implementation does not require semantic suggestions. If suggestions are later added, Authoring Intent must distinguish them from authored values and the Applier must reject unconfirmed semantic suggestions.
