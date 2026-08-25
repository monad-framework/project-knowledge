# Capture Plan

## Purpose

A Capture Plan is the review boundary between human intent and persistent S2 mutation.

It is a **transient operational artifact**, not canonical project knowledge. Saving a plan for review or automation does not make the plan itself an S2 record.

## Envelope

A serialized plan should use its own version, for example:

```json
{
  "schema": "pk-capture-plan/v1",
  "plan_id": "...",
  "created_at": "...",
  "operations": [],
  "preconditions": [],
  "warnings": []
}
```

## Operations

CA-1 only needs create-record operations. Existing S2 records are referenced, not silently rewritten.

Conceptually:

```json
{
  "op": "create_record",
  "path": ".pk/records/claim/<uuid>.json",
  "record": {
    "schema": "pk/v1",
    "kind": "claim",
    "id": "<uuid>",
    "subject_id": "<uuid>",
    "concern": "decision_status",
    "value": "accepted"
  },
  "field_origins": {
    "/schema": "generated",
    "/kind": "generated",
    "/id": "generated",
    "/subject_id": "authored",
    "/concern": "authored",
    "/value": "authored"
  }
}
```

`field_origins` explains where material plan values came from. It is plan metadata and is not copied into `pk/v1` unless the domain model independently requires equivalent provenance.

## Relevant preconditions

A plan must become stale when a relevant input changes, but **must not become stale merely because unrelated repository state changed**.

Relevant preconditions include:

- an output record path expected to be absent;
- an existing S2 record selected by the author remaining byte/object-equivalent to the version inspected during planning;
- a native file/object whose current state is embedded in a planned Representation or Evidence Evaluation remaining at the observed Git object state; and
- any explicit native source-state used to derive a plan field remaining unchanged.

A global `HEAD` equality check is insufficiently precise because an unrelated commit should not invalidate a plan whose relevant inputs are unchanged.

Git object identity may be obtained mechanically with native Git, including for working-tree files through `git hash-object`, so the implementation does not need a new hashing subsystem merely for plan preconditions.

## Warnings and blockers

Plans may contain:

- **info** — useful context;
- **warning** — review-worthy but not necessarily unsafe;
- **blocker** — an unresolved semantic or structural issue that prevents apply.

Examples of blockers:

- multiple possible Subjects when an existing Subject must be selected;
- missing authority basis for a requested Authority record;
- missing target Claim for evidence;
- an unconfirmed semantic suggestion;
- duplicate planned UUID/path;
- cross-reference to a missing record; or
- invalid valid-time interval.

## Plan immutability

Once serialized for apply, the plan should be treated as an immutable proposal. Editing semantic intent should produce a new plan rather than mutating a reviewed plan in place.

Generated UUIDs are fixed within the plan. Replanning may produce different UUIDs; applying one plan must use exactly the IDs reviewed in that plan.

## Apply semantics

Apply must:

1. reject plans with blockers;
2. re-check relevant preconditions;
3. verify planned output paths do not conflict;
4. overlay the proposed records onto the current corpus in memory/staging;
5. run the same structural, semantic, and cross-reference validation used by normal record loading;
6. write only if preflight succeeds; and
7. run normal validation again on the resulting corpus.

Validation errors discovered before final writes must leave no semantic records partially applied.

## Idempotence

Re-applying an already completed plan should be safe:

- if every planned record already exists with semantically/byte-equivalent content, return a successful no-op result;
- if a planned path exists with divergent content, fail rather than overwrite it.

CA-1 does not authorize automatic mutation of existing S2 records.

## Crash behavior

Cross-file filesystem writes are not assumed to be intrinsically transactional. The implementation should stage complete record bytes before finalization and use atomic per-file replacement/creation where available.

If an operating-system failure interrupts finalization, `pk validate` must detect any incomplete corpus, and re-applying the same immutable plan must either complete safely or report the divergent state. Full crash-journal machinery is not required unless implementation testing shows it is necessary.
