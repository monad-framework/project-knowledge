# Initial Corpus Observations

These observations are derived from `PKC-0001` through `PKC-0007`. They are provisional discovery findings, not requirements or architectural decisions.

## O-001 — One logical thing may have many representations

A work packet can exist as a canonical governed artifact, a GitHub coordination issue, machine projections, execution records, evidence records, and implementation history without those representations being independent logical objects.

The useful distinction is not simply `duplicate versus unique`. Representations can have different purposes, authorities, update cadences, and validity intervals.

## O-002 — Storage identity and semantic identity differ

A file path, issue number, worktree path, branch name, or generated document identifies a storage or tool representation. It does not necessarily identify the underlying engineering concept globally or durably.

Stable semantic identity may need to survive moves, projections, regenerated files, historical snapshots, and tool boundaries.

## O-003 — Authority is contextual

An artifact can be authoritative for one property and non-authoritative for another.

Examples include Git/EOS authority over work lifecycle, GitHub as coordination projection, and an execution worktree as implementation context while canonical control state remains elsewhere.

Authority therefore appears to be scoped rather than an undifferentiated boolean property.

## O-004 — Current truth and historical truth must coexist

A current artifact can preserve statements that were valid earlier but are not current instructions. A stale coordination projection may be historically accurate even when it is no longer current.

Deleting obsolete statements loses reasoning and history; presenting them without temporal context creates contradictions.

## O-005 — Provenance has several meanings that should not be collapsed

The corpus distinguishes at least:

- who or what produced information;
- what source state information was derived from;
- where an event occurred;
- what symbolic locator was requested;
- what immutable identity that locator resolved to;
- what was actually observed; and
- what an authoritative record claims occurred.

A single generic `source` field would hide important differences.

## O-006 — Observation context can change interpretation

Checkout, branch, worktree, baseline, host filesystem, CI environment, and lifecycle moment can change what a query sees or how a record is interpreted.

Context can be relevant knowledge rather than incidental execution detail.

## O-007 — Derived information needs lineage

Machine projections, GitHub coordination artifacts, validation evidence, and generated contracts are useful derived representations.

Without lineage, however, derivation can obscure which source was authoritative, when the derivation occurred, and whether the derived representation is still synchronized.

## O-008 — Evidence validity is claim-relative

Evidence is not simply `fresh` or `stale` in the abstract. Its validity depends on what claim it supports and which changes are semantically relevant to that claim.

Repository-wide byte or commit identity can be too broad when control-plane or derived artifacts change without changing the verified proposition.

## O-009 — Environment locators are not durable reconstruction identities

A worktree path can document where an execution occurred while being unsuitable as the durable basis for reconstructing the source state later.

Durable history may need both historical location and immutable/reconstructable state identity.

## O-010 — Classification precedes organization

Not every parsable file belongs to canonical project knowledge. Git administrative metadata demonstrated that discovery by filesystem presence alone can admit incidental state and contaminate downstream projections.

Before organizing information, the system must understand whether and in what role the information belongs.

## O-011 — Validation claims themselves need interpretation

A passing verifier may establish a weaker invariant than a human assumes. In `PKC-0006`, ancestor validation passed while the recorded execution baseline was semantically wrong.

Knowing `verification passed` is less useful than knowing `which proposition was checked, against which inputs, with which result`.

## O-012 — Causality is operationally useful

Several cases are best understood as causal chains rather than static artifact collections:

`representation difference -> classification error -> generated projection contamination`

`verification transition -> control mutation -> fingerprint change -> self-invalidated evidence`

`requested base -> incorrect baseline recording -> misleading diff semantics -> verifier still passes`

Preserving causal explanation appears valuable for diagnosis and learning.

## Emerging dimensions

The corpus currently suggests that engineering information may need to be understood along several independent dimensions:

1. identity;
2. representation;
3. type or semantic role;
4. relationship;
5. authority scope;
6. provenance;
7. valid time;
8. event/transaction time;
9. observation context;
10. derivation lineage;
11. epistemic or verification state;
12. claim/evidence scope; and
13. causal relationship.

This list is deliberately not a schema. Discovery should attempt to remove, combine, refine, or falsify dimensions before domain modeling.

## Important consequence

The initial hypothesis that this problem might be solved principally by representing engineering information as a graph is incomplete.

Graph relationships may be useful, but the corpus demonstrates requirements-shaped pressure around temporal semantics, authority, provenance, context, classification, evidence, and derivation that a generic graph representation does not solve by itself.
