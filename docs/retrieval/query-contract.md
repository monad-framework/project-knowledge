# Retrieval Query Contract

## Goals

The RT-1 query contract must be:

- exact enough to avoid semantic identity guessing;
- bounded enough to remain predictable;
- deterministic enough for tests and downstream tooling;
- expressive enough to recover DF-003; and
- reusable by future projections without making the CLI syntax canonical.

## Entity selectors

RT-1 supports exact selectors only.

A selector may identify an entity by:

1. Project Knowledge UUID;
2. exact native locator when the locator resolves uniquely to a Representation or other record carrying that native reference; or
3. exact Subject label when it resolves uniquely.

The library-level contract models selector resolution separately from traversal.

### Ambiguity rule

If an exact label or locator matches more than one semantic entity, selection fails closed and returns the candidate IDs/kinds needed for explicit disambiguation.

RT-1 does not introduce fuzzy matching, similarity-based identity, or automatic Subject merging.

## Traversal query

Conceptually:

```text
TraversalQuery
  root: EntitySelector
  target: EntitySelector?      # path mode when present
  direction: outgoing|incoming|both
  max_depth: positive integer
  relations: [exact strings]?
  bindings: [binding kinds]?
  origins: [authored|imported|derived|inferred]?
  include_structural: bool
  include_native: bool
  result_limit: positive integer
```

The exact Rust representation may evolve during implementation, but these semantics are normative for RT-1.

## Defaults

For a human recovery neighborhood:

```text
direction       both
max_depth       2
relations       unrestricted
bindings        unrestricted
include_structural true
include_native  true
```

For an explicit path query, the caller supplies both root and target; direction defaults to `both` unless constrained.

The implementation must use a finite result limit and report truncation.

## Relation filtering

Recorded relation filters are exact strings. No stemming, synonym expansion, or semantic similarity is applied.

Example:

```text
relation = motivates
```

matches only the recorded relation `motivates`.

## Structural-binding filtering

Structural binding filters are typed values from the retrieval contract rather than arbitrary strings from S2.

This separates:

```text
Relationship.relation = "evidence_for_claim"
```

from the retrieval engine's deterministic binding:

```text
EvidenceEvaluation.claim_id → Claim
```

Even if the textual labels coincide, their provenance is different and must remain distinguishable.

## Time and context

Traversal edge existence is normally historical/structural and is not filtered away by current valid time.

Recovery projections may additionally request:

```text
at: RFC3339 timestamp?
context: Context UUID?
```

These values are passed to existing resolution semantics for current/historical concern resolution. They do not rewrite the traversal graph.

This lets an explanation say, for example:

```text
current at T: accepted
recorded historical relationship: X motivates Y
```

without pretending the relationship itself is a current-state Claim.

## Deterministic ordering

Results must be deterministic for the same compiled project-memory state and query.

Ordering rules:

1. shortest depth first;
2. entity kind in stable enum order;
3. UUID lexical order for node tie-breaks;
4. edge class in stable order (`recorded`, `structural`, `native`);
5. relation/binding label lexical order;
6. edge record ID or endpoint identity as final tie-break.

Human presentation may group sections differently but must preserve semantic parity with JSON output.

## Missing roots

If a selector resolves to no entity, RT-1 returns a typed `not_found` selection error. It does not silently fall back to content search.

## No path

A path query that resolves both endpoints but finds no qualifying path within the requested bounds returns:

```text
outcome: no_path
```

with the bounds and filters used.

This differs from `not_found`.

## Truncation

If depth/result bounds omit otherwise reachable entities or paths, the result includes:

```text
truncated: true
reason: depth_limit | result_limit | path_limit
```

A truncated result must never be presented as a complete dependency or reasoning graph.

## Native-source state

When native links are included, the query result may attach the latest source observation already available to the compiler/read model.

RT-1 does not make a fresh network request for arbitrary native references during traversal.

## Mutation

All RT-1 query operations are read-only.

No retrieval command may:

- add S2 records;
- infer and persist Relationships;
- update freshness metadata as canonical state; or
- modify native artifacts.

Any future “accept this inferred link” workflow belongs to authoring/capture, not retrieval.
