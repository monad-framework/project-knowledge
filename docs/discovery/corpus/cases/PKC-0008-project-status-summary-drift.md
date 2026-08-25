# PKC-0008 — Project status summary drift

## Case type

Dogfood case; current-state summary; projection drift; low-complexity counterexample.

## Source project

Project Knowledge (`monad-framework/project-knowledge`).

## Observed situation

After PR #2 merged the initial Discovery corpus into `main`, the repository root `README.md` still stated:

- `Phase 0 — Inception`; and
- `Current work lives under docs/inception/`.

At that point the durable repository already contained a substantial `docs/discovery/` corpus and the merge commit explicitly described the work as an evidence-driven discovery corpus.

The README was not false historically: it accurately described an earlier project state. It had become false as a present-tense status summary.

## Why this case matters

This is a deliberately ordinary case. It does not depend on Monad's EOS machinery, generated projections, execution worktrees, or complex governance.

A human-maintained summary document can drift simply because the project moves forward and the summary is not updated in the same transaction.

This demonstrates that the underlying problem exists even in a small repository with a handful of Markdown files.

## Distinctions exposed

### Historical truth versus current truth

The statement `Phase 0 — Inception` was once correct. Preserving that historical fact is useful. Presenting it as current status after Discovery begins is misleading.

### Summary representation versus underlying state

The root README is a reader-facing summary representation. The actual project state can be inferred from merged repository history and the presence/current use of Discovery artifacts.

The summary is useful, but it is not automatically synchronized with those sources.

### Property-specific authority

The README may remain authoritative for project purpose and development philosophy while being stale for the narrower property `current phase`.

This reinforces that authority and freshness can be property-scoped rather than whole-document booleans.

## Failure mode

A reader arriving through the repository root receives a stale orientation signal and may navigate to the wrong working area or infer that Discovery has not begun.

No individual artifact is corrupt. The failure is a mismatch among representations of current state.

## Recovery task

A maintainer should be able to answer:

1. What phase is the project actually in?
2. Which source establishes that state?
3. Which summaries still present an older state?
4. Can those summaries be updated without erasing the historical fact that Inception was previously current?

## Pressure on the eventual system

A useful system may need to distinguish:

- durable historical statements;
- current-status assertions;
- summaries derived from or dependent on other state;
- the last synchronization or validation point for a summary; and
- the scope within a document for which freshness matters.

The case does **not** imply that README files should be generated automatically. Manual summaries can be valuable. It suggests only that dependencies between summary claims and project state should be representable or checkable when the cost is justified.

## Immediate disposition

Correct the root README on the discovery branch after recording this case. Git history preserves the stale state as evidence without requiring the incorrect present-tense status to remain on `main`.