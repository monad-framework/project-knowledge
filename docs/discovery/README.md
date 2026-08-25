# Discovery

Discovery turns the inception problem statement into evidence-backed requirements.

This phase should resist the temptation to design the system too early. Its job is to understand the information we need to preserve, the workflows in which that information appears, the failure modes of current practices, what existing approaches already solve, and the minimum capabilities required to improve what remains.

## Current status

Discovery is active.

Three major passes are now represented in the repository:

1. real-information corpus and initial cross-case analysis;
2. counterexamples / knowledge-evolution stress testing; and
3. existing-approach research and composition analysis.

The next major pass is **evidence-to-capability derivation**.

### First evidence pass

The first evidence-driven pass established:

- [`corpus/README.md`](corpus/README.md) — corpus method and case index;
- [`corpus/case-template.md`](corpus/case-template.md) — neutral case-study structure;
- `corpus/cases/PKC-0001` through `PKC-0007` — seed cases from real Monad engineering history;
- [`observations/initial-corpus-observations.md`](observations/initial-corpus-observations.md) — cross-case findings;
- [`user-jobs/initial-user-jobs.md`](user-jobs/initial-user-jobs.md) — context-recovery jobs exposed by the cases;
- [`failure-modes/initial-failure-mode-catalog.md`](failure-modes/initial-failure-mode-catalog.md) — failure patterns exposed by the cases; and
- [`open-questions.md`](open-questions.md) — unresolved questions and evidence targets.

### Second evidence pass

The second pass deliberately introduced dogfood cases and counterexamples rather than adding only more Monad governance cases:

- `PKC-0008` — Project Knowledge's own root-status summary drift;
- `PKC-0009` — a working hypothesis refined by evidence without being wholly rejected;
- `PKC-0010` — an ADR retaining semantic identity across repository relocation;
- `PKC-0011` — a lower-ceremony frontend project where ordered authoritative documents and Git provide a useful project-memory model;
- [`observations/second-pass-observations.md`](observations/second-pass-observations.md) — findings about assertion-scoped freshness, epistemic refinement, relocation, progressive structure, ordering, selective capture, and negative evidence; and
- [`corpus/coverage-matrix.md`](corpus/coverage-matrix.md) — explicit support levels and missing case types.

The corpus is no longer Monad-only, but it remains too narrow to establish a universal domain model. The coverage matrix remains active and should continue to expose under-evidenced concepts rather than allowing attractive ideas to become assumptions.

### Existing-approaches pass

The existing-approaches track is under [`existing-approaches/`](existing-approaches/).

It evaluates:

- Git and repository history;
- docs-as-code and wikis;
- Architecture Decision Records;
- issue/work tracking;
- linked-note / PKM systems;
- event sourcing;
- temporal and bitemporal data models;
- W3C PROV provenance semantics;
- RDF-style graph representation; and
- lexical, semantic, and hybrid search / RAG-style retrieval.

Key outputs include:

- [`existing-approaches/evaluation-method.md`](existing-approaches/evaluation-method.md);
- [`existing-approaches/source-notes.md`](existing-approaches/source-notes.md);
- focused approach assessments under `existing-approaches/approaches/`;
- [`existing-approaches/approach-capability-matrix.md`](existing-approaches/approach-capability-matrix.md); and
- [`existing-approaches/composition-findings.md`](existing-approaches/composition-findings.md).

The strongest result is compositional: existing mechanisms already solve much of the problem well. Project Knowledge is increasingly justified only by the **residual cross-tool project-memory gap** rather than by a need to replace Git, docs, ADRs, issue trackers, provenance standards, temporal models, graphs, or search.

## Discovery tracks

### 1. Real information corpus

Collect representative project information from actual engineering work. The corpus should include both difficult cases and counterexamples where ordinary repository practices are already sufficient.

Important case types include:

- unresolved questions;
- competing hypotheses;
- decisions with alternatives;
- superseded decisions;
- requirements and constraints;
- experiments and evidence;
- implementation artifacts;
- failures and corrections;
- terminology changes;
- work and milestone context;
- narrative explanations; and
- information that legitimately belongs to several views.

The coverage matrix should be used to distinguish observed evidence from attractive but untested concepts.

### 2. User jobs and recovery scenarios

Describe what a person is trying to accomplish when they need project memory. Examples include returning after a long absence, understanding why a component exists, reviewing a changed decision, onboarding, investigating a regression, and reconstructing a milestone.

### 3. Existing approaches

Study how current tools, standards, models, and practices address portions of the problem.

This track's guiding question is:

> What should Project Knowledge reuse, integrate, or extend before inventing anything new?

The initial pass is complete enough to support capability derivation, but research should remain open where later capabilities expose a more specific precedent.

### 4. Failure modes

Record concrete ways project knowledge becomes difficult to use: duplication, drift, stale authority, lost rationale, weak provenance, context fragmentation, hierarchy mismatch, unbounded capture, excessive structure, or retrieval without understanding.

### 5. Evidence-to-capability derivation — next

Candidate capabilities should now be derived using an explicit trace:

```text
Corpus case(s)
    ↓
Failure mode(s)
    ↓
User job(s)
    ↓
Existing approach coverage
    ↓
Residual gap
    ↓
Candidate capability
```

Each candidate should be classified as:

- **REUSE** — an existing mechanism already solves the need;
- **INTEGRATE** — Project Knowledge needs to connect an existing mechanism into a coherent project-memory view;
- **EXTEND** — mature semantics exist but require engineering-specific additions; or
- **NEW** — the evidence supports behavior not adequately supplied by the surveyed mechanisms.

This classification is intended to prevent unnecessary invention.

### 6. Requirements derivation

Only after candidate capabilities have traceable evidence should they be promoted into functional requirements, quality attributes, constraints, or explicit non-requirements.

## Key validation questions

> Can the emerging model represent the messy reality of engineering work without forcing information into unnatural categories or imposing more cognitive burden than it removes?

> Can a project remain simple when simple files, links, ordering, search, and Git are already enough?

> After composing existing mechanisms honestly, what behavior is still missing strongly enough to justify Project Knowledge-specific software?

## Discovery outputs

Expected outputs include:

- representative corpus and case studies;
- corpus coverage and counterexamples;
- user jobs and use cases;
- existing-approach analysis;
- failure-mode catalog;
- evidence-to-capability trace matrix;
- candidate capability model;
- functional requirements;
- quality attributes;
- constraints;
- unresolved questions; and
- candidate domain concepts for the subsequent domain-modeling phase.

No implementation architecture is an expected discovery output unless a requirement necessarily constrains it.

## Exit discipline

Discovery should not be considered complete merely because a plausible ontology or architecture has emerged. Before requirements are treated as stable, the project should have:

1. tested findings against cases outside the initial Monad corpus;
2. investigated existing approaches that may already solve portions of the problem;
3. connected important user jobs to concrete failure modes and evidence;
4. distinguished general engineering-knowledge needs from Monad-specific governance conventions;
5. preserved negative evidence showing where richer structure is unnecessary;
6. classified proposed behavior as reuse, integration, extension, or genuinely new capability; and
7. demonstrated that proposed capabilities reduce rather than increase context-management burden.
