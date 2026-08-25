# Project Knowledge

Project Knowledge is an exploratory software engineering project investigating how to preserve, organize, relate, query, and present the evolving knowledge of a complex engineering effort.

The project begins from a practical problem: large software projects produce more information, reasoning, history, decisions, evidence, and context than a person can reliably keep in working memory. Existing tools preserve pieces of that information, but the whole project record is difficult to maintain as an intelligible, navigable system.

## Status

**Phase 1 — Discovery**

The project is currently building and stress-testing an evidence corpus before deriving stable requirements or selecting an implementation architecture.

Current work lives under [`docs/discovery/`](docs/discovery/). The completed project foundation remains under [`docs/inception/`](docs/inception/).

## Working hypothesis

A useful engineering knowledge system may need to preserve more than documents. It may need to represent knowledge objects, their relationships, provenance, temporal evolution, epistemic state, and multiple projections over the same underlying information.

Discovery has already refined this hypothesis: relationships alone are not sufficient, and any richer model must remain progressive enough that projects can continue using ordinary files, ordering, links, search, and Git when those already solve the relevant recovery problem.

These are hypotheses to test, not architectural commitments.

## Core question

> How can a complex engineering project preserve its evolving knowledge and reasoning so that both current state and historical process remain intelligible, navigable, and useful?

## Development approach

1. Define the problem and project intent.
2. Study real engineering information and existing approaches.
3. Derive requirements from evidence rather than preferred technology.
4. Model the domain.
5. Design the architecture.
6. Build the smallest useful vertical prototype.
7. Use the project to document its own development and evaluate the model.

## Relationship to Monad

Project Knowledge was motivated by information-management problems encountered during development of the broader Monad engineering work. Monad provides an initial real-world corpus and use case, but Project Knowledge is maintained as a distinct project so its model can remain general-purpose.

## Repository principle

The repository records the durable project state. Conversations, experiments, and working sessions may generate ideas, but conclusions should become reviewable repository artifacts when they are mature enough to persist.
