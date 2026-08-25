# Project Knowledge

Project Knowledge is an exploratory software engineering project investigating how to preserve, organize, relate, query, and present the evolving knowledge of a complex engineering effort.

The project begins from a practical problem: large software projects produce more information, reasoning, history, decisions, evidence, and context than a person can reliably keep in working memory. Existing tools preserve pieces of that information, but the whole project record is difficult to maintain as an intelligible, navigable system.

## Status

**Phase 3 — Domain Modeling**

The first formal, evidence-traceable requirements baseline has been merged under [`docs/requirements/`](docs/requirements/). Current work is deriving the minimum technology-neutral semantic kernel under [`docs/domain/`](docs/domain/).

Discovery remains open under [`docs/discovery/`](docs/discovery/) where the evidence matrix still identifies under-evidenced concepts. The completed project foundation remains under [`docs/inception/`](docs/inception/).

No implementation architecture or technology stack has been selected.

## Current domain hypothesis

The requirements can be satisfied by a relatively small cross-tool semantic kernel rather than a universal ontology of every engineering artifact.

The candidate kernel centers on:

- native Source Systems and inspectable Native References;
- optional stable Subjects where cross-representation continuity matters;
- concrete Representations and their roles/lineage;
- Claims separated from source-bound Assertions;
- scoped Authority Assignments;
- typed Relationships only where semantics justify them;
- provenance Activities and Agents compatible with mature provenance concepts;
- conditional valid/recorded time and material Context;
- claim-relative Evidence Evaluations;
- optional epistemic annotations; and
- recovery Projections over the shared model.

Current truth, historical truth, contradiction diagnostics, freshness, impact, and recovery paths are treated as **derived views**, not independent truth stores.

A hard constraint remains: richer structure must be progressive. Projects must be able to continue using ordinary files, ordering, links, search, and Git when those already solve the relevant recovery problem.

## Core question

> How can a complex engineering project preserve its evolving knowledge and reasoning so that both current state and historical process remain intelligible, navigable, and useful?

## Development approach

1. Define the problem and project intent. **Complete**
2. Study real engineering information and existing approaches. **Initial discovery complete**
3. Derive requirements from evidence rather than preferred technology. **Initial requirements baseline complete**
4. Model the domain. **Current phase**
5. Design the architecture.
6. Build the smallest useful vertical prototype.
7. Use the project to document its own development and evaluate the model.

## Relationship to Monad

Project Knowledge was motivated by information-management problems encountered during development of the broader Monad engineering work. Monad provides an initial real-world corpus and use case, but Project Knowledge is maintained as a distinct project so its model can remain general-purpose.

## Repository principle

The repository records the durable project state. Conversations, experiments, and working sessions may generate ideas, but conclusions should become reviewable repository artifacts when they are mature enough to persist.
