# Event Sourcing and Temporal Models

The Project Knowledge corpus repeatedly distinguishes current state from historical process. Event sourcing and temporal databases are established approaches for preserving those dimensions, but they solve different problems and should not be collapsed into one concept.

## Event sourcing

Event sourcing records an append-only sequence of events and derives current state from that history.

Conceptually:

```text
Event 1
  ↓
Event 2
  ↓
Event 3
  ↓
materialized current state
```

### Strong fit

This directly supports ideas behind:

- `UJ-004` — explain why something changed;
- `UJ-008` — reconstruct a work episode;
- `UJ-014` — preserve a correction without erasing the mistake;
- `FM-013` — current structure hides causal history; and
- `FM-016` — correction destroys the path by which knowledge improved.

It also gives a mature precedent for **canonical history plus projections**: the durable event sequence can produce one or more materialized views without making those views the source history.

### Limits

An event stream still requires domain semantics.

`DecisionChanged` or `DocumentUpdated` says little unless the system knows:

- which semantic object changed;
- which assertion changed;
- what authority governed the change;
- why it changed;
- which evidence triggered it; and
- which downstream representations are affected.

Event order also does not automatically tell us when a proposition was considered valid in the domain.

If a correction recorded today says that an architectural assumption became invalid three weeks ago, event time and valid time differ.

Event sourcing is therefore strong for **recorded change history**, but incomplete for the temporal/epistemic model observed by Project Knowledge.

## System-versioned temporal data

System-versioned temporal tables preserve prior row versions and allow point-in-time queries over database history.

This strongly addresses:

- exact historical current-state reconstruction;
- audit/forensics; and
- questions such as “what did the database contain at time T?”

This resembles `UJ-002`, but only for one temporal dimension.

## Bitemporality

Bitemporal systems distinguish at least:

1. **system/transaction time** — when a fact/version entered or was recorded by the system; and
2. **valid time** — when the fact is considered effective or true in the modeled domain.

That distinction is strikingly close to the corpus's unresolved question:

> Do we need to distinguish when something was true from when the system learned or recorded that it was true?

A bitemporal model can distinguish questions such as:

- What do we currently believe was true on June 1?
- What did we believe on June 1 about the state on June 1?
- When did we learn that our earlier understanding was wrong?

### Strong fit

This is directly relevant to:

- `UJ-002` historical truth;
- `UJ-004` change explanation;
- `UJ-014` correction without erasure;
- `FM-002` historical truth presented as current;
- `O-004` current and historical truth coexistence; and
- `PKC-0009` refinement/correction without erasure.

### Important distinction from Git

Git tells us when repository revisions were committed and what bytes those revisions contained.

Bitemporal semantics can additionally express that an assertion committed today is considered valid beginning at some earlier or future domain time.

Therefore:

```text
Git commit time ≠ necessarily domain valid time
```

and:

```text
repository history ≠ complete temporal semantics
```

## What temporal models still do not solve

A temporal database does not inherently define:

- semantic identity across tools;
- scoped authority;
- rationale;
- decision alternatives;
- provenance roles;
- evidence propositions;
- causal explanation; or
- educational narrative.

It preserves versions of whatever model it is given. If the domain model collapses concepts incorrectly, temporal versioning faithfully preserves the wrong abstraction.

## Provisional reuse direction

Project Knowledge should carry forward the **semantic distinction** between:

- transaction/system/recorded time; and
- valid/effective time.

It should also retain the event-sourcing lesson that:

- durable change history and current projections are different concerns.

This does **not** yet imply an event store or bitemporal database implementation.

A repository-native MVP might encode the semantics in ordinary artifacts first. The architecture phase should later determine whether dedicated infrastructure earns its cost.
