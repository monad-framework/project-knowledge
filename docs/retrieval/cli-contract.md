# Retrieval CLI Contract

## Command family

RT-1 adds two read-only surfaces:

```text
pk explain
pk trace
```

They serve different user intents while sharing one traversal engine.

## `pk explain`

### Purpose

Recover a trustworthy, human-oriented explanation around one known semantic root.

### Initial syntax

```text
pk explain \
  (--id <uuid> | --path <native-locator> | --label <exact-subject-label>) \
  [--at <rfc3339>] \
  [--context <uuid>] \
  [--depth <n>] \
  [--json]
```

Exactly one root selector is required.

`--depth` controls related-context traversal only. Current-state resolution, direct Authority, Evidence, Representation, and provenance bindings needed for the root explanation are included even if they would otherwise be awkward to reach through user-chosen relation filters.

### Human result

Example shape for DF-003:

```text
CA-1 — Guided Capture and Scaffolding capability

Current state
  capability_status = implemented_and_validated
  authority: 14d840f2-...

Representations
  motivating_evidence  docs/dogfooding/DF-001-adr-status-recovery.md
  motivating_evidence  docs/dogfooding/DF-002-serialization-choice.md
  capability_design    docs/authoring/README.md
  architecture_decision docs/decisions/ADR-0003-plan-review-apply-authoring.md
  verification_record  docs/authoring/closure.md

Recorded relationships
  DF-001 --motivates [authored]--> authoring design
  DF-002 --motivates [authored]--> authoring design
  ADR-0003 --governs_design_of [authored]--> authoring design
  CA-1 closure --verifies [authored]--> capability_status Claim

Evidence
  CA-1 acceptance suite and closure verification: pass, current

Provenance
  evidence_driven_capability_delivery
    used: DF-001, DF-002
    generated: authoring design, ADR-0003, CA-1 closure

Sources
  <native locators + states>
```

The exact formatting may be refined, but the semantic sections are normative.

### Root-kind behavior

`pk explain` works for any semantic entity kind.

- Subject — emphasizes current concerns, Representations, relationships, Authority, Evidence, provenance.
- Claim — emphasizes assertion, Authority context, evidence and connected rationale.
- Representation — emphasizes Subject, role, native state, freshness, provenance and relationships.
- Activity — emphasizes used native sources, generated Representations and activity-scoped Relationships.
- Evidence Evaluation — emphasizes exact supported Claim, method/result/current evidence state and inputs.
- other record kinds — show identity, deterministic bindings and bounded related context.

The command must not fail merely because a root has no rich explanation. A sparse but accurate explanation is valid.

## `pk trace`

### Purpose

Expose the neutral traversal primitive for explicit path/neighborhood work and machine consumers.

### Neighborhood syntax

```text
pk trace \
  (--id <uuid> | --path <native-locator> | --label <exact-subject-label>) \
  [--direction outgoing|incoming|both] \
  [--relation <exact-relation>]... \
  [--binding <binding-kind>]... \
  [--origin authored|imported|derived|inferred]... \
  [--depth <n>] \
  [--no-structural] \
  [--no-native] \
  [--json]
```

### Path syntax

```text
pk trace \
  --from-id <uuid> \
  --to-id <uuid> \
  [--direction outgoing|incoming|both] \
  [--relation <exact-relation>]... \
  [--binding <binding-kind>]... \
  [--origin <origin>]... \
  [--depth <n>] \
  [--json]
```

RT-1 may initially restrict path endpoints to UUID selectors even if neighborhood roots support exact locator/label selection. This keeps path CLI parsing unambiguous while leaving the library selector model reusable.

### Human result

Neighborhood output is a deterministic edge-oriented listing grouped by depth.

Path output displays alternating entities and exact edges:

```text
representation:e089d460-...
  --motivates [recorded/authored]-->
representation:c26cdd90-...
```

Structural bindings must visibly identify themselves as structural rather than authored project Relationships.

## Why `explain`, not `why`

RT-1 deliberately selects `explain` rather than `why` as the primary command name.

`why` strongly implies a causal answer. Project Knowledge often has rationale-like edges, but FR-606 forbids upgrading chronology/dependency/correlation into causality.

`explain` can safely present:

- current truth;
- recorded relationships;
- Authority;
- Evidence;
- provenance; and
- missing information

without implying that every displayed connection is causal.

A future `why` projection may be justified only when the project has enough explicit rationale/causal vocabulary to define it safely.

## Why `trace` remains lower level

`trace` is allowed to be graph-adjacent terminology because it is explicitly the technical traversal surface.

The normal recovery experience is `explain`; users do not need to think in graph operations to answer the DF-003 question.

## JSON mode

Global `--json` applies to both commands.

- `pk trace --json` emits `TraversalResult`.
- `pk explain --json` emits `RecoveryExplanation`.

No extra explanatory prose is mixed into stdout JSON.

Diagnostics intended for humans go to stderr when necessary.

## Exit behavior

Initial exit semantics:

- success with recovered result/path — exit `0`;
- successful query with `no_path` — exit `0`, because `no_path` is a valid semantic result;
- root/endpoint not found — existing typed input/not-found error class;
- ambiguous selector — typed ambiguity error with candidates;
- invalid depth/filter — usage/input error;
- invalid project memory / compile failure — existing validation/compile error semantics.

## No mutation

Neither command accepts confirmation or write flags.

They must work under the same read-only CI token/file permissions as the normal compiler and resolver.
