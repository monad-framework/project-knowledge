# Retrieval Result Model

## Purpose

RT-1 needs one machine-readable result contract that can support:

- `pk trace`;
- `pk explain`;
- future impact projections;
- future web/UI consumers; and
- tests.

The result model is derived S3/query state. It is not persisted canonical S2.

## Traversal result

Conceptually:

```text
TraversalResult
  query
  root
  target?
  outcome
  nodes[]
  edges[]
  paths[]
  native_sources[]
  truncated
  diagnostics[]
```

### Outcome

Initial outcomes:

- `found` — a neighborhood was recovered;
- `path_found` — one or more paths satisfy the query;
- `no_path` — endpoints exist but no qualifying path was found within bounds.

Selection failures such as `not_found` or `ambiguous` are typed errors rather than successful traversal outcomes.

## Node summary

Each semantic node contains:

```text
EntitySummary
  kind
  id
  display
  attributes
```

`display` is deterministic and record-kind specific:

- Subject — label when present, otherwise UUID;
- Representation — role plus native locator;
- Claim — concern plus compact JSON value;
- Assertion — Claim ID + Representation ID;
- Authority — concern + Representation ID;
- Relationship — exact relation;
- Activity — activity type;
- Context — compact sorted dimensions;
- Evidence Evaluation — method + result.

`attributes` may expose useful structured fields from the source record, but the full raw S2 JSON is not required for the human projection.

## Edge summary

```text
TraversalEdge
  class: recorded|structural|native
  from
  to
  label
  recorded_relationship_id?
  origin?
  activity_id?
  source_detail?
```

For recorded edges, `label` is the exact Relationship relation string.

For structural edges, `label` is a typed retrieval binding rendered as its stable contract name.

For native edges, `source_detail` contains the native-reference information rather than a semantic entity ID.

## Path result

```text
TraversalPath
  nodes[]
  edges[]
```

A path is an alternating sequence beginning and ending with semantic entities. Native terminal links are not valid intermediate path nodes in RT-1.

Paths are ordered deterministically by length and then by their serialized sequence of entity IDs / edge labels.

## Recovery explanation result

`pk explain --json` returns a higher-level projection rather than forcing callers to reconstruct the human explanation from plain text.

Conceptually:

```text
RecoveryExplanation
  root
  identity
  current_state[]
  representations[]
  recorded_relationships[]
  provenance[]
  evidence[]
  authority[]
  sources[]
  related_context
  completeness
```

### Identity

Contains the root entity summary and, when deterministically known, its Subject context.

### Current state

For a Subject root, the projection enumerates distinct recorded Claim concerns for that Subject and invokes the existing resolver for each concern under the optional time/context query.

Each current-state item preserves:

- concern;
- resolution outcome;
- Claim IDs;
- Authority IDs;
- resolved values when unambiguous; and
- resolver explanation.

The retrieval layer does not choose a Claim independently of the resolver.

### Representations

Lists Representation summaries attached to the root Subject or otherwise directly relevant to the selected entity.

Each item includes role, native locator/state, and freshness when the existing freshness model can calculate it.

### Recorded relationships

Shows recorded semantic Relationship edges in the recovered neighborhood, including exact relation and origin.

This section does not rewrite relation strings as stronger causal language.

### Provenance

Shows relevant Activities and their deterministic links to generated Representations and native inputs.

### Evidence

Shows Evidence Evaluations attached to recovered Claims plus current evidence state from the existing evidence evaluator.

Evidence is always associated with its Claim ID.

### Authority

Shows Authority assignments relevant to recovered concerns/Representations, including scope (`subject + concern`), basis, validity interval, and context.

### Sources

Collects unique native references used by displayed records, sorted deterministically.

### Related context

Carries the underlying bounded `TraversalResult` or a normalized subset sufficient for machine consumers to inspect connections omitted from the top-level human sections.

### Completeness

```text
completeness
  truncated: bool
  reasons[]
  unknowns[]
```

This prevents a partial explanation from masquerading as exhaustive project history.

## Human output

Plain-text output is a rendering of `RecoveryExplanation`, not a separately computed answer.

Recommended section order:

```text
<root display>

Current state
Representations
Recorded relationships
Authority
Evidence
Provenance
Sources
Notes / incompleteness
```

Empty sections may be omitted, but unknown/truncated conditions must remain visible.

## Semantic parity

Human and `--json` output must be generated from the same result object.

A test should be able to assert that IDs, relation labels, evidence states, resolution outcomes, and truncation conditions visible in human output are all present in the JSON result without a second traversal.
