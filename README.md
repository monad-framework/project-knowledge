# Project Knowledge

Project Knowledge is an exploratory software engineering project investigating how to preserve, organize, relate, query, and present the evolving knowledge of a complex engineering effort.

The project begins from a practical problem: large software projects produce more information, reasoning, history, decisions, evidence, and context than a person can reliably keep in working memory. Existing tools preserve pieces of that information, but the whole project record is difficult to maintain as an intelligible, navigable system.

## Status

**Phase 6 — Dogfooding and Real-Project Validation**

The first evidence-traceable requirements baseline, minimal semantic domain kernel, federated portable-core architecture, and executable M0 vertical slice are complete under [`docs/requirements/`](docs/requirements/), [`docs/domain/`](docs/domain/), [`docs/architecture/`](docs/architecture/), and [`docs/m0/`](docs/m0/).

M0 demonstrates the selected architecture against its first executable falsification scenarios. The closure record is [`docs/m0/closure.md`](docs/m0/closure.md).

Current work should now use the executable kernel on Project Knowledge itself and representative Monad material to measure real capture burden, recovery value, missing semantics, and operational friction before authorizing broader capability expansion.

Discovery remains open under [`docs/discovery/`](docs/discovery/) where the evidence matrix still identifies under-evidenced concepts. The completed project foundation remains under [`docs/inception/`](docs/inception/).

## Current implementation

The selected architecture separates three classes of state:

1. **S1 — Native authoritative state** — files, Git, issues, ADRs, CI evidence, external sources, and other native systems remain authoritative within their declared scopes.
2. **S2 — Portable Project Knowledge semantic state** — only cross-system semantics that Project Knowledge itself owns are persisted canonically, such as Subject continuity, Representation bindings, scoped authority, selected Claims/Assertions, Relationships, provenance/context/evidence records, and project policy.
3. **S3 — Derived/query-optimized state** — indexes, current-state results, freshness diagnostics, retrieval support, and generated projections are rebuildable conveniences rather than independent sources of truth.

M0 makes that architecture executable using:

- Rust 1.98 / edition 2024;
- JSON + JSON Schema Draft 2020-12 for portable S2 records;
- UUIDv4 for Project Knowledge-generated identifiers;
- SQLite as disposable S3;
- the native `git` executable as the first S1 adapter; and
- the `pk` CLI/library as the first runtime surface.

Current truth remains a derived resolution over Claims, Assertions, scoped authority, valid time, Context, policy, and source state. Richer structure remains progressive: `pk status` can observe an ordinary Markdown + Git project without requiring any persistent Project Knowledge records.

## Core question

> How can a complex engineering project preserve its evolving knowledge and reasoning so that both current state and historical process remain intelligible, navigable, and useful?

## Development approach

1. Define the problem and project intent. **Complete**
2. Study real engineering information and existing approaches. **Initial discovery complete**
3. Derive requirements from evidence rather than preferred technology. **Initial requirements baseline complete**
4. Model the domain. **Initial semantic kernel complete**
5. Design the architecture. **Initial architecture complete**
6. Build the smallest useful vertical prototype. **M0 complete**
7. Use the project to document its own development and evaluate the model. **Current phase — dogfooding**

## Dogfooding objective

The immediate question is no longer whether the architecture can be implemented. It is whether the implemented semantics are worth using in real engineering work.

The next evidence pass should determine:

- which real project facts deserve S2 representation;
- which relationships and authority rules are worth the capture cost;
- which metadata can be inferred safely and which requires explicit authorship;
- whether current-state, historical, freshness, evidence, and identity recovery materially reduce context loss;
- where the M0 vocabulary is too coarse, too verbose, or incomplete; and
- which capability should become the first genuine post-M0 product increment.

## Relationship to Monad

Project Knowledge was motivated by information-management problems encountered during development of the broader Monad engineering work. Monad provides an initial real-world corpus and use case, but Project Knowledge is maintained as a distinct project so its model can remain general-purpose.

## Repository principle

The repository records the durable project state. Conversations, experiments, and working sessions may generate ideas, but conclusions should become reviewable repository artifacts when they are mature enough to persist.
