# Project Knowledge Requirements

This directory contains the first formal, technology-neutral requirements specification for Project Knowledge.

The specification is derived from the discovery chain rather than from a preferred architecture:

```text
problem / vision
    ↓
corpus evidence
    ↓
user jobs + failure modes
    ↓
existing-approach analysis
    ↓
evidence-derived capabilities
    ↓
formal requirements
```

## Status

**Initial requirements baseline merged — Domain Modeling active.**

These requirements are now the normative input to [`../domain/`](../domain/). Discovery remains open where the corpus explicitly marks concepts as under-evidenced, and requirements may be refined if domain validation exposes ambiguity or new evidence changes the need.

Architecture is not yet authorized by implication; the domain model must first settle semantic boundaries and invariants.

## Contents

- [`requirements-model.md`](requirements-model.md) — requirement classes, IDs, normative language, and traceability rules
- [`functional-requirements.md`](functional-requirements.md) — functional behavior grouped by RF-1 through RF-6
- [`quality-attributes.md`](quality-attributes.md) — usability, portability, explainability, determinism, scalability, resilience, and other quality requirements
- [`constraints.md`](constraints.md) — cross-cutting constraints imposed by discovery evidence
- [`non-requirements.md`](non-requirements.md) — explicit boundaries and held concepts
- [`traceability-matrix.md`](traceability-matrix.md) — requirement → capability → evidence trace
- [`acceptance-principles.md`](acceptance-principles.md) — how later specifications and implementations must demonstrate conformance

## Requirement families

### RF-1 — Native interoperability and progressive adoption

Project Knowledge must improve project memory without forcing projects to abandon useful native engineering artifacts or adopt maximum structure up front.

### RF-2 — Semantic identity, representation, and relationships

Project Knowledge must be able to preserve continuity across heterogeneous representations and expose important relationships without mistaking storage identity for semantic identity.

### RF-3 — Authority and current truth

Project Knowledge must make current authoritative state recoverable while preserving historical truth and unresolved disagreement.

### RF-4 — Provenance, time, and context

Project Knowledge must preserve enough provenance, temporal meaning, context, and derivation lineage to reconstruct how information was produced and when it was valid.

### RF-5 — Evidence and epistemic evolution

Project Knowledge must model evidence at the proposition level where required and support correction/refinement without pretending that every project needs a universal knowledge-state machine.

### RF-6 — Retrieval, impact, and explanation

Project Knowledge must make project memory discoverable and explainable across tools while ensuring retrieval relevance never substitutes for authority or truth semantics.

## Normative boundary

These requirements define behavior and constraints. They do **not** select:

- a database;
- graph storage;
- RDF/OWL;
- an event store;
- a temporal database;
- a search product;
- a web framework;
- a CLI framework;
- a service topology;
- an AI provider; or
- an MVP implementation stack.

Those decisions belong to architecture after the domain model has settled the semantic meaning the architecture must preserve.

## Progressive-formalization rule

A requirement that enables rich semantics must not be interpreted as requiring those semantics for every artifact.

The default principle is:

> Add structure when it solves a demonstrated recovery, correctness, governance, or learning problem; preserve simpler native workflows when they already work.
