# Selected Architecture — Federated Portable Core

## Decision

Project Knowledge will use a **federated portable-core architecture with rebuildable derived read models**.

This document defines component boundaries and dependency direction. It does not select implementation language, database engine, graph engine, or UI framework.

## System context

```text
                         ┌─────────────────────────────┐
                         │       Project Knowledge     │
                         │                             │
Native sources ─Adapters─┤ Portable semantic records │
                         │           │                 │
                         │      Compiler / Sync        │
                         │           │                 │
                         │      Derived read model     │
                         │           │                 │
                         │   Resolution / Query APIs   │
                         └───────────┬─────────────────┘
                                     │
                            UIs / CLI / docs / APIs
```

## State classes

### S1 — Native authoritative state

Examples:

- repository files and Git objects;
- issue/work-item fields;
- ADR documents;
- CI/test records;
- external standards;
- other integrated systems.

Project Knowledge may cache/index these, but the native source may remain authoritative for its declared concern.

### S2 — Project Knowledge authoritative semantic state

Durable semantics that Project Knowledge itself must own because no native source necessarily does.

Examples:

- Subject identifiers;
- Representation bindings;
- cross-system Relationships;
- Authority Assignments and project authority policy;
- Project Knowledge-authored Claims/Assertions;
- provenance Activities not fully represented by a native source;
- Context retained intentionally for interpretation/reconstruction;
- Evidence Evaluations;
- Epistemic Annotations;
- correction/refinement relationships;
- project configuration/policy necessary to interpret these records.

S2 is stored in a portable, inspectable representation.

### S3 — Derived state

Examples:

- normalized source observations;
- reverse links;
- graph/relationship adjacency;
- current-state resolution caches;
- freshness classifications;
- contradiction diagnostics;
- retrieval indexes;
- search embeddings;
- generated summaries;
- impact indexes.

S3 is always rebuildable from accessible S1 plus S2, subject to source availability and retention policy.

## Core components

### C1 — Source Adapter

Responsibility:

- identify a Source System;
- enumerate or resolve Native References;
- expose source-specific structured observations;
- expose immutable/reconstructable source-state identity when available;
- expose source change/version information;
- preserve authorization/access metadata needed downstream;
- never silently reinterpret a source representation as authoritative outside its native scope.

Adapter output is an **observation contract**, not a universal imported object model.

### C2 — Portable Record Store

Responsibility:

- persist S2 Project Knowledge-owned semantics;
- retain record version/schema identity;
- remain human-inspectable and source-control friendly;
- support deterministic parsing and validation;
- avoid dependence on a particular runtime/database;
- support partial/project-scoped adoption.

The logical record model is canonical. Concrete physical organization may be optimized later as long as portability and stable identity are preserved.

### C3 — Semantic Compiler / Synchronizer

Responsibility:

- read Project Knowledge records;
- invoke adapters for needed native observations;
- normalize references without losing source identity;
- resolve record/reference dependencies;
- build or update S3 read models;
- retain derivation/source lineage;
- classify missing/ambiguous source state as unknown/error rather than inventing values;
- propagate access partitions.

The compiler is incremental where feasible, but full deterministic rebuild remains a conformance path.

### C4 — Derived Read Model

Responsibility:

Provide query-efficient normalized state for:

- Subjects and Representation bindings;
- Claims and Assertions;
- Authority Assignments;
- Relationships;
- provenance Activities;
- temporal qualifiers;
- Context;
- Evidence Evaluations;
- source/access lineage;
- derived dependency relationships.

The architecture does not require one physical read model. A conforming implementation may use one embedded relational store, multiple indexes, or other replaceable structures.

### C5 — Resolution Engine

Responsibility:

Implement domain semantics that must not be delegated to search/UI code:

- semantic identity lookup/binding status;
- authority resolution;
- current-state resolution;
- contradiction classification;
- evidence freshness/validity status where dependencies are known;
- derivation freshness;
- historical/as-of interpretation;
- unknown/unresolved outcomes.

Resolvers must be explainable: outputs carry basis/lineage sufficient for QA-004.

### C6 — Traversal / Impact Engine

Responsibility:

- typed relationship traversal;
- reverse-dependency traversal;
- provenance/derivation path reconstruction;
- impact candidate generation;
- recovery paths over recorded relationships.

It must distinguish asserted/imported/derived/inferred links.

### C7 — Retrieval Index

Responsibility:

- lexical/exact lookup;
- optional semantic retrieval;
- source-aware filtering;
- semantic metadata filtering/ranking;
- citations back to native references and semantic records.

It must not determine authority, identity, truth, or causality.

### C8 — Projection Layer

Responsibility:

Expose purpose-specific views such as:

- current-state view;
- history/timeline;
- provenance/evidence view;
- contradiction view;
- impact view;
- context-recovery summary;
- generated indexes;
- inputs for authored narrative.

Projection materialization does not grant authority.

### C9 — Conformance / Validation Engine

Responsibility:

- schema/syntax validation;
- reference integrity;
- domain invariant checks;
- adapter contract checks;
- resolution scenario tests;
- rebuild equivalence checks where declared deterministic;
- access-boundary tests.

## Dependency direction

The durable semantic contracts must not depend on replaceable infrastructure.

```text
Domain semantics / record contracts
            ↑
Adapter + compiler contracts
            ↑
Read model / storage / indexes
            ↑
Query implementations
            ↑
UI / CLI / API surfaces
```

A UI may depend on resolver APIs. A resolver may depend on a read-model contract. The domain meaning of Authority Assignment must not depend on a specific SQL table, graph edge type, or UI control.

## Write paths

### Native write path

Project Knowledge does not own ordinary native changes:

```text
user/tool → native system → adapter observes new state → compiler updates derived model
```

### Semantic write path

For Project Knowledge-owned semantics:

```text
user/tool → validated portable semantic record → source control/storage → compiler → derived model
```

A future API may author records, but it must produce the same portable canonical semantics.

### Derived write path

```text
compiler/resolver/indexer → disposable S3 storage
```

Derived state must never be the only retained copy of an S2 semantic decision.

## Read path

```text
query
  ↓
resolver / traversal / retrieval
  ↓
derived read model
  ↓ when explanation/source required
portable semantic record + native reference/source state
```

## Partial availability

Source systems may be unavailable.

Architecture behavior:

- retain last known observations only if policy allows;
- label observation/source-state age and lineage;
- return unavailable/unknown when fresh source state is required;
- do not silently treat cached observations as current authority merely because the source is unreachable.

## Local and remote operation

The architecture supports:

### Local mode

- repository/local adapters;
- portable semantic records in local/version-controlled storage;
- embedded derived read model;
- no always-on service required.

### Service-assisted mode

- remote adapters;
- shared synchronization service;
- centralized or replicated derived read model;
- access-controlled multi-user query surfaces.

Both modes must preserve the same semantic contracts. Service mode is an extension, not a different domain model.

## Failure containment

### Corrupt/missing derived read model

Rebuild from S1 + S2.

### Search index unavailable

Exact/source access and semantic resolver behavior remain available where the core read model is intact.

### Adapter unavailable

Queries depending on fresh native state return unavailable/unknown or last-known-with-age according to policy.

### Portable semantic records invalid

Compilation fails closed for affected records and reports diagnostics; invalid records do not silently enter authoritative resolution.

### Conflicting authority

Resolver returns unresolved conflict with basis; it does not pick by import order or retrieval rank.

## Architecture consequence

Project Knowledge is best understood as a **semantic compiler and recovery runtime over federated engineering state**, with a portable authored semantic layer and disposable optimized projections.