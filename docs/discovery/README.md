# Discovery

Discovery turns the inception problem statement into evidence-backed requirements.

This phase resists designing the system too early. Its job is to understand the information we need to preserve, the workflows in which that information appears, the failure modes of current practices, what existing approaches already solve, and the minimum capabilities required to improve what remains.

## Current status

The **first discovery sequence is complete enough to seed formal requirements**.

Five major passes are now represented in the repository:

1. real-information corpus and initial cross-case analysis;
2. counterexamples / knowledge-evolution stress testing;
3. existing-approach research and composition analysis;
4. evidence-to-capability derivation; and
5. promotion of evidence-backed capabilities into the formal requirements specification under [`../requirements/`](../requirements/).

Discovery remains open. The coverage matrix still identifies under-evidenced areas that must not be silently universalized during domain modeling or architecture.

## Discovery baseline

### Evidence corpus

The corpus under [`corpus/`](corpus/) contains `PKC-0001` through `PKC-0011`, spanning high-complexity Monad cases, Project Knowledge dogfood cases, semantic-identity relocation, and a lower-ceremony counterexample where ordinary ordered documents and Git are largely sufficient.

Key outputs:

- [`corpus/README.md`](corpus/README.md)
- [`corpus/case-template.md`](corpus/case-template.md)
- [`corpus/coverage-matrix.md`](corpus/coverage-matrix.md)
- `corpus/cases/PKC-*`

Important remaining evidence gaps include:

- competing simultaneous hypotheses;
- an actual superseded-decision chain;
- experiment-driven decisions;
- terminology evolution;
- narrative/learning projections;
- capture-overhead failures; and
- genuine multi-person disagreement.

### User jobs and failure modes

The current baseline contains:

- `UJ-001` through `UJ-014` under [`user-jobs/`](user-jobs/); and
- `FM-001` through `FM-016` under [`failure-modes/`](failure-modes/).

These are sufficient to seed requirements but remain open to refinement as new cases expose genuinely new recovery work or root causes.

### Existing approaches

The track under [`existing-approaches/`](existing-approaches/) evaluates Git, docs-as-code, ADRs, issue/work tracking, linked notes, event sourcing, temporal/bitemporal models, W3C PROV, graph representation, and hybrid retrieval.

Its strongest conclusion is compositional:

> Project Knowledge is justified by the residual cross-tool project-memory gap, not by a need to replace mature native engineering systems.

Key outputs include:

- [`existing-approaches/evaluation-method.md`](existing-approaches/evaluation-method.md)
- [`existing-approaches/approach-capability-matrix.md`](existing-approaches/approach-capability-matrix.md)
- [`existing-approaches/composition-findings.md`](existing-approaches/composition-findings.md)

### Evidence-to-capability derivation

The track under [`capabilities/`](capabilities/) derives twenty candidate capabilities through the explicit chain:

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

Candidates are classified as **REUSE**, **INTEGRATE**, **EXTEND**, or **NEW** so that Project Knowledge does not reinvent mature behavior unnecessarily.

Key outputs include:

- [`capabilities/derivation-method.md`](capabilities/derivation-method.md)
- [`capabilities/candidate-capabilities.md`](capabilities/candidate-capabilities.md)
- [`capabilities/trace-matrix.md`](capabilities/trace-matrix.md)
- [`capabilities/emerging-capability-shape.md`](capabilities/emerging-capability-shape.md)
- [`capabilities/promotion-boundary.md`](capabilities/promotion-boundary.md)

## Requirements handoff

The promoted capabilities have now seeded six formal requirement families:

- **RF-1 — Native interoperability and progressive adoption**
- **RF-2 — Semantic identity, representation, and relationships**
- **RF-3 — Authority and current truth**
- **RF-4 — Provenance, time, and context**
- **RF-5 — Evidence and epistemic evolution**
- **RF-6 — Retrieval, impact, and explanation**

The formal specification under [`../requirements/`](../requirements/) contains:

- functional requirements;
- quality attributes;
- cross-cutting constraints;
- explicit non-requirements;
- requirements traceability; and
- acceptance principles.

Every promoted functional requirement retains traceability back through capabilities to discovery evidence.

## Discovery principles that remain binding

### Credit existing systems first

Reuse mature mechanisms and semantics before inventing Project Knowledge-specific replacements.

### Preserve progressive structure

A small project must remain simple when ordinary files, links, ordering, search, and Git already solve its recovery problem.

### Keep uncertainty visible

Under-evidenced concepts remain under-evidenced. Requirements must not turn open questions into universal ontology by wording alone.

### Preserve negative evidence

Cases showing that richer structure is unnecessary are as important as cases showing that richer structure is needed.

### Keep architecture deferred

Discovery and requirements do not select database, graph, event-store, temporal-store, search, application-framework, AI, or deployment architecture.

## Key validation questions

> Can the emerging model represent the messy reality of engineering work without forcing information into unnatural categories or imposing more cognitive burden than it removes?

> Can a project remain simple when simple files, links, ordering, search, and Git are already enough?

> After composing existing mechanisms honestly, what behavior is still missing strongly enough to justify Project Knowledge-specific software?

> Can every mandatory requirement explain which recovery problem justifies its capture and maintenance cost?

## What discovery does next

Discovery is no longer the critical path for the next project phase. Domain modeling may begin after the formal requirements PR is reviewed and merged.

Discovery should continue in parallel when:

1. domain modeling encounters an under-evidenced concept;
2. architecture requires a narrower existing-approach comparison;
3. a new real project case falsifies or refines a requirement; or
4. the coverage matrix identifies evidence needed before a scoped capability can become universal.

This preserves discovery as an evidence discipline without allowing it to become an endless prerequisite to engineering progress.
