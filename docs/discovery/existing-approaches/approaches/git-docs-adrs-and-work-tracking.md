# Git, Docs, ADRs, and Work Tracking

This assessment groups four mechanisms that commonly coexist in software projects because they solve different portions of project memory with very low integration cost.

## Git

### What Git solves exceptionally well

Git provides:

- immutable commit/tree/blob objects;
- content-addressed identity for exact stored states;
- parent-linked historical revisions;
- branches and tags as named references into history;
- diffs between states; and
- a ubiquitous repository-native collaboration substrate.

For Project Knowledge, this makes Git a strong existing answer for:

- exact artifact history (`C-01`);
- reconstructing repository state at known revisions;
- preserving correction history without deleting old bytes;
- identifying immutable source states for evidence/provenance; and
- carrying ordinary Markdown/YAML/JSON artifacts with effectively zero new infrastructure.

### What Git does not model by itself

Git history does not tell us, without project-specific conventions:

- which of several files or tools is authoritative for a particular claim;
- that two moved or projected artifacts represent the same semantic object;
- whether a statement was valid in the project domain at a different time than it was committed;
- whether one document `supports`, `contradicts`, `supersedes`, or `implements` another;
- which proposition an evidence artifact validates;
- whether a correction changes one assertion or the meaning of an entire document; or
- how to present a coherent learning narrative from thousands of commits.

A commit graph is therefore excellent **revision history**, but it is not automatically **project knowledge semantics**.

### Important corpus alignment

The Git distinction between immutable object identity and mutable/contextual refs directly supports the findings from `PKC-0003`, `PKC-0006`, and `PKC-0007`: `HEAD`, branch names, checkout-relative paths, and filesystem locations must not be treated as equivalent to immutable source-state identity.

## Docs as code

Docs-as-code reuses Git, plain text, code review, issue tracking, and automated validation for documentation.

### Strengths

- human-readable and diffable;
- low tooling burden;
- natural co-location with code;
- reviewable changes;
- straightforward current-state documentation;
- works well with hierarchy and ordered documents;
- can be generated, linted, linked, and tested; and
- already proven sufficient for the lower-ceremony case in `PKC-0011`.

### Limits

The file/page remains the dominant unit. Relationships, authority, provenance, and temporal semantics usually live in prose, filenames, front matter, or project convention.

This is not a defect in docs-as-code; it is simply beyond the technique's primary responsibility.

## Wikis

Wikis are strong for long-form, cross-linked explanation, onboarding, conceptual pages, and human navigation.

They add a presentation/navigation layer that can be more approachable than repository browsing and often retain their own revision history.

However, a wiki page hierarchy or page-link graph has the same fundamental limitation as files: links are navigational, but the semantics of those links are generally implicit.

A wiki can say “see also ADR-0005,” but it does not inherently know whether that page:

- implements ADR-0005;
- supersedes it;
- explains it;
- contradicts it;
- is generated from it; or
- is merely topically related.

This supports treating a wiki as a valuable **projection/view**, not automatically the canonical knowledge model.

## Architecture Decision Records

ADRs address a much narrower problem extremely well.

They preserve:

- decision context;
- decision pressure;
- options and trade-offs;
- chosen outcome;
- status;
- consequences; and
- often explicit supersession links.

### Strong fit

ADRs are one of the clearest precedents for Project Knowledge's desire to preserve not only **what** is true now, but **why a project arrived there**.

The corpus also demonstrates that a stable ADR identity can survive repository relocation (`PKC-0010`).

### Limits

Turning every observation, question, experiment, implementation, or evidence record into an ADR would destroy the pattern's usefulness.

ADRs should therefore remain a specialized project artifact type or practice, not be generalized into “everything is a decision record.”

Project Knowledge should learn from ADR lifecycle and rationale semantics rather than replacing ADRs.

## Issue and work tracking

Modern issue trackers are strong coordination systems. GitHub Issues, for example, supports:

- discussion;
- issue types and metadata;
- parent/sub-issue hierarchy;
- blocking relationships;
- links to branches, commits, and pull requests; and
- project views over current work.

### Strong fit

This is valuable for:

- work decomposition;
- active ownership;
- coordination state;
- dependency visualization; and
- chronological discussion around implementation work.

### Limits

Issue state is typically **workflow state**, not universal knowledge authority.

The Monad corpus makes that distinction concrete: GitHub work items can be coordination projections while canonical lifecycle authority lives elsewhere (`PKC-0002`).

Treating the issue tracker as the universal source of project truth would recreate `FM-011` (global authority assumed from a locally authoritative artifact).

## Combined assessment

Together, Git + docs-as-code/wiki + ADRs + issues already solve a large fraction of ordinary project memory:

```text
Git            → exact history / source states
Docs / wiki    → explanation / narrative / reference
ADRs           → significant decision rationale
Issues         → active coordination / work relationships
```

This combination is likely the correct baseline for Project Knowledge, not a legacy stack to replace.

The remaining problem is **cross-cutting semantics and recovery** across those systems: stable identity, scoped authority, temporal truth, provenance, epistemic state, evidence, derivation, causality, and queryable relationships among their artifacts.

## Provisional reuse direction

Reuse as concepts/capabilities:

- Git immutable revision/state identity;
- repository-native plain text;
- docs-as-code review workflows;
- ADR decision lifecycle and rationale pattern;
- issue hierarchy/dependency semantics; and
- wiki/document views for authored narrative.

Do not yet decide that any one of them owns the canonical Project Knowledge data model.
