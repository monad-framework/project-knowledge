# Evidence and Scope

## Evidence basis

The capture/authoring capability is promoted from repeated real dogfood evidence rather than from a general desire for a friendlier CLI.

### DF-001

Recovery shape:

```text
historical decision status
        ↓
current decision status
        ↓
scoped authority
        ↓
claim-relative evidence
```

The semantic model passed without production changes, but the narrow thread required 10 manually authored records.

### DF-002

Recovery shape:

```text
unresolved question
        ↓
alternatives considered
        ↓
selected answer
        ↓
implementation evidence
```

The semantic model again passed without production changes, but required 14 additional manually authored records.

The repeated burden occurred across different semantic shapes, which is sufficient to promote structural authoring assistance into detailed design.

## Requirements trace

This design primarily advances:

- **FR-105 — Support progressive enrichment**: richer structure should be introduced only where useful.
- **FR-107 — Make enrichment reversible or non-destructive**: capture writes S2 without rewriting native artifacts.
- **FR-201 / FR-203**: semantic identity and Representation bindings remain explicit when continuity matters.
- **FR-301 / FR-302**: authority remains scoped and its basis remains explicit.
- **FR-401 and related provenance requirements**: capture may scaffold provenance structure without fabricating provenance meaning.
- **QA-001 — Progressive adoption and low capture burden**: capture effort should be proportional to recovery value.
- **QA-004 — Traceability**: consequential semantic output must expose its authored/observed basis.
- **QA-005 — Explainability over silent inference**: suggestions and inference must not masquerade as authored fact.
- **QA-008 — Source fidelity**: native artifacts are observed, not rewritten.
- **QA-009 — Authority safety**: convenience tooling cannot silently promote authority.
- **QA-013 — Incremental maintainability**: adding one semantic thread should not require unrelated records to be rewritten.

## In scope for the first authoring increment

The first increment must be able to scaffold all existing executable `pk/v1` record kinds used by DF-001 and DF-002:

- Subject;
- Representation;
- Claim;
- Assertion;
- Authority;
- Relationship;
- Activity; and
- Evidence Evaluation.

Context may be selectable when an existing Context is already present. Creating rich Context records can remain a generic advanced step unless another dogfood case demonstrates a high-value guided workflow.

The first increment must support both:

1. an interactive guided flow for humans; and
2. a compact non-interactive authoring-intent document for scripts/tests.

Both surfaces compile into the same Capture Plan and then the same S2 records.

## Out of scope

The following remain outside this increment:

- new semantic record kinds;
- ontology or controlled-vocabulary expansion;
- semantic similarity matching;
- automatic Subject merging;
- automatic authority policy derivation;
- automatic interpretation of GitHub PR state as decision semantics;
- generic natural-language-to-S2 extraction;
- AI-generated semantic decisions;
- remote collaborative transactions;
- real-time ingestion;
- bulk migration of an existing repository into S2; and
- automatic capture of every change.

## Progressive-adoption rule

Capture is optional infrastructure for projects or project areas that need stronger recovery semantics.

A project with only ordinary Markdown and Git remains valid. Running `pk status` or `pk validate` must not require authoring intent, capture plans, or generated S2 records.

## Promotion boundary

This design authorizes implementation of **mechanical authoring assistance** only.

It does not authorize changing the domain model merely to make the authoring UI simpler. If an existing semantic decision feels burdensome because it genuinely represents distinct meaning, the tool should help the human express that meaning rather than erase the distinction.
