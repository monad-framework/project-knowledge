# Project Knowledge

Project Knowledge is an exploratory software engineering project investigating how to preserve, organize, relate, query, and present the evolving knowledge of a complex engineering effort.

The project begins from a practical problem: large software projects produce more information, reasoning, history, decisions, evidence, and context than a person can reliably keep in working memory. Existing tools preserve pieces of that information, but the whole project record is difficult to maintain as an intelligible, navigable system.

## Status

**Phase 4 — Architecture**

The first evidence-traceable requirements baseline is merged under [`docs/requirements/`](docs/requirements/), and the first minimal semantic domain kernel is merged under [`docs/domain/`](docs/domain/).

Current work is defining the technology-neutral system architecture under [`docs/architecture/`](docs/architecture/).

Discovery remains open under [`docs/discovery/`](docs/discovery/) where the evidence matrix still identifies under-evidenced concepts. The completed project foundation remains under [`docs/inception/`](docs/inception/).

No implementation language, database engine, graph technology, search product, UI framework, or deployment platform has been selected.

## Current architecture direction

The domain model supports a relatively small cross-tool semantic kernel rather than a universal ontology of every engineering artifact.

Architecture now separates three classes of state:

1. **Native authoritative state** — files, Git, issues, ADRs, CI evidence, external sources, and other native systems remain authoritative within their declared scopes.
2. **Portable Project Knowledge semantic state** — only cross-system semantics that Project Knowledge itself owns are persisted canonically, such as Subject continuity, Representation bindings, scoped authority, selected Claims/Assertions, Relationships, provenance/context/evidence records, and project policy.
3. **Derived/query-optimized state** — indexes, relationship adjacency, current-state caches, freshness diagnostics, retrieval indexes, and generated projections are rebuildable conveniences rather than independent sources of truth.

The selected architectural shape is a **federated portable core with disposable derived read models**.

Current truth remains a derived resolution over Claims, Assertions, scoped authority, time, Context, policy, and source state. Richer structure remains progressive: an ordinary Markdown + Git project can participate with near-zero additional modeling.

## Core question

> How can a complex engineering project preserve its evolving knowledge and reasoning so that both current state and historical process remain intelligible, navigable, and useful?

## Development approach

1. Define the problem and project intent. **Complete**
2. Study real engineering information and existing approaches. **Initial discovery complete**
3. Derive requirements from evidence rather than preferred technology. **Initial requirements baseline complete**
4. Model the domain. **Initial semantic kernel complete**
5. Design the architecture. **Current phase**
6. Build the smallest useful vertical prototype.
7. Use the project to document its own development and evaluate the model.

## Relationship to Monad

Project Knowledge was motivated by information-management problems encountered during development of the broader Monad engineering work. Monad provides an initial real-world corpus and use case, but Project Knowledge is maintained as a distinct project so its model can remain general-purpose.

## Repository principle

The repository records the durable project state. Conversations, experiments, and working sessions may generate ideas, but conclusions should become reviewable repository artifacts when they are mature enough to persist.