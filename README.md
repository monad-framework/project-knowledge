# Project Knowledge

Project Knowledge is an exploratory software engineering project investigating how to preserve, organize, relate, query, and present the evolving knowledge of a complex engineering effort.

The project begins from a practical problem: large software projects produce more information, reasoning, history, decisions, evidence, and context than a person can reliably keep in working memory. Existing tools preserve pieces of that information, but the whole project record is difficult to maintain as an intelligible, navigable system.

## Status

**Phase 2 — Requirements**

The project has completed the first discovery sequence from real corpus evidence through existing-approach research and evidence-derived capability promotion. Formal technology-neutral requirements are now being specified under [`docs/requirements/`](docs/requirements/).

Discovery remains open under [`docs/discovery/`](docs/discovery/) where the evidence matrix still identifies under-evidenced concepts. The completed project foundation remains under [`docs/inception/`](docs/inception/).

No implementation architecture or technology stack has been selected.

## Working hypothesis

Project Knowledge is increasingly understood not as a replacement wiki, issue tracker, source-control system, or generic knowledge graph, but as a **low-friction project-memory layer** that can connect heterogeneous engineering artifacts through stable semantic identity, scoped authority, provenance, temporal and epistemic evolution, claim-relative evidence, and explicit relationships.

That shared memory can then support current-state, historical, impact, retrieval, provenance, and narrative views without replacing the native tools that created the information.

A second hypothesis has become a hard constraint: richer structure must remain progressive. Projects must be able to continue using ordinary files, ordering, links, search, and Git when those already solve the relevant recovery problem.

These statements remain subject to requirements review, domain modeling, and architecture validation.

## Core question

> How can a complex engineering project preserve its evolving knowledge and reasoning so that both current state and historical process remain intelligible, navigable, and useful?

## Development approach

1. Define the problem and project intent. **Complete**
2. Study real engineering information and existing approaches. **Initial discovery complete**
3. Derive requirements from evidence rather than preferred technology. **Current phase**
4. Model the domain.
5. Design the architecture.
6. Build the smallest useful vertical prototype.
7. Use the project to document its own development and evaluate the model.

## Relationship to Monad

Project Knowledge was motivated by information-management problems encountered during development of the broader Monad engineering work. Monad provides an initial real-world corpus and use case, but Project Knowledge is maintained as a distinct project so its model can remain general-purpose.

## Repository principle

The repository records the durable project state. Conversations, experiments, and working sessions may generate ideas, but conclusions should become reviewable repository artifacts when they are mature enough to persist.
