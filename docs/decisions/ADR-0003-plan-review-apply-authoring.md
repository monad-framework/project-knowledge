# ADR-0003 — Use a Plan → Review → Apply Boundary for Semantic Authoring

## Decision state

**Proposed on this branch. Merge of the PR containing this ADR into `main` constitutes acceptance of this decision.**

This explicit acceptance rule is part of the decision record; tools must not generalize it into a rule that every merged PR means every contained proposal is accepted.

## Context

DF-001 and DF-002 proved that the existing M0 semantic model can represent two materially different real Project Knowledge recovery problems, but each required substantial mechanical authoring:

- DF-001: 10 S2 records;
- DF-002: 14 S2 records.

The repeated burden includes UUID generation, file placement, record envelopes, timestamps, Git source-state lookup, and cross-reference plumbing.

At the same time, the dogfood cases reinforced that semantic choices such as authority, identity, evidence breadth, valid time, and epistemic meaning must not be inferred silently.

A direct “smart capture” command that writes immediately would reduce friction but create an unsafe boundary between mechanical assistance and semantic inference.

## Decision

Adopt a three-stage authoring architecture:

```text
Authoring Intent
     ↓
PLAN — read-only construction and relevant-state observation
     ↓
REVIEW — inspect semantics, origins, warnings, and exact write set
     ↓
APPLY — re-check preconditions, validate prospective corpus, write S2
```

Capture Plans are operational artifacts, not canonical S2.

The Planner may generate mechanical structure and observe selected native state. Material semantic suggestions must remain visibly distinct from authored values and cannot be applied without confirmation.

Plan staleness is based on relevant inspected inputs rather than global repository `HEAD` equality when more precise state is available.

## Consequences

### Positive

- removes repeated structural boilerplate without weakening the domain model;
- creates an inspectable boundary for automation and future AI assistance;
- supports interactive and non-interactive authoring through one planner/applier path;
- preserves native artifacts;
- makes stale-input detection explicit;
- allows plans to be reviewed/tested before mutation;
- keeps runtime semantics dependent only on final S2, not on authoring-session artifacts.

### Costs

- introduces two operational document types (`pk-authoring/v1` and `pk-capture-plan/v1`) that need versioning and validation;
- requires planner/applier logic and additional CLI surface;
- does not eliminate the need for humans to understand important semantic choices;
- requires careful stale-plan and idempotence behavior.

## Rejected alternatives

### Immediate-write interactive wizard

Rejected because semantic choices and generated structure would be committed before the user can review the complete resulting record graph.

### Hand-authored S2 only

Rejected as the default enriched-authoring experience because dogfooding independently reproduced excessive mechanical burden.

Hand-authored S2 remains valid for advanced/debugging use because the portable format must stay inspectable.

### Natural-language/AI capture as the first solution

Rejected for CA-1. It would combine the structural-ergonomics problem with a much harder semantic-inference problem before the deterministic workflow is proven.

### Global `HEAD` as the only plan precondition

Rejected because unrelated commits would invalidate otherwise safe plans and would conflict with the project's existing relevant-input freshness principle.

### New simplified canonical record model

Rejected. The dogfood evidence showed that the current semantic distinctions were useful; the burden was structural authoring, not demonstrated semantic excess.

## Follow-up

Implement CA-1 only after this detailed-design PR is accepted. Validate it against DF-001/DF-002-equivalent fixtures and then use it in a third real dogfood experiment before authorizing further feature expansion.
