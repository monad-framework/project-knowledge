# Initial Failure-Mode Catalog

These failure modes are derived from the seed corpus and are intentionally phrased independently of any proposed implementation.

## FM-001 — Representation drift

Two representations of one logical engineering object diverge, and the user cannot tell which is current or authoritative.

Observed in: `PKC-0002`.

## FM-002 — Historical truth presented as current truth

A statement that was valid earlier remains visible without enough temporal context and is mistaken for a current instruction or state.

Observed in: `PKC-0001`, `PKC-0002`.

## FM-003 — Canonical knowledge is inaccessible from the user's working context

The information exists but the current checkout, worktree, tool, or interface cannot observe it, causing false absence or blocker conclusions.

Observed in: `PKC-0003`.

## FM-004 — Storage location mistaken for semantic identity

A file path, checkout-relative path, worktree path, branch name, or tool identifier is treated as if it uniquely and durably identifies the underlying knowledge.

Observed in: `PKC-0001`, `PKC-0003`, `PKC-0007`.

## FM-005 — Incidental artifacts admitted as canonical knowledge

Tool metadata or environment artifacts are classified as project source because they are discoverable or parsable.

Observed in: `PKC-0004`.

## FM-006 — Derivation amplifies a classification mistake

A bad source classification propagates into generated documents, indexes, graphs, manifests, summaries, or other projections.

Observed in: `PKC-0004`.

## FM-007 — Evidence invalidates because irrelevant state changed

Evidence freshness is coupled to changes that do not alter the proposition actually verified.

Observed in: `PKC-0005`.

## FM-008 — Evidence remains apparently valid despite incorrect provenance

A verifier checks an invariant weaker than the semantic claim users believe the record makes.

Observed in: `PKC-0006`.

## FM-009 — Durable history depends on ephemeral environment state

Later interpretation or verification changes depending on whether a historical worktree, path, machine state, or other non-canonical environment artifact still exists.

Observed in: `PKC-0007`.

## FM-010 — Provenance concepts collapse into one field

Requested origin, resolved origin, observed origin, recorded origin, producer, location, and derivation source are treated as though they were the same concept.

Observed in: `PKC-0005`, `PKC-0006`, `PKC-0007`.

## FM-011 — Authority is assumed globally rather than by scope

An artifact that is authoritative for one concern is treated as authoritative for every property of the logical object.

Observed in: `PKC-0001`, `PKC-0002`, `PKC-0003`.

## FM-012 — Context-dependent identifiers are interpreted globally

Names such as `HEAD`, relative file paths, worktree-local state, or checkout-derived locations are interpreted without recording the context that gives them meaning.

Observed in: `PKC-0003`, `PKC-0006`, `PKC-0007`.

## FM-013 — Current artifact structure hides causal history

The current repository contains the result of a correction but does not by itself reveal the sequence of observations, failures, decisions, and fixes that produced the present state.

Observed across the defect cases.

## FM-014 — Relationship burden exceeds human working memory

A single engineering object depends on enough requirements, decisions, specifications, work structures, code, evidence, and historical states that a person cannot reliably maintain the complete mental model from directory navigation alone.

Observed in: `PKC-0001` and reinforced by the corpus as a whole.

## FM-015 — Derived repetition is mistaken for corroboration or authority

The same source information appears in several generated or projected forms and gives the false impression of independent evidence or multiple authoritative sources.

Suggested by: `PKC-0002`, `PKC-0004`.

## FM-016 — Correction destroys the path by which knowledge improved

A wrong or obsolete artifact is simply rewritten or deleted, making the current state cleaner while erasing the evidence, reasoning, and learning that explain why it changed.

Risk demonstrated by the temporal and defect cases; preservation of Git history alone may not make this path intelligible.

## Use in later discovery

This catalog is not yet a requirements checklist. Later discovery should:

1. search for counterexamples and additional cases;
2. identify which failures are general versus Monad-specific;
3. group root causes rather than proliferating surface symptoms;
4. rank severity and frequency;
5. connect user jobs to the failures that block them; and
6. derive the minimum capabilities necessary to mitigate the important failures.
