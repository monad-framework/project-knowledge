# Minimal Semantic Kernel

This document defines the candidate domain kernel for Project Knowledge.

The kernel is deliberately smaller than the requirements surface. Many user-visible capabilities—current-state views, contradiction diagnosis, impact analysis, recovery paths, and narratives—are projections over shared semantics rather than independent domain subsystems.

## 1. Project

A **Project** is the semantic boundary within which Project Knowledge interprets identity, policy, authority, retention, and integrations.

A Project may span:

- one repository or many;
- several source systems;
- external standards or documents;
- multiple workspaces or environments; and
- historical states.

Project identity does not imply that all project information is physically stored together.

## 2. Source System

A **Source System** is a native system that owns or exposes project information.

Examples include:

- a Git repository;
- GitHub Issues;
- a wiki;
- CI/build systems;
- a filesystem;
- a documentation platform;
- an external standards repository.

A Source System may be authoritative for some concerns and non-authoritative for others.

## 3. Native Reference

A **Native Reference** identifies a native object or source state without pretending to be a Project Knowledge semantic identity.

A Native Reference contains enough information to distinguish, as applicable:

- source system;
- native object identifier;
- locator;
- immutable/reconstructable source-state identifier; and
- optional fragment/subobject locator.

Mutable and immutable identity MUST remain distinguishable.

## 4. Subject

A **Subject** is an optional Project Knowledge semantic identity for a logical engineering thing whose continuity matters independently of any one native representation.

Examples may include:

- one architectural decision that moved directories;
- one work packet represented in Git and GitHub;
- one subsystem described in architecture docs and implemented in code;
- one requirement represented in several artifacts.

### Subject rules

- A Subject MUST have stable identity within its Project.
- A Subject MAY have zero or more Representations.
- A Subject MAY exist before a Representation is known, but the system SHOULD avoid creating unsupported Subjects merely because it can.
- A native artifact does not automatically require a Subject.
- Two native artifacts MUST NOT be merged into one Subject without an explicit binding, deterministic project rule, or inspectable inference.

## 5. Representation

A **Representation** is a concrete native artifact or addressable fragment that concerns a Subject or Claim and participates in project memory.

A Representation references exactly one Native Reference at a particular interpretation point, although the same native artifact may participate in several semantic bindings or roles.

Representation roles are descriptive, not authority declarations.

Possible roles include:

- authored source;
- projection;
- generated derivative;
- implementation;
- evidence;
- coordination representation;
- historical representation;
- external source;
- narrative.

The exact vocabulary remains extensible.

## 6. Claim

A **Claim** is a proposition whose meaning must be distinguishable from one particular occurrence of that proposition in a source.

Examples:

- “WP-X lifecycle state is Closed.”
- “ADR-0001 is the governing decision for concern Y.”
- “validation method V passed for proposition P against source state S.”

A Claim SHOULD become first-class only when independent treatment of truth, authority, evidence, time, disagreement, or reuse is valuable.

A Claim need not be represented as an RDF-style triple. The implementation may later support structured predicates, values, opaque propositions, or both.

## 7. Assertion

An **Assertion** is a source-bound occurrence in which a Representation, Source System, Agent, or Activity presents a Claim.

The distinction is:

```text
Claim      = what is being asserted
Assertion  = who/what asserted it, where, when, and under what context
```

This separation allows Project Knowledge to represent:

- the same Claim asserted by several sources;
- contradictory Claims asserted by different sources;
- a generated repetition derived from one upstream Assertion;
- an earlier Assertion that was valid historically but is no longer current.

An Assertion MAY carry:

- source Representation;
- attribution/Agent;
- recorded time;
- valid/effective time;
- observation Context;
- epistemic annotation;
- provenance; and
- derivation lineage.

## 8. Authority Assignment

An **Authority Assignment** states that a source has governing authority over a defined **Authority Scope** under a recorded basis.

An Authority Scope may include:

- Subject;
- concern/property/claim family;
- applicable context;
- valid interval; and
- project policy boundary.

An Authority Assignment contains or references its basis, such as:

