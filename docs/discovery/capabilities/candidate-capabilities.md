# Candidate Capability Registry

This registry records capabilities justified by the current discovery evidence. It is not yet a product-requirements document.

## Summary

| ID | Candidate capability | Disposition | Responsibility | Confidence |
| --- | --- | --- | --- | --- |
| CAP-001 | Preserve native artifact authority and history | REUSE | Foundation dependency | High |
| CAP-002 | Federate heterogeneous native artifacts | INTEGRATE | Project-memory integration | High |
| CAP-003 | Stable semantic identity across representations | NEW | Project-memory semantics | High |
| CAP-004 | Representation roles and equivalence/derivation bindings | EXTEND | Project-memory semantics | High |
| CAP-005 | Claim/property-scoped authority | NEW | Project-memory semantics | High |
| CAP-006 | Structured provenance using a mature base vocabulary | EXTEND | Project-memory semantics | High |
| CAP-007 | Valid/effective time distinct from recorded/system time | EXTEND | Project-memory semantics | High |
| CAP-008 | Material observation/execution context | EXTEND | Project-memory semantics | High |
| CAP-009 | Epistemic state and knowledge evolution | NEW | Project-memory semantics | Medium |
| CAP-010 | Claim-relative evidence and validity | NEW | Project-memory semantics | High |
| CAP-011 | Derivation lineage and projection freshness | EXTEND | Project-memory semantics | High |
| CAP-012 | Typed cross-artifact relationships and impact traversal | INTEGRATE | Project-memory integration | High |
| CAP-013 | Source/admission classification | EXTEND | Project-memory integration | Medium |
| CAP-014 | Current-state and historical project-memory views | INTEGRATE | Human-facing projection | High |
| CAP-015 | Contradiction diagnosis | NEW | Human-facing projection | High |
| CAP-016 | Hybrid retrieval over project-memory semantics | INTEGRATE | Human-facing projection | High |
| CAP-017 | Traceable authored narrative / learning views | INTEGRATE | Human-facing projection | Medium |
| CAP-018 | Progressive formalization and selective retention | NEW | Cross-cutting constraint | High |
| CAP-019 | Causal/recovery path reconstruction | EXTEND | Human-facing projection | Medium |
| CAP-020 | Preserve correction without erasing prior belief | EXTEND | Human-facing projection | High |

---

## CAP-001 — Preserve native artifact authority and history

**Disposition:** REUSE  
**Responsibility:** Foundation dependency  
**Confidence:** High

### Evidence trace

- Cases: `PKC-0001`, `PKC-0008`, `PKC-0010`, `PKC-0011`
- Failure modes: `FM-002`, `FM-004`, `FM-016`
- User jobs: `UJ-002`, `UJ-003`, `UJ-014`
- Existing coverage: Git is strong for immutable repository history; ADRs are strong for decision rationale/lifecycle; docs-as-code is strong for authored technical narrative; issue trackers are strong for their local workflow state.

### Residual gap

There is no justification for replacing those native records merely to create project memory.

### Candidate capability

Project Knowledge must preserve links to and respect the history/authority of native source artifacts rather than requiring them to be rewritten into a new canonical store.

### Counterpressure

This capability explicitly forbids reimplementing Git history, ADR lifecycle, or issue-tracker workflow merely for conceptual purity.

---

## CAP-002 — Federate heterogeneous native artifacts

**Disposition:** INTEGRATE  
**Responsibility:** Project-memory integration  
**Confidence:** High

### Evidence trace

- Cases: `PKC-0001`, `PKC-0002`, `PKC-0003`, `PKC-0011`
- Failure modes: `FM-003`, `FM-014`
- User jobs: `UJ-003`, `UJ-006`, `UJ-008`, `UJ-011`, `UJ-013`
- Existing coverage: individual systems expose files, issues, ADRs, work records, commits, and search indexes well within their own boundaries; no single surveyed mechanism supplies a coherent project view across all of them by default.

### Residual gap

A user must manually assemble context across repositories, files, Git history, issue trackers, execution records, and generated artifacts.

### Candidate capability

Project Knowledge can reference and query heterogeneous native artifacts as members of one project memory without requiring wholesale migration into one proprietary representation.

---

## CAP-003 — Stable semantic identity across representations

