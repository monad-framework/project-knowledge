# Retrieval and Traversal Detailed Design

## Status

**Detailed design — evidence-authorized by DF-003; implementation not yet started.**

DF-003 demonstrated a new boundary in Project Knowledge: the semantic kernel successfully preserves an evidence-to-implementation reasoning chain, but the current user-facing runtime cannot recover that chain without programmatic record traversal.

The design question is therefore:

> What is the smallest retrieval surface that lets a human recover reasoning already represented in Project Knowledge without exposing raw storage mechanics or inventing semantics that are not present?

## Design result

The selected design separates two layers:

```text
portable pk/v1 + native state
            ↓
   Semantic Traversal Engine
 typed nodes · typed bindings · recorded relationships
 provenance · authority · evidence · source references
            ↓
      Recovery Projection
 current state · recorded reasons · evidence · provenance
 sources · bounded related context
            ↓
          pk explain
```

A lower-level `pk trace` surface exposes the same traversal result for explicit path/neighborhood work and machine use.

`pk explain` is therefore **not** a second semantic engine. It is a recovery-oriented projection over:

- the traversal primitive;
- existing current-state resolution;
- existing evidence freshness;
- existing representation freshness; and
- the same native/source observations used by the compiler.

## Evidence boundary

This design is grounded in:

- DF-003, where representation succeeded but retrieval blocked the recovery job;
- UJ-003, explain why an artifact or implementation exists;
- UJ-004, explain why something changed;
- UJ-005, reconstruct provenance;
- UJ-007, determine evidence validity;
- UJ-010, determine what a derived artifact came from;
- UJ-011, return after loss of mental context;
- UJ-013, assess impact before change;
- FR-601 through FR-608; and
- CAP-012, CAP-016, and CAP-019.

The design does **not** treat every retrieval ambition as authorized by DF-003. Full-text search, semantic search, AI synthesis, automatic causal inference, graph visualization, and generalized impact-policy inference remain outside the first increment.

## Files

- [`evidence-and-scope.md`](evidence-and-scope.md) — why this design is authorized and what remains out of scope.
- [`recovery-jobs.md`](recovery-jobs.md) — concrete recovery questions translated into retrieval behavior.
- [`semantic-traversal.md`](semantic-traversal.md) — the neutral traversal model and edge classes.
- [`query-contract.md`](query-contract.md) — selectors, bounds, filters, ordering, and failure semantics.
- [`result-model.md`](result-model.md) — machine-readable traversal and recovery result shapes.
- [`cli-contract.md`](cli-contract.md) — `pk explain` and `pk trace` command behavior.
- [`safety-and-trust.md`](safety-and-trust.md) — authority, causality, inference, provenance, and mutation boundaries.
- [`acceptance-scenarios.md`](acceptance-scenarios.md) — executable behavior expected from the first increment.
- [`implementation-sequence.md`](implementation-sequence.md) — smallest implementation order and dogfood gate.

## Proposed implementation increment

If this design is accepted, the first implementation increment is:

> **RT-1 — Semantic Recovery Traversal**

RT-1 is intentionally narrower than “search,” “knowledge graph UI,” or a broad M1. Its purpose is to make already-recorded Project Knowledge recoverable through typed traversal and a human-oriented explanation projection.

## Non-conclusions

This design does not select or require:

- a graph database;
- RDF/SPARQL;
- a web UI;
- a graph visualization;
- embeddings or vector search;
- an LLM;
- generated narrative as authority;
- a universal causal relation vocabulary;
- a universal impact/dependency vocabulary; or
- a canonical schema expansion.

The current portable model remains the source of Project Knowledge-owned semantics. Retrieval is a derived capability over that model and native source state.
