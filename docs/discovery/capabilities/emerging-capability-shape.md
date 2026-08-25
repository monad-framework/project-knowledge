# Emerging Capability Shape

This document records the structural pattern revealed by the evidence-to-capability trace. It is **not** a domain model or architecture.

## Observation

The candidate registry contains twenty capabilities, but the trace does not justify twenty independent subsystems.

The evidence instead suggests four layers of responsibility.

```text
┌───────────────────────────────────────────────────────────────┐
│                    HUMAN RECOVERY VIEWS                       │
│ current state · history · contradiction · search · impact    │
│ narrative · causal path · correction                         │
├───────────────────────────────────────────────────────────────┤
│                  PROJECT-MEMORY SEMANTICS                     │
│ identity · representation · authority · provenance · time     │
│ context · epistemics · claim/evidence · lineage               │
├───────────────────────────────────────────────────────────────┤
│                CROSS-ARTIFACT INTEGRATION                     │
│ native references · typed relations · admission · federation  │
├───────────────────────────────────────────────────────────────┤
│                 NATIVE ENGINEERING SYSTEMS                    │
│ Git · files/docs · ADRs · issues · source · tests · CI · etc. │
└───────────────────────────────────────────────────────────────┘

               progressive formalization applies throughout
```

## Layer 1 — Native engineering systems

Project Knowledge should not replace the systems that already create useful primary information.

Relevant candidates:

- `CAP-001` preserve native artifact authority/history — **REUSE**

This layer includes, depending on the project:

- version control;
- source files;
- Markdown/YAML/JSON documentation;
- ADRs;
- issue/work tracking;
- CI/test output;
- external standards;
- execution tools; and
- other domain-native artifacts.

## Layer 2 — Cross-artifact integration

This layer answers:

> What belongs to project memory, where is it, and how is it connected?

Primary candidates:

- `CAP-002` heterogeneous artifact federation;
- `CAP-012` typed cross-artifact relationships;
- `CAP-013` source/admission classification; and
- portions of `CAP-016` retrieval integration.

This is mostly an integration problem. Existing graph/link/search mechanisms should be reused where possible.

## Layer 3 — Project-memory semantics

This layer is the strongest candidate for Project Knowledge-specific value.

It answers questions native tools usually cannot answer project-wide:

- What semantic thing do these representations concern?
- Which representation is authoritative for this claim/property?
- What produced this assertion or projection?
- When was this assertion valid, and when was it recorded?
- Which observation/execution context gives this record meaning?
- Is this a question, hypothesis, correction, decision, or verified claim?
- What proposition does this evidence support?
- What changed that makes a projection or evidence record stale?

Primary candidates:

- `CAP-003` semantic identity;
- `CAP-004` representation roles/bindings;
- `CAP-005` scoped authority;
- `CAP-006` provenance;
- `CAP-007` temporal semantics;
- `CAP-008` material context;
- `CAP-009` epistemic evolution;
- `CAP-010` claim-relative evidence; and
- `CAP-011` derivation lineage/freshness.

These are not necessarily independent object types. Domain modeling must determine the smallest coherent primitives capable of expressing them.

## Layer 4 — Human recovery views

These capabilities should consume the same underlying semantics rather than maintain separate truth.

Primary candidates:

- `CAP-014` current/historical views;
- `CAP-015` contradiction diagnosis;
- `CAP-016` hybrid retrieval;
- `CAP-017` authored narrative support;
- `CAP-019` causal/recovery paths; and
- `CAP-020` correction-without-erasure views.

This layer is where Project Knowledge becomes useful to a human rather than merely formally expressive.

## Cross-cutting rule — Progressive formalization

`CAP-018` cuts across every layer.

A future system should be able to stop at the lowest layer that solves the recovery problem.

Examples:

### Simple project

```text
Markdown + Git + search
```

No semantic identity layer may be needed.

### Moderate project

```text
native artifacts
+ a few stable semantic identities
+ links/aliases
+ search
```

No full provenance or temporal model may be needed.

### Complex project

```text
native artifacts
+ semantic identities
+ scoped authority
+ provenance
+ temporal/context semantics
+ claim-relative evidence
+ derived views
```

The richer model is available because the recovery problem demands it, not because the platform requires ceremony.

## Candidate semantic kernel

Without asserting a final ontology, the current evidence suggests that the eventual domain model will need to explain at least these conceptual questions:

1. **Reference** — How do we point to a native artifact/state?
2. **Identity** — What logical engineering thing persists across representations?
3. **Representation** — What role does a native artifact play relative to that thing?
4. **Relationship** — How are engineering things/assertions/artifacts explicitly related?
5. **Authority** — Which source governs which claim/property/role, and why?
6. **Provenance** — What entity/activity/agent/source produced or derived something?
7. **Time** — When was an assertion effective versus recorded?
8. **Context** — Which contextual dimensions are required to interpret it?
9. **Epistemics** — What kind/state of knowledge is being expressed?
10. **Evidence** — Which proposition is supported, by what method/source/context?
11. **Lineage** — Which inputs produced a projection and what makes it stale?

This list is a set of questions for domain modeling, not eleven required entities.

## Important non-conclusions

The current evidence does **not** imply:

- one graph database;
- one universal artifact schema;
- one central source of truth replacing native systems;
- event sourcing as the implementation architecture;
- RDF/OWL as the required representation;
- a temporal database for every record;
- a mandatory knowledge ID for every file/note;
- an LLM-generated narrative as canonical documentation; or
- fully automatic conflict resolution.

Those remain architecture choices to be evaluated later against requirements.
