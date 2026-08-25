# Requirements Traceability Matrix

This matrix preserves the audit path from formal requirements back to discovery evidence.

The detailed capability-level trace remains canonical in [`../discovery/capabilities/trace-matrix.md`](../discovery/capabilities/trace-matrix.md). This file promotes that trace to the requirement level.

## Reading the matrix

- **CAP** — discovery capability or capabilities directly promoted into the requirement.
- **UJ** — principal user jobs supported.
- **FM / constraint** — principal observed failure modes or counterpressure.
- **PKC** — representative corpus cases. The capability trace may contain additional cases.

## RF-1 — Native interoperability and progressive adoption

| Requirement | CAP | UJ | FM / constraint | Representative evidence |
| --- | --- | --- | --- | --- |
| FR-101 Preserve native artifacts | 001, 018 | 002, 003, 011, 014 | FM-002, 004, 016; progressive-structure counterpressure | PKC-0001, 0008, 0010, 0011 |
| FR-102 Preserve native identity/source state | 001, 002 | 003, 005, 008 | FM-004, 009, 012 | PKC-0001, 0003, 0007, 0010 |
| FR-103 Federate heterogeneous systems | 002 | 003, 006, 008, 011, 013 | FM-003, 014 | PKC-0001, 0002, 0003, 0011 |
| FR-104 Admission/exclusion policy | 013 | 005, 010, 011 | FM-005, 006 | PKC-0004; PKC-0011 counterpressure |
| FR-105 Progressive enrichment | 018 | 011, 012, 013 | over-modeling / relationship burden | PKC-0008, 0011 |
| FR-106 Preserve simple organization | 001, 018 | 011, 012 | hierarchy-is-not-the-enemy counterevidence | PKC-0011 |
| FR-107 Non-destructive enrichment | 001, 018 | 003, 011 | FM-004, 016; progressive adoption | PKC-0010, 0011 |

## RF-2 — Semantic identity, representation, and relationships

| Requirement | CAP | UJ | FM / constraint | Representative evidence |
| --- | --- | --- | --- | --- |
| FR-201 Semantic identity where needed | 003 | 001, 003, 006, 013 | FM-004, 012, 014 | PKC-0001, 0010 |
| FR-202 Identity survives relocation | 003 | 003, 006 | FM-004 | PKC-0010 |
| FR-203 Bind representations to subjects | 003, 004 | 001, 006, 010 | FM-001, 015 | PKC-0001, 0002, 0010 |
| FR-204 Distinguish representation roles | 004 | 001, 006, 010 | FM-001, 006, 015 | PKC-0001, 0002, 0004, 0008 |
| FR-205 Prevent false corroboration | 004, 011 | 001, 006, 010 | FM-006, 015 | PKC-0002, 0004, 0008 |
| FR-206 Typed relationships when useful | 012 | 003, 004, 008, 013 | FM-014 | PKC-0001; corpus-wide |
| FR-207 Cross-native traversal | 002, 012 | 003, 006, 008, 013 | FM-014 | PKC-0001, 0002, 0003 |
| FR-208 Relationship provenance | 006, 012 | 005, 010, 013 | FM-006, 010, 015 | PKC-0004, 0006, 0007 |

## RF-3 — Authority and current truth

| Requirement | CAP | UJ | FM / constraint | Representative evidence |
| --- | --- | --- | --- | --- |
| FR-301 Scoped authority | 005 | 001, 006, 009, 013 | FM-001, 002, 011, 015 | PKC-0001, 0002, 0003, 0008 |
| FR-302 Explain authority basis | 005 | 001, 009 | FM-011 | PKC-0001, 0002, 0003 |
| FR-303 Recover current truth | 005, 014 | 001, 011 | FM-001, 002 | PKC-0002, 0008, 0009 |
| FR-304 Preserve unresolved conflicts | 005, 015 | 009 | no fabricated reconciliation | PKC-0002, 0003, 0006, 0007 |
| FR-305 Distinguish current/history | 007, 014, 020 | 001, 002, 004, 014 | FM-002, 013, 016 | PKC-0002, 0008, 0009 |
| FR-306 Diagnose disagreement | 015 | 009 | FM-001, 002, 003, 008, 011, 012 | PKC-0002, 0003, 0006, 0007, 0008 |
| FR-307 Correction without erasure | 020 | 004, 005, 014 | FM-002, 008, 016 | PKC-0006, 0008, 0009 |
| FR-308 No authority by repetition/rank | 004, 005, 016 | 001, 006, 009 | FM-015; retrieval limitation | PKC-0002, 0004; existing-approach research |

## RF-4 — Provenance, time, and context

