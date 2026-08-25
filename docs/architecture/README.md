# Project Knowledge Architecture

This directory defines the first technology-neutral architecture for Project Knowledge.

The architecture is derived from the merged requirements and domain invariants. It does not begin from a database, framework, graph engine, search product, or deployment preference.

## Status

**Architecture draft — implementation kernel not yet authorized.**

The architecture must preserve the domain model in `docs/domain/`, especially the invariants in `docs/domain/invariants.md` and the architecture-entry scenarios in `docs/domain/open-questions.md`.

## Architectural thesis

The selected architectural shape is a **federated portable core with disposable derived read models**.

In concise form:

```text
Native engineering systems
(files, Git, issues, ADRs, CI, external sources)
        │
        │ observe / reference through adapters
        ▼
Portable Project Knowledge records
(only Project Knowledge-owned semantics)
        │
        │ deterministic or explainable compilation
        ▼
Derived local read model / indexes
        │
        ├── current-state resolution
        ├── relationship traversal
        ├── provenance / evidence views
        ├── freshness / contradiction diagnostics
        └── retrieval support
```

Native systems remain authoritative for their declared scopes. Project Knowledge persists only the cross-system semantics it must own: semantic identity bindings, explicit relationships, authority assignments/policies, retained claims/assertions when necessary, provenance extensions, context, evidence evaluations, corrections, and other Project Knowledge-managed facts.

Derived databases, indexes, caches, and search structures are rebuildable conveniences rather than the sole authoritative home of those semantics.

## Why this shape

The architecture must satisfy both extremes:

1. a small project that should remain ordinary Markdown + Git with almost no Project Knowledge-owned data; and
2. a complex project spanning multiple native systems with semantic identity, authority, provenance, time, evidence, derived views, and retrieval.

A central knowledge database over-owns native truth and weakens graceful degradation. A pure repository scan/index has nowhere durable to own Project Knowledge-specific semantics. A virtual federation with no portable semantic records cannot preserve authored cross-source decisions when adapters or indexes disappear.

The selected shape separates those concerns.

## Architecture layers

### A1 — Native source layer

Existing engineering systems and artifacts. They remain independently usable and may remain authoritative.

### A2 — Integration/adaptation layer

Adapters expose native identity, immutable source-state identity where available, structured native state, authorization boundaries, and change observations without rewriting source meaning.

### A3 — Portable semantic record layer

Durable Project Knowledge-owned records. This is the portable canonical representation for Project Knowledge-specific semantics, not a replacement canonical repository for all engineering artifacts.

### A4 — Compilation/synchronization layer

Resolves adapters and semantic records into a normalized derived model, preserves lineage, detects changed dependencies, and records unknown/error states rather than silently guessing.

### A5 — Derived read-model layer

Disposable/rebuildable structures optimized for traversal, resolution, diagnostics, retrieval, and projections.

### A6 — Query/resolution layer

Implements semantic operations such as identity lookup, current-state resolution, contradiction classification, impact traversal, evidence freshness, and provenance reconstruction.

### A7 — Presentation/integration surfaces

CLI, API, wiki, web UI, IDE integration, generated docs, or other consumers. No primary UI is selected by this architecture.

## Contents

- [`architecture-drivers.md`](architecture-drivers.md) — architectural forces and non-negotiable invariants
- [`candidate-architectures.md`](candidate-architectures.md) — candidate shapes and rejection analysis
- [`selected-architecture.md`](selected-architecture.md) — component responsibilities and dependency direction
- [`portable-records.md`](portable-records.md) — what Project Knowledge persists canonically and what it does not
- [`integration-and-sync.md`](integration-and-sync.md) — adapters, source observations, incremental synchronization, lineage
- [`read-models-and-resolution.md`](read-models-and-resolution.md) — disposable derived state and semantic resolvers
- [`retrieval-security-and-operations.md`](retrieval-security-and-operations.md) — search boundary, access boundaries, local/offline behavior, recovery
- [`validation-and-m0.md`](validation-and-m0.md) — architecture conformance and first vertical slice
- [`traceability.md`](traceability.md) — domain/requirements to architecture mapping
- [`open-questions.md`](open-questions.md) — choices intentionally deferred to detailed design or implementation

## Dependency rule

Higher layers may depend on lower-layer contracts, but the semantic kernel must not depend on a specific storage engine, search engine, UI, or integration product.

The portable record semantics are the long-lived compatibility boundary. Derived infrastructure is replaceable.

## Architecture anti-goals

This architecture is not:

- a universal knowledge database;
- a Git replacement;
- a wiki replacement;
- an issue tracker replacement;
- an event-sourcing mandate;
- a graph-database mandate;
- an RDF/OWL mandate;
- a temporal-database mandate;
- an AI/RAG architecture;
- a SaaS-only design; or
- a requirement that all project artifacts be imported.

## Architecture exit criterion

This phase is sufficient to authorize an M0 implementation kernel when:

1. every domain invariant has an architectural preservation mechanism;
2. the eight architecture-entry scenarios have explicit execution paths;
3. canonical versus derived state is unambiguous;
4. source authority and access boundaries survive federation;
5. the minimal-project case requires near-zero ceremony;
6. the rich-project case can add semantics without changing operating model;
7. derived state is rebuildable from authoritative/native inputs plus portable semantic records; and
8. the M0 vertical slice is small enough to implement and falsify the architecture.