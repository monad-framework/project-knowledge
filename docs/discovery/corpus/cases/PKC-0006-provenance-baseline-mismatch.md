# PKC-0006 — Recorded execution baseline disagrees with requested operational baseline

## Source context

Project: Monad

Source:

- GitHub issue `#178` — `[Defect] EOSE --base does not set recorded execution baseline`
- https://github.com/monad-framework/monad/issues/178

Observed during continuation execution `EXEC-0003` for `WP-MVP-0001`.

## Observed situation

EOSE accepted an explicit `--base` ref and created the execution worktree at that requested base, but the execution registry and generated contract recorded the canonical control checkout `HEAD` as the baseline instead.

The worktree therefore represented one operational starting point while the durable execution metadata claimed another.

Contract verification still passed because it checked only that the recorded baseline was an ancestor of the worktree head. The validation rule was internally satisfied while the recorded provenance was semantically inaccurate.

## Information involved

- requested base ref;
- resolved base commit;
- canonical checkout head;
- worktree initial head;
- recorded execution baseline;
- execution contract;
- execution registry;
- changed-file claims;
- validation invariant; and
- actual implementation diff.

## Why this is difficult to organize

The case cannot be understood from one artifact alone. The command intent, resolved Git state, worktree state, registry metadata, generated contract, and verifier semantics must be compared.

Several records can all be syntactically valid while disagreeing about the real-world event they are meant to describe.

This is a provenance failure: the durable record is not merely missing context; it asserts the wrong origin state.

## Candidate relationships

- execution `requested base` ref;
- ref `resolved to` commit;
- worktree `initialized at` commit;
- registry `records baseline` commit;
- contract `records baseline` commit;
- verifier `checks invariant` ancestor relation;
- changed-file claim `computed relative to` baseline;
- recorded provenance `disagrees with` operational provenance.

## Time, authority, provenance, and context

### Time

The baseline describes execution creation time. Later commits cannot repair the historical meaning of an incorrectly recorded baseline without an explicit correction trail.

### Authority

The registry and contract are intended to be authoritative execution records, but authority does not guarantee factual correctness.

### Provenance

Provenance is the central dimension. The case distinguishes declared origin, requested origin, resolved origin, and actual worktree origin.

### Context

The canonical checkout and execution worktree have different heads by design, so a generic `HEAD` reference is context-dependent.

## Recovery questions

- What base did the operator request?
- Which commit did that ref resolve to at execution creation?
- At which commit was the execution worktree actually initialized?
- What baseline did the registry and contract record?
- Why did verification pass despite the mismatch?
- Which changed-file calculations were affected?
- Was the record later corrected, and if so, how should both the original and correction be preserved?

## Provisional observations

1. Provenance needs semantic invariants, not only structurally valid metadata.
2. Authority and correctness are separate properties.
3. Context-sensitive identifiers such as `HEAD` should not be treated as globally unambiguous provenance.
4. Validation can prove a weaker proposition than users assume it proves.
5. Project memory should preserve enough information to distinguish requested, recorded, resolved, and observed state when those concepts differ.

## Open questions

- Should provenance records preserve both symbolic refs and resolved immutable identities?
- How should corrections to historically inaccurate provenance be represented without rewriting history?
- Can validation claims themselves be modeled so that users can see exactly what proposition was checked?
