# PKC-0003 — Canonical execution state is invisible from the executor context

## Source context

Project: Monad

Source:

- GitHub issue `#175` — `[Defect] EOSE contract verification is not visible from isolated execution worktree`
- https://github.com/monad-framework/monad/issues/175

Observed context named by the issue:

- work packet `WP-MVP-0001`;
- execution `EXEC-0002`;
- execution branch `wp/mvp-0001`;
- isolated linked worktree;
- canonical control checkout.

## Observed situation

EOSE creates an isolated execution worktree from a committed baseline and then creates live execution registry/session/contract state in the canonical control checkout.

The generated execution instructions tell the executor to verify the live contract. When verification is run from the isolated worktree, that worktree sees only its baseline snapshot of `.eos` and reports the execution session as unknown. The same verification succeeds from the canonical checkout.

The execution state exists and is valid, but its visibility depends on the filesystem/repository context from which the query is performed.

## Information involved

- execution identity;
- execution contract;
- canonical control state;
- isolated source snapshot;
- branch identity;
- baseline commit;
- worktree identity;
- operating instructions;
- query context; and
- control-plane versus product-plane boundaries.

## Why this is difficult to organize

The same repository-shaped interface exists in multiple checkouts, but those checkouts do not contain the same categories or versions of information.

A path such as `.eos/...` is therefore not sufficient identity by itself. Its meaning depends on checkout, branch, baseline, and whether the requested knowledge is intended to be immutable execution input or live canonical control state.

A simple knowledge snapshot may be internally consistent while still being incomplete for a user job that needs live state.

## Candidate relationships

- execution `has canonical control record` registry/session;
- execution `has source snapshot` worktree;
- worktree `created from` baseline commit;
- generated instructions `refer to` live execution contract;
- query `executed within` repository context;
- context `can observe` particular knowledge version;
- live control record `created after` worktree baseline.

## Time, authority, provenance, and context

### Time

The worktree's source view predates creation of the live execution record.

### Authority

The canonical checkout owns mutable execution-control state; the worktree owns an isolated implementation context.

### Provenance

The worktree can be traced to its baseline, while the execution record was created subsequently in another context.

### Context

Context is central to this case: an identical command name produces different knowledge because it resolves repository state relative to the current worktree.

## Recovery questions

- Where is the authoritative record for `EXEC-0002`?
- Which source snapshot was the executor assigned?
- Which knowledge was intentionally frozen at execution creation?
- Which knowledge must remain live during execution?
- Why did the verification fail in one checkout and pass in another?
- Which commands are valid from which contexts?
- What information changed after the execution worktree was created?

## Provisional observations

1. Knowledge availability can depend on execution context even when identities appear identical.
2. Source snapshot semantics and live control-state semantics must not be conflated.
3. File path alone may be insufficient to identify a knowledge artifact when multiple repository contexts exist.
4. Useful provenance may need to include source context, baseline, and observation context in addition to artifact identity.
5. A system can preserve correct data yet still fail a recovery job if the user cannot determine where authoritative live data resides.

## Open questions

- Is observation context a first-class property of a query, an artifact, or both?
- How should project memory describe intentionally frozen versus intentionally live knowledge?
- What should remain globally addressable across repository clones, worktrees, branches, and historical snapshots?
