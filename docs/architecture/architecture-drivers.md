# Architecture Drivers

Architecture must be selected by preserving the requirements and domain invariants under realistic operating conditions.

This document identifies the forces that materially constrain the architectural shape.

## AD-001 — Native systems remain authoritative

Project Knowledge integrates native files, Git history, issues, ADRs, CI/test evidence, external standards, and other systems without silently transferring their authority into a Project Knowledge database.

Architecture consequence:

- source adapters must preserve native identity and source boundaries;
- Project Knowledge-owned records must distinguish native facts from Project Knowledge-authored semantics;
- copied/indexed source content is derived unless explicitly configured otherwise.

Primary invariants: INV-001, INV-006, INV-030.

## AD-002 — Project Knowledge must own some durable semantics

Pure federation is insufficient because the project needs durable cross-source information that no native system necessarily owns, including:

- semantic Subject identity;
- Representation bindings;
- cross-system Relationships;
- Authority Assignments/policy;
- Project Knowledge-authored Claims/Assertions;
- provenance/context/evidence records;
- correction relationships; and
- project policy/configuration.

Architecture consequence:

A portable durable representation for Project Knowledge-owned semantics is required.

## AD-003 — Portable state must outlive implementations

Project Knowledge-controlled semantics should remain inspectable, versionable, exportable, and reconstructable if a particular service, cache, database, or UI is removed.

Architecture consequence:

Do not make an opaque derived database the only durable representation of Project Knowledge-owned meaning.

Primary quality attributes: QA-002, QA-003, QA-007, QA-008.

## AD-004 — Rich query behavior must not dictate canonical storage

The domain needs relationship traversal, current-state resolution, provenance reconstruction, impact analysis, contradiction diagnostics, temporal filtering, and retrieval.

Those behaviors may benefit from optimized indexes or relational/graph read models, but INV-025 explicitly prevents graph storage from becoming a domain assumption.

Architecture consequence:

Separate canonical portable records from query-optimized derived models.

## AD-005 — Current truth is computed

Current truth is not a mutable master record. It is a resolution over retained Assertions/Claims, scoped authority, temporal qualifiers, Context, policy, and source state.

Architecture consequence:

- current-state views are projections/resolution outputs;
- cached current state must retain dependency lineage;
- unknown and unresolved conflict are first-class resolver outcomes.

Primary invariants: INV-009 through INV-013, INV-031.

## AD-006 — Derived information must be disposable and explainable

Indexes, graph/read models, search indexes, cached projections, generated summaries, and current-state caches can improve performance and usability but must retain source lineage.

Architecture consequence:

- derived stores are rebuildable;
- dependency/fingerprint information is retained where freshness matters;
- missing/corrupt derived state degrades convenience, not authoritative meaning.

Primary invariants: INV-008, INV-020, INV-021, INV-030.

## AD-007 — Progressive formalization is object-scoped

A project may contain Level-0 native files and Level-5 evidence semantics indefinitely.

Architecture consequence:

- adapters and record formats must tolerate partial enrichment;
- no project-wide migration into a maximum schema is allowed;
- indexed native references must coexist with fully modeled Subjects/Claims.

Primary invariants: INV-003, INV-014, INV-016, INV-027, INV-028.

## AD-008 — Incremental operation matters

Large projects cannot require a full corpus rebuild after every source change.

Architecture consequence:

The synchronization model should identify affected native references, semantic records, and derived projections and recompute only dependencies that may have changed.

This is an architecture quality goal rather than a requirement for event sourcing.

## AD-009 — Reconstructability outranks local convenience

Host-local paths, temporary worktrees, mutable refs, and current URLs may be useful Context but are not sufficient historical identity when stronger source-state identity exists.

Architecture consequence:

Adapters should expose both contextual locators and durable/reconstructable source-state identifiers where available.

Primary invariants: INV-002, INV-015.

## AD-010 — Evidence is proposition-scoped

Evidence freshness and validity depend on the Claim/proposition, source state, method, Context, and relevant dependencies.

Architecture consequence:

Generic repository-version freshness is insufficient as the only evidence mechanism. The model must permit explicit evidence dependency scopes and unknown freshness when relevance cannot be determined.

Primary invariants: INV-017 through INV-019.

## AD-011 — Retrieval is a consumer of semantics

Lexical search, embeddings, RAG, generated answers, and semantic ranking can improve discovery but cannot establish identity, authority, truth, or provenance.

Architecture consequence:

Retrieval indexes consume normalized semantic metadata; they are not the semantic source of truth.

Primary invariant: INV-022.

## AD-012 — Access boundaries must survive indexing

Federating or indexing restricted material must not make it visible outside its source or configured policy boundary.

Architecture consequence:

Every source observation and derived record needs sufficient access-partition/visibility lineage for query surfaces to enforce source-equivalent restrictions.

Exact authorization technology remains open.

Primary invariant: INV-029.

## AD-013 — Local/offline usefulness is strongly preferred

The project originated as a cognitive/project continuity tool and requirements explicitly avoid SaaS dependence.

Architecture consequence:

The core semantic workflow should be capable of operating with repository/local sources and a local derived read model. Remote services may extend federation but should not be mandatory for the basic operating model.

## AD-014 — Validation must target semantics, not just schema shape

A record can be schema-valid while semantically wrong: mutable identity may be mislabeled immutable, a projection may be treated authoritative, or evidence may claim more than its method established.

Architecture consequence:

Validation requires layers:

1. syntactic/schema validation;
2. reference/integrity validation;
3. invariant validation;
4. resolver scenario tests; and
5. adapter conformance tests.

## AD-015 — Architecture must remain replaceable below semantic contracts

Persistence engines, search products, runtimes, and UIs will evolve.

Architecture consequence:

The stable contracts should be:

- portable semantic record meaning;
- adapter observation contracts;
- normalized semantic operations/resolution outcomes; and
- conformance fixtures.

The internal read-model technology should be replaceable without changing domain meaning.