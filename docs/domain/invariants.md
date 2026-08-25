# Domain Invariants

These invariants are normative constraints on any later schema, architecture, API, or implementation.

## INV-001 — Native authority is not erased by integration

Integrating a native artifact MUST NOT silently transfer its authoritative scope to a Project Knowledge-owned copy, index, or projection.

## INV-002 — Mutable locators are not immutable identities

Paths, URLs, aliases, branches, symbolic refs, and host-local locations MUST remain distinguishable from stable native or reconstructable source-state identity.

## INV-003 — Semantic identity is optional

A native artifact MUST NOT require a Subject solely to participate in Project Knowledge.

## INV-004 — Subject identity is stable

Once assigned, a Subject identifier MUST NOT be reused for a different logical Subject.

## INV-005 — Identity resolution may remain unknown

The system MUST support unresolved identity rather than forcing false merges or false distinctions.

## INV-006 — Representation role is not authority

A Representation role describes participation; authority requires an Authority Assignment or applicable project policy.

## INV-007 — Assertion is not truth

An Assertion records that a source presented a Claim. It MUST NOT automatically make that Claim true or authoritative.

## INV-008 — Repetition is not independent corroboration

Representations or Assertions that derive from one upstream source MUST retain lineage sufficient to prevent derived repetition from masquerading as independent support.

## INV-009 — Authority is scoped

Authority MUST be resolvable at a concern/property/claim scope fine enough to avoid granting whole-artifact global authority when only part is governed.

## INV-010 — Authority basis is explainable

Any Project Knowledge assertion that one source governs another Claim/concern MUST expose an inspectable basis or return the authority state as unresolved/unknown.

## INV-011 — Unresolved conflict is valid state

When Claims or Authority Assignments cannot be safely reconciled, the model MUST preserve the conflict rather than fabricate a winner.

## INV-012 — Current truth is derived

Current truth MUST be computed/interpreted from retained Claims, Assertions, authority, time, Context, and policy rather than represented solely by destructive replacement of prior state.

## INV-013 — Historical truth remains distinguishable from current truth

A retained historical Assertion MUST carry or be recoverable with enough context to prevent it from silently appearing as current authoritative state.

## INV-014 — Temporal richness is conditional

Valid-time and recorded-time semantics MUST be available when needed but MUST NOT be mandatory for every artifact or Claim.

## INV-015 — Ephemeral context is not sufficient reconstruction identity

If reconstructability matters, host-local paths, temporary worktrees, mutable refs, or transient sessions MUST NOT be the only retained basis for historical source-state reconstruction when stronger identity is available.

## INV-016 — Context capture is selective

Context SHOULD include only dimensions material to interpretation, reconstruction, evidence, or diagnostics. Mere observability does not justify retention.

## INV-017 — Evidence is Claim-relative

An Evidence Evaluation MUST identify the Claim/proposition it evaluates rather than being generically valid for an entire repository or artifact.

## INV-018 — Evidence cannot prove more than its method

An Evidence Evaluation MUST NOT be presented as establishing a broader Claim than its recorded inputs and method justify.

## INV-019 — Evidence freshness is relevance-scoped when known

When semantic/input relevance is known, unrelated change MUST NOT be sufficient by itself to invalidate evidence; proposition-relevant change MUST be able to invalidate/question it.

## INV-020 — Derived information retains lineage

A Projection or generated Representation MUST retain enough derivation lineage to explain its inputs and currentness when presented as a current-state view.

## INV-021 — Inference is labeled

Inferred relationships, identity matches, classifications, or Claims MUST remain distinguishable from authored/imported/derived facts.

## INV-022 — Search relevance is not authority

Search rank, vector similarity, generated synthesis, backlink count, or frequency MUST NOT determine truth or authoritative scope.

## INV-023 — Chronology is not causality

Temporal adjacency MUST NOT become a causal Relationship without an explicit assertion or inspectable derivation rule.

## INV-024 — Correction preserves retained history

A managed correction MUST allow current state to change without requiring deletion of retained prior Claims/Assertions and their provenance.

## INV-025 — Relationship semantics do not require graph storage

Any storage model selected later must preserve relationship meaning and traversal behavior without the domain model assuming graph technology.

## INV-026 — Provenance compatibility is semantic, not technological

PROV-compatible meaning MAY be represented using any architecture that preserves the required semantics.

## INV-027 — Progressive formalization is object-scoped

Different artifacts/Subjects in the same Project MAY exist at different enrichment levels indefinitely.

## INV-028 — Enrichment is not a prerequisite to native usability

Project Knowledge-specific enrichment MUST NOT make a native artifact unusable in its ordinary native workflow solely because the enrichment exists.

## INV-029 — Access boundaries survive integration

Integration or indexing MUST NOT make protected native information visible beyond its source/policy authorization boundary.

## INV-030 — Projections do not gain authority through materialization

A generated/current/search/narrative Projection has no greater authority than its explicit Authority Assignment or source lineage/policy permits.

## INV-031 — Unknown is a safe first-class outcome

Identity, authority, freshness, contradiction, evidence validity, causality, or current-truth resolution MAY return unknown/insufficient-information when the model cannot justify a stronger conclusion.

## INV-032 — Domain categories remain extensible

Representation roles, Relationship types, Activity kinds, Context dimensions, epistemic annotations, and correction/evolution types MUST be extensible without redefining the meaning of established kernel concepts.

## Architecture implication

An architecture that cannot preserve these invariants is non-conforming even if it can store all of the data fields described by the domain model.