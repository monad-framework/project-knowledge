# Project Knowledge

Project Knowledge is an exploratory software engineering project investigating how to preserve, organize, relate, query, and present the evolving knowledge of a complex engineering effort.

The project begins from a practical problem: large software projects produce more information, reasoning, history, decisions, evidence, and context than a person can reliably keep in working memory. Existing tools preserve pieces of that information, but the whole project record is difficult to maintain as an intelligible, navigable system.

## Status

**Phase 6 — Dogfooding and Real-Project Validation**

The first evidence-traceable requirements baseline, minimal semantic domain kernel, federated portable-core architecture, executable M0 vertical slice, and first evidence-authorized authoring increment are complete under [`docs/requirements/`](docs/requirements/), [`docs/domain/`](docs/domain/), [`docs/architecture/`](docs/architecture/), [`docs/m0/`](docs/m0/), and [`docs/authoring/`](docs/authoring/).

M0 demonstrates the selected architecture against its first executable falsification scenarios. The closure record is [`docs/m0/closure.md`](docs/m0/closure.md).

Two self-dogfood experiments are complete under [`docs/dogfooding/`](docs/dogfooding/):

- DF-001 recovered historical/current ADR status and evidence;
- DF-002 preserved an unresolved technology question, considered alternatives, the later selected answer, and implementation evidence.

Both passed using the existing M0 semantic kernel. Across the two cases, the repository accumulated 24 portable S2 records and independently reproduced the same manual structural-authoring burden.

That repeated evidence authorized **CA-1 — Guided Capture and Scaffolding**. CA-1 implements a plan → review → apply boundary so `pk` can generate UUIDs, canonical record paths, envelopes, ordinary capture timestamps, and relevant Git source-state identity without silently deciding semantic identity, authority, evidence breadth, valid time, or epistemic meaning. Its closure record is [`docs/authoring/closure.md`](docs/authoring/closure.md).

The next Phase 6 step is **DF-003**, which must use CA-1 rather than hand-authoring another S2 bundle.

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
- the `pk` CLI/library as the local runtime surface.

CA-1 adds two explicitly noncanonical operational layers:

- `pk-authoring/v1` for compact semantic Authoring Intent; and
- `pk-capture-plan/v1` for immutable reviewable Capture Plans.

They compile into ordinary `pk/v1` records. Deleting an applied plan does not change project-memory reconstruction.

The capture command family is:

```text
pk capture
pk capture plan [--intent <file|->] [--out <file>] [--json]
pk capture apply --plan <file> [--yes] [--json]
```

Current truth remains a derived resolution over Claims, Assertions, scoped authority, valid time, Context, policy, and source state. Richer structure remains progressive: `pk status` can observe an ordinary Markdown + Git project without requiring any persistent Project Knowledge records or use of the capture layer.

## Core question

> How can a complex engineering project preserve its evolving knowledge and reasoning so that both current state and historical process remain intelligible, navigable, and useful?

## Development approach

1. Define the problem and project intent. **Complete**
2. Study real engineering information and existing approaches. **Initial discovery complete**
3. Derive requirements from evidence rather than preferred technology. **Initial requirements baseline complete**
4. Model the domain. **Initial semantic kernel complete**
5. Design the architecture. **Initial architecture complete**
6. Build the smallest useful vertical prototype. **M0 complete**
7. Use the project to document its own development and evaluate the model. **Current phase — dogfooding; CA-1 complete; DF-003 next**

## Current Phase 6 objective

The immediate question is no longer whether deterministic authoring scaffolding can be implemented. CA-1 demonstrates that it can.

The next question is whether that layer materially improves real engineering use.

DF-003 should therefore:

1. begin from a real recovery problem rather than a synthetic fixture;
2. express the semantic intent through CA-1;
3. generate and review the Capture Plan;
4. apply the resulting S2 records;
5. verify the intended recovery result through the compiler/resolver; and
6. measure what manual semantic and interaction burden remains.

Product changes after CA-1 should again require evidence from dogfooding rather than feature-list speculation.

## Relationship to Monad

Project Knowledge was motivated by information-management problems encountered during development of the broader Monad engineering work. Monad provides an initial real-world corpus and use case, but Project Knowledge is maintained as a distinct project so its model can remain general-purpose.

## Repository principle

The repository records the durable project state. Conversations, experiments, and working sessions may generate ideas, but conclusions should become reviewable repository artifacts when they are mature enough to persist.
