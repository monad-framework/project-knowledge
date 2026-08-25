# Architecture Traceability

This document shows where the selected architecture preserves the normative requirements/domain model.

It is intentionally architectural rather than field-level. Detailed schema/API traceability belongs to the implementation specifications.

## Requirement-family mapping

| Requirement family | Primary architecture mechanisms |
| --- | --- |
| RF-1 Native interoperability / progressive adoption | S1 native state, adapters, optional S2 records, L0-compatible operation, portable record layer, admission policy |
| RF-2 Identity / representation / relationships | S2 Subject + bindings + Relationships, adapter native identity, RM-2/RM-5, traversal engine |
| RF-3 Authority / current truth | Authority records/policy, RM-3/RM-4, Authority Resolver, Current-State Resolver, conflict/unknown outcomes |
| RF-4 Provenance / time / context | Activity records, adapter source-state identity, RM-5/RM-7, temporal qualifiers, Context, lineage compiler |
| RF-5 Evidence / epistemic evolution | Evidence Evaluation records, RM-6, evidence resolver, optional epistemic records, correction history |
| RF-6 Retrieval / impact / explanation | retrieval index, traversal engine, resolver explanation contract, projection layer |

## Quality-attribute mapping

| Quality attribute | Architecture preservation |
| --- | --- |
| QA-001 Progressive adoption | S2 is optional/sparse; minimal project may have zero semantic records |
| QA-002 Graceful degradation | native sources and S2 remain usable; S3 is disposable |
| QA-003 Portability | S2 portable semantic record contract |
| QA-004 Traceability | resolver explanation/basis + source/derivation lineage |
| QA-005 Explainable inference | relationship/identity origin labels; unknown allowed |
| QA-006 Deterministic derived state | full rebuild conformance path where declared deterministic |
| QA-007 Reconstructability | immutable source-state identity + Context distinction |
| QA-008 Source fidelity | adapters observe; they do not rewrite/import as canonical truth |
| QA-009 Authority safety | S1/S2/S3 separation; resolver owns authority semantics |
| QA-010 Temporal clarity | resolver/projections carry current/historical/freshness state |
| QA-011 Cognitive scalability | optimized read models + scoped projections/retrieval |
| QA-012 Project-scale scalability | incremental sync + replaceable query indexes |
| QA-013 Incremental maintainability | dependency-aware compiler/invalidation |
| QA-014 Integration extensibility | adapter contract + kernel semantic boundary |
| QA-015 Access boundaries | access partitions propagated through derived state |
| QA-016 Audit corrections | S2 retained corrections + native/VCS history + lineage |
| QA-017 Interoperable semantics | provenance/time semantics kept independent of storage technology |
| QA-018 Testability | V1–V6 conformance layers + S-1–S-8 fixtures |

## Domain invariant mapping

| Invariant | Preserving mechanism |
| --- | --- |
| INV-001 Native authority | S1 remains distinct from S2/S3; adapter observation contract |
| INV-002 Mutable locator != immutable identity | Native Reference explicitly separates locator/source-state identity |
| INV-003 Semantic identity optional | zero-S2/minimal-project path |
| INV-004 Stable Subject identity | portable S2 Subject record identity rules |
| INV-005 Identity may remain unknown | Subject Resolver outcome |
| INV-006 Role != authority | Representation binding and Authority Assignment stored/resolved separately |
| INV-007 Assertion != truth | RM-3 Assertion storage + resolver layer |
| INV-008 Repetition != corroboration | Activity/derivation lineage + RM-5 |
| INV-009 Scoped authority | Authority Assignment concern scope + RM-4 |
| INV-010 Explainable authority | resolver basis contract |
| INV-011 Conflict valid | authority/current resolver outcomes include unresolved conflict |
| INV-012 Current truth derived | R-4 Current-State Resolver |
| INV-013 Historical/current distinction | temporal qualifiers + as-of resolver/projections |
| INV-014 Temporal richness conditional | optional S2 temporal qualifiers; native history reused |
| INV-015 Ephemeral context insufficient | adapter exposes reconstructable source-state identity separately |
| INV-016 Context selective | only retained material Context enters S2 |
| INV-017 Evidence Claim-relative | Evidence Evaluation record + RM-6 |
| INV-018 Evidence method-bounded | Evidence Resolver + M0 C1/C2 negative scenario |
| INV-019 Relevance-scoped freshness | dependency-aware evidence status; unknown if relevance absent |
| INV-020 Derived lineage | compiler dependency lineage + RM-5/RM-7 |
| INV-021 Inference labeled | relationship/identity origin metadata |
| INV-022 Search != authority | retrieval subsystem is downstream consumer only |
| INV-023 Chronology != causality | traversal engine preserves relationship type/origin |
| INV-024 Correction preserves history | S2 correction records + native history + as-of resolver |
| INV-025 No graph-storage assumption | replaceable S3 read-model contract |
| INV-026 PROV semantic not technological | Activity/Relationship semantics independent of serialization/store |
| INV-027 Object-scoped formalization | sparse independent S2 records |
| INV-028 Enrichment doesn't break native usage | sidecar/reference model; adapters do not rewrite source |
| INV-029 Access boundaries | access lineage propagated adapter → compiler → read model → query |
| INV-030 Projection not authority | S3/projection state class + resolver-only authority logic |
| INV-031 Unknown first-class | all core resolvers include unknown/unavailable states |
| INV-032 Extensible categories | namespaced portable-record extensions + adapter extensions |

## Architecture-entry scenario mapping

| Scenario | Primary path |
| --- | --- |
| S-1 Minimal project | Native Git/files → adapter/index only; zero S2 required |
| S-2 Identity continuity | Subject S2 → Representation bindings → Git source-state observations |
| S-3 Authority conflict | Assertions → Authority Assignment → R-3/R-4 |
| S-4 Historical correction | retained Assertions/correction + time → R-5 |
| S-5 Context-dependent observation | adapter observation + Context + source-state identity |
| S-6 Claim-relative evidence | Evidence Evaluation → RM-6 → R-8 |
| S-7 Derived freshness | Activity/lineage → dependency tracking → R-7 |
| S-8 Unknown resolution | resolver outcome model + no fallback-by-rank/import-order |

## Canonical/derived ownership test

For any proposed field or artifact, architecture review should ask:

1. Is this owned authoritatively by a native source? → S1/reference it.
2. Is this a durable cross-system semantic decision owned by Project Knowledge? → S2/persist portably.
3. Can it be deterministically/reliably recomputed for query convenience? → S3/derive it.
4. Is ownership ambiguous? → keep it unknown/provisional until policy/domain clarifies.

This four-question test is the primary guardrail against future architectural drift.