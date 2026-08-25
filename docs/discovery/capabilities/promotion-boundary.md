# Capability Promotion Boundary

This document decides which discovery capabilities are mature enough to seed formal requirements and which must remain scoped, optional, or on hold.

Promotion does not mean implementation authorization. It means the evidence is sufficient to write technology-neutral requirements in the next phase.

## Group A — Requirements-ready core constraints and semantics

The following capabilities have High confidence and clear evidence traces. They are ready to seed requirements, subject to progressive-structure scope.

### `CAP-001` — Preserve native artifact authority and history

Requirement direction:

> Project Knowledge must interoperate with native engineering artifacts and preserve their identity/history/authority rather than requiring destructive migration into a new proprietary source of truth.

### `CAP-002` — Federate heterogeneous native artifacts

Requirement direction:

> Project Knowledge must be able to reference and compose project memory across heterogeneous native artifact systems.

### `CAP-003` — Stable semantic identity across representations

Requirement direction:

> Where cross-representation continuity is required, Project Knowledge must support stable semantic identity distinct from native storage/tool identity.

### `CAP-004` — Representation roles and bindings

Requirement direction:

> Project Knowledge must be able to distinguish and relate canonical, projected, generated, historical, evidentiary, external, and other materially distinct representations without treating them as independent corroborating truth.

The final vocabulary remains a domain-modeling question.

### `CAP-005` — Claim/property-scoped authority

Requirement direction:

> Project Knowledge must represent authority at a scope fine enough to explain which source governs a claim/property/role and why.

### `CAP-006` — Structured provenance

Requirement direction:

> Project Knowledge must support structured provenance adequate to reconstruct origin and derivation, preferentially extending mature provenance semantics rather than inventing incompatible primitives.

### `CAP-007` — Valid versus recorded time

Requirement direction:

> Project Knowledge must support the distinction between effective/valid time and recorded/system time where that distinction affects recovery or correctness.

This must remain optional when Git history alone is sufficient.

### `CAP-008` — Material observation/execution context

Requirement direction:

> Project Knowledge must preserve the contextual dimensions necessary to interpret or reconstruct knowledge whose meaning depends on repository/execution/environment context.

### `CAP-010` — Claim-relative evidence

Requirement direction:

> Project Knowledge must be able to associate evidence with the proposition, source state, method, and relevant context it supports, rather than treating evidence freshness as generic artifact freshness.

### `CAP-011` — Derivation lineage and projection freshness

Requirement direction:

> Derived project-memory views must retain lineage to their relevant inputs and expose enough information to determine or explain whether the derived information is current.

### `CAP-012` — Typed relationships and impact traversal

Requirement direction:

> Project Knowledge must support project-wide traversal of important relationships across native artifacts while allowing ordinary untyped links where stronger semantics are unnecessary.

### `CAP-014` — Current and historical views

Requirement direction:

> Users must be able to recover current authoritative state and relevant historical state without confusing one for the other.

### `CAP-015` — Contradiction diagnosis

Requirement direction:

> When project-memory statements disagree, the system should provide enough semantic/contextual information to distinguish stale projection, historical difference, context difference, authority difference, provenance error, semantic mismatch, or unresolved disagreement where determinable.

It must not fabricate reconciliation.

### `CAP-016` — Hybrid retrieval over project-memory semantics

Requirement direction:

> Project Knowledge must make project memory discoverable using mature retrieval mechanisms while exposing semantic metadata as filters/context/citations rather than allowing retrieval relevance to determine authority.

### `CAP-018` — Progressive formalization and selective retention

Requirement direction:

> Rich project-memory semantics must be progressively adoptable and should not be mandatory where simpler native artifacts, links, Git, and search already satisfy the recovery need.

This is a cross-cutting requirement on all later requirements.

### `CAP-020` — Preserve correction without erasure

Requirement direction:

> Project Knowledge must support correction/refinement in a way that makes current truth clear while preserving the prior belief, its historical context, and the basis for change where retained.

## Group B — Requirements-ready only as scoped/optional behavior

These capabilities have useful evidence but should not drive a universal mandatory model.

