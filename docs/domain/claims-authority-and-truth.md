# Claims, Authority, and Truth

This document defines how Project Knowledge represents statements without collapsing source occurrence, proposition meaning, authority, and truth into one field.

## 1. Why Claim and Assertion are separate

A source artifact contains statements, but the project-memory problem often needs to reason about the meaning of a statement independently of where one occurrence appears.

Therefore the model distinguishes:

```text
Claim      = proposition / meaning
Assertion  = source-bound occurrence presenting that Claim
```

Example:

```text
Claim C1:
  WP-123 lifecycle state = Closed

Assertion A1:
  canonical EOS artifact says C1

Assertion A2:
  generated GitHub issue projection says C1
```

If A2 was derived from A1, the two Assertions are not independent corroboration.

## 2. Claim

A Claim is first-class only when Project Knowledge must reason about one or more of:

- authority;
- contradiction;
- evidence;
- validity over time;
- epistemic status;
- reuse across Assertions; or
- current/historical truth.

A Claim may be structured or opaque in later implementation.

Conceptually it has:

```text
Claim
├── subject?             # what it is about
├── concern?             # lifecycle state, architecture choice, etc.
├── proposition          # meaning/content
└── semantic identity?   # only when useful
```

The domain does not require every sentence in every document to become a Claim.

## 3. Assertion

An Assertion presents a Claim from a source occurrence.

Conceptually:

```text
Assertion
├── claim
├── asserting_source     # Representation / Source System / Agent / Activity
├── recorded_time?
├── valid_time?
├── context?
├── provenance?
├── epistemic_annotation?
└── derivation?
```

An Assertion is evidence that a source presented a Claim. It is not automatically evidence that the Claim is true.

## 4. Concern

A **Concern** is the semantic scope over which Claims may compete for authority or current state.

Examples:

- work-packet lifecycle state;
- accepted architecture decision;
- current implementation target;
- requirement text;
- generated projection freshness.

The model intentionally does not prescribe a universal concern taxonomy.

Concern exists because authority often means:

> source X governs concern Y for Subject S

rather than:

> artifact X is globally authoritative.

## 5. Authority Scope

An Authority Scope is a value describing the domain over which an Authority Assignment applies.

Conceptually:

```text
AuthorityScope
├── project
├── subject?             # exact subject or broader policy selector
├── concern
├── context_selector?
└── valid_interval?
```

A broader scope may be defined by project policy, for example:

```text
All EOS work-packet lifecycle-state concerns
  → governed by canonical EOS artifact state
```

The implementation may support rule-based selectors later; the domain only requires explicit scope semantics.

## 6. Authority Assignment

An Authority Assignment connects a governing source to an Authority Scope and records why.

Conceptually:

```text
AuthorityAssignment
├── scope
├── authority_source
├── basis
├── valid_time?
├── recorded_time?
└── provenance?
```

### Authority source

May be:

- Source System;
- Representation;
- Subject/decision;
- external governing source; or
- project policy identity.

### Basis

Must be inspectable where authority is presented. Examples:

- project policy;
- accepted ADR;
- native ownership rule;
- external regulatory/standards authority;
- explicit human assignment.

## 7. Authority resolution

Authority resolution is not a primitive field lookup.

For a Subject + Concern + query Context/time, resolution considers applicable Authority Assignments.

Valid outcomes include:

- one governing source;
- several compatible governing sources;
- ambiguous/conflicting assignments;
- no applicable assignment.

The resolver MUST NOT manufacture one source when the model yields ambiguity.

## 8. Current truth

Current truth is a **derived state**.

For a Subject/Concern, the conceptual resolution is:

```text
candidate Claims
    ↓ via Assertions
applicable time/context
    ↓
applicable Authority Assignments
    ↓
provenance / derivation distinctions
    ↓
current-state resolution
```

Possible outcomes:

### Resolved

One authoritative current Claim is supported by the applicable authority model.

### Compatible set

Several Claims can coexist without contradiction.

### Unknown

No adequate current Claim or authority information is available.

### Unresolved conflict

Claims or authority assignments conflict and the model cannot safely resolve them.

The last two outcomes are domain-correct, not error states to hide.

## 9. Historical truth

Historical queries must distinguish at least:

- what a source asserted at/after a recorded time;
- what a Claim was considered valid for during an effective interval; and
- what authority/policy applied if historical authority affects interpretation.

Git history alone may supply enough source history in simple cases. Explicit temporal qualifiers are only added when needed.

## 10. Contradiction

Two Assertions are not contradictory merely because their bytes differ.

Potential contradiction requires at least:

1. Claims concern the same semantic Subject/Concern or otherwise overlap materially;
2. their propositions cannot both hold under the relevant interpretation; and
3. temporal/context distinctions do not already explain the difference.

A diagnostic may classify disagreement as:

- stale projection;
- historical-versus-current;
- context-dependent;
- authority-scope difference;
- provenance error;
- semantic identity mismatch;
- unresolved substantive disagreement; or
- unknown.

This classification is itself derived and must remain explainable.

## 11. Correction and refinement

Correction does not mutate history out of existence.

A managed correction is represented as new project-memory state plus preserved relationship/provenance to what changed.

Possible semantics include:

- corrected;
- refined;
- narrowed;
- strengthened;
- weakened;
- rejected;
- superseded.

Only semantics with sufficient project evidence should become normalized relationship types.

The domain therefore permits an extensible change-kind value rather than imposing a universal lifecycle.

## 12. Repetition and independence

The model distinguishes:

```text
independent Assertion
```

from

```text
derived repetition of another Assertion/source
```

A generated issue, summary, index, and wiki page all repeating one canonical source MUST NOT count as four independent authoritative Assertions when lineage shows one origin.

## 13. Epistemic annotation

Epistemic Annotation may qualify a Claim or Assertion with project-specific meaning such as:

- unresolved question;
- hypothesis;
- observation;
- disputed claim;
- accepted result;
- correction.

Rules:

- no universal state machine;
- uncertainty may remain unresolved;
- epistemic annotation does not create authority by itself;
- transition meaning should be explicit when retained.

## 14. Requirement coverage

This model primarily satisfies:

- FR-301 through FR-308;
- FR-305/307 through temporal and correction semantics;
- FR-501/505/506/507;
- FR-603;
- CON-003/004/005;
- QA-004/005/009/010/016.

It also protects NR-007, NR-008, NR-012, and NR-021.