**Disposition:** NEW  
**Responsibility:** Project-memory semantics  
**Confidence:** High

### Evidence trace

- Cases: `PKC-0001`, `PKC-0010`
- Failure modes: `FM-004`, `FM-012`, `FM-014`
- User jobs: `UJ-001`, `UJ-003`, `UJ-006`, `UJ-013`
- Existing coverage: Git, issue trackers, ADR IDs, URLs, and graph systems provide useful native identifiers; graph models can preserve global identity if rules already exist.

### Residual gap

The project still needs to express that several native identities concern one durable engineering concept, while preserving each native identity and not assigning stable semantic IDs to everything indiscriminately.

### Candidate capability

Project Knowledge can assign or recognize a stable semantic identity for objects whose continuity across files, moves, projections, revisions, or tools matters, and bind native identities to that semantic identity.

### Counterpressure

Ephemeral notes and simple artifacts may remain identified only by their native locator.

---

## CAP-004 — Representation roles and equivalence/derivation bindings

**Disposition:** EXTEND  
**Responsibility:** Project-memory semantics  
**Confidence:** High

### Evidence trace

- Cases: `PKC-0001`, `PKC-0002`, `PKC-0004`, `PKC-0008`, `PKC-0010`
- Failure modes: `FM-001`, `FM-006`, `FM-015`
- User jobs: `UJ-001`, `UJ-006`, `UJ-010`
- Existing coverage: W3C PROV supports alternate/specialization/derivation relations; ADR and tool conventions expose some local representation roles.

### Residual gap

The engineering project needs to distinguish representations that are canonical sources, projections, revisions, generated forms, evidence, summaries, coordination copies, or historical views of the same or related semantic object.

### Candidate capability

Project Knowledge can state the role a representation plays and how it relates to a semantic object or another representation, using mature provenance/graph relations where they fit and engineering-specific roles only where required.

---

## CAP-005 — Claim/property-scoped authority

**Disposition:** NEW  
**Responsibility:** Project-memory semantics  
**Confidence:** High

### Evidence trace

- Cases: `PKC-0001`, `PKC-0002`, `PKC-0003`, `PKC-0008`
- Failure modes: `FM-001`, `FM-002`, `FM-011`, `FM-015`
- User jobs: `UJ-001`, `UJ-006`, `UJ-009`, `UJ-013`
- Existing coverage: ADRs and issue trackers can be authoritative within local domains; surveyed approaches do not define cross-project authority at property/claim scope.

### Residual gap

A representation can legitimately own one property while being non-authoritative for another. Artifact-wide `authoritative: true/false` is too coarse.

### Candidate capability

Project Knowledge can represent authority for a claim, property, role, or concern; explain the source/policy of that authority; and use the scope when reconciling conflicting views.

---

## CAP-006 — Structured provenance using a mature base vocabulary

**Disposition:** EXTEND  
**Responsibility:** Project-memory semantics  
**Confidence:** High

### Evidence trace

- Cases: `PKC-0003`, `PKC-0004`, `PKC-0006`, `PKC-0007`
- Failure modes: `FM-006`, `FM-008`, `FM-009`, `FM-010`
- User jobs: `UJ-005`, `UJ-007`, `UJ-010`, `UJ-014`
- Existing coverage: W3C PROV strongly covers entities, activities, agents, generation, usage, derivation, attribution, association, revision, primary source, specialization, and qualified relations.

### Residual gap

Engineering cases still require repository/execution/evidence semantics that generic provenance alone does not define.

### Candidate capability

Project Knowledge should reuse a PROV-compatible conceptual base and extend it only for engineering-specific provenance such as resolved repository state, execution context, validation method, or evidence binding.

---

## CAP-007 — Valid/effective time distinct from recorded/system time

**Disposition:** EXTEND  
**Responsibility:** Project-memory semantics  
**Confidence:** High

### Evidence trace

- Cases: `PKC-0002`, `PKC-0005`, `PKC-0007`, `PKC-0008`, `PKC-0009`
- Failure modes: `FM-002`, `FM-007`, `FM-009`, `FM-016`
- User jobs: `UJ-001`, `UJ-002`, `UJ-004`, `UJ-014`
- Existing coverage: bitemporal models maturely distinguish valid/effective time from system/recorded time; Git provides commit history but not explicit business/project valid time.

### Residual gap

Project memory sometimes needs to state both when an assertion was considered true/effective and when the system learned or recorded it.

### Candidate capability

Where material, Project Knowledge can attach valid/effective intervals and recorded/system time to assertions or relationships without requiring explicit temporal metadata for every artifact.

---

## CAP-008 — Material observation/execution context

**Disposition:** EXTEND  
**Responsibility:** Project-memory semantics  
**Confidence:** High

### Evidence trace

- Cases: `PKC-0003`, `PKC-0004`, `PKC-0006`, `PKC-0007`
- Failure modes: `FM-003`, `FM-009`, `FM-010`, `FM-012`
- User jobs: `UJ-005`, `UJ-007`, `UJ-008`, `UJ-009`
- Existing coverage: Git commits, execution systems, CI, and provenance systems each expose pieces of context.

### Residual gap

Repository, branch, commit, checkout, worktree, host, tool version, environment, and lifecycle context can materially change interpretation, but generic provenance does not determine which engineering context is semantically relevant.

### Candidate capability

Project Knowledge can capture and reconstruct only the context dimensions material to interpretation, distinguishing immutable/reconstructable state identity from ephemeral location metadata.

---

## CAP-009 — Epistemic state and knowledge evolution

**Disposition:** NEW  
**Responsibility:** Project-memory semantics  
**Confidence:** Medium

### Evidence trace

- Cases: `PKC-0009`; broader project discovery practice provides supporting examples
- Failure modes: `FM-002`, `FM-013`, `FM-016`
- User jobs: `UJ-002`, `UJ-004`, `UJ-009`, `UJ-014`
- Existing coverage: ADR lifecycle covers decisions; version history records change; provenance models revision; none of the surveyed general approaches fully models question/hypothesis/observation/claim/refinement/correction/verification semantics.

### Residual gap

Knowledge can be refined, narrowed, strengthened, weakened, contradicted, rejected, corrected, or superseded. A generic `version` relation loses why the epistemic status changed.

### Candidate capability

Project Knowledge can represent selected epistemic roles and transitions when they materially improve recovery.

### Hold boundary

The exact state machine is not ready for requirements. Competing simultaneous hypotheses, explicit rejection, and real supersession chains remain under-evidenced.

---

## CAP-010 — Claim-relative evidence and validity

**Disposition:** NEW  
**Responsibility:** Project-memory semantics  
**Confidence:** High

### Evidence trace

- Cases: `PKC-0005`, `PKC-0006`, `PKC-0007`
- Failure modes: `FM-007`, `FM-008`, `FM-009`, `FM-010`
- User jobs: `UJ-005`, `UJ-007`, `UJ-009`, `UJ-014`
- Existing coverage: provenance can explain where evidence came from; testing systems can record pass/fail; generic freshness mechanisms compare files/commits.

### Residual gap

The corpus requires semantics equivalent to:

```text
Evidence E supports proposition P
against source state S
using method V
under context C
```

and later invalidation only when change is relevant to `P`.

### Candidate capability

Project Knowledge can bind evidence to the proposition it supports, source/context/method under which it was obtained, and rules or dependencies sufficient to reason about later relevance/freshness.

---

## CAP-011 — Derivation lineage and projection freshness

**Disposition:** EXTEND  
**Responsibility:** Project-memory semantics  
**Confidence:** High

### Evidence trace

- Cases: `PKC-0001`, `PKC-0002`, `PKC-0004`, `PKC-0008`
- Failure modes: `FM-001`, `FM-006`, `FM-015`
- User jobs: `UJ-001`, `UJ-006`, `UJ-010`, `UJ-013`
- Existing coverage: W3C PROV models derivation; event-sourcing patterns model projections; build systems often model dependency invalidation.

### Residual gap

A project view needs to know which exact source assertions/artifacts produced it, whether relevant inputs changed, and whether only a portion of a document became stale.

### Candidate capability

Project Knowledge can preserve derivation lineage for generated or curated projections and compute or explain freshness at the finest granularity justified by the projection's dependencies.

---

## CAP-012 — Typed cross-artifact relationships and impact traversal

**Disposition:** INTEGRATE  
**Responsibility:** Project-memory integration  
**Confidence:** High

### Evidence trace

