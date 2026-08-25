# Capability Derivation Method

## Purpose

This method converts discovery evidence into candidate capabilities while minimizing two opposite errors:

1. **under-modeling** — leaving a repeated recovery problem unsolved because existing tools each solve only one local part; and
2. **over-modeling** — inventing structure that is intellectually attractive but costs more to capture and maintain than it returns.

## Step 1 — Start from a user recovery job

Capabilities are justified by something a user needs to recover, explain, compare, validate, diagnose, or learn.

The current job catalog is `UJ-001` through `UJ-014`.

A capability that cannot identify a user job is normally not a product capability yet.

## Step 2 — Require an observed failure or constraint

The capability must mitigate at least one documented failure mode or preserve a demonstrated counterexample constraint.

The current failure catalog is `FM-001` through `FM-016`.

Positive constraints from the counterexample corpus also matter. In particular:

- `PKC-0011` constrains the system not to require rich structure where ordered documents and Git are enough;
- `PKC-0008` demonstrates that curated summaries can become stale even in a small repository;
- `PKC-0009` demonstrates knowledge refinement without wholesale replacement; and
- `PKC-0010` demonstrates identity surviving relocation.

## Step 3 — Separate local tool capability from cross-project capability

For each recovery job, ask:

1. Does a surveyed approach already solve the problem strongly within its own domain?
2. Is the remaining problem merely integration across several domains?
3. Do existing semantics need an engineering-specific extension?
4. Is the remaining behavior genuinely absent from the surveyed mechanisms?

This produces the `REUSE / INTEGRATE / EXTEND / NEW` disposition.

## Step 4 — Minimize the semantic unit

When the corpus shows that whole-document semantics are too coarse, derive the smallest unit required by the evidence.

Examples:

- authority may be scoped to a property or claim rather than a whole artifact;
- freshness may apply to an assertion rather than a whole file;
- evidence supports a proposition, not simply an artifact;
- semantic identity may apply only to objects whose continuity across representations matters.

Do not infer finer granularity than the evidence requires.

## Step 5 — Preserve native authority

A Project Knowledge capability must not silently seize authority from the system that legitimately owns a concern.

Examples:

- Git remains authoritative for committed source-state identity;
- an accepted ADR can remain authoritative for its architectural decision;
- an issue tracker may remain authoritative for workflow fields it owns;
- an external standard remains authoritative for its own normative requirement;
- a generated view receives only authority justified by its source lineage.

Cross-project memory should explain and compose authority, not flatten it.

## Step 6 — Apply the progressive-structure test

For every candidate capability ask:

> Can a project that does not need this capability remain simple?

A candidate fails the test if it requires all artifacts to be imported, assigned heavyweight schemas, manually classified, or duplicated into a proprietary system before basic use is possible.

The preferred shape is progressive enrichment and graceful degradation.

## Step 7 — Apply the reconstruction test

A capability should improve at least one realistic recovery query such as:

- What is true now?
- What was believed then?
- Why did this change?
- Which representation is authoritative for this property?
- What does this evidence actually support?
- What source state produced this projection?
- What depends on this decision?
- Why are these two artifacts contradictory?
- How did the project get from the original problem to the current implementation?

## Step 8 — Apply the counterevidence test

Record what would make the capability unnecessary, narrower, or optional.

Examples:

- stable semantic identity is unnecessary for an ephemeral note that never crosses representations;
- explicit temporal metadata may be unnecessary when commit history alone satisfies the recovery job;
- structured causality should not be required where ordinary rationale prose is sufficient;
- a generated current-state projection should not replace a simple human-authored README when the latter remains cheap and reliable.

## Step 9 — Assign confidence

Use the corpus coverage matrix and approach analysis to classify support:

- **High** — ready for requirement-level treatment, subject to explicit scope;
- **Medium** — candidate can enter requirements only with careful scoping or as an optional capability;
- **Low / HOLD** — continue discovery before formal requirement derivation.

## Step 10 — Do not derive implementation

A capability statement should say **what semantic or user behavior is required**, not how it is stored.

Good:

> The system can associate several native artifacts with one stable semantic object while preserving each artifact's native identity.

Premature:

> Store every object as an RDF resource in Neo4j with a UUID.

Good:

> The system can distinguish valid/effective time from recorded/system time where that distinction is material.

Premature:

> Use a bitemporal PostgreSQL schema for every record.

## Promotion rule

A candidate capability may be promoted toward product requirements when all of the following are true:

1. at least one user job is explicit;
2. supporting corpus evidence is identifiable;
3. relevant failure modes or counterexample constraints are explicit;
4. existing-approach coverage has been credited;
5. the residual gap can be stated without implementation assumptions;
6. progressive structure is preserved; and
7. confidence is High, or Medium with narrow/optional scope.
