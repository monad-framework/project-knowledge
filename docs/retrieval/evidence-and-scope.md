# Retrieval Evidence and Scope

## Triggering evidence

DF-003 asked:

> Why does CA-1 exist, which project evidence motivated it, which design decision governs it, and what evidence shows that the capability was actually implemented and validated?

The repository successfully represented the answer using existing portable semantics:

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

The DF-003 executable test can traverse those records programmatically. The ordinary CLI cannot.

That is a different class of failure from DF-001 and DF-002:

| Layer | DF-001 / DF-002 | DF-003 |
| --- | --- | --- |
| Semantic representation | sufficient | sufficient |
| Mechanical authoring | repeated burden | materially improved by CA-1 |
| Current-state resolution | sufficient | sufficient |
| Reasoning-path retrieval | not yet the blocking question | blocks the complete recovery job |

The new problem is therefore not “store more semantics.” It is “recover semantics already stored.”

## Requirements trace

The design realizes existing requirements rather than inventing a new requirements family:

- **FR-601** — project-memory retrieval across integrated sources;
- **FR-602** — semantic metadata available to retrieval;
- **FR-603** — relevance remains separate from truth semantics;
- **FR-604** — recorded relationship traversal for impact work;
- **FR-605** — explainable recovery paths with source traceability;
- **FR-606** — no invented causality;
- **FR-608** — context recovery without exhaustive rereading.

It is also the first strong self-dogfood evidence for the user-facing portion of **CAP-019 — causal/recovery path reconstruction**.

## Scope selected for RT-1

RT-1 should support three things only:

1. **Entity recovery** — identify one semantic entity and summarize the information directly attached to it.
2. **Bounded semantic traversal** — follow recorded relationships and deterministic structural bindings in a controlled neighborhood/path query.
3. **Recovery explanation** — project that traversal into a human-readable view that answers “what is this, what is true now, what recorded relationships explain it, what evidence supports it, where did it come from, and where are the native sources?”

These behaviors are sufficient to make DF-003 recoverable without writing Rust or reading raw `.pk/records/*.json` files.

## Explicitly deferred

### Full-text and semantic search

FR-601 eventually requires suitable discovery mechanisms, but DF-003 already has a known semantic root. RT-1 therefore does not need an indexer, embeddings, reranking, or RAG.

Exact selectors are sufficient for the first increment.

### Automatic impact semantics

FR-604 requires impact traversal where recorded relationships exist. RT-1 supplies the traversal primitive needed for this, but does not guess which open-ended relationship strings mean “depends on,” “affected by,” or “must change with.”

A dedicated `pk impact` projection should wait until real relationship vocabularies provide enough evidence to define safe direction and inclusion rules.

### Automatic causal explanation

RT-1 may display an authored relation such as `motivates` or `governs_design_of`. It must not transform arbitrary sequence, adjacency, or dependency into “caused.”

### Generated narrative

The first explanation is a structured projection, not an LLM-generated essay. A future narrative layer may consume the same traceable result, but generated fluency must not become authority.

### Graph visualization

A visualization may later consume traversal JSON. It is not required to solve the current recovery problem.

## Progressive-structure rule

RT-1 must preserve progressive formalization:

- native-only projects remain valid;
- entities with no Relationships remain inspectable;
- one-hop structural bindings can still provide value;
- richer recorded Relationships produce richer recovery paths;
- absence of a path remains `not found`/`insufficient recorded relationship`, not an invitation to invent one.

## Success boundary

RT-1 succeeds if DF-003 can be recovered through the public runtime with source-traceable semantics and without raw record inspection.

RT-1 does **not** need to answer every natural-language question or reconstruct reasoning that the project never recorded.
