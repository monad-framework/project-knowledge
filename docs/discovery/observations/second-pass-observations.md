# Second-Pass Corpus Observations

These observations are derived from `PKC-0008` through `PKC-0011` and are interpreted together with the initial corpus. They remain provisional discovery findings, not product requirements or architecture decisions.

## O-013 — Freshness can be assertion-scoped

`PKC-0008` shows that a document can remain useful and authoritative for some statements while one present-tense status assertion becomes stale.

Treating an entire document as simply `fresh` or `stale` is often too coarse. Freshness can attach to a claim, field, section, projection, or declared dependency.

## O-014 — Knowledge evolution is richer than supersession

`PKC-0009` does not fit a simple `old claim -> superseded by new claim` model.

The original working hypothesis remains partly supported while later evidence narrows the sufficiency of one framing. Engineering knowledge can be strengthened, weakened, generalized, narrowed, conditioned, or partially rejected without becoming wholly obsolete.

## O-015 — Relocation is not semantic revision

`PKC-0010` demonstrates a clean separation between semantic identity and storage locator.

A representation can move while the underlying decision retains its identity, status, meaning, and historical continuity. Repository operations such as rename/move therefore should not automatically be interpreted as semantic lifecycle transitions.

## O-016 — Progressive structure is necessary for generality

`PKC-0011` provides counterevidence against a universally rich capture model.

A numbered document sequence, repository conventions, README summary, implementation source, and Git history can provide sufficient project memory for many tasks. Richer semantics should be introduced when they resolve concrete ambiguity, retrieval, traceability, evolution, or recovery problems.

A system that requires heavy modeling before delivering value would reproduce the cognitive burden it is intended to reduce.

## O-017 — Ordering and hierarchy are useful projections, not mistakes

The initial discussion emphasized graph-shaped relationships because one item can participate in many views. `PKC-0011` shows the complementary fact: deliberate linear order can itself encode valuable semantics.

The problem is not hierarchy or sequence. The problem is forcing all knowledge into a single hierarchy when several organizations are needed.

## O-018 — Capture completeness is not automatically desirable

A smaller project can operate effectively without preserving every discussion, abandoned thought, or intermediate rationale.

The goal should not become `capture everything`. Capture has cost, and excessive retained material can reduce intelligibility.

The eventual system needs a principled way to distinguish useful durable memory from incidental working context, while allowing projects to choose different retention depths.

## O-019 — Current-state summaries are valuable even when derived imperfectly

`PKC-0008` and `PKC-0011` both show the usefulness of a compact README that compresses a much larger body of project material.

The solution to summary drift should not be to eliminate summaries. A better system should preserve the human value of curated compression while making dependencies, freshness, or derivation more visible where useful.

## O-020 — Discovery must preserve negative evidence

`PKC-0011` is important precisely because it does **not** exhibit all the complexity seen in Monad.

If the corpus contains only pathological or governance-heavy cases, the resulting design will overfit them. The domain model and requirements should be constrained by both:

- cases showing why richer structure is needed; and
- cases showing when ordinary files, ordering, links, search, and Git are already enough.

## Revised emerging interpretation

The working problem is increasingly better described as **adaptive project memory** than as a fixed knowledge-graph problem.

A useful system may need to preserve and connect semantic identity, representations, provenance, time, authority, epistemic evolution, evidence, context, lineage, and causality—but it must allow those semantics to remain implicit when ordinary project artifacts already serve the user's recovery jobs adequately.

This interpretation is still a discovery hypothesis. It should not be promoted to architecture yet.