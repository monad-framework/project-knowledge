# Discovery

Discovery turns the inception problem statement into evidence-backed requirements.

This phase should resist the temptation to design the system too early. Its job is to understand the information we need to preserve, the workflows in which that information appears, the failure modes of current practices, what existing approaches already solve, and the minimum capabilities required to improve what remains.

## Current status

Discovery is active and has reached the **requirements boundary**.

Four major passes are now represented in the repository:

1. real-information corpus and initial cross-case analysis;
2. counterexamples / knowledge-evolution stress testing;
3. existing-approach research and composition analysis; and
4. evidence-to-capability derivation.

The next major pass is **formal requirements derivation** from the promoted capability boundary.

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

The strongest result is compositional: existing mechanisms already solve much of the problem well. Project Knowledge is justified by the **residual cross-tool project-memory gap**, not by a need to replace Git, docs, ADRs, issue trackers, provenance standards, temporal models, graphs, or search.

### Evidence-to-capability pass

The capability derivation track is under [`capabilities/`](capabilities/).

It establishes an explicit evidence chain:

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

Key outputs include:

- [`capabilities/derivation-method.md`](capabilities/derivation-method.md) — admission and anti-overmodeling rules;
- [`capabilities/candidate-capabilities.md`](capabilities/candidate-capabilities.md) — twenty evidence-derived candidates classified as REUSE, INTEGRATE, EXTEND, or NEW;
- [`capabilities/trace-matrix.md`](capabilities/trace-matrix.md) — compact case/failure/job/approach traceability;
- [`capabilities/emerging-capability-shape.md`](capabilities/emerging-capability-shape.md) — four-layer capability shape and candidate semantic-kernel questions; and
- [`capabilities/promotion-boundary.md`](capabilities/promotion-boundary.md) — requirements-ready, scoped/optional, delegated, and held concepts.

The capability pass demonstrates that the twenty candidate capabilities should not become twenty subsystems. They cluster into:

1. native engineering systems that should remain authoritative for their own data;
2. cross-artifact integration;
3. a smaller project-memory semantic layer; and
4. human recovery views over that shared memory.

`CAP-018` progressive formalization applies across all four layers.

## Discovery tracks

### 1. Real information corpus

Collect representative project information from actual engineering work. The corpus should include both difficult cases and counterexamples where ordinary repository practices are already sufficient.

Important under-evidenced case types remain:

- competing simultaneous hypotheses;
- an actual superseded-decision chain;
- experiment-driven decisions;
- terminology evolution;
- narrative/learning projections;
- capture-overhead failures; and
- genuine multi-person disagreement.

The coverage matrix should continue to distinguish observed evidence from attractive but untested concepts.

### 2. User jobs and recovery scenarios

The current job catalog (`UJ-001` through `UJ-014`) is sufficient to seed requirements, but should evolve when new corpus cases expose materially new recovery work.

### 3. Existing approaches

The initial broad research pass is complete enough for requirements derivation.

Research remains open when a later requirement raises a narrower precedent question. The guiding rule remains:

> Reuse mature semantics and mechanisms before inventing Project Knowledge-specific equivalents.

### 4. Failure modes

The current failure catalog (`FM-001` through `FM-016`) is sufficient to seed requirements. Later cases should refine/root-cause the catalog rather than simply proliferate surface symptoms.

### 5. Evidence-to-capability derivation

The first pass is complete enough to establish a promotion boundary.

Candidates are classified as:

- **REUSE** — mature existing behavior should remain delegated;
- **INTEGRATE** — Project Knowledge connects mature behavior into coherent project memory;
- **EXTEND** — mature semantics form the base but engineering-specific behavior is needed; or
- **NEW** — the corpus supports genuinely missing Project Knowledge behavior.

### 6. Requirements derivation — next

Formal requirements should now be derived from the promoted capability families in [`capabilities/promotion-boundary.md`](capabilities/promotion-boundary.md):

- **RF-1 — Native interoperability and progressive adoption**
- **RF-2 — Semantic identity, representation, and relationships**
- **RF-3 — Authority and current truth**
- **RF-4 — Provenance, time, and context**
- **RF-5 — Evidence and epistemic evolution**
- **RF-6 — Retrieval, impact, and explanation**

Requirements should remain technology-neutral and trace back to the capability registry, user jobs, failure modes, and corpus.

## Key validation questions

> Can the emerging model represent the messy reality of engineering work without forcing information into unnatural categories or imposing more cognitive burden than it removes?

> Can a project remain simple when simple files, links, ordering, search, and Git are already enough?

> After composing existing mechanisms honestly, what behavior is still missing strongly enough to justify Project Knowledge-specific software?

> Can every mandatory requirement explain which recovery problem justifies its capture and maintenance cost?

## Discovery outputs

Current outputs include:

- representative corpus and case studies;
- corpus coverage and counterexamples;
- user jobs and use cases;
- existing-approach analysis;
- failure-mode catalog;
- evidence-to-capability trace matrix;
- candidate capability registry;
- capability promotion boundary;
- unresolved questions; and
- candidate domain questions for the subsequent domain-modeling phase.

The next outputs are formal functional requirements, quality attributes, constraints, and explicit non-requirements.

No implementation architecture is an expected Discovery output unless a requirement necessarily constrains it.

## Exit discipline

Discovery should not be considered globally complete merely because requirements can now begin. Before requirements are treated as stable, the project should continue to ensure that it has:

1. tested important findings against cases outside the initial Monad corpus;
2. credited existing approaches before inventing new mechanisms;
3. connected mandatory behavior to user jobs, failure modes, and evidence;
4. distinguished general engineering-knowledge needs from Monad-specific governance conventions;
5. preserved negative evidence showing where richer structure is unnecessary;
6. classified proposed behavior as reuse, integration, extension, or genuinely new capability;
7. scoped medium-confidence capabilities rather than universalizing them; and
8. demonstrated that proposed requirements reduce rather than increase context-management burden.
