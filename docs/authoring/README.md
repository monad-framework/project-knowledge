# Capture and Authoring

## Status

**CA-1 — Guided Capture and Scaffolding: implementation complete within the evidence-authorized scope.**

DF-001 and DF-002 independently showed that the M0 semantic kernel can answer materially different real recovery questions without schema expansion, while requiring 10 and 14 hand-authored S2 records respectively. Both cases repeated the same friction: humans had to construct UUIDs, record paths, timestamps, Git state bindings, cross-references, and JSON envelopes even when those values were mechanical consequences of a smaller set of semantic decisions.

That evidence authorized CA-1. The implementation now turns compact human semantic intent into a reviewable Capture Plan and then into ordinary `pk/v1` records after explicit review and relevant-input validation.

The closure and acceptance evidence is recorded in [`closure.md`](closure.md).

## Objective

A human should express the semantic intent of a capture operation while `pk` handles mechanical structure.

The implemented interaction is:

```text
semantic intent
     │
     ▼
Capture Planner
     │  inspect existing S2 + relevant native state
     ▼
reviewable Capture Plan
     │
     ├── authored fields
     ├── mechanically generated fields
     ├── observed source-state fields
     ├── explicit suggestions, if any
     ├── preconditions
     └── warnings / blockers
     │
     ▼
Human review / confirmation
     │
     ▼
Capture Applier
     │
     ▼
portable S2 records
     │
     ▼
existing validate / compile / resolver path
```

The authoring layer does not create a second truth model. It compiles human-declared intent into the existing `pk/v1` portable record model.

## Central safety rule

> Automate structure; require confirmation for meaning.

The tool may generate identifiers, filenames, record envelopes, timestamps for the capture event, and current Git object identity. It may discover existing records and require exact selection. It must not silently decide semantic identity, authority scope, evidence breadth, valid-time meaning, epistemic state, or causality.

## Implemented command surface

```text
pk capture
pk capture plan [--intent <file|->] [--out <file>] [--json]
pk capture apply --plan <file> [--yes] [--json]
```

`pk capture plan` is read-only with respect to S2. `pk capture apply` rechecks relevant preconditions, validates the prospective corpus, refuses divergent existing output, and supports idempotent re-application of an already completed plan.

The current interactive shortcut wraps Authoring Intent, semantic plan review, confirmation, and apply. A richer field-by-field authoring wizard is intentionally deferred until dogfooding demonstrates that it is needed.

## Selected interaction architecture

CA-1 uses a **plan → review → apply** boundary.

1. **Plan** — collect semantic intent and inspect only the repository state needed to construct the proposed records.
2. **Review** — show exactly what will be created, which values were authored/generated/observed/suggested, and any unresolved semantic choices.
3. **Apply** — re-check relevant preconditions, validate the prospective corpus, write the records, and run normal semantic validation.

No semantic mutation occurs during planning.

## Documents

- [Evidence and scope](evidence-and-scope.md)
- [Interaction model](interaction-model.md)
- [Authoring intent](authoring-intent.md)
- [Capture plan](capture-plan.md)
- [Safety boundaries](safety-boundaries.md)
- [CLI contract](cli-contract.md)
- [Record generation rules](record-generation-rules.md)
- [Acceptance specification](acceptance.md)
- [Implementation plan](implementation-plan.md)
- [CA-1 closure](closure.md)

The architectural decision is recorded in [`ADR-0003`](../decisions/ADR-0003-plan-review-apply-authoring.md).

## Non-goals retained

CA-1 does not:

- add a first-class Question or Epistemic Annotation record;
- infer ADR acceptance from merges;
- infer that two artifacts are the same Subject;
- infer authority from filenames, directories, recency, repetition, or search rank;
- infer the proposition supported by evidence;
- modify native source artifacts;
- replace the portable `pk/v1` record schema;
- introduce AI/LLM inference;
- introduce a daemon, service, web UI, or remote authoring protocol; or
- require projects that do not need S2 enrichment to use capture tooling.

## Next validation step

The next activity is **DF-003**.

DF-003 must use CA-1 to create a real S2 bundle rather than hand-authoring structural JSON. It should measure whether CA-1 materially reduces capture effort and what semantic or interaction friction remains once UUIDs, paths, source-state hashes, envelopes, and ordinary timestamps are no longer manually managed.
