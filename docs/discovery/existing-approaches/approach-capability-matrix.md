# Existing-Approach Capability Matrix

This matrix is a compact comparison using the criteria in [`evaluation-method.md`](evaluation-method.md).

Ratings are intentionally qualitative:

- **S** — Strong
- **P** — Partial
- **W** — Weak / convention-dependent
- **—** — Not normally intended to solve the criterion

The matrix compares mechanism families, not specific product configurations. Extensions can raise individual capabilities, but that does not mean the underlying approach defines the missing semantics.

## Matrix A — History, identity, relationships, provenance, time, epistemics

| Approach | C-01 Exact history | C-02 Authority clarity | C-03 Semantic identity | C-04 Typed relationships | C-05 Provenance | C-06 Temporal semantics | C-07 Epistemic evolution | C-08 Claim evidence |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Git | **S** | W | W | P | P | W | W | W |
| Docs as code / wiki | P | P | W | P | W | W | P | W |
| ADR practice | P | **S** within decisions | **S** within ADR IDs | **S** for decision relations | P | P | **S** for decision lifecycle | P |
| Issue/work tracking | P | **S** for workflow state | P | **S** for work hierarchy/dependency | P | P | P | W |
| Linked notes / PKM | P | W | P | P | W | W | P | W |
| Event sourcing | **S** | P | P | P | P | P | P | W |
| Bitemporal model | **S** | P | P | P | P | **S** | P | W |
| W3C PROV semantics | P | W | P | **S** for provenance relations | **S** | P | P (`revision`, invalidation) | W |
| RDF/graph representation | P | W | **S** if identity rules exist | **S** | P | P | P | P |
| Hybrid search / RAG | — | W | W | W | W | W | W | W |

## Matrix B — Narrative, discovery, progressive structure, composition

| Approach | C-09 Narrative | C-10 Discovery | C-11 Progressive structure | C-12 Cross-tool composition |
| --- | --- | --- | --- | --- |
| Git | W | P | **S** | P |
| Docs as code / wiki | **S** | P | **S** | P |
| ADR practice | **S** for decisions | P | P | P |
| Issue/work tracking | P | **S** for work | **S** | **S** within integrations |
| Linked notes / PKM | **S** | **S** | **S** | P |
| Event sourcing | W | W | W | P |
| Bitemporal model | W | P | P | P |
| W3C PROV semantics | W | P | **S** conceptually | **S** |
| RDF/graph representation | W | **S** for traversal/query | P | **S** |
| Hybrid search / RAG | **S** for synthesized views | **S** | **S** as an index | **S** as retrieval federation |

## Reading the matrix

### No universal winner

No row is strong across all criteria. This is important negative evidence against building Project Knowledge as a renamed version of any one existing mechanism.

### Several mature semantics should be reused

The strongest cells suggest areas where novelty would be wasteful:

- exact repository history → **Git**;
- decision rationale/lifecycle → **ADRs**;
- work coordination/dependencies → **issue trackers**;
- low-friction linking/progressive enrichment → **linked-note systems**;
- state-transition history/projections → **event sourcing pattern**;
- valid-time versus system-time distinction → **bitemporal models**;
- provenance vocabulary → **W3C PROV**;
- many-to-many relationship representation/traversal → **graph models**; and
- findability/context recovery → **hybrid retrieval**.

### Weak cells cluster around the same cross-cutting problem

The difficult criteria repeatedly span boundaries between approaches:

1. scoped authority across multiple representations;
2. semantic identity across files/tools/projections;
3. epistemic state beyond a single artifact type;
4. claim-relative evidence;
5. causal explanation joining events, rationale, evidence, and resulting state; and
6. presenting current truth, history, provenance, and narrative from the same underlying project memory.

Those clusters are more significant than the fact that any individual tool lacks a feature.

## User-job coverage interpretation

A practical composition of existing approaches can already address much of:

- `UJ-002` historical reconstruction;
- `UJ-008` work-episode reconstruction;
- `UJ-010` derivation tracing, when provenance is recorded;
- `UJ-011` context recovery, with good search; and
- `UJ-012` narrative learning, with authored documentation.

The hardest remaining jobs are likely:

- `UJ-001` recover current truth across conflicting representations;
- `UJ-005` reconstruct precise provenance across tools;
- `UJ-006` compare representations of one semantic object;
- `UJ-007` determine claim-relative evidence validity;
- `UJ-009` diagnose the nature of contradictions;
- `UJ-013` impact analysis across heterogeneous artifacts; and
- `UJ-014` preserve corrections with both temporal and epistemic clarity.

These should receive special attention in the next evidence-to-capability derivation pass.

## Matrix limitation

This matrix is not quantitative validation. It is a structured research summary.

Before architecture, important candidate capabilities should still trace back through:

```text
capability
  ↓
user job
  ↓
failure mode
  ↓
corpus evidence
```

Existing approaches should remain in that trace so we can distinguish:

- **reuse/integrate**;
- **extend**; and
- **genuinely new Project Knowledge behavior**.
