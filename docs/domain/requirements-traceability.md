# Requirements-to-Domain Traceability

This matrix maps the formal requirements to the smallest domain concepts intended to satisfy them.

The purpose is to prevent the domain model from accumulating concepts that have no requirement basis and to show where one concept serves several user-facing behaviors.

| Requirement area | Primary domain concepts | Notes |
| --- | --- | --- |
| FR-101–107 Native interoperability / progressive adoption | Project, Source System, Native Reference, progressive-formalization levels | Native participation precedes semantic enrichment |
| FR-201–203 Semantic continuity | Subject, Representation, Native Reference | Subject identity is conditional |
| FR-204 Representation roles | Representation.role | Role vocabulary extensible; role != authority |
| FR-205 False corroboration prevention | Representation, Assertion, Activity/derivation lineage | Multiple derived occurrences remain one lineage family |
| FR-206–208 Typed relationships / origin | Relationship, provenance/origin | Ordinary links remain valid |
| FR-301–304 Scoped authority / unresolved conflict | Claim, Assertion, Concern, Authority Scope, Authority Assignment | Unknown/unresolved are valid outcomes |
| FR-305 Current vs historical truth | Claim, Assertion, Temporal Qualifiers, Authority Assignment | Current truth is derived, not primitive |
| FR-306 Contradiction diagnosis | Subject, Claim, Assertion, Authority, Context, provenance, time | Diagnostic is a projection/composition |
| FR-307 Correction without erasure | Assertion, Relationship, Activity, temporal/provenance semantics | No universal supersession state machine |
| FR-308 No authority by rank/repetition | Assertion lineage, Authority Assignment | Retrieval remains separate |
| FR-401 Structured provenance | Entity-role domain objects, Activity, Agent, provenance Relationships | PROV-compatible semantics preferred |
| FR-402 Derivation lineage | Activity, Representation, Relationship | Used/generated chain |
| FR-403 Valid vs recorded time | Temporal Qualifiers on Claims/Assertions/Authority/Activities as needed | Conditional enrichment |
| FR-404 Observation context | Context | Selective material dimensions |
| FR-405 Durable reconstruction identity | Native Reference + Context | Immutable source state distinct from locator |
| FR-406–407 Freshness | Activity lineage, source-state refs, Relationship relevance | Freshness derived |
| FR-408 Provenance correction | Assertion/Relationship history + Activity | Current correction with retained prior record |
| FR-501 Explicit proposition | Claim | Evidence targets Claim |
| FR-502 Evaluation context | Evidence Evaluation, Context, Native Reference, method | Specialized Activity |
| FR-503 Bound evidence scope | Claim + Evidence Evaluation.method/result | No broader proof than method supports |
| FR-504 Claim-relative invalidation | Evidence Evaluation + relevant input Relationships | Relevance-scoped freshness |
| FR-505 Epistemic roles | Epistemic Annotation | Optional, extensible |
| FR-506 Knowledge evolution | Relationship/change-kind + Epistemic Annotation | No universal lifecycle |
| FR-507 Preserve uncertainty | Claim/Assertion + Epistemic Annotation + unresolved resolution outcomes | Unknown/disputed valid |
| FR-601–603 Retrieval | Native Reference, Subject, Claim, semantic metadata exposed to Projection | Search tech outside domain kernel |
| FR-604 Impact traversal | Relationship + origin/provenance | Traversal result != proof |
| FR-605 Recovery paths | Relationship + Activity + Projection | Selected path over recorded semantics |
| FR-606 No invented causality | Relationship.origin/type + invariant INV-023 | Chronology insufficient |
| FR-607 Narrative | Projection/Representation + source traceability | Narrative not canonical by readability |
| FR-608 Context recovery | Projection over shared kernel | Outcome, not one mandated UI |

## Quality-attribute mapping