- Cases: `PKC-0001` and the corpus as a whole
- Failure modes: `FM-014`
- User jobs: `UJ-003`, `UJ-004`, `UJ-008`, `UJ-013`
- Existing coverage: graph models strongly represent typed many-to-many relations; issue trackers handle work dependencies; ADRs handle decision relations; linked-note systems provide lightweight links/backlinks.

### Residual gap

Relationships are fragmented across tools and prose, preventing project-wide traversal from requirement → decision → specification → work → code → evidence.

### Candidate capability

Project Knowledge can integrate explicit and inferred/suggested relationships across native artifacts and support impact/recovery traversal, while keeping ordinary links sufficient where typed semantics provide no added value.

---

## CAP-013 — Source/admission classification

**Disposition:** EXTEND  
**Responsibility:** Project-memory integration  
**Confidence:** Medium

### Evidence trace

- Cases: `PKC-0004`, supported indirectly by `PKC-0011`
- Failure modes: `FM-005`, `FM-006`
- User jobs: `UJ-005`, `UJ-010`, `UJ-011`
- Existing coverage: ignore rules, content discovery, provenance, repository policy, and source manifests each solve portions locally.

### Residual gap

Discoverability or parsability does not imply that a file is project knowledge. Incidental environment state can contaminate downstream projections.

### Candidate capability

Project Knowledge can classify or policy-gate discovered material as canonical source, derived representation, coordination projection, external reference, transient control artifact, incidental state, or unclassified input before treating it as project memory.

### Hold boundary

Only one direct classification failure is in the corpus; the exact classification vocabulary remains provisional.

---

## CAP-014 — Current-state and historical project-memory views

**Disposition:** INTEGRATE  
**Responsibility:** Human-facing projection  
**Confidence:** High

### Evidence trace

- Cases: `PKC-0002`, `PKC-0008`, `PKC-0009`
- Failure modes: `FM-001`, `FM-002`, `FM-013`, `FM-016`
- User jobs: `UJ-001`, `UJ-002`, `UJ-004`, `UJ-011`, `UJ-014`
- Existing coverage: Git provides historical revisions; event sourcing provides projections; docs/wikis provide curated current summaries; temporal models provide point-in-time semantics.

### Residual gap

Users need current truth and historical truth together without either rewriting history or presenting old statements as current instructions.

### Candidate capability

Project Knowledge can produce or assist views that clearly distinguish current authoritative assertions from historical assertions and can reconstruct relevant project state at a selected time/context.

---

## CAP-015 — Contradiction diagnosis

**Disposition:** NEW  
**Responsibility:** Human-facing projection  
**Confidence:** High

### Evidence trace

- Cases: `PKC-0002`, `PKC-0003`, `PKC-0006`, `PKC-0007`, `PKC-0008`
- Failure modes: `FM-001`, `FM-002`, `FM-003`, `FM-008`, `FM-011`, `FM-012`
- User job: `UJ-009`
- Existing coverage: diff/search can find disagreement; no surveyed mechanism combines identity, authority, time, context, provenance, and epistemic state to classify the nature of the disagreement.

### Residual gap

Two contradictory statements may reflect stale projection, historical truth, context difference, semantic mismatch, incorrect provenance, differing authority scope, or genuine disagreement.

### Candidate capability

Project Knowledge can explain likely contradiction type using the underlying semantic model and preserve genuine unresolved disagreement rather than forcing false reconciliation.

---

## CAP-016 — Hybrid retrieval over project-memory semantics

**Disposition:** INTEGRATE  
**Responsibility:** Human-facing projection  
**Confidence:** High

### Evidence trace

- Cases: corpus-wide relationship burden; especially `PKC-0001` and `PKC-0011`
- Failure mode: `FM-014`
- User jobs: `UJ-001` through `UJ-013` broadly, especially `UJ-011`
- Existing coverage: lexical, semantic, hybrid search, filtering, and RAG retrieval are mature mechanisms.

### Residual gap

Search can find relevant text but cannot manufacture semantic identity, authority, valid time, provenance, or evidence scope safely.

### Candidate capability

Project Knowledge can expose its project-memory semantics as filters, facets, ranking signals, context, and citations to an integrated hybrid retrieval layer.

### Constraint

Retrieval is an access mechanism over project memory, not the authority model.

