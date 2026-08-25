# Existing-Approach Evaluation Method

This document defines how Project Knowledge compares existing mechanisms against the observed corpus without turning the comparison into a technology popularity contest.

## Unit of evaluation

The unit is an **approach or mechanism family**, not a product purchase decision.

Examples:

- Git as a content-addressed revision/history model;
- ADRs as a decision-preservation practice;
- event sourcing as an append-only state-transition model;
- bitemporality as a time model;
- PROV as a provenance vocabulary; and
- hybrid search as a retrieval strategy.

Specific products may be used as concrete evidence of what a mechanism can do, but product-specific features are not automatically Project Knowledge requirements.

## Primary criteria

Each approach is evaluated against these problem dimensions derived from the corpus.

### C-01 — Exact historical reconstruction

Can the approach recover an earlier exact state or version?

Related jobs/failures: `UJ-002`, `UJ-008`, `FM-002`, `FM-009`.

### C-02 — Current truth / authority clarity

Can it distinguish what is current and authoritative from historical, derived, or stale representations?

Related: `UJ-001`, `UJ-006`, `UJ-009`, `FM-001`, `FM-011`, `FM-015`.

### C-03 — Stable semantic identity

Can logical identity survive moves, renames, generated representations, or tool boundaries?

Related: `UJ-006`, `FM-004`, `FM-012`, `PKC-0010`.

### C-04 — Explicit semantic relationships

Can it represent relationships such as depends-on, supersedes, derived-from, implements, supports, contradicts, or represents without burying them entirely in prose?

Related: `UJ-003`, `UJ-004`, `UJ-010`, `UJ-013`, `FM-014`.

### C-05 — Provenance and derivation lineage

Can it distinguish producer, source, activity, derivation, revision, attribution, and related provenance concepts?

Related: `UJ-005`, `UJ-010`, `FM-006`, `FM-010`.

### C-06 — Temporal semantics beyond revision order

Can it distinguish when a claim was effective/valid from when it was recorded or known?

Related: `UJ-002`, `UJ-004`, `UJ-014`, `FM-002`, `FM-016`.

### C-07 — Epistemic evolution and correction

Can it express that a proposition was tentative, supported, rejected, refined, corrected, superseded, or disputed without simply overwriting history?

Related: `UJ-004`, `UJ-007`, `UJ-009`, `UJ-014`, `PKC-0009`.

### C-08 — Claim-relative evidence

Can evidence be tied to the proposition and source state it actually supports, rather than treated as generically fresh or stale?

Related: `UJ-007`, `FM-007`, `FM-008`.

### C-09 — Human-authored narrative

Can the approach support coherent explanation and learning paths rather than only machine traversal or state reconstruction?

Related: `UJ-011`, `UJ-012`, `FM-013`.

### C-10 — Discovery and retrieval

Can a user find relevant information without already knowing exact location or terminology?

Related: `UJ-011`, `UJ-013`, `FM-014`.

### C-11 — Progressive structure / low capture burden

Can users start simple and add structure only when it becomes valuable?

Related: `PKC-0011`, second-pass observations, and the project's cognitive-burden questions.

### C-12 — Cross-tool / cross-representation composition

Can the approach naturally connect repository files, work items, generated artifacts, external records, and other representations without pretending they are all one native object type?

Related: `UJ-006`, `UJ-010`, `FM-001`, `FM-015`.

## Rating language

The comparison matrix uses qualitative ratings:

- **Strong** — the approach directly and intentionally addresses the criterion.
- **Partial** — useful support exists, but important semantics remain convention- or application-dependent.
- **Weak** — possible only indirectly, through manual discipline, or through substantial extensions.
- **Not intended** — the criterion lies outside the approach's normal responsibility.

A `Strong` rating does not mean Project Knowledge should adopt the implementation. It means the underlying semantics or pattern should be considered for reuse.

## Evaluation rules

1. **Prefer primary documentation.** Standards and official product/tool documentation are preferred over secondary summaries.
2. **Separate storage from semantics.** A system can store metadata without defining what that metadata means.
3. **Separate retrieval from truth.** Finding a passage does not establish that it is current, authoritative, or correct.
4. **Separate history from temporal semantics.** Revision order is not automatically valid time, transaction time, or knowledge time.
5. **Separate links from typed relationships.** A hyperlink proves navigability, not the meaning of the relation.
6. **Separate audit from explanation.** An immutable log can reconstruct events while still failing to explain rationale.
7. **Credit composition.** An approach does not need to solve every criterion to be valuable.
8. **Treat complexity as a cost.** A technically expressive model may still be unsuitable if it increases authoring and recovery burden.
9. **Record gaps, do not force a winner.** The output is a capability map, not a product ranking.

## What would falsify the current Project Knowledge hypothesis?

The strongest falsification would be evidence that a practical composition of existing tools already satisfies the important user jobs with acceptable cognitive and operational cost.

If that is true, Project Knowledge should become an integration/convention layer or perhaps cease to exist as a separate software product.

Conversely, the project is justified only to the extent that a meaningful cross-cutting gap remains after crediting existing mechanisms for what they already do well.
