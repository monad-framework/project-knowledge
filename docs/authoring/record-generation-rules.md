# Record Generation Rules

## Goal

Convert reviewed semantic intent into ordinary `pk/v1` records without changing the meaning of the existing portable contract.

## Universal generated fields

For every new record, the Planner may generate:

- `schema = pk/v1`;
- `kind` from the requested record operation;
- a UUIDv4 `id`; and
- destination `.pk/records/<kind>/<id>.json`.

Generated IDs remain stable within one Capture Plan.

## References and aliases

Local authoring aliases are resolved to the generated or selected stable UUIDs before the Capture Plan is finalized.

Aliases never appear in final S2 records.

If two intent items use the same alias, planning fails.

## Recorded time

For records whose schema requires `recorded_at`, the Planner may generate the capture timestamp if the author does not provide a different explicit recorded time.

This is mechanical because `recorded_at` describes when Project Knowledge recorded the semantic statement/evaluation.

The Planner must not infer `valid_from` or `valid_until` from `recorded_at`.

## Git-backed Representation

When the author selects a repository file as a Representation and confirms its semantic role, the Planner may construct:

```json
{
  "source_system": "git",
  "object_type": "blob",
  "locator": "<path>",
  "state": "<observed Git blob identity>"
}
```

If the portable Representation contract intentionally omits `state` for mutable/current binding semantics, the plan may still keep the observed state as a precondition. The authoring layer must not change the domain meaning of Representation simply to store planning metadata.

## Subject creation and selection

### New Subject

The author supplies semantic intent such as label; the tool generates the stable ID.

### Existing Subject

The tool may list/search candidate Subjects. Selection is explicit when more than one candidate exists.

The Planner never auto-merges Subjects because two labels, paths, titles, or contents appear similar.

## Representation role

`role` is semantic and must be authored/confirmed.

The Planner may offer recently used or documented role strings for convenience, but it may not assign a role solely from a path or filename.

## Claims

The author supplies:

- Subject;
- concern; and
- value.

The Planner generates only envelope/identity/path structure.

The first increment does not attempt Claim normalization beyond the existing M0 semantics.

## Assertions

The author explicitly binds a Claim to a Representation.

The Planner:

- resolves aliases to UUIDs;
- captures ordinary recorded time;
- may capture selected native source-state mechanically; and
- writes explicit valid/context fields only when authored.

## Authority

Every Authority requires authored:

- Subject;
- concern/scope;
- Representation; and
- basis.

Valid-time boundaries are authored when needed.

No Authority record is generated merely because a Representation is newer, located under `docs/decisions`, or associated with a merged PR.

## Relationships

The author supplies endpoints and semantic relation type.

The Planner may resolve aliases and generate ID/path.

For an interactively authored relationship, `origin = authored` may be mechanically selected only because the user is explicitly creating that relationship in the authoring session. Imported/derived/inferred origins require their corresponding workflow semantics and are not interchangeable convenience values.

## Activities

The author supplies the activity type and which source inputs/generated Representations the Activity is intended to connect.

The Planner may capture recorded time and selected Git source states.

Chronology alone does not generate an Activity or causal edge.

## Evidence Evaluations

The author supplies:

- exact target Claim;
- method;
- result;
- relevant input artifacts; and
- optional notes/context.

The Planner may capture current Git state for each selected relevant input.

Evidence is never copied to another Claim merely because it shares a Subject or concern.

## Context

Existing Context may be selected explicitly.

Creating a new Context uses the existing record kind and requires authored dimensions that matter to interpretation. The Planner may scaffold IDs/envelope/source state but must not silently record sensitive environment information.

## File writing

Final JSON should use the same pretty, newline-terminated serialization convention as existing `write_record` behavior so capture-created records are ordinary inspectable S2 files rather than a special generated format.
