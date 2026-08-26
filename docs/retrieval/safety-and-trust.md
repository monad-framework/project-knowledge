# Retrieval Safety and Trust Boundaries

## Principle

Retrieval may make project knowledge easier to see. It must not silently make project knowledge stronger than what the project actually recorded.

## Authority boundary

Traversal density, edge count, shortest path, relevance, or presentation order must never determine authority.

When `pk explain` presents current truth, it delegates to the existing resolver and reports the Authority assignments used by that resolver.

A heavily linked Claim with no applicable Authority does not become current truth because it looks important.

## Causality boundary

FR-606 is normative for RT-1.

The system may present the exact recorded relation:

```text
A --motivates--> B
```

It may not restate that as:

```text
A caused B
```

unless the project explicitly recorded a causal relation whose semantics justify that wording.

Chronological ordering alone never supplies causality.

## Relationship-origin boundary

Recorded `Relationship.origin` remains visible.

An `inferred` Relationship must not be rendered as though it were authored.

A deterministic structural binding must not be rendered as though a human asserted that relation.

Recommended human markers:

```text
[recorded/authored]
[recorded/imported]
[recorded/derived]
[recorded/inferred]
[structural]
[native]
```

## Evidence boundary

Evidence remains Claim-relative.

`pk explain` must show:

- which Claim an Evidence Evaluation supports;
- method;
- result;
- current evidence state/freshness;
- relevant inputs; and
- context where recorded.

It must not aggregate several Evidence Evaluations into a broader “verified project” conclusion unless a separate Claim explicitly represents that proposition.

## Historical evidence boundary

Stale, failed, or inconclusive evidence may still be historically important.

RT-1 should display its current state rather than silently deleting it from the recovered reasoning path.

The caller may later request filters, but the default explanation favors honest context over present-only sanitization.

## Current-state boundary

Current-state sections use the existing resolver.

Traversal itself does not decide whether:

- an Assertion is currently valid;
- an Authority is applicable;
- competing Claims conflict; or
- absence means false.

Those remain resolution semantics.

## Identity boundary

Selectors are exact and ambiguity-safe.

A label/locator ambiguity returns candidates. Retrieval must not choose the “most likely” Subject.

Any future fuzzy lookup may help discover candidates, but semantic identity confirmation belongs to a separate explicit step.

## Native-source boundary

A native locator is evidence of where a source lives, not proof that its current content still matches an older recorded state.

When an immutable/reconstructable state is present, output should display it separately from the locator.

When source observation reports missing/unavailable/stale conditions, explanation must not hide that fact.

## Incompleteness boundary

Project Knowledge can only explain what has been recorded or deterministically bound.

The output must expose:

- `unknown` current concerns;
- no-path outcomes;
- missing native inputs;
- traversal truncation; and
- sparse/no recorded relationships

without filling gaps from model intuition.

## Ranking boundary

RT-1 does not use semantic similarity or relevance scoring.

Future search may rank candidates, but FR-603 remains absolute: ranking must not determine truth, authority, provenance correctness, evidence validity, or semantic identity.

## Narrative boundary

Human output is templated structured rendering from `RecoveryExplanation`.

A future LLM or narrative generator may consume the same result, but:

- citations/IDs must remain available;
- generated synthesis must be visibly derived;
- the underlying structured result remains inspectable; and
- generated text cannot become canonical truth by being fluent.

## Mutation boundary

RT-1 is read-only.

If traversal discovers a useful missing connection, the system may later offer a separate capture workflow, but retrieval itself must not persist that connection.

This preserves a clean sequence:

```text
retrieve
  ↓
observe gap
  ↓
explicit authoring decision
  ↓
plan / review / apply
```

rather than:

```text
retrieve
  ↓
automatically rewrite project memory
```

## Determinism boundary

For the same compiled S1/S2 state and query, RT-1 returns semantically identical ordered results.

This is required for:

- reviewability;
- tests;
- stable downstream tools; and
- preventing nondeterministic traversal order from being mistaken for significance.
