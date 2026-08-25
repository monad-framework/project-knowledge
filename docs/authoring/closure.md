# CA-1 Closure — Guided Capture and Scaffolding

## Status

**CA-1 implementation complete within its authorized scope.**

CA-1 implements the plan → review → apply authoring boundary selected by ADR-0003 without changing the canonical `pk/v1` semantic model.

The implementation demonstrates that the repeated structural-authoring burden observed in DF-001 and DF-002 can be reduced while preserving the semantic decisions that made those recovery cases correct.

## Implemented surfaces

CA-1 adds:

- `pk-authoring/v1` transient Authoring Intent;
- `pk-capture-plan/v1` transient Capture Plans;
- local aliases and exact existing-record selectors;
- deterministic UUID and canonical record-path generation;
- ordinary capture-time generation;
- relevant Git working-tree blob-state observation;
- authored/generated/observed/suggested field-origin metadata;
- prospective whole-corpus validation;
- relevant-input stale-plan detection;
- idempotent apply;
- divergent-output protection;
- semantic plan rendering; and
- the `pk capture`, `pk capture plan`, and `pk capture apply` command family.

The authoring formats remain operational artifacts. Runtime reconstruction still depends only on native state plus final portable `pk/v1` records.

## Safety result

CA-1 automates mechanical structure but does not silently decide:

- semantic Subject identity;
- Representation role;
- authority scope or basis;
- valid-time meaning;
- evidence breadth;
- epistemic interpretation; or
- causality.

A plan carrying an unconfirmed `suggested` field origin is rejected by the applier.

`recorded_at` may be generated as capture time, but `valid_from` and `valid_until` remain absent unless explicitly authored.

## Relevant-input plan validity

Plan validity is not tied to repository HEAD.

The planner records content identity for native files and existing S2 records it actually inspects. Apply rechecks those inputs with native Git object hashing.

Therefore:

- changing a relevant source after planning makes the plan stale; and
- changing only unrelated repository content does not invalidate the plan.

This preserves the claim-relative/relevant-dependency discipline already established for evidence freshness.

## Acceptance matrix

| Case | Result | Executable evidence |
| --- | --- | --- |
| CA-A01 — DF-001 equivalence | PASS | `ca_a01_and_a18_df001_equivalence_without_structural_boilerplate` |
| CA-A02 — DF-002 equivalence | PASS | `ca_a02_df002_equivalence_preserves_unknown_alternatives_provenance_and_evidence_scope` |
| CA-A03 — no silent authority | PASS | `ca_a03_authority_requires_explicit_scope_and_basis` |
| CA-A04 — identity ambiguity fails closed | PASS | `ca_a04_ambiguous_existing_identity_fails_closed` |
| CA-A05 — relevant source-state precondition | PASS | `ca_a05_relevant_source_change_stales_plan` |
| CA-A06 — unrelated change does not stale plan | PASS | `ca_a06_unrelated_change_does_not_stale_plan` |
| CA-A07 — field origin inspectable | PASS | `ca_a07_field_origins_are_inspectable_and_suggestions_fail_closed` |
| CA-A08 — prospective validation | PASS | `ca_a08_prospective_validation_fails_before_persistent_writes` |
| CA-A09 — idempotent reapply | PASS | `ca_a09_same_plan_reapply_is_idempotent` |
| CA-A10 — divergent path protected | PASS | `ca_a10_divergent_existing_output_is_not_overwritten` |
| CA-A11 — native artifacts unchanged | PASS | asserted by CA-A01 while resolving the generated S2 bundle |
| CA-A12 — recorded time is not valid time | PASS | `ca_a12_recorded_time_never_implies_valid_time` |
| CA-A13 — evidence breadth explicit | PASS | `ca_a13_evidence_breadth_remains_exactly_claim_scoped` |
| CA-A14 — minimal project remains minimal | PASS | `ca_a14_minimal_project_remains_zero_ceremony` |
| CA-A15 — plans are not canonical S2 | PASS | `ca_a15_saved_plan_is_noncanonical_and_deletable` |
| CA-A16 — composable project memory | PASS | `ca_a16_capture_bundles_compose_without_fixed_global_size` |
| CA-A17 — existing M0 suite remains green | PASS | full locked test suite includes M0, DF-001, and DF-002 |
| CA-A18 — human semantic decision count preserved | PASS | CA-A01 emits 10 records and CA-A02 emits 14 records from compact intent without deleting the semantic distinctions required by those cases |

## Verification gate

The final feature branch passes on a clean GitHub runner with the ordinary read-only CI workflow:

```text
cargo fmt --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-targets --all-features
```

All three gates pass.

## What CA-1 demonstrates

CA-1 demonstrates that the Project Knowledge semantic kernel can have a substantially lower-friction authoring surface without weakening the distinction between mechanical structure and semantic judgment.

In particular, a human no longer has to manually provide:

- Project Knowledge UUIDs;
- canonical S2 filenames;
- `schema`/`kind` envelopes;
- Git blob identities; or
- ordinary capture timestamps.

The human still declares the meaning-bearing choices required for correct recovery.

## Scope limitations

CA-1 is not a complete authoring UX.

The interactive CLI currently provides the plan/review/confirmation transaction around Authoring Intent rather than a fully developed field-by-field conversational wizard. This is sufficient to test structural scaffolding and deterministic planning, but DF-003 should measure whether users need a richer guided interaction layer.

CA-1 also does not add:

- fuzzy semantic identity resolution;
- AI/LLM inference;
- automatic authority inference;
- first-class epistemic annotations;
- mutation of existing S2 records;
- crash journaling across multi-file finalization; or
- a remote/web authoring service.

These remain unauthorized unless later evidence requires them.

## Closure conclusion

> CA-1 satisfies its acceptance specification and is ready to be dogfooded on a third real project-memory recovery problem.

The next activity after merge is **DF-003**, which should use `pk capture plan` / `pk capture apply` rather than hand-authoring its S2 bundle.

DF-003 should measure both:

1. whether CA-1 materially reduces mechanical capture burden in real use; and
2. which semantic or interaction friction remains after that burden is removed.
