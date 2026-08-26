# Project Knowledge

Project Knowledge is an exploratory software engineering project investigating how to preserve, organize, relate, query, and present the evolving knowledge of a complex engineering effort.

The project begins from a practical problem: large software projects produce more information, reasoning, history, decisions, evidence, and context than a person can reliably keep in working memory. Existing tools preserve pieces of that information, but the whole project record is difficult to maintain as an intelligible, navigable system.

## Status

**Phase 6 — Dogfooding and Real-Project Validation**

The first evidence-traceable requirements baseline, minimal semantic domain kernel, federated portable-core architecture, executable M0 vertical slice, and first evidence-authorized authoring increment are complete under [`docs/requirements/`](docs/requirements/), [`docs/domain/`](docs/domain/), [`docs/architecture/`](docs/architecture/), [`docs/m0/`](docs/m0/), and [`docs/authoring/`](docs/authoring/).

M0 demonstrates the selected architecture against its first executable falsification scenarios. The closure record is [`docs/m0/closure.md`](docs/m0/closure.md).

Three self-dogfood experiments are complete under [`docs/dogfooding/`](docs/dogfooding/):

- DF-001 recovered historical/current ADR status and evidence;
- DF-002 preserved an unresolved technology question, considered alternatives, the later selected answer, and implementation evidence; and
- DF-003 used CA-1 itself to preserve the evidence → promoted capability → design decision → implementation → verification chain for Guided Capture and Scaffolding.

DF-001 and DF-002 independently reproduced the same manual structural-authoring burden across 24 portable S2 records. That repeated evidence authorized **CA-1 — Guided Capture and Scaffolding**.

CA-1 implements a plan → review → apply boundary so `pk` can generate UUIDs, canonical record paths, envelopes, ordinary capture timestamps, and relevant Git source-state identity without silently deciding semantic identity, authority, evidence breadth, valid time, or epistemic meaning. Its closure record is [`docs/authoring/closure.md`](docs/authoring/closure.md).

DF-003 validated CA-1 in real use: a reviewed 15-operation Capture Plan safely produced 15 S2 records with zero hand-authored UUIDs, output paths, Git blob identities, portable-record envelopes, or capture timestamps. The repository now contains 39 self-dogfood S2 records across three recovery threads.

DF-003 also exposed the strongest current Phase 6 gap: Project Knowledge can represent reasoning/provenance Relationships that the current CLI cannot yet recover directly. `pk resolve` answers scoped current-state questions, but there is no user-facing traversal/explanation surface for questions such as “what motivated this?”, “which decision governs it?”, or “what verifies this Claim?”

A **Retrieval and Traversal Detailed Design** is now proposed under [`docs/retrieval/`](docs/retrieval/). It separates a neutral bounded semantic traversal engine from a human-oriented recovery projection and proposes **RT-1 — Semantic Recovery Traversal** as the next evidence-authorized implementation increment if the design is accepted.

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
7. Use the project to document its own development and evaluate the model. **Current phase — three self-dogfood cases complete; retrieval/traversal detailed design proposed**

## Current Phase 6 objective

CA-1 has now been validated on a real self-dogfood capture rather than only against isolated acceptance fixtures.

The immediate question has shifted from **how to author the semantics safely** to **how to recover the represented reasoning without exposing raw storage mechanics**.

DF-003 establishes a concrete failed recovery path:

```text
stored successfully
DF-001/DF-002 ──motivates──> authoring design
ADR-0003 ──governs_design_of──> authoring design
CA-1 closure ──verifies──> implementation Claim

user-facing recovery today
pk resolve capability_status  ✓
recover the reasoning chain   ✗ direct CLI surface
```

The proposed retrieval design selects this architecture:

```text
S1 + S2
  ↓
bounded semantic traversal
  ↓
structured recovery explanation
  ↓
pk explain

same traversal primitive
  ↓
pk trace
```

The design intentionally does not begin with a graph database, graph browser, LLM, semantic search, or automatic causal inference. Those remain separate evidence questions.

A richer guided capture UX also remains a candidate because DF-003 Authoring Intent still contains safely reducible alias/kind/reference ceremony. Semantic auto-decision, fuzzy identity resolution, and AI inference remain unauthorized.

## Relationship to Monad

Project Knowledge was motivated by information-management problems encountered during development of the broader Monad engineering work. Monad provides an initial real-world corpus and use case, but Project Knowledge is maintained as a distinct project so its model can remain general-purpose.

## Repository principle

The repository records the durable project state. Conversations, experiments, and working sessions may generate ideas, but conclusions should become reviewable repository artifacts when they are mature enough to persist.
