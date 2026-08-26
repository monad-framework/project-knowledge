# Retrieval Recovery Jobs

RT-1 is designed from recovery questions rather than graph operations.

## RJ-001 — Explain why a capability or implementation exists

**Primary traces:** UJ-003, UJ-011, DF-003

Starting from a known Subject, Representation, Claim, or native source, recover:

- its identity and role;
- current authoritative state when applicable;
- recorded incoming and outgoing rationale/decision relationships;
- relevant provenance Activities;
- supporting Evidence Evaluations;
- authoritative Representations; and
- native source locators.

The result must distinguish a recorded `motivates`/`governs_design_of` edge from a merely adjacent artifact.

### DF-003 expected answer shape

Starting from the CA-1 Subject, the user should be able to see that:

- CA-1 currently has `capability_status = implemented_and_validated`;
- DF-001 and DF-002 are recorded as motivating evidence for the authoring design;
- ADR-0003 governs that design;
- the CA-1 closure verifies the completion Claim; and
- the relevant native files are directly locatable.

## RJ-002 — Explain what supports a Claim

**Primary traces:** UJ-007, FR-501 through FR-504

Starting from a Claim, recover:

- Assertions of that Claim;
- their Representations;
- applicable Authority where relevant;
- Evidence Evaluations bound to that exact Claim;
- the current freshness/state of those evaluations; and
- the exact native evidence inputs.

Evidence for another Claim must not be silently included as support.

## RJ-003 — Explain where a Representation came from

**Primary traces:** UJ-005, UJ-010

Starting from a Representation, recover:

- its Subject binding;
- native reference and observed source state;
- Activities that generated or used it where recorded;
- explicit derivation/provenance Relationships;
- other Representations of the same Subject; and
- freshness when the Representation has declared derivation inputs.

## RJ-004 — Recover related project context after absence

**Primary traces:** UJ-011, FR-608

Starting from a known Subject, recover enough bounded context to resume work:

- current resolved concerns;
- important Representations;
- recorded rationale/decision/evidence relationships;
- unresolved or missing information as `unknown`/absent rather than fabricated; and
- source locations for deeper reading.

RT-1 does not promise a complete project briefing. It provides a trustworthy local recovery neighborhood.

## RJ-005 — Follow a specific recorded path

**Primary traces:** UJ-003, UJ-004, UJ-010, FR-605

A caller should be able to ask for a bounded path/neighborhood beginning from an exact semantic entity and optionally ending at another exact entity.

Examples:

```text
Representation(DF-001) → motivates → Representation(authoring design)

Representation(ADR-0003) → governs_design_of → Representation(authoring design)

Representation(CA-1 closure) → verifies → Claim(capability_status)
```

If no recorded path exists within the supplied constraints, the result must say so rather than synthesize one.

## RJ-006 — Inspect deterministic structural context

Not every useful semantic connection is stored as a `Relationship` record. Existing record fields already define deterministic bindings:

- Subject ↔ Representation;
- Subject ↔ Claim;
- Claim ↔ Assertion;
- Assertion ↔ Representation;
- Subject/concern ↔ Authority ↔ Representation;
- Claim ↔ Evidence Evaluation;
- Activity ↔ generated Representation;
- Context ↔ attached Assertion/Authority/Evidence; and
- records ↔ their Native References.

RT-1 must expose these bindings as traversal edges without persisting duplicate S2 Relationship records.

## RJ-007 — Preserve ambiguity and incompleteness

When:

- a selector matches multiple entities;
- no authoritative current Claim exists;
- evidence is stale or inconclusive;
- a requested relation/path is absent; or
- traversal is truncated by bounds,

the user must see that state explicitly.

The retrieval layer must not “complete” the answer from similarity, chronology, or guesswork.

## RJ-008 — Support downstream machine use without a second semantic model

The same traversal result should be available as stable JSON so future:

- CLI projections;
- impact analysis;
- search/ranking layers;
- web views;
- narrative tooling; or
- external integrations

can consume one retrieval contract rather than reimplement semantic traversal independently.
