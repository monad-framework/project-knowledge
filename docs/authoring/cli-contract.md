# CLI Contract

## Command family

The first implementation extends the existing `pk` CLI with a `capture` command family.

```text
pk capture
pk capture plan [--intent <file|->] [--out <file>] [--json]
pk capture apply --plan <file> [--yes] [--json]
```

The exact Clap layout may vary during implementation, but the behavioral contracts below are normative for CA-1.

## `pk capture`

Interactive shortcut for guided human authoring.

Expected flow:

1. gather semantic intent;
2. build a plan;
3. render the plan in human-readable form;
4. require explicit confirmation; and
5. apply if confirmed.

The command must not write S2 before the review/confirmation boundary.

Aborting review leaves the S2 corpus unchanged.

## `pk capture plan`

Creates a Capture Plan without applying it.

### Interactive

With no `--intent`, the planner conducts a guided session.

### Non-interactive

`--intent <file|->` reads a `pk-authoring/v1` document from a file or stdin.

If semantic input is incomplete or ambiguous in non-interactive mode, the command exits with an error rather than guessing.

### Output

By default, render a human-readable plan.

`--out <file>` serializes the immutable plan for later apply.

`--json` emits machine-readable plan/result output. If both `--out` and `--json` are provided, the output file remains the applyable plan while stdout may contain the same plan or a documented summary.

## `pk capture apply`

Applies a previously reviewed plan.

It must:

- verify plan version;
- reject blockers;
- re-check relevant preconditions;
- validate the prospective corpus;
- avoid overwriting divergent existing records;
- write planned records; and
- return created/no-op paths plus validation status.

Without `--yes`, an interactive terminal should require confirmation immediately before mutation unless the plan carries an implementation-defined signed/approved state in a future version. CA-1 does not require plan signing.

For non-interactive automation, `--yes` explicitly authorizes applying exactly the supplied plan; it does not authorize filling semantic gaps.

## Human-readable review

A plan review should be organized by meaning, not only by raw JSON files.

Example:

```text
Subject
  NEW  ADR-0001 architecture decision

Representations
  NEW  decision_record
       docs/decisions/ADR-0001-federated-portable-core.md
       observed blob: <sha>

Claim
  decision_status = accepted

Assertion
  selected-architecture representation asserts accepted
  valid from: 2026-08-25T15:24:13Z

Authority
  selected-architecture representation governs decision_status
  basis: Architecture PR #8 adoption decision
  valid from: 2026-08-25T15:24:13Z

Files to create: 5
Warnings: 0
```

Generated UUIDs and exact paths must remain inspectable, but they should not dominate the primary review presentation.

## Candidate selection

Interactive lookup may support selectors such as:

- Subject label;
- Representation path/locator;
- record kind;
- exact UUID.

If a selector yields multiple plausible semantic identities, the CLI presents choices and requires explicit selection.

## Exit behavior

Suggested categories:

- `0` — success or idempotent no-op;
- `2` — invalid command/input syntax;
- `3` — semantic intent incomplete or ambiguous;
- `4` — stale plan/precondition failure;
- `5` — schema/semantic/cross-reference validation failure;
- `6` — conflicting existing output/divergent plan state;
- other runtime/I/O errors use a documented general failure code.

Exact numeric codes may be adjusted for consistency with a future project-wide CLI error contract, but distinct machine-detectable categories are required.

## Compatibility

Existing M0 commands remain valid:

```text
pk init
pk validate
pk compile
pk rebuild
pk status
pk resolve
pk freshness
pk evidence
```

Capture is additive. Projects that never invoke it retain the M0 operating model.
