# Corpus Validation

This document stress-tests the candidate domain kernel against `PKC-0001` through `PKC-0011`.

The purpose is not to force every case into maximum structure. The test is:

> Can the domain model represent the information needed to solve the case without inventing an unnatural special-purpose entity or violating progressive formalization?

## PKC-0001 — Work packet as a multi-view engineering object

### Domain mapping

```text
Subject
  WP logical identity

Representations
  governed work-packet artifact
  GitHub coordination issue
  machine projection
  execution/evidence records
  implementation artifacts

Relationships
  represents / implements / derived-from / evidence links

Authority Assignments
  lifecycle concern → canonical governed source
  coordination concern → GitHub issue where applicable
```

### Result

**Fits.**

The case does not require a special universal `WorkPacket` kernel entity. Native work semantics remain native while the cross-view identity/authority problem is represented by Subject + Representations + Authority.

## PKC-0002 — Coordination projection diverges from canonical lifecycle state

### Domain mapping

- same Subject;
- canonical and coordination Representations;
- competing lifecycle Claims/Assertions;
- Authority Assignment selects canonical lifecycle concern;
- derivation/synchronization Activity links projection to source;
- stale Projection freshness result.

### Result

**Fits strongly.**

Current truth resolves from authority + Assertions; stale GitHub state remains historical/diagnostic rather than being deleted.

## PKC-0003 — Canonical execution state invisible from executor context

### Domain mapping

- Native References identify canonical state and executor-visible state;
- Context records worktree/checkout/execution dimensions;
- Assertion interpretation differs by Context;
- authority remains attached to canonical control state, not whichever checkout is visible.

### Result

**Fits.**

The model explains false absence without creating a special worktree ontology.

## PKC-0004 — Administrative metadata misclassified as project source

### Domain mapping

- Source System exposes discoverable artifacts;
- admission policy decides whether a Native Reference participates in project memory;
- incidental Git metadata remains excluded;
- generated projection Activity would otherwise amplify the wrong admission decision.

### Result

**Fits with Level-1 policy rather than richer kernel semantics.**

This is important: the case does not justify creating Subjects/Claims for excluded material.

## PKC-0005 — Verification evidence invalidates itself through lifecycle persistence

### Domain mapping

- explicit Claim being evaluated;
- Evidence Evaluation records inputs/method/context/result;
- lifecycle mutation is a later Activity/change;
- relevance analysis asks whether that mutation changes the Claim's evaluated proposition;
- evidence freshness is Claim-relative rather than repository-global.

### Result

**Fits strongly.**

The Evidence Evaluation specialization appears justified by this case.

## PKC-0006 — Recorded execution baseline disagrees with requested operational baseline

### Domain mapping

Two distinct Claims are required:

```text
C1: recorded baseline is ancestor of execution commit
C2: recorded baseline equals requested operational baseline
```

The validator's Evidence Evaluation supports C1 only.

Context/Native References preserve requested, resolved, and recorded source identities.

### Result

**Fits strongly and validates Claim/Assertion separation.**

Without explicit Claim semantics, a passing validator can be misread as establishing C2.

## PKC-0007 — Evidence freshness changes with host-local historical state

### Domain mapping

- Context preserves host/worktree locator as historical context;
- Native Reference preserves reconstructable source-state identity separately;
- Evidence Evaluation refers to durable state rather than relying only on path existence;
- later host-local deletion does not rewrite what source state was evaluated.

### Result

**Fits.**

Validates INV-015 and the Native Reference / Context distinction.

## PKC-0008 — Project status summary drift

### Domain mapping

Minimal representation is sufficient:

- root README is a Representation;
- its phase statement may become a first-class Claim only if current-status recovery is being managed;
- current and historical Assertions differ over time;
- the README is an authored summary Projection whose freshness can become stale relative to phase/state inputs.

### Result

**Fits progressively.**

The model does not require extracting every README sentence into Claims. Only the status assertion needs enrichment if automated current-state checking is desired.

## PKC-0009 — Hypothesis refinement without erasure

### Domain mapping

- original hypothesis Claim/Assertion retained;
- later refined Claim added;
- optional Epistemic Annotation describes hypothesis/refinement;
- a Relationship/Activity records the refinement basis;
- historical view preserves the earlier belief; current view may prefer the refined Claim if authority/status supports it.

### Result

**Fits without a universal epistemic state machine.**

Validates optional Epistemic Annotation plus extensible evolution relationships.

## PKC-0010 — Semantic ADR identity survives repository relocation

### Domain mapping

```text
Subject: ADR-0001 decision identity
Representation R1: old path
Representation R2: new path
Native identity: ADR-0001 + Git states
Relationship: revision/relocation lineage as needed
```

### Result

**Fits strongly.**

Validates Subject identity as distinct from storage locator.

The case also demonstrates that a stable native-domain identifier may be sufficient evidence to establish the Subject binding.

## PKC-0011 — Ordered documents can be enough

### Domain mapping

Most project information remains:

```text
Level 0: native Markdown + ordered filenames + Git
Level 1: indexed/retrievable Native References if Project Knowledge is enabled
```

No Subject, Claim, Authority Assignment, explicit temporal metadata, or graph-like Relationship is required unless a new recovery need appears.

### Result

**Fits by doing less.**

This is the critical counterexample. A model that required enriching all ten documents would fail the case even if it could represent them perfectly.

# Cross-case result

No current corpus case requires a new kernel primitive beyond the candidate model.

The cases exercise distinct portions of the kernel:

| Concept | Strongest validating cases |
| --- | --- |
| Source System / Native Reference | PKC-0003, 0004, 0006, 0007 |
| Subject | PKC-0001, 0010 |
| Representation | PKC-0001, 0002, 0008, 0010 |
| Claim / Assertion | PKC-0002, 0005, 0006, 0008, 0009 |
| Authority Assignment | PKC-0001, 0002, 0003, 0008 |
| Activity / provenance | PKC-0004, 0005, 0006, 0008 |
| Context | PKC-0003, 0006, 0007 |
| Evidence Evaluation | PKC-0005, 0006, 0007 |
| Epistemic Annotation | PKC-0009 |
| Relationship | corpus-wide, especially 0001 and causal/derivation cases |
| Projection | PKC-0002, 0004, 0008 |
| Progressive formalization | PKC-0008, 0011 |

# Remaining evidence pressure

The domain model still needs future stress testing against under-evidenced cases from Discovery:

- simultaneous competing hypotheses;
- a complete real decision-supersession chain;
- experiment-driven decision change;
- terminology evolution;
- multi-person disagreement/concurrent authorship;
- deliberate capture-overhead failure;
- richer authored educational narrative.

Those cases may refine optional vocabularies or policies. They do not currently justify adding speculative kernel entities.