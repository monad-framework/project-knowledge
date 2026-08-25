# Candidate Architecture Evaluation

This pass compares architecture shapes before selecting technology.

The candidates are intentionally broad. A concrete implementation may use relational, document, graph, or other technologies internally, but the shape must preserve the domain invariants.

## Evaluation criteria

Candidates are evaluated against:

1. native authority preservation;
2. graceful degradation;
3. portable Project Knowledge-owned semantics;
4. minimal-project burden;
5. rich-project expressiveness;
6. current-state resolution;
7. relationship/provenance/evidence traversal;
8. incremental synchronization;
9. access-boundary preservation;
10. reconstructability;
11. derived-state rebuildability;
12. implementation complexity; and
13. technology lock-in.

Ratings:

- **S** — strong fit
- **P** — partial / requires compensating design
- **W** — weak fit
- **F** — structural conflict with requirements/invariants

---

## Candidate A — Central canonical knowledge database

### Shape

```text
Native systems
   ↓ ingest/copy
Central Project Knowledge database
   ↓
queries / UI / search
```

All integrated information is normalized into one database that becomes the principal source for current project knowledge.

### Strengths

- simple query boundary;
- straightforward global IDs and indexes;
- efficient cross-source traversal;
- centralized policy enforcement;
- convenient search and analytics.

### Structural problems

- encourages imported copies to become de facto authority;
- weakens native/offline usability and graceful degradation;
- makes portability dependent on export fidelity;
- creates pressure to ingest everything;
- makes the minimal Markdown/Git case disproportionately expensive;
- risks conflating canonical semantic records with query-optimized state.

### Invariant pressure

Conflicts or strongly pressures INV-001, INV-003, INV-027, INV-028, INV-030.

### Decision

**Reject as the primary architecture.**

A database may exist as a derived read model, but not as the sole canonical home for both native facts and Project Knowledge semantics.

---

## Candidate B — Pure repository-native sidecars

### Shape

```text
Repository files + Git
   ├── native project artifacts
   └── Project Knowledge YAML/JSON/Markdown sidecars

Queries scan/parse files directly.
```

### Strengths

- excellent portability and reviewability;
- strong Git integration;
- local/offline by default;
- graceful degradation;
- low conceptual infrastructure;
- native artifacts remain first-class.

### Structural problems

- cross-repository/external-source federation becomes awkward;
- global traversal and current-state resolution may require expensive repeated scans;
- remote source observations need a durable observation/index model;
- large-project incremental query performance is difficult without derived state;
- access partitioning and source synchronization become ad hoc;
- compound derived views still need freshness/dependency tracking.

### Decision

**Retain as the canonical persistence inspiration, but reject as the complete runtime architecture.**

Portable files are strong for Project Knowledge-owned durable semantics; a derived read model is still needed.

---

## Candidate C — Virtual federation / query-time adapters only

### Shape

```text
Native systems
  ├── adapter A
  ├── adapter B
  └── adapter C
       ↓
query-time federation
```

Little or no Project Knowledge state is persisted. Queries fetch native systems and derive cross-source meaning dynamically.

### Strengths

- native authority preserved;
- minimal duplication;
- easy source freshness in principle;
- no central migration requirement.

### Structural problems

Project Knowledge has genuine semantics that no source necessarily owns:

- Subject continuity;
- cross-tool bindings;
- Authority Assignments;
- authored cross-source Relationships;
- correction/provenance annotations;
- evidence evaluations;
- project policies.

Without durable Project Knowledge-owned records these semantics either disappear, become adapter configuration, or are hidden in an opaque service database.

Query-time dependence also weakens offline operation and historical reproducibility for remote APIs.

### Decision

**Reject as complete architecture.**

Federation remains essential, but requires a durable portable semantic layer.

---

## Candidate D — Event-sourced Project Knowledge core

### Shape

```text
Native observations + authored actions
         ↓
append-only Project Knowledge event log
         ↓
materialized projections
```

### Strengths

- strong audit history;
- natural projection rebuilding;
- explicit correction/change chronology;
- good incremental processing model.

### Structural problems