- project policy;
- native-system ownership;
- accepted decision;
- governing external source;
- explicit human assignment.

Authority is therefore not a global boolean on a Representation.

## 9. Relationship

A **Relationship** is an explicit semantic connection whose type is valuable for recovery, impact, provenance, authority, evidence, or explanation.

Relationships MAY connect Subjects, Representations, Claims, Assertions, Activities, Contexts, or other modeled entities.

Every material Relationship SHOULD retain origin semantics distinguishing at least:

- asserted/authored;
- imported;
- generated/derived; or
- inferred.

Ordinary untyped links remain valid and need not become Relationships.

## 10. Activity

An **Activity** represents an occurrence through which project-memory entities are used, generated, derived, imported, transformed, evaluated, or corrected.

This concept is intentionally compatible with the W3C PROV distinction among Entity, Activity, and Agent without requiring PROV-O serialization.

Activities allow Project Knowledge to explain derivation as an event/process rather than collapsing all provenance into static `source` links.

Examples:

- generate a summary from files A and B;
- import issue state from GitHub;
- run a validator against commit X;
- correct an earlier provenance record;
- render a current-state projection.

## 11. Context

A **Context** records material conditions needed to interpret an Assertion, Activity, or Evidence Evaluation.

Context may include only the dimensions that matter, such as:

- repository;
- commit/source state;
- branch or symbolic ref;
- checkout/worktree;
- host/environment;
- tool and version;
- lifecycle state;
- execution identity.

Context is not a dumping ground for all environment variables. It is selective and recovery-driven.

## 12. Evidence Evaluation

An **Evidence Evaluation** is a specialized Activity that evaluates an explicit Claim.

It connects:

```text
Claim under evaluation
      +
source state / evidence inputs
      +
method
      +
material Context
      ↓
result
```

The result does not automatically become globally authoritative. It is evidence concerning the evaluated Claim.

Evidence validity is therefore claim-relative.

## 13. Epistemic Annotation

An **Epistemic Annotation** is optional metadata or a Relationship describing the knowledge status or evolution of a Claim/Assertion when useful.

Examples may include:

- uncertain;
- hypothesis;
- observed;
- disputed;
- corrected;
- refined;
- rejected;
- accepted.

This is deliberately **not** a universal state machine.

## 14. Projection / View

A **Projection/View** presents selected project-memory information for a recovery task.

Examples include:

- current authoritative state;
- historical state at a time;
- contradiction diagnosis;
- provenance lineage;
- impact traversal;
- search results;
- authored learning narrative.

A Projection is a representation or presentation over underlying domain information, not a new independent truth source unless explicitly assigned authority by project policy.

## Derived domain concepts

The following important concepts are **derived**, not primitive kernel entities.

### Current truth

Current truth is a resolution over:

```text
Claims + Assertions + Authority Assignments + time + Context + policy
```

It may resolve to:

- one current authoritative Claim;
- several compatible Claims;
- no known current Claim; or
- unresolved conflict.

### Historical truth

Historical truth is a projection over retained Assertions, effective/recorded time, historical policy/authority where relevant, and source history.

### Contradiction diagnostic

A contradiction diagnostic is an explanation derived from identity, Claims, Assertions, authority, time, provenance, Context, and lineage.

It MUST be able to return `unresolved/unknown`.

### Freshness

Freshness is derived from a Projection's lineage, the source states it consumed, relevant changes, and—where known—the semantic scope of what the Projection claims.

### Impact

Impact is a traversal result over recorded Relationships and lineage. It is not proof that every reached item will actually change.

### Causal/recovery path

A recovery path is a selected traversal over asserted rationale/causal/derivation/work Relationships. Chronology alone does not create causality.

## Kernel boundary test

A concept belongs in this kernel only if at least one of the following is true:

1. several native systems need to share its meaning;
2. it is necessary to resolve current/historical truth safely;
3. it is necessary to explain provenance, evidence, or contradiction;
4. it is necessary for impact/recovery traversal; or
5. removing it would force a user-facing projection to invent its own incompatible truth semantics.

Native engineering concepts that fail this test should remain native rather than being absorbed into a universal Project Knowledge ontology.