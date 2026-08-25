# Real Information Corpus

The discovery corpus is a collection of difficult, real engineering-information cases used to derive requirements for Project Knowledge.

The corpus is not a showcase of neatly structured documentation. It intentionally favors cases where information is fragmented, duplicated, temporal, ambiguous, context-dependent, derived, stale, or distributed across several artifacts and tools.

## Purpose

Each case should help answer one or more of these questions:

- What information must survive?
- What makes the information hard to recover or interpret later?
- Which relationships matter to understanding it?
- Which notions of authority, provenance, time, evidence, or context are involved?
- What information legitimately participates in multiple views?
- What information is source versus projection, observation versus assertion, current versus historical, or semantic versus incidental?
- What user job becomes difficult when this information is not modeled well?
- What capability might be required without prematurely choosing an implementation?

## Evidence policy

Corpus cases should be grounded in identifiable source material. Sources may include repository files, commits, issues, pull requests, reviews, execution records, logs, conversations, experiments, or other durable artifacts.

A case may summarize evidence, but it should preserve enough source identity that another investigator can reconstruct the case.

Do not copy an artifact merely because it exists. Capture the engineering-information problem demonstrated by the artifact.

## Case IDs

Cases use stable identifiers:

`PKC-NNNN`

The identifier refers to the discovery case, not to the underlying Monad artifact.

## Case structure

Each case records:

1. source context;
2. observed situation;
3. information involved;
4. why ordinary organization is difficult;
5. candidate relationships;
6. temporal and authority concerns;
7. recovery questions;
8. provisional observations; and
9. open questions.

Observations are evidence-derived but remain provisional during discovery. They are not requirements or architecture decisions.

## Initial corpus

The initial corpus uses Monad because its engineering process already contains rich interactions among product intent, architecture, specifications, governed work, source code, execution state, evidence, Git history, and GitHub coordination projections.

The corpus should later expand beyond Monad so that Project Knowledge does not merely encode Monad's local conventions.

## Current cases

- `PKC-0001` — A work packet as a multi-view engineering object
- `PKC-0002` — Coordination projection diverges from canonical lifecycle state
- `PKC-0003` — Canonical execution state is invisible from the executor context
- `PKC-0004` — Administrative metadata is misclassified as project source
- `PKC-0005` — Verification evidence invalidates itself through lifecycle persistence
- `PKC-0006` — Recorded execution baseline disagrees with requested operational baseline
- `PKC-0007` — Evidence freshness changes with host-local historical state
