# RT-1 Acceptance Scenarios

These scenarios define the first executable boundary for **RT-1 — Semantic Recovery Traversal**.

They are technology-neutral at the behavioral level even where examples use the proposed CLI.

## RT-A01 — DF-003 explanation is recoverable through the public runtime

Given the merged DF-003 corpus, explaining the CA-1 Subject must expose:

- `capability_status = implemented_and_validated`;
- the applicable Authority;
- both `motivates` Relationships;
- the `governs_design_of` Relationship;
- the `verifies` Relationship;
- the evidence-driven delivery Activity;
- the current Evidence Evaluation state; and
- native source locators.

No raw `.pk/records/*.json` inspection is required.

## RT-A02 — Current state delegates to the existing resolver

For a Subject explanation, each current concern shown must match the result of the existing resolution engine for the same Subject/time/context.

Traversal edge count or ordering must not alter the resolved Claim.

## RT-A03 — Recorded Relationship semantics remain exact

A recorded relation `motivates` must be returned/rendered as `motivates` with its Relationship ID and origin.

The retrieval layer must not silently rename it to `caused`, `depends_on`, or another stronger/different semantic.

## RT-A04 — Structural bindings remain distinguishable from authored Relationships

A Claim→Evidence Evaluation connection derived from `EvidenceEvaluation.claim_id` must be marked structural.

It must not appear as an authored `Relationship` unless such a record actually exists.

## RT-A05 — Evidence remains Claim-relative

Explaining the CA-1 completion Claim may include its recorded Evidence Evaluation.

Evidence attached to unrelated Claims in DF-001/DF-002 must not be presented as direct support for CA-1 completion merely because those records are nearby in the traversal neighborhood.

## RT-A06 — Evidence state remains visible

If a relevant evidence input changes and the existing evidence evaluator reports stale/non-current state, explanation must retain the Evidence Evaluation and display that state.

It must not silently remove the evidence edge or continue to label it current.

## RT-A07 — Exact UUID selection works for every record kind

Each `pk/v1` entity kind can be selected by UUID and produces a valid sparse-or-rich traversal/explanation result.

## RT-A08 — Exact native-locator selection is ambiguity-safe

An exact native locator that identifies one Representation selects it.

If the locator maps to multiple semantic entities, retrieval returns an ambiguity result with candidates rather than selecting one.

## RT-A09 — Exact Subject-label selection is ambiguity-safe

A unique exact Subject label selects the Subject.

Duplicate exact labels fail closed with candidate IDs.

## RT-A10 — Missing selectors do not fall back to fuzzy search

A non-existent UUID/locator/label produces typed `not_found` behavior.

RT-1 does not silently run semantic/content search.

## RT-A11 — Neighborhood traversal is bounded and cycle-safe

A cyclic recorded relationship set terminates deterministically within the requested depth/result bounds.

The result reports truncation when reachable context is omitted.

## RT-A12 — Path query distinguishes `no_path` from `not_found`

When both endpoints exist but no qualifying path is found within bounds, the result is `no_path`.

When an endpoint does not exist, selection fails as `not_found`.

## RT-A13 — Path results preserve exact edge provenance

Every path edge identifies whether it is:

- recorded/authored;
- recorded/imported;
- recorded/derived;
- recorded/inferred; or
- deterministic structural binding.

## RT-A14 — Native-source links preserve locator and state separately

When a native reference contains both a locator and immutable/reconstructable state, both are shown as distinct fields.

A mutable locator is never presented as though it were the immutable source-state identity.

## RT-A15 — Human and JSON output have semantic parity

For the same `pk explain` or `pk trace` query, the human and JSON forms are rendered from the same result object.

IDs, relation labels, origins, resolution outcomes, evidence states, and truncation information agree.

## RT-A16 — Results are deterministic

Two executions against the same compiled S1/S2 state and identical query return the same ordered semantic result.

No map/hash iteration order or scoring nondeterminism changes presentation order.

## RT-A17 — Retrieval is read-only

Running `pk explain` and `pk trace` changes neither native files nor canonical S2 records.

The commands function under read-only repository permissions.

## RT-A18 — Minimal/native-only project remains valid

A project with no S2 enrichment continues to support existing `pk status` behavior.

RT-1 does not require semantic records merely to initialize or inspect a native-only project.

If `pk explain` is asked to select an S2 entity that does not exist, it returns `not_found` without requiring enrichment.

## RT-A19 — Sparse semantic roots remain explainable

A Subject with no Claims or Relationships still produces an accurate explanation containing identity/Representations/source context that exists, plus explicit absence of richer recorded reasoning.

Sparse memory is not treated as invalid memory.

## RT-A20 — Historical resolution composes with traversal

When `pk explain` receives `--at` / `--context`, the current-state section reflects the existing historical/contextual resolver while recorded relationship/provenance edges remain available as historical context unless separately filtered.

## RT-A21 — Existing M0/CA-1/dogfood suite remains green

RT-1 implementation must preserve all existing M0, CA-1, DF-001, DF-002, and DF-003 semantics.

No canonical `pk/v1` schema change is required merely to make these retrieval cases pass.

## RT-A22 — No invented causality

A corpus containing only chronological proximity or a non-causal structural binding must never cause `pk explain` to emit a causal statement.

Only exact recorded relation semantics may justify causal/rationale wording, and the exact source relation remains inspectable.

## Acceptance conclusion

RT-1 is complete only when DF-003's recovery question can be answered through the public runtime and the result remains bounded, source-traceable, deterministic, read-only, and semantically no stronger than the underlying project memory.