| Quality attribute | Domain response |
| --- | --- |
| QA-001 Low capture burden | progressive-formalization levels; optional Subject/Claim/time/context semantics |
| QA-002 Graceful degradation | Native Reference/source ownership remains primary; Project Knowledge projection is additive |
| QA-003 Portability | kernel concepts defined independently of proprietary storage |
| QA-004 Traceability | origin/provenance/basis on assertions, authority, relationships, derived views |
| QA-005 Explainable inference | Relationship/assertion origin distinguishes inferred from recorded |
| QA-006 Deterministic derived state | Activity + lineage + policy inputs make derivation inspectable |
| QA-007 Reconstructability | Native Reference distinguishes reconstructable source state from ephemeral Context locators |
| QA-008 Source fidelity | Representation references native meaning rather than rewriting it into universal ontology |
| QA-009 Authority safety | Authority Assignment separate from Representation, search, derivation |
| QA-010 Temporal clarity | valid/recorded time plus current/historical Projection semantics |
| QA-011 Cognitive scalability | recovery projections are selective views over shared kernel |
| QA-012 Project-scale scalability | enrichment is object-scoped rather than one migration mode |
| QA-013 Incremental maintainability | derived relationships/projections use lineage rather than repeated manual duplication |
| QA-014 Integration extensibility | Source System/Native Reference boundary isolates native integrations from kernel semantics |
| QA-015 Access boundaries | Source System/Project policy boundary retained; architecture must enforce |
| QA-016 Correction auditability | retained Assertion/Activity/Relationship history |
| QA-017 Interoperable semantics | PROV-compatible provenance and bitemporal conceptual distinction |
| QA-018 Testability | domain invariants supply observable semantic outcomes |

## Constraint mapping

- CON-001 → Source System / Native Reference / Authority Assignment separation
- CON-002 → progressive formalization
- CON-003 → retrieval Projection separated from Authority Assignment
- CON-004 → unresolved resolution outcomes
- CON-005 → Representation + derivation lineage
- CON-006 → Native Reference identity layers
- CON-007 → Activity/Agent/provenance compatibility
- CON-008 → Temporal Qualifiers are optional value semantics
- CON-009 → Relationship is storage-agnostic
- CON-010 → Activity/history does not require event sourcing
- CON-011 → no AI concept exists in the semantic kernel
- CON-012 → Evidence Evaluation targets Claim
- CON-013 → Context/retention remain selective
- CON-014 → narrative is Projection/Representation, not authority by default
- CON-015 → this domain model explicitly settles the previously provisional boundaries

## Non-requirement preservation

The kernel deliberately does **not** introduce domain requirements for:

- Git implementation;
- issue/project-management workflow;
- wiki authoring;
- ADR authoring lifecycle;
- universal engineering artifact ontology;
- Subject ID on every artifact;
- universal supersession;
- universal epistemic lifecycle;
- experiment-management workflow;
- automatic terminology evolution;
- causal inference;
- conflict auto-resolution;
- conversation capture;
- event sourcing;
- graph/temporal/vector databases;
- AI/RAG;
- SaaS deployment;
- central physical storage;
- indefinite retention;
- build/test execution;
- prescribed UI; or
- prescribed language/framework.

## Minimality check

The candidate kernel should be challenged by attempting to remove each concept.

Current conclusion:

- remove **Subject** → cross-representation continuity becomes locator-bound;
- remove **Claim/Assertion distinction** → source occurrence, proposition, evidence, and authority collapse;
- remove **Authority Assignment** → current truth cannot explain why one source governs;
- remove **Activity/provenance** → derivation and generated repetition cannot be reconstructed safely;
- remove **Context** → several corpus cases become semantically ambiguous;
- remove **Evidence Evaluation** → evidence scope/freshness collapses into generic artifact freshness;
- remove **Relationship** → impact/recovery semantics become prose-only and non-traversable;
- remove **Projection** as a domain boundary → user-facing views risk inventing incompatible truth models.

`Source System`, `Native Reference`, Representation, time qualifiers, Agent, Concern, and Epistemic Annotation remain necessary supporting concepts/value semantics but do not imply separate services or stores.