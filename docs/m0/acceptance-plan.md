# M0 Acceptance Plan

M0 is evaluated as an architecture falsification experiment.

## S-1 — Minimal project

Given a Git repository containing only ordinary Markdown, `pk status`/the in-memory compiler must observe it without requiring `.pk/records` or persistent Project Knowledge state.

## S-2 — Identity continuity

Given one logical decision represented at an old and new file location, both Representations may bind to the same Subject without path identity becoming Subject identity.

## S-3 — Scoped authority versus stale projection

Given a canonical source asserting `closed` and a projection asserting stale `ready`, with authority assigned only to the canonical representation, current resolution must return the canonical Claim.

The projection role itself does not make it non-authoritative; the explicit authority assignment does.

## S-4 — Historical correction

Given one old Claim valid before a boundary and a corrected Claim valid after the boundary, as-of resolution must return each at the correct valid time without deleting the earlier Assertion.

## S-5 — Context-dependent observation

A retained Context may identify an earlier Git source state while the current source observation points at a newer state. The compiler must preserve both instead of overwriting the historical Context.

## S-6 — Claim-relative evidence

Evidence for C1 tied to one Git blob must:

- remain current when an unrelated blob changes;
- become stale when the declared input blob changes; and
- not appear as evidence for broader C2.

## S-7 — Derived freshness

A generated Representation whose Activity declares an input Git blob must be current while that blob identity matches and stale after that input changes.

## S-8 — Unknown

An Assertion without sufficient authority must resolve to `unknown`, not truth-by-presence.

## S-9 — Clean-room rebuild

Deleting the S3 SQLite database and recompiling from S1 + S2 must preserve the semantic resolution result.

## Additional gates

CI also requires:

- `cargo fmt --check`;
- Clippy with warnings denied;
- JSON Schema validity;
- rejection of unknown record kinds; and
- all tests passing on a clean runner.
