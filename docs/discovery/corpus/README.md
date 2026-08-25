# Real Information Corpus

The discovery corpus is a collection of real engineering-information cases used to derive requirements for Project Knowledge.

The corpus deliberately includes both difficult cases and counterexamples. If it contains only pathological examples, the project will overfit complexity; if it contains only clean examples, it will miss the situations that motivated the work.

## Purpose

Each case should help answer one or more of these questions:

- What information must survive?
- What makes the information hard to recover or interpret later?
- Which relationships matter to understanding it?
- Which notions of authority, provenance, time, evidence, or context are involved?
- What information legitimately participates in multiple views?
- What information is source versus projection, observation versus assertion, current versus historical, or semantic versus incidental?
- What user job becomes difficult when this information is not modeled well?
- When are ordinary files, ordering, links, search, and Git already sufficient?
- What capability might be required without prematurely choosing an implementation?

## Evidence policy

Corpus cases should be grounded in identifiable source material. Sources may include repository files, commits, issues, pull requests, reviews, execution records, logs, conversations, experiments, or other durable artifacts.

A case may summarize evidence, but it should preserve enough source identity that another investigator can reconstruct the case.

Do not copy an artifact merely because it exists. Capture the engineering-information problem—or the useful simplicity—demonstrated by the artifact.

Negative evidence is first-class. A case showing that a simple project does **not** need richer structure can constrain the eventual design as strongly as a complex failure case.

## Case IDs

Cases use stable identifiers:

`PKC-NNNN`

The identifier refers to the discovery case, not to the underlying project artifact.

## Case structure

The initial template records:

1. source context;
2. observed situation;
3. information involved;
4. why ordinary organization is difficult;
5. candidate relationships;
6. temporal and authority concerns;
7. recovery questions;
8. provisional observations; and
9. open questions.

Counterexamples may adapt this structure when the important evidence is that ordinary organization is *not* difficult enough to justify richer machinery.

Observations are evidence-derived but remain provisional during discovery. They are not requirements or architecture decisions.

## Corpus evolution

The first pass used Monad because its engineering process contains rich interactions among product intent, architecture, specifications, governed work, source code, execution state, evidence, Git history, and GitHub coordination projections.

The second pass adds Project Knowledge dogfood cases and a lower-ceremony frontend project so the corpus begins testing its own assumptions.

The corpus remains narrow. See [`coverage-matrix.md`](coverage-matrix.md) for supported dimensions, confidence, counterevidence, and missing case types.

## Current cases

### Monad-heavy seed cases

- `PKC-0001` — A work packet as a multi-view engineering object
- `PKC-0002` — Coordination projection diverges from canonical lifecycle state
- `PKC-0003` — Canonical execution state is invisible from the executor context
- `PKC-0004` — Administrative metadata is misclassified as project source
- `PKC-0005` — Verification evidence invalidates itself through lifecycle persistence
- `PKC-0006` — Recorded execution baseline disagrees with requested operational baseline
- `PKC-0007` — Evidence freshness changes with host-local historical state

### Counterexample and evolution pass

- `PKC-0008` — Project status summary drifts behind actual project phase
- `PKC-0009` — A working hypothesis is refined by evidence without being erased
- `PKC-0010` — Semantic ADR identity survives repository relocation
- `PKC-0011` — Ordered authoritative documents can be enough

## Admission discipline

Before adding a case, ask whether it adds at least one of:

- a genuinely new failure pattern;
- a materially different project context;
- counterevidence to an emerging assumption;
- a missing epistemic/temporal/authority pattern;
- a new recovery job; or
- evidence strong enough to increase or decrease confidence in an existing observation.

The corpus should not grow through repetitive examples that add volume without increasing discriminating evidence.