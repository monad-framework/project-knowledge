# Provenance, Time, Context, and Evidence

This document defines the parts of the domain model needed to reconstruct where knowledge came from, what state it referred to, under what conditions it was observed, and what an evaluation actually established.

## 1. Provenance foundation

Project Knowledge should remain compatible with the conceptual distinctions in W3C PROV without requiring RDF, OWL, or PROV-O serialization.

The useful foundation is:

```text
Entity   — something that exists as information/state
Activity — something that uses or generates Entities
Agent    — something responsible for or associated with an Activity/Entity
```

Project Knowledge domain objects may map into those roles rather than duplicating a second incompatible provenance ontology.

Examples:

- Representation → Entity
- Assertion → Entity
- generated Projection → Entity
- import/generation/validation → Activity
- human/tool/agent → Agent

## 2. Activity

An Activity records an occurrence whose lineage matters.

Conceptually:

```text
Activity
├── kind
├── started_at?
├── ended_at?
├── used[]
├── generated[]
├── associated_agents[]
├── context?
├── method/tool?
└── source_state_refs[]?
```

Kinds may include:

- import;
- generation;
- validation;
- observation;
- transformation;
- correction;
- synchronization;
- indexing.

The vocabulary remains extensible.

## 3. Provenance relationships

The model should be able to express, where needed:

- generated-by;
- used-input;
- derived-from;
- attributed-to;
- associated-with;
- revision-of;
- primary-source;
- alternate/specialization relationships.

These are semantic relationships, not storage requirements.

## 4. Recorded time and valid time

The model distinguishes two temporal questions.

### Recorded/system time

> When did Project Knowledge or a native system record/observe this information?

### Valid/effective time

> During what period was this Claim/Assertion considered true, operative, or applicable?

Conceptually:

```text
TemporalQualifiers
├── recorded_at?
├── recorded_until?
├── valid_from?
└── valid_until?
```

The exact representation may use instants, intervals, open bounds, or native history references later.

### Rules

- These fields are conditional, not universal.
- Native revision history may satisfy recorded-time needs without duplicated explicit timestamps.
- Retrospective correction may create a later recorded time for a Claim whose valid interval begins earlier.
- Unknown temporal bounds remain valid unknowns.

## 5. Context

Context records only interpretation-relevant conditions.

Conceptually:

```text
Context
├── project
├── repository?
├── immutable_source_state?
├── symbolic_ref?
├── checkout/worktree?
├── execution_id?
├── host/environment?
├── tool/version?
├── lifecycle_state?
└── extensions{}
```

### Context is selective

A Context MUST NOT imply that every environment detail is valuable project knowledge.

Capture a dimension when changing that dimension can alter:

- interpretation;
- reconstruction;
- evidence validity;
- source resolution; or
- diagnostic outcome.

### Durable reconstruction rule

If historical reconstruction matters, Context should include a reconstructable source-state identity when available rather than relying solely on:

- local filesystem paths;
- temporary worktree names;
- `HEAD` without repository/commit context;
- mutable branch names;
- ephemeral session IDs.

## 6. Evidence input versus Evidence Evaluation

An artifact may be **evidence input**, but “evidence” becomes meaningful only relative to a Claim and evaluation context.

The model therefore distinguishes:

```text
Evidence input          concrete source/entity used
Evidence Evaluation     activity evaluating a Claim
Evaluation result       outcome of the method
```

## 7. Evidence Evaluation

Conceptually:

```text
EvidenceEvaluation : Activity
├── claim
├── evidence_inputs[]
├── evaluated_source_state
├── method
├── context?
├── result
├── recorded_time
└── evaluator/agent?
```

### Method

The method may be:

- automated test;
- validator;
- benchmark;
- manual inspection;
- experiment result;
- external certification;
- another reviewable procedure.

Project Knowledge does not execute the method merely by modeling it.

### Result

Result semantics remain extensible but must be able to distinguish at least:

- supports/passed;
- refutes/failed;
- inconclusive;
- error/not evaluated.

The model should not collapse tool execution success with proposition support unless the method defines that equivalence.

## 8. Claim-relative evidence validity

Evidence Evaluation E supports only the Claim scope it actually evaluated.

Conceptually:

```text
E valid-for Claim C
  under source state S
  using method M
  under Context K
```

A later change Δ should invalidate/question E only when:

```text
Δ intersects inputs/semantics material to C or M
```

when the system has enough information to determine relevance.

This prevents both failure directions:

- invalidating evidence because unrelated bytes changed; and
- preserving evidence even though the proposition-relevant state changed.

## 9. Evidence scope

An Evidence Evaluation MUST NOT silently support a broader proposition than the method checked.

Example:

```text
Method checks:
  recorded baseline is an ancestor of execution commit

Method does NOT prove:
  recorded baseline equals the requested operational baseline
```

The domain therefore needs the evaluated Claim to be explicit enough for this distinction to be inspectable.

## 10. Derivation and freshness

A Projection or generated Representation should be associated with the Activity that generated it and the inputs used.

Conceptually:

```text
inputs at source states
      ↓
generation Activity
      ↓
derived Representation
```

Freshness is a derived question:

> Are the relevant current inputs semantically equivalent to or unchanged from the inputs used to generate this Representation?

The answer may be:

- current;
- stale;
- partially stale;
- unknown;
- not applicable.

Whole-repository commit equality is only one possible signal and is not the universal definition.

## 11. Provenance correction

A wrong provenance record is corrected by adding a newer corrected Assertion/relationship and preserving the retained historical record when policy requires it.

The current projection should prefer the corrected provenance under its authority/time rules while allowing reconstruction of what was originally recorded.

## 12. Agent

An Agent is a responsible actor when attribution matters.

Examples:

- person;
- team;
- tool;
- CI runner identity;
- automation agent.

Agent identity and authorization are distinct concerns.

Project Knowledge should not infer that an Agent has authority merely because it produced an artifact.

## 13. Requirement coverage

This model primarily satisfies:

- FR-401 through FR-408;
- FR-501 through FR-504;
- FR-402/406/407 for derived state;
- FR-605 through provenance-backed recovery paths;
- QA-004, QA-007, QA-016, QA-017;
- CON-006, CON-007, CON-008, CON-012.

It preserves the architecture freedom in NR-014, NR-016, and NR-022.