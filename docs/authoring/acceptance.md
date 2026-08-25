# Capture/Authoring Acceptance Specification

CA-1 is accepted only if the implementation preserves semantic safety while materially reducing mechanical authoring burden.

## CA-A01 — DF-001 equivalence

Using capture tooling against an isolated fixture equivalent to DF-001, a user must be able to reproduce the same resolver behavior without manually typing:

- Project Knowledge UUIDs;
- S2 destination filenames;
- `schema`/`kind` envelopes;
- Git blob hashes; or
- ordinary capture timestamps.

The resulting S2 must still distinguish historical/current Claims, Assertions, scoped Authority, and claim-relative Evidence.

## CA-A02 — DF-002 equivalence

Using capture tooling against a fixture equivalent to DF-002:

- pre-selection state resolves `unknown`;
- post-selection state resolves JSON;
- alternatives remain non-authoritative unless explicitly asserted;
- provenance Activity is preserved; and
- evidence remains claim-relative.

No new semantic record kind is required.

## CA-A03 — No silent authority

If the user requests an Authority but does not provide/confirm scope and basis, planning must block. The tool must not infer authority from recency, path, PR state, or file role.

## CA-A04 — Identity ambiguity fails closed

When existing Subject lookup yields multiple plausible matches, planning requires explicit selection. No automatic merge is allowed.

## CA-A05 — Relevant source-state precondition

If a Git source artifact whose state is embedded/relevant to a plan changes after planning but before apply, apply must reject the plan as stale.

## CA-A06 — Unrelated repository change does not stale the plan

If only unrelated repository state changes, apply should remain valid when every relevant precondition still holds.

## CA-A07 — Field origin is inspectable

Plan review/JSON must distinguish authored, generated, observed, and suggested material fields.

Unconfirmed semantic suggestions cannot be applied.

## CA-A08 — Prospective validation

A plan containing broken cross-references or schema-invalid output fails before persistent semantic writes.

## CA-A09 — Idempotent reapply

Applying the same completed plan again returns success/no-op when all planned files already contain equivalent content.

## CA-A10 — Divergent existing path is protected

If a planned output path already exists with different content, apply fails rather than overwriting it.

## CA-A11 — Native artifacts remain unchanged

Capture of DF-001/DF-002-equivalent flows must not modify the source Markdown/native artifacts.

## CA-A12 — Recorded time is not valid time

When `valid_from` is omitted, the tool must not silently copy generated `recorded_at` into it.

If an interactive convenience offers “valid from now,” the review must show that as an explicit authored choice.

## CA-A13 — Evidence breadth remains explicit

Selecting evidence inputs for Claim C1 must not create an Evidence Evaluation for related Claim C2 without an explicit separate semantic action.

## CA-A14 — Minimal project remains minimal

A repository can continue to use `pk status`/native files without creating authoring intent, capture plans, or S2 records.

## CA-A15 — Plans are not canonical S2

Deleting saved Capture Plans after successful apply must not alter the semantic results produced from final S2 records.

## CA-A16 — Composable project memory

Adding a new capture bundle must not require previous capture tests/records to assume a fixed global corpus size. Acceptance fixtures and checks must be slice-local where appropriate.

## CA-A17 — Existing M0 suite remains green

All M0 architecture scenarios, schema validation, DF-001, and DF-002 continue to pass after the authoring implementation is added.

## CA-A18 — Human semantic decision count is preserved

The implementation may reduce keystrokes and structural fields, but it must not obtain its usability improvement by silently deleting semantic decisions that were necessary to DF-001 or DF-002 correctness.

This is a safety acceptance criterion, not a UI preference.
