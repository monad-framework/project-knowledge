# Capture/Authoring Implementation Plan

## Increment name

**CA-1 — Guided Capture and Scaffolding**

CA-1 is the first evidence-authorized post-M0 increment. This name is intentionally narrower than declaring a broad M1 milestone.

## Implementation sequence

### CA-1.1 — Authoring model

Add non-S2 Rust types for:

- Authoring Intent;
- local aliases/selectors;
- Capture Plan;
- plan operations;
- field-origin metadata;
- preconditions; and
- warnings/blockers.

These types must live outside the canonical `Record` enum.

### CA-1.2 — Repository catalog

Provide read-only lookup over existing S2 sufficient to:

- list/select Subjects;
- list/select Representations;
- resolve exact UUID references;
- detect ambiguous convenience selectors; and
- capture the content/object state of existing records used as plan inputs.

### CA-1.3 — Git observation helpers

Reuse the native Git adapter boundary to obtain relevant blob/object identity for selected source paths and evidence inputs.

Avoid a global-HEAD precondition when more precise relevant state is available.

### CA-1.4 — Planner

Compile Authoring Intent into a Capture Plan:

- generate UUIDs and output paths;
- resolve aliases;
- construct existing `Record` values;
- attach field-origin metadata to the plan;
- create relevant preconditions;
- detect blockers; and
- never mutate S2.

### CA-1.5 — Plan rendering

Implement:

- human-readable semantic review; and
- stable JSON plan serialization for programmatic use.

### CA-1.6 — Applier

Implement relevant precondition checking, prospective validation, safe writes, idempotent reapply, divergent-output protection, and final validation.

The existing `write_record` conventions should be reused where possible rather than creating a second S2 serializer.

### CA-1.7 — CLI

Add the `pk capture`, `pk capture plan`, and `pk capture apply` surfaces defined by the CLI contract.

Interactive prompting should be isolated from planner semantics so non-interactive tests exercise the same planner/applier path.

### CA-1.8 — Acceptance fixtures

Create isolated fixture tests for CA-A01 through CA-A18.

DF-001 and DF-002 equivalence tests should prove that capture-generated records produce the same semantic resolver outcomes without rewriting the live repository's already-committed self-dogfood records.

## Likely code topology

A small module boundary is preferred over a new crate:

```text
src/
├── authoring/
│   ├── mod.rs
│   ├── intent.rs
│   ├── plan.rs
│   ├── planner.rs
│   ├── apply.rs
│   └── render.rs
├── main.rs
└── ... existing M0 modules

tests/
└── capture_authoring.rs

schemas/
└── authoring/
    └── v1/
        ├── intent.schema.json
        └── plan.schema.json
```

The authoring schemas govern operational input/plan documents only. They do not alter `schemas/v1/record.schema.json`.

## Dependency policy

Prefer the existing dependency set.

Relevant plan state should use native Git object identity rather than introducing a hashing dependency solely for stale-plan checks.

A new dependency requires a specific implementation need and a deliberate lockfile change.

## Testing order

1. Authoring Intent parsing/validation.
2. Alias/reference resolution.
3. Field-origin classification.
4. Planner no-write guarantee.
5. Relevant precondition behavior.
6. Prospective validation.
7. Idempotent apply/conflict behavior.
8. DF-001 equivalence.
9. DF-002 equivalence.
10. Full locked M0 + dogfood suite.

## Implementation constraints

CA-1 must not:

- modify the portable `Record` meaning to simplify authoring;
- change current-state resolution semantics;
- add epistemic vocabulary based only on DF-002;
- add AI inference;
- require a daemon;
- write native artifacts;
- make Capture Plans necessary for runtime reconstruction; or
- create a hidden database source of truth for authoring sessions.

## Exit criterion

CA-1 is complete when every capture acceptance case passes on a clean runner and a third real dogfood experiment can use the new authoring surface to create its S2 bundle without hand-authoring structural boilerplate.

That third dogfood experiment is intentionally deferred until after CA-1 implementation so it can test the new workflow rather than merely reproduce the old pain again.
