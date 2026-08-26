# Semantic Traversal Model

## Purpose

The traversal engine provides a neutral, deterministic view of semantic connections already present in Project Knowledge.

It does not rank truth, infer causality, or generate narrative.

## Traversable entities

Every existing `pk/v1` record kind is addressable as an entity:

- Subject;
- Representation;
- Claim;
- Assertion;
- Authority;
- Relationship;
- Activity;
- Context; and
- Evidence Evaluation.

Native references are exposed as source endpoints, but RT-1 does not require them to become first-class S2 entities.

## Edge classes

RT-1 recognizes three edge classes.

### 1. Recorded semantic relationships

These come directly from `Relationship` records.

Each returned edge preserves:

- Relationship record ID;
- `from` entity;
- exact relation string;
- `to` entity;
- `origin` (`authored`, `imported`, `derived`, `inferred`); and
- optional Activity ID.

The retrieval layer must not rename a relation in a way that changes its meaning.

### 2. Deterministic structural bindings

These are query-time edges derived directly from existing record fields. They are not persisted as additional Relationship records.

Initial binding kinds and canonical orientation:

| Binding | Canonical edge | Source field |
| --- | --- | --- |
| `representation_of` | Representation → Subject | `Representation.subject_id` |
| `claim_about` | Claim → Subject | `Claim.subject_id` |
| `asserts_claim` | Assertion → Claim | `Assertion.claim_id` |
| `asserted_by` | Assertion → Representation | `Assertion.representation_id` |
| `authority_for_subject` | Authority → Subject | `Authority.subject_id` |
| `authority_representation` | Authority → Representation | `Authority.representation_id` |
| `evidence_for_claim` | Evidence Evaluation → Claim | `EvidenceEvaluation.claim_id` |
| `generated_by_activity` | Representation → Activity | membership in `Activity.generated_representation_ids` |
| `uses_context` | Assertion / Authority / Evidence Evaluation → Context | corresponding `context_id` |
| `relationship_activity` | Relationship → Activity | `Relationship.activity_id` |

These labels belong to the retrieval contract, not to the open-ended canonical Relationship vocabulary.

They express structural bindings already guaranteed by the record model; they do not add new project assertions.

The canonical orientation is stable even when a query discovers the edge by traversing `incoming` from the target. The returned edge always preserves this canonical orientation.

### 3. Native-source links

Where a record contains a `NativeReference`, traversal may expose a source link carrying:

- source system;
- object type;
- locator;
- immutable/reconstructable state when present;
- current source observation status when available.

Native-source links are canonically oriented from the semantic record to the native source endpoint.

Native-source links are terminal in RT-1. The first increment does not recursively crawl arbitrary external systems from traversal.

## Direction

Traversal supports:

- `outgoing`;
- `incoming`; and
- `both`.

Direction is interpreted against the stored/derived canonical edge orientation. The result always preserves original orientation even when discovered through an incoming query.

## Bounds

Traversal must always be bounded.

A query declares a maximum depth. The CLI default is **2 hops**, because DF-003 requires more than a single direct edge while still favoring local recovery over corpus-wide dumping.

The implementation should enforce a finite hard ceiling. The initial ceiling may be implementation-defined and tuned by dogfood, but unbounded traversal is not permitted.

The result must report when additional reachable nodes were omitted because of bounds.

## Cycles

Traversal must be cycle-safe.

The engine maintains visited `(entity, depth)`/equivalent state and must not infinitely re-expand cycles. Returned edges may still reveal a cycle if those edges are within bounds.

## Path queries

The same engine supports two related operations:

### Neighborhood

Given one root, return the bounded connected semantic neighborhood that satisfies the query filters.

### Path

Given `from` and `to`, return one or more deterministic shortest recorded/structural paths within the bound.

Initial path ordering should be deterministic, not relevance-scored.

If several shortest paths exist, all may be returned up to the configured result bound; truncation must be explicit.

## Relationship filters

Queries may filter recorded semantic relationships by exact relation string.

Structural bindings use a separate binding filter namespace so an open-ended project relation named `asserted_by` cannot be confused with the retrieval engine's deterministic structural binding of the same spelling.

## Origin filters

Recorded semantic edges may be filtered by Relationship origin.

Human recovery projections should display origin when it matters, especially for `inferred` or `derived` edges.

RT-1 must not hide that distinction behind one undifferentiated “related” link.

## Current-state semantics are not traversal semantics

Traversal answers:

> What recorded/structural connections exist?

Resolution answers:

> What Claim is authoritative for a concern under time/context?

`pk explain` may combine both, but the traversal engine itself must not decide current truth merely because one node has more incoming edges or appears on more paths.

## Evidence freshness is not edge existence

An Evidence Evaluation can remain structurally linked to a Claim while its current evidence state is stale, unavailable, failed, or otherwise non-current.

The recovery projection must present both facts:

```text
evidence_for_claim edge exists
current evidence state = <state>
```

It must not drop historical/stale evidence merely because it is not currently fresh unless the caller explicitly requests such filtering.

## No implicit causality

The traversal engine may return:

```text
A --motivates--> B
```

because that edge is recorded.

It must not create:

```text
A --caused--> B
```

from timestamps, adjacency, dependency, similarity, shared Subject identity, or path ordering.