| Requirement | CAP | UJ | FM / constraint | Representative evidence |
| --- | --- | --- | --- | --- |
| FR-401 Structured provenance | 006 | 005, 007, 010, 014 | FM-006, 008, 009, 010 | PKC-0003, 0004, 0006, 0007 |
| FR-402 Derivation lineage | 006, 011 | 001, 006, 010, 013 | FM-001, 006, 015 | PKC-0001, 0002, 0004, 0008 |
| FR-403 Valid vs recorded time | 007 | 001, 002, 004, 014 | FM-002, 007, 009, 016 | PKC-0002, 0005, 0007, 0008, 0009 |
| FR-404 Material observation context | 008 | 005, 007, 008, 009 | FM-003, 009, 010, 012 | PKC-0003, 0004, 0006, 0007 |
| FR-405 Context locator vs state identity | 008 | 005, 008 | FM-009, 012 | PKC-0003, 0006, 0007 |
| FR-406 Derivation freshness | 011 | 001, 006, 010, 013 | FM-001, 006, 015 | PKC-0002, 0004, 0008 |
| FR-407 Freshness scoped to inputs | 010, 011 | 007, 010, 013 | FM-007 | PKC-0005, 0006, 0007 |
| FR-408 Provenance corrections | 006, 020 | 005, 014 | FM-008, 010, 016 | PKC-0006, 0008, 0009 |

## RF-5 — Evidence and epistemic evolution

| Requirement | CAP | UJ | FM / constraint | Representative evidence |
| --- | --- | --- | --- | --- |
| FR-501 Evidence supports proposition | 010 | 005, 007, 009 | FM-007, 008, 010 | PKC-0005, 0006, 0007 |
| FR-502 Evidence evaluation context | 006, 008, 010 | 005, 007, 008 | FM-008, 009, 010, 012 | PKC-0005, 0006, 0007 |
| FR-503 Bound evidence claims | 010 | 007, 009 | FM-008 | PKC-0006 |
| FR-504 Claim-relative invalidation | 010 | 007, 014 | FM-007, 009 | PKC-0005, 0007 |
| FR-505 Epistemic roles when useful | 009 | 002, 004, 009, 014 | FM-002, 013, 016 | PKC-0009; medium-confidence |
| FR-506 Refinement/correction semantics | 009, 020 | 004, 014 | FM-013, 016 | PKC-0008, 0009; medium-confidence for general epistemics |
| FR-507 Preserve uncertainty | 009, 015 | 009, 014 | no automatic truth promotion | PKC-0009 + explicit evidence gap for competing hypotheses |

## RF-6 — Retrieval, impact, and explanation

| Requirement | CAP | UJ | FM / constraint | Representative evidence |
| --- | --- | --- | --- | --- |
| FR-601 Cross-source retrieval | 016 | 001–013, esp. 011 | FM-014 | PKC-0001, 0011; corpus-wide |
| FR-602 Semantic retrieval context | 005, 007, 016 | 001, 002, 009, 011 | FM-002, 011, 014 | corpus + existing-approach research |
| FR-603 Relevance separate from truth | 016 | 001, 009 | retrieval limitation | existing-approach research + FM-015 |
| FR-604 Impact traversal | 012 | 003, 004, 013 | FM-014 | PKC-0001; corpus-wide |
| FR-605 Explainable recovery paths | 019 | 003, 004, 008, 012 | FM-013, 016 | PKC-0004, 0005, 0006, 0007 |
| FR-606 Do not invent causality | 019 | 004, 009, 012 | causal-evidence constraint | defect causal chains + medium-confidence limitation |
| FR-607 Traceable authored narrative | 017 | 012 | FM-013, 014, 016 | project motivation; medium-confidence |
| FR-608 Context recovery after absence | 002, 014, 016, 018 | 011 | FM-014 + progressive counterpressure | PKC-0001, 0008, 0011 |

## Cross-cutting quality/constraint trace

| Requirement group | Primary discovery support |
| --- | --- |
| QA-001, QA-011, CON-002, CON-013 | CAP-018; PKC-0008, PKC-0011; over-modeling counterpressure |
| QA-002, QA-003, QA-008, CON-001 | CAP-001, CAP-002; repository-native findings EA-001/EA-002 |
| QA-004, QA-005, QA-009 | CAP-005, CAP-006, CAP-010, CAP-015; authority/provenance/evidence corpus |
| QA-006, QA-007, QA-013 | CAP-008, CAP-011; projection and host-dependent freshness cases |
| QA-010 | CAP-007, CAP-014, CAP-020; PKC-0002, 0008, 0009 |
| QA-014, QA-017 | Existing-approach composition findings; CAP-002, CAP-006, CAP-012 |
| QA-015 | Foundational safety constraint on heterogeneous integration; later security research still required |
| CON-003, CON-011 | CAP-016 + search/RAG existing-approach findings |
| CON-005 | CAP-004, CAP-011; FM-006, FM-015 |
| CON-007–CON-010 | Existing-approach research: reuse semantics/patterns without prematurely selecting storage architecture |
| CON-012 | CAP-010; PKC-0005 through 0007 |
| CON-014 | CAP-017; authored-narrative composition finding EA-011 |
| CON-015 | Discovery-to-domain-model boundary |

## Trace gaps intentionally preserved

Some normative statements are foundational safety/engineering constraints rather than conclusions from repeated corpus cases. Most notably:

- access-boundary preservation (`QA-015`) is required for any viable cross-system integration but has not yet received a dedicated security discovery pass;
- exact performance/scalability targets are not yet evidence-backed;
- collaboration/concurrency semantics remain under-evidenced;
- detailed retention/privacy policy remains open.

These gaps must remain visible during domain modeling and architecture rather than being silently filled by convention.
