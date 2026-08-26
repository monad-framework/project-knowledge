# RT-1 Implementation Sequence

## Increment

> **RT-1 — Semantic Recovery Traversal**

RT-1 should be implemented as the smallest vertical slice that makes DF-003 recoverable through the public runtime without changing the canonical `pk/v1` schema.

## RT-1.1 — Entity selection and summaries

Add reusable exact selection helpers for:

- UUID;
- exact native locator; and
- exact Subject label.

Ambiguity must fail closed with candidates.

Add deterministic `EntitySummary` rendering for every current record kind.

### Gate

Tests cover unique, ambiguous, missing, and sparse entities.

## RT-1.2 — Structural adjacency

Add read-model/library helpers that expose deterministic structural bindings from existing record fields.

Do not persist duplicate Relationship records.

### Gate

Tests cover:

- Subject↔Representation;
- Subject↔Claim;
- Claim↔Assertion;
- Assertion↔Representation;
- Authority bindings;
- Claim↔Evidence Evaluation;
- Activity↔generated Representation; and
- Context bindings.

## RT-1.3 — Recorded Relationship adjacency

Index or derive incoming/outgoing recorded Relationship edges with exact relation, origin, Relationship ID, and Activity ID.

The implementation may use disposable SQLite S3 indexes or in-memory indexes; this is an implementation choice, not a new canonical store.

### Gate

DF-003's four recorded Relationships are recoverable with exact provenance.

## RT-1.4 — Bounded traversal engine

Implement `TraversalQuery` neighborhood traversal with:

- direction;
- exact relation filters;
- structural-binding filters;
- origin filters;
- depth bound;
- result bound;
- native-source inclusion;
- deterministic ordering; and
- explicit truncation.

Traversal must be cycle-safe.

### Gate

RT-A03, A04, A07 through A14, A16, A17, A19, and A22 have direct executable coverage.

## RT-1.5 — Path mode

Add bounded endpoint-to-endpoint path traversal over the same adjacency engine.

Return deterministic shortest paths, preserving exact edge classes/origins.

### Gate

`no_path` and `not_found` are distinguishable and path ordering is deterministic.

## RT-1.6 — RecoveryExplanation builder

Build the human-oriented projection by composing:

- root/entity summary;
- Subject concern enumeration + existing resolver;
- Representation summaries + existing freshness;
- recorded Relationships;
- relevant Authority assignments;
- Evidence Evaluations + existing evidence state;
- provenance Activities;
- native sources; and
- bounded related context.

Do not duplicate resolution or evidence algorithms.

### Gate

DF-003's complete recovery question is represented in one `RecoveryExplanation` object.

## RT-1.7 — `pk trace`

Expose neighborhood/path traversal through CLI and `--json`.

### Gate

Human and JSON forms preserve identical IDs, relation labels, origins, bounds, and outcomes.

## RT-1.8 — `pk explain`

Expose `RecoveryExplanation` through CLI and `--json`.

### Gate

RT-A01, A02, A05, A06, A15, A18, A20 have direct executable coverage.

## RT-1.9 — Full acceptance and regression

Run:

```text
cargo fmt --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-targets --all-features
```

All M0, CA-1, DF-001, DF-002, DF-003, and RT-A01 through RT-A22 checks must pass.

## RT-1.10 — DF-004

After merge, do not immediately design full-text search or a graph UI.

Use RT-1 on a new real recovery problem.

Recommended DF-004 shape:

> Start from an existing engineering artifact or decision whose downstream consequences are not obvious and use `pk explain` / `pk trace` to recover what depends on it, what evidence supports it, and where the recorded chain stops.

DF-004 should measure:

1. whether two-hop default explanation is sufficient;
2. whether exact selection is too burdensome without broader discovery;
3. whether a dedicated impact projection is justified;
4. which relationship types recur enough to support safe impact semantics;
5. whether structured explanation is sufficient or authored/generated narrative becomes necessary; and
6. whether retrieval now exposes missing semantic capture rather than missing retrieval mechanics.

## Expected source changes

Likely implementation areas:

```text
src/
  retrieval.rs          # new query/result/traversal/recovery types
  store.rs              # adjacency/index helpers
  resolver.rs           # reuse only; avoid semantic duplication
  evidence.rs           # reuse only
  freshness.rs          # reuse only
  main.rs               # pk explain / pk trace CLI
  lib.rs                # public retrieval API

tests/
  retrieval_traversal.rs
  dogfood_ca1_explain.rs
```

Exact file layout is not normative.

## Schema expectation

The implementation should begin with the assumption:

> **No `pk/v1` canonical schema change is required for RT-1.**

If implementation discovers that the recovery job cannot be solved without new canonical semantics, that is a design-falsification result. Stop and document the missing semantic evidence rather than smuggling it into a retrieval-only type.