---

## CAP-017 — Traceable authored narrative / learning views

**Disposition:** INTEGRATE  
**Responsibility:** Human-facing projection  
**Confidence:** Medium

### Evidence trace

- Motivation/user job: `UJ-012`
- Supporting cases: corpus defect chains and Project Knowledge's own discovery history
- Failure modes: `FM-013`, `FM-014`, `FM-016`
- Existing coverage: docs-as-code, wikis, and linked-note systems are strong for authored narrative; generated summaries can assist source selection.

### Residual gap

A teaching narrative needs deliberate sequence, emphasis, explanation, and pedagogy, while still remaining traceable to authoritative/current and historical source material.

### Candidate capability

Project Knowledge can help authors compose narrative/learning views from traceable project-memory sources without treating generated traversal or LLM summaries as automatically authoritative narrative.

### Hold boundary

Direct corpus evidence for educational projection remains limited, so detailed requirements should remain narrow until a concrete narrative case is captured.

---

## CAP-018 — Progressive formalization and selective retention

**Disposition:** NEW  
**Responsibility:** Cross-cutting constraint  
**Confidence:** High

### Evidence trace

- Cases: `PKC-0008`, `PKC-0011`
- Failure/counterpressure: `FM-014` plus the risk of excessive capture identified during discovery
- User jobs: `UJ-011`, `UJ-012`, `UJ-013`
- Existing coverage: linked-note systems and PROV support progressive enrichment; repository-native tools remain useful with minimal structure.

### Residual gap

A rich system that requires full classification, stable IDs, temporal metadata, provenance, and typed relationships at capture time would recreate the cognitive burden it is intended to remove.

### Candidate capability

Project Knowledge supports a low-friction progression from ordinary native artifacts to richer semantics only when value justifies the cost, including the ability to decline enrichment or retention.

### Constraint

This is a requirement on every later capability: richer semantics must be optional/scoped unless correctness requires them.

---

## CAP-019 — Causal/recovery path reconstruction

**Disposition:** EXTEND  
**Responsibility:** Human-facing projection  
**Confidence:** Medium

### Evidence trace

- Cases: `PKC-0004`, `PKC-0005`, `PKC-0006`, `PKC-0007`
- Failure modes: `FM-013`, `FM-016`
- User jobs: `UJ-003`, `UJ-004`, `UJ-008`, `UJ-012`
- Existing coverage: graphs can traverse relations; Git orders changes; ADRs preserve rationale; event histories preserve events.

### Residual gap

The useful explanation is often a causal/reasoning chain rather than a bag of related artifacts.

### Candidate capability

Project Knowledge can preserve or reconstruct explicit explanatory paths such as:

```text
problem → question → evidence → decision → implementation → outcome → correction/lesson
```

without asserting causality merely because two events are related or sequential.

### Hold boundary

The corpus mostly contains clear defect chains; uncertain/contested causality remains under-evidenced.

---

## CAP-020 — Preserve correction without erasing prior belief

**Disposition:** EXTEND  
**Responsibility:** Human-facing projection  
**Confidence:** High

### Evidence trace

- Cases: `PKC-0006`, `PKC-0008`, `PKC-0009`
- Failure modes: `FM-002`, `FM-008`, `FM-016`
- User jobs: `UJ-004`, `UJ-005`, `UJ-014`
- Existing coverage: Git preserves revision history; temporal models distinguish truth intervals; provenance can model revision; epistemic semantics can explain correction/refinement.

### Residual gap

Simply overwriting a wrong statement cleans current state but loses why it was believed and what corrected it; showing both without status creates ambiguity.

### Candidate capability

Project Knowledge can preserve the old assertion and its context, record the correction/refinement and evidence, and clearly establish which assertion is current for the relevant scope.

---

# Deliberately not promoted yet

The following concepts remain discovery targets rather than general capabilities:

- a complete supersession model across arbitrary engineering objects;
- a fixed epistemic state machine;
- a terminology-evolution subsystem;
- experiment management as a first-class Project Knowledge feature;
- automatic causal inference;
- automatic authoritative conflict resolution;
- mandatory knowledge-graph storage; and
- mandatory capture of conversations, every commit, or every transient tool event.

They may become capabilities later, but the current corpus does not justify general requirements for them.
