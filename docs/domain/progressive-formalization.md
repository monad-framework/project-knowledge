# Progressive Formalization

Progressive formalization is a normative domain constraint, not merely a user-interface convenience.

Project Knowledge must support increasingly rich semantics without requiring every project or artifact to begin at the richest level.

## Principle

> Structure is introduced when the expected recovery, correctness, governance, or learning value exceeds its capture and maintenance cost.

The domain therefore supports several participation levels.

## Level 0 — Native only

```text
ordinary repository / docs / issues / links / Git history
```

Characteristics:

- no Project Knowledge semantic identity required;
- no typed relationships required;
- no explicit provenance beyond native history;
- ordinary hierarchy and search remain primary.

This is a valid Project Knowledge operating mode when it satisfies the user's recovery jobs.

## Level 1 — Indexed native reference

Project Knowledge knows that a native artifact exists and can retrieve/reference it.

Adds:

- Source System identity;
- Native Reference;
- basic source metadata;
- admission/exclusion state where discovery is automated.

Does not require a Subject.

## Level 2 — Semantic continuity

A Subject is introduced because continuity across representations or locations matters.

Adds selectively:

- Subject identity;
- Representation binding;
- representation role;
- aliases/native identities.

Typical trigger:

> “These several artifacts are representations of the same logical engineering thing.”

## Level 3 — Explicit relationship semantics

Typed Relationships are added where ordinary links are insufficient.

Adds selectively:

- dependency/impact relations;
- representation/derivation relations;
- rationale or evidence links;
- relationship origin/provenance.

Typical trigger:

> “I need to traverse this relationship reliably, not merely know that the pages link.”

## Level 4 — Authority, provenance, time, and context

Richer semantics are added when correctness depends on distinguishing source, authority, historical state, or observation context.

Adds selectively:

- Claims / Assertions;
- Authority Assignments;
- structured provenance;
- valid/recorded time;
- Context;
- projection lineage/freshness.

Typical trigger:

> “Several representations disagree and I need to know which governs, when, and why.”

## Level 5 — Evidence and epistemic semantics

Adds only where validation or uncertainty itself needs first-class treatment.

Adds selectively:

- Evidence Evaluation;
- proposition-scoped validity;
- epistemic annotations;
- correction/refinement semantics.

Typical trigger:

> “What exactly did this evidence prove, against which state, and is it still relevant?”

## Level 6 — Recovery projections and validation

Derived views consume the structured memory accumulated at lower levels.

May include:

- current-state views;
- historical views;
- impact analysis;
- contradiction diagnosis;
- recovery paths;
- narrative views;
- deterministic project-memory projections;
- semantic validation.

The existence of Level 6 views does not require every underlying artifact to be enriched to Level 5.

## Non-monotonic richness

The levels describe available semantic depth, not a mandatory maturity ladder.

A project may have:

```text
README.md                    Level 0
ADR-0001                     Level 2 + authority semantics
GitHub Issue #18             Level 1
CI evidence for critical WP  Level 5
architecture learning path   Level 6 projection
```

Different artifacts can remain at different levels indefinitely.

## Enrichment trigger rule

Before adding a stronger semantic layer, the project should be able to name at least one concrete benefit such as:

- preserve identity across relocation;
- resolve authority safely;
- explain conflicting representations;
- enable impact traversal;
- preserve derivation lineage;
- validate claim-relative evidence;
- reconstruct historical context;
- support recurring context recovery.

“Because the schema supports it” is not a valid trigger.

## Enrichment should be reversible/non-destructive

Where practical, enrichment should be external, additive, embedded compatibly, or otherwise non-destructive to the native artifact.

Removal of Project Knowledge tooling should degrade semantic convenience rather than corrupt the native source.

## Selective retention

Progressive formalization also applies to retention.

The domain distinguishes:

```text
possible to observe
```

from

```text
worth retaining as project memory
```

A project may intentionally omit:

- transient build artifacts;
- incidental filesystem state;
- unimportant conversations;
- generated noise;
- redundant telemetry;
- sensitive material outside policy.

Historical preservation requirements apply to retained material, not every observable event.

## Promotion from informal information

Information may move from lower to higher semantic structure over time.

Example:

```text
chat observation
   ↓ selected by human
Markdown note
   ↓ recurring cross-reference need
Subject + Representation
   ↓ decision pressure
Claim + Authority / rationale
   ↓ validation
Evidence Evaluation
```

The domain does not require the earlier transient source to remain indefinitely unless policy chooses to retain it.

## Demotion / archival depth

A project may reduce active semantic maintenance while retaining history.

For example:

- an old Projection may become `historical` rather than being continuously refreshed;
- a retired integration may preserve last-known Native References and provenance;
- detailed Context may be archived while current views use only reconstructable source-state identity.

Architecture must eventually define how archival policies interact with queryability.

## Minimum-project conformance

A conforming implementation must support a fixture in which:

- most artifacts remain ordinary files;
- only a small number receive Subject identity;
- no graph database is required;
- no temporal database is required;
- no AI is required;
- native Git/document workflows remain usable.

If the system requires maximum metadata to become useful, it violates the domain model.

## Requirement coverage

This model directly implements the meaning of:

- FR-101 through FR-107;
- FR-201's conditional identity;
- FR-206's ordinary-link escape hatch;
- FR-403's conditional temporal metadata;
- FR-505's optional epistemic semantics;
- FR-608;
- QA-001 through QA-003, QA-011, QA-012;
- CON-002, CON-013;
- NR-006, NR-013, NR-019, NR-020.