### `CAP-009` — Epistemic state and knowledge evolution

**Confidence:** Medium

Promote only as a scoped capability:

> Project Knowledge should be able to represent epistemic roles/transitions when they materially improve recovery.

Do **not** yet specify a universal epistemic state machine.

Additional evidence needed:

- simultaneous competing hypotheses;
- explicit rejection;
- an actual supersession chain;
- hypothesis-to-decision/requirement transition.

### `CAP-013` — Source/admission classification

**Confidence:** Medium

Promote only as a safety/integration requirement:

> Ingestion/discovery must not assume every discoverable or parsable artifact is canonical project knowledge, and must support project policy for excluding or classifying incidental/transient material.

Do not yet freeze a universal classification taxonomy.

### `CAP-017` — Traceable authored narrative

**Confidence:** Medium

Promote only as an optional human-facing capability:

> The system should support authored learning/narrative views whose statements remain traceable to project-memory sources.

Do not require every project to maintain an educational narrative.

### `CAP-019` — Causal/recovery path reconstruction

**Confidence:** Medium

Promote only as an explanatory capability:

> The system should support explicit reasoning/causal paths when such relationships have been asserted or can be safely derived from stronger evidence.

Do not require automatic causal inference.

## Group C — Delegated behavior, not Project Knowledge implementation scope

The following mature mechanisms are required by the overall project-memory experience but should ordinarily remain external/native:

- immutable repository history → Git or equivalent VCS;
- decision rationale/lifecycle → ADR practice or equivalent decision records;
- authored documentation → Markdown/docs-as-code/wiki systems;
- work hierarchy/status → native issue/work systems;
- generic graph traversal → suitable graph/query representation;
- generic provenance vocabulary → W3C PROV-compatible semantics;
- generic temporal storage/query → mature temporal mechanisms when infrastructure is warranted;
- lexical/semantic/hybrid retrieval → existing search/indexing mechanisms;
- primary source execution/testing → native build/test/CI systems.

Project Knowledge may integrate these capabilities but should not implement replacements unless later requirements demonstrate a concrete gap.

## Group D — Hold for additional evidence

The following ideas remain explicitly outside the requirements-ready boundary:

1. universal decision/object supersession semantics;
2. a fixed question → hypothesis → experiment → decision state machine;
3. automatic terminology evolution tracking;
4. first-class experiment-management workflow;
5. universal semantic identity for every artifact;
6. mandatory bitemporal metadata for every assertion;
7. automatic causal inference;
8. automatic authoritative conflict resolution;
9. mandatory capture of chats/conversations;
10. mandatory event sourcing;
11. mandatory graph storage; and
12. mandatory AI/RAG participation.

## Proposed requirements families

The requirements phase should group promoted capabilities into a smaller number of requirement families rather than writing one feature specification per capability.

### RF-1 — Native interoperability and progressive adoption

Derived primarily from:

- CAP-001
- CAP-002
- CAP-013
- CAP-018

### RF-2 — Semantic identity, representation, and relationships

Derived primarily from:

- CAP-003
- CAP-004
- CAP-012

### RF-3 — Authority and current truth

Derived primarily from:

- CAP-005
- CAP-014
- CAP-015
- CAP-020

### RF-4 — Provenance, time, and context

Derived primarily from:

- CAP-006
- CAP-007
- CAP-008
- CAP-011

### RF-5 — Evidence and epistemic evolution

Derived primarily from:

- CAP-009 (scoped)
- CAP-010
- CAP-020

### RF-6 — Retrieval, impact, and explanation

Derived primarily from:

- CAP-012
- CAP-015
- CAP-016
- CAP-017 (optional)
- CAP-019 (scoped)

## Discovery exit implication

After this pass, the project will have completed the minimum conceptual sequence needed to begin formal requirements derivation:

```text
problem / vision
    ↓
real corpus
    ↓
user jobs + failure modes
    ↓
counterexamples
    ↓
existing-approach analysis
    ↓
evidence-derived capability boundary
    ↓
requirements  ← next
```

Discovery is not globally finished: the coverage matrix still identifies missing cases. But requirements can now begin without pretending those under-evidenced areas are settled.
