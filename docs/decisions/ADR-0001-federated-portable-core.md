# ADR-0001 — Federated Portable Core with Rebuildable Derived Read Models

- **Status:** Proposed for acceptance with Architecture PR
- **Date:** 2026-08-25
- **Decision scope:** Project Knowledge system architecture

## Context

Project Knowledge must integrate heterogeneous engineering artifacts while preserving native authority, source identity, historical meaning, access boundaries, and progressive adoption.

The domain model also requires durable cross-system semantics that no native tool necessarily owns, including semantic Subject identity, Representation bindings, scoped authority, cross-system Relationships, provenance extensions, Evidence Evaluations, and selected correction/context records.

At the same time, the system needs efficient traversal, current-state resolution, provenance reconstruction, contradiction diagnostics, impact analysis, and retrieval.

These pressures create three distinct classes of state:

1. native authoritative state;
2. Project Knowledge-owned semantic state; and
3. derived/query-optimized state.

## Decision

Adopt a **federated portable-core architecture**:

- native engineering systems remain authoritative within declared scopes;
- Project Knowledge persists only the durable cross-system semantics it owns in a portable, inspectable record representation;
- adapters observe native systems without converting observations directly into truth;
- a semantic compiler/synchronizer combines accessible native observations with portable records;
- disposable/rebuildable read models optimize traversal, resolution, diagnostics, and retrieval;
- current truth remains a resolver result, not a destructive stored master record;
- derived projections retain lineage and do not gain authority through materialization.

## Considered alternatives

### Central canonical knowledge database

Rejected as primary architecture because it pressures native authority, portability, graceful degradation, and minimal-project operation.

### Repository sidecars only, queried directly

Retained as the portable-record persistence inspiration but insufficient alone for federation, project-scale traversal, incremental synchronization, and optimized recovery views.

### Virtual federation with no durable Project Knowledge records

Rejected because cross-system semantic identity, authority, corrections, evidence semantics, and authored Relationships require durable ownership somewhere.

### Mandatory event-sourced core

Rejected because event history is not the same as authority/valid-time semantics and would impose unnecessary ceremony on small projects.

### Graph-first canonical store

Rejected because graph representation solves traversal, not authority/time/evidence/source-fidelity, and would couple domain semantics to one storage paradigm.

## Consequences

### Positive

- native tools remain usable independently;
- Project Knowledge state is portable and inspectable;
- query/index technologies remain replaceable;
- minimal and rich projects share one operating model;
- local/offline operation is natural;
- full rebuild provides a strong correctness/recovery path;
- current-state logic remains centralized in explainable resolvers;
- search/AI can be added without becoming truth infrastructure.

### Negative

- canonical-vs-derived discipline must be enforced consistently;
- adapters and synchronization become core infrastructure;
- access restrictions must propagate through derived state;
- portable record versioning/schema design becomes a compatibility concern;
- partial native-source availability must be modeled explicitly;
- there are more moving concepts than in a single-database CRUD application.

## Invariants protected

This decision is primarily motivated by:

- INV-001 native authority;
- INV-003 semantic identity optional;
- INV-012 current truth derived;
- INV-020 derived lineage;
- INV-025 storage independence;
- INV-027 object-scoped progressive formalization;
- INV-028 native usability;
- INV-029 access boundaries;
- INV-030 projection authority safety;
- INV-031 unknown as first-class outcome.

All 32 domain invariants remain binding.

## Reconsideration triggers

Revisit this decision if implementation evidence shows that:

- Project Knowledge semantics cannot remain portable without extensive native-data duplication;
- derived read models cannot be reliably rebuilt;
- source/access lineage cannot survive federation;
- current-state resolution requires architecture that conflicts with the portable core;
- the minimal-project case becomes materially burdensome; or
- a fundamentally different architecture demonstrates better invariant preservation with lower total complexity.

## Next decision

After this ADR is accepted, the M0 detailed-design/bootstrap pass should select:

- concrete portable serialization + schema standard;
- identifier encoding;
- embedded read-model technology;
- implementation language/runtime;
- repository layout;
- initial CLI/library boundary;
- Git adapter contract; and
- executable fixture/conformance harness.