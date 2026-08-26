# ADR-0004 — Separate Semantic Traversal from Recovery Projection

## Decision state

**Proposed on this branch. Merge of the PR containing this ADR into `main` constitutes acceptance of this decision.**

This acceptance rule applies only to this ADR and this PR. It must not be generalized into a rule that every merged PR accepts every proposal it contains.

## Context

DF-003 demonstrated that Project Knowledge can already preserve a real evidence-to-implementation reasoning chain using the existing semantic kernel:

```text
DF-001 ──motivates──────────┐
                            ▼
                      Authoring Design
                            ▲
DF-002 ──motivates──────────┘
                            ▲
ADR-0003 ──governs_design_of

CA-1 Closure ──verifies──> implemented_and_validated Claim
```

The runtime can resolve CA-1's current `capability_status`, and programmatic tests can inspect the relevant Relationships, Activity, Evidence Evaluation, Authority, and native sources.

The user-facing CLI cannot recover that reasoning path directly.

A tempting response would be to put all recovery behavior directly into one `pk explain` implementation. That would couple traversal mechanics, current-state resolution, evidence state, human presentation, and future retrieval use cases into one projection-specific code path.

Another tempting response would be to expose only a generic graph traversal command, forcing users to understand Project Knowledge's storage/graph shape to recover ordinary engineering context.

Both approaches would make later impact analysis, search integration, UI projections, and narrative views harder to keep semantically consistent.

## Decision

Adopt two distinct retrieval layers:

```text
S1 + S2
  ↓
Semantic Traversal Engine
  - exact entity selection
  - recorded Relationship edges
  - deterministic structural bindings
  - native-source links
  - bounded neighborhood/path traversal
  - deterministic machine-readable result
  ↓
Recovery Projection
  - identity
  - current/historical resolved concerns
  - representations/freshness
  - recorded relationships
  - authority
  - evidence/current evidence state
  - provenance
  - sources
  - incompleteness/truncation
  ↓
pk explain
```

Expose the neutral traversal engine through a lower-level `pk trace` surface.

`pk explain` must be a projection over shared traversal and existing resolver/freshness/evidence semantics. It must not become an independent semantic engine.

## Consequences

### Positive

- DF-003 becomes recoverable through a human-oriented command;
- traversal is reusable by future impact, search, web, narrative, and integration layers;
- current-state authority remains owned by the resolver;
- evidence freshness remains owned by the evidence evaluator;
- structural bindings can be traversed without duplicating them as canonical Relationship records;
- machine and human retrieval can share one result contract;
- graph mechanics remain available without becoming the required user mental model; and
- no canonical `pk/v1` schema expansion is required for the initial retrieval increment.

### Costs

- introduces a dedicated traversal/result abstraction;
- requires deterministic adjacency/path logic and truncation semantics;
- requires a second CLI surface (`trace`) in addition to the recovery projection (`explain`);
- requires careful distinction between authored Relationships and derived structural bindings; and
- explanation composition must avoid duplicating resolver/evidence/freshness logic.

## Rejected alternatives

### `pk explain` as a monolithic query engine

Rejected because it would hard-code traversal, truth resolution, evidence, provenance, and presentation into one consumer-specific path.

### Generic graph browser / graph CLI as the primary experience

Rejected because DF-003 is a recovery problem, not evidence that users should reason in vertices and edges for ordinary use.

A graph UI may later consume traversal results if visual evaluation justifies it.

### `pk why` as the first command

Rejected because `why` implies causality. Project Knowledge must not convert chronology, adjacency, structural binding, dependency, or similarity into causal explanation.

`explain` can present exact recorded rationale relationships without overstating them.

### Persist every structural binding as a Relationship record

Rejected because the current record model already guarantees many bindings (`Claim.subject_id`, `EvidenceEvaluation.claim_id`, `Assertion.representation_id`, etc.). Persisting duplicates would create new drift/freshness problems and blur authored versus deterministic structure.

### Add a graph database before retrieval

Rejected. The current 39-record dogfood corpus and SQLite/in-memory S3 are sufficient to define and test traversal behavior. Storage optimization should follow measured need.

### Add semantic/full-text search first

Rejected for RT-1 because DF-003 begins from a known semantic root. Exact retrieval is enough to solve the observed blocker. Broader discovery remains an existing requirement for later evidence-driven work.

## Follow-up

If this ADR is accepted:

1. implement **RT-1 — Semantic Recovery Traversal** according to `docs/retrieval/`;
2. keep RT-1 read-only and schema-neutral unless implementation falsifies that assumption;
3. validate RT-A01 through RT-A22 plus all existing regression suites; and
4. dogfood the result in DF-004 before promoting full-text search, impact projection, graph visualization, or narrative generation.