- domain requirements explicitly do not require event sourcing;
- native Git/source history would be duplicated into another event model;
- simple projects pay event-model complexity without need;
- valid-time and authority are not solved merely by event order;
- historical source reconstruction may still depend on native systems;
- event identity and schema evolution become architectural commitments early.

### Decision

**Reject as mandatory architecture.**

Append/change journals may be useful internally for synchronization, but they are not the canonical semantic foundation.

---

## Candidate E — Graph-first canonical knowledge store

### Shape

```text
Native sources
   ↓ ingest/map
canonical graph
   ↓
traversal / search / views
```

### Strengths

- excellent relationship traversal;
- natural many-to-many representation;
- flexible schema evolution;
- provenance and impact queries fit well.

### Structural problems

- graph representation does not itself solve authority, temporal semantics, evidence scope, or source fidelity;
- encourages every artifact to become a node;
- canonical graph can become another central imported source of truth;
- poor graceful degradation if the graph is the sole durable semantic store;
- representation technology becomes entangled with domain meaning.

### Decision

**Reject as canonical architecture.**

A graph-shaped read model or graph-capable query layer remains compatible with the selected architecture.

---

## Candidate F — Federated portable core + disposable derived read models

### Shape

```text
             Native engineering systems
          ┌────────┼──────────────┐
          │        │              │
        files    issues           CI / external
          │        │              │
          └──── adapters/observations ────┐
                                          │
Portable Project Knowledge semantic records
(only PK-owned semantics, versionable/exportable)
                    │
                    ▼
         normalization / synchronization
                    │
                    ▼
          disposable derived read model
          ├── identities/representations
          ├── relationships/provenance
          ├── claims/authority/time
          ├── evidence/freshness
          └── source/access lineage
                    │
        ┌───────────┼───────────┐
        ▼           ▼           ▼
     resolver    traversal    retrieval
        │           │           │
        └───────────┼───────────┘
                    ▼
               projections/UI
```

### Strengths

- native authority preserved;
- portable Project Knowledge-owned semantics;
- derived query performance without canonical lock-in;
- simple projects can remain almost entirely native;
- rich projects add semantics incrementally;
- source federation and offline/local use coexist;
- read models can be relational, graph-shaped, indexed, or hybrid without changing domain semantics;
- deterministic rebuild is possible where inputs are available;
- current truth remains a resolver outcome rather than destructive state.

### Costs

- requires explicit canonical-vs-derived discipline;
- synchronization/adapters become important infrastructure;
- source access boundaries must propagate into derived state;
- partial source availability must be represented honestly;
- portable semantic record design requires careful versioning.

### Decision

**Select.**

The costs are implementation concerns rather than contradictions with the domain model.

---

## Decision matrix

| Criterion | A Central DB | B Sidecars only | C Virtual federation | D Event sourced | E Graph-first | F Federated portable core |
| --- | --- | --- | --- | --- | --- | --- |
| Native authority | W | **S** | **S** | P | W | **S** |
| Graceful degradation | W | **S** | P | P | W | **S** |
| Portable PK semantics | P | **S** | W | P | P | **S** |
| Minimal-project burden | W | **S** | **S** | W | W | **S** |
| Rich-project expressiveness | **S** | P | P | **S** | **S** | **S** |
| Current-state resolution | **S** | P | P | **S** | **S** | **S** |
| Traversal/provenance | **S** | P | P | **S** | **S** | **S** |
| Incremental synchronization | **S** | P | P | **S** | **S** | **S** |
| Access-boundary preservation | **S** | P | P | P | P | **S** with explicit lineage |
| Reconstructability | P | **S** | W/P | **S** if sources retained | P | **S** |
| Derived-state rebuildability | P | n/a/P | n/a | **S** | P | **S** |
| Low technology lock-in | W | **S** | P | W | W | **S** |

## Selected architecture

Candidate F is selected because it is the only shape that treats all three categories correctly:

```text
native authoritative facts
        ≠
Project Knowledge-owned cross-system semantics
        ≠
derived/query-optimized state
```

Conflating any two of those categories recreates one or more of the original discovery failure modes.