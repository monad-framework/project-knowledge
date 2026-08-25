# PKC-0007 — Evidence freshness changes with host-local historical state

## Source context

Project: Monad

Source:

- GitHub issue `#214` — `[Defect] EOSV execution-bound freshness depends on local historical worktree existence`
- https://github.com/monad-framework/monad/issues/214

Observed after `WP-MVP-0003` had closed and its implementation was merged.

## Observed situation

EOSV reconstructed evidence source context differently depending on whether a historical execution worktree path still existed on the current machine.

On a developer machine where the old worktree remained, strict verification used that historical filesystem. In GitHub Actions, where the path did not exist, verification fell back to repository root. The same committed evidence therefore produced different freshness outcomes on different machines.

The difference was caused by host-local historical state that was neither canonical nor guaranteed to exist.

## Information involved

- evidence records;
- execution identity;
- recorded worktree path;
- immutable baseline commit;
- source fingerprint;
- current repository source;
- historical filesystem contents;
- host environment;
- CI environment; and
- freshness result.

## Why this is difficult to organize

A durable record included a path to an execution environment, but the later verification process implicitly treated the continued existence of that environment as meaningful input.

The path served as provenance, locator, and executable reconstruction hint at the same time. Those are different roles.

The record itself was identical across machines, but interpretation depended on non-canonical ambient state. Without preserving that dependency explicitly, the discrepancy appears mysterious.

## Candidate relationships

- evidence `bound to` execution;
- execution `records historical worktree locator` path;
- execution `has immutable baseline` commit;
- verifier `attempts reconstruction from` historical context;
- host `may contain` historical worktree;
- verification result `depends on` selected source view;
- source view `should derive from` canonical reconstructable state.

## Time, authority, provenance, and context

### Time

The worktree existed during execution but may legitimately disappear after closure. Historical provenance therefore outlives the environment it once identified.

### Authority

Host-local filesystem existence is not canonical authority even when the path was once operationally valid.

### Provenance

A recorded locator can explain where an event happened without being sufficient to reconstruct that event later.

### Context

Developer and CI machines have different incidental historical state, which changes interpretation of otherwise identical records.

## Recovery questions

- What source state did the evidence originally verify?
- Can that source state be reconstructed from durable canonical information?
- Was the recorded worktree path intended as provenance, a live locator, or both?
- Why did local verification differ from CI?
- Which host-local state influenced the result?
- Can deleting an obsolete environment change historical truth?

## Provisional observations

1. Durable provenance should not silently depend on ephemeral environment existence for later interpretation.
2. A locator and a reconstructable identity are different concepts.
3. Historical environments may disappear while the knowledge about what occurred in them must remain intelligible.
4. Re-evaluation of canonical knowledge should be deterministic across hosts unless environment is explicitly part of the claim.
5. Project memory may need to distinguish `where this happened` from `what immutable state this refers to`.

## Open questions

- What minimum durable state is necessary to reconstruct historical execution context?
- When should environment characteristics be part of a claim rather than incidental metadata?
- How should the system represent evidence that was valid under an environment-specific condition?
