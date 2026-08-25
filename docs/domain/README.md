# Domain Model

This directory defines the first technology-neutral domain model for Project Knowledge.

The domain model is derived from the formal requirements under `docs/requirements/`. It exists to settle the minimum semantic concepts, boundaries, cardinalities, and invariants needed before architecture begins.

## Status

**Phase 3 — Domain Modeling**

This is a candidate semantic kernel, not a storage schema or implementation model.

The domain model MUST remain compatible with the requirements constraints:

- native systems remain legitimate sources of truth;
- semantic enrichment is progressive rather than universal;
- typed relationships do not imply graph storage;
- temporal semantics do not imply a temporal database;
- provenance should reuse mature semantics where practical;
- retrieval relevance does not determine truth or authority; and
- unresolved disagreement is a valid domain state.

## Core domain question

> What is the smallest set of semantic concepts Project Knowledge must understand in order to connect heterogeneous engineering artifacts, recover current and historical truth, explain provenance and evidence, and support impact/recovery views without replacing native tools?

## Candidate kernel

The current model contains these primary concepts:

1. **Project** — the boundary within which project-memory policy and identity are interpreted.
2. **Source System** — a native system that owns artifacts or state, such as Git, GitHub Issues, a wiki, CI, or an external standard.
3. **Native Reference** — an inspectable reference to a native object or source state.
4. **Subject** — an optional stable semantic identity for a logical engineering thing whose continuity matters across representations.
5. **Representation** — a native artifact, fragment, projection, implementation, evidence item, or other concrete representation that concerns a Subject.
6. **Claim** — a proposition whose truth, authority, evidence, time, or disagreement matters independently of one particular representation.
7. **Assertion** — a source-bound occurrence of a Representation or Agent asserting, recording, observing, or otherwise presenting a Claim.
8. **Authority Assignment** — a scoped statement describing which source governs a concern and why.
9. **Relationship** — an explicit semantic relation among domain objects where typed meaning is worth preserving.
10. **Activity** — an occurrence that uses, produces, derives, validates, imports, generates, or changes project-memory entities.
11. **Context** — material observation/execution conditions needed to interpret or reconstruct information.
12. **Evidence Evaluation** — a specialized Activity that evaluates an explicit Claim against a source state using a method and context, producing a result.
13. **Epistemic Annotation** — optional semantics describing uncertainty or knowledge evolution where useful.
14. **Projection/View** — a current-state, historical, impact, provenance, retrieval, or narrative presentation derived from the shared memory model.

The model intentionally does **not** define separate domain types for every engineering artifact category such as requirement, ADR, work packet, source file, experiment, issue, or tutorial. Those remain native-domain objects and may be represented through the kernel when cross-cutting semantics are needed.

## Documents

- [`semantic-kernel.md`](semantic-kernel.md) — concepts, boundaries, cardinalities, and derived concepts
- [`identity-and-representation.md`](identity-and-representation.md) — native identity, semantic identity, Subjects, Representations, and bindings
- [`claims-authority-and-truth.md`](claims-authority-and-truth.md) — Claims, Assertions, authority scope, conflict, and current truth
- [`provenance-time-context-evidence.md`](provenance-time-context-evidence.md) — Activity/provenance, temporal semantics, Context, and Evidence Evaluation
- [`relationships-projections-and-recovery.md`](relationships-projections-and-recovery.md) — relationship semantics, derivation, freshness, projections, and explanatory paths
- [`progressive-formalization.md`](progressive-formalization.md) — enrichment levels and selective-retention rules
- [`invariants.md`](invariants.md) — normative domain invariants
- [`requirements-traceability.md`](requirements-traceability.md) — requirements-to-domain mapping
- [`open-questions.md`](open-questions.md) — questions deliberately deferred to architecture or additional evidence

## Domain-modeling boundary

This phase decides **meaning** before deciding **representation**.

It does not select:

- JSON/YAML/RDF/SQL schemas;
- UUID/ULID/URI identifier formats;
- relational, document, graph, or event storage;
- a provenance serialization;
- temporal database infrastructure;
- indexing/search technology;
- API shapes;
- service boundaries;
- implementation language; or
- user interface.

Those are architecture decisions constrained by this model and the requirements.