# Composition Findings

The existing-approaches pass changes the shape of the Project Knowledge problem.

The project is no longer investigating whether we need to invent a complete system for history, provenance, relationships, temporal data, documentation, work tracking, and retrieval. Mature mechanisms already exist for each of those concerns.

The remaining question is:

> What project-memory semantics and integration behavior are missing when those mechanisms are composed around a real engineering project?

## Finding EA-001 — Project Knowledge should begin as an integration/modeling problem

The corpus does not justify replacing Git, issue trackers, ADRs, documentation, or search.

Those systems are useful precisely because they have specialized responsibilities and mature workflows.

The likely Project Knowledge responsibility is to help a user recover a coherent project model **across** them.

## Finding EA-002 — Repository-native artifacts should remain first-class

`PKC-0011` and docs-as-code practice both show that ordinary version-controlled text can be sufficient for many projects.

A future system should therefore be able to observe or enrich existing Markdown/YAML/JSON/source artifacts without requiring wholesale migration into a proprietary store.

This preserves:

- portability;
- diffability;
- reviewability;
- offline access;
- Git history; and
- graceful degradation if Project Knowledge tooling is unavailable.

## Finding EA-003 — Existing semantics should be adopted selectively

Several concepts are mature enough that inventing incompatible equivalents would be difficult to justify.

Examples:

### From Git

- immutable source-state identity;
- distinction between immutable objects and mutable symbolic refs;
- diffs/history as source evidence.

### From ADR practice

- durable decision identity;
- decision status;
- alternatives/rationale;
- explicit supersession rather than historical rewrite.

### From linked-note systems

- aliases;
- backlinks;
- low-friction links;
- suggested/unlinked relationships;
- lightweight optional properties.

### From event sourcing

- immutable change/event history;
- projections/materialized current views derived from history.

### From bitemporality

- system/recorded time distinct from valid/effective time.

### From W3C PROV

- Entity / Activity / Agent;
- generation / usage / derivation;
- attribution / association;
- revision / primary source;
- specialization / alternate representation;
- qualified relationships; and
- progressive provenance detail.

### From graph models

- explicit typed many-to-many relationships;
- traversal and impact queries.

### From hybrid search

- combine exact lexical identity with semantic retrieval;
- filter/rerank rather than relying on vector similarity alone.

## Finding EA-004 — Semantic identity is a coordination layer across native identities

Git commits, file paths, issue numbers, URLs, execution IDs, and database keys are all useful native identities.

The problem appears when the project needs to say:

> These several native objects are representations, revisions, evidence, projections, or work records concerning the same engineering concept.

Project Knowledge may therefore need semantic identity **in addition to**, not instead of, native tool identity.

This identity layer should be introduced only for things whose cross-representation continuity matters.

## Finding EA-005 — Authority needs an explicit model because existing tools are locally authoritative

Each tool can be authoritative within its domain:

- Git for committed source state;
- an ADR for a particular architectural decision;
- an issue tracker for its own workflow state;
- an external standard for requirements it governs;
- a generated projection for nothing beyond what its lineage permits.

The corpus failure occurs when one representation's local authority is treated as global.

This suggests a genuinely cross-cutting need: **authority scoped to claims/properties/roles**, with enough provenance to explain why one representation wins when views disagree.

No surveyed mechanism provides that project-wide by itself.

## Finding EA-006 — Provenance is largely a reuse problem, not an invention problem

The initial corpus identified multiple meanings hidden behind a generic `source` field.

W3C PROV already distinguishes many of them.

Therefore Project Knowledge should first test how far PROV concepts can cover the engineering corpus before designing custom provenance primitives.

Engineering-specific extensions may still be needed for evidence, authority, execution context, or repository state, but they should extend a mature base rather than begin from zero.

## Finding EA-007 — Temporal truth requires more than Git history

Git provides revision history; event sourcing provides recorded event order; system-versioned tables provide database history; bitemporal models distinguish recorded/system time from valid time.

The corpus needs at least the conceptual distinction between:

```text
when the project recorded/believed X
```

and:

```text
when X was considered valid/effective
```

This is especially important for retrospective corrections.

The architecture phase must decide how much of this requires explicit infrastructure versus metadata over repository artifacts.

## Finding EA-008 — Epistemic semantics remain under-served

None of the surveyed general mechanisms directly gives Project Knowledge the full vocabulary for:

- question;
- hypothesis;
- observation;
- claim;
- uncertainty;
- support;
- contradiction;
- rejection;
- refinement;
- correction;
- decision; and
- verified result.

Some specialized systems or ontologies may model portions of this, and more research may be warranted before inventing a model.

For now this remains one of the clearest potentially project-specific gaps.

## Finding EA-009 — Claim-relative evidence remains under-served

PROV can say where evidence came from, but the corpus's EOSV cases require something more specific:

```text
Evidence E supports proposition P
against source state S
using validation method V
under context C
```

and later:

```text
Change Δ invalidates E only if Δ is relevant to P
```

Generic file freshness, commit equality, or provenance alone is insufficient.

This appears to be another important candidate for Project Knowledge-specific semantics.

## Finding EA-010 — Search should consume project-memory semantics, not manufacture them

Hybrid lexical/semantic search can solve major findability problems.

But relevance ranking cannot safely determine:

- current authority;
- semantic identity;
- valid time;
- provenance correctness;
- evidence validity; or
- whether two contradictions are historical, contextual, or genuine.

Project Knowledge semantics can improve search through filters, context, ranking signals, and source annotations.

Search can then make those semantics usable at human scale.

## Finding EA-011 — Human narrative must remain deliberately authored

Graph traversal and generated summaries are useful, but `UJ-012` asks for something stronger: a coherent educational path through the engineering process.

A narrative chooses emphasis, sequence, explanation, and pedagogy. Those are not equivalent to dumping every related node or event.

Therefore multiple views should likely coexist:

```text
structured project memory
        ├── current-state view
        ├── history/timeline view
        ├── provenance view
        ├── impact view
        ├── search/recovery view
        └── authored learning narrative
```

The final branch remains a human composition even if the system can assist with source selection and traceability.

## Finding EA-012 — Progressive structure is now a hard design constraint

The combination of `PKC-0011`, linked-note systems, PROV's incremental vocabulary, and repository-native documentation strongly supports a progression like:

```text
ordinary artifact
    ↓ optional
stable semantic identity
    ↓ optional
lightweight properties / links
    ↓ optional
explicit typed relationships
    ↓ optional
provenance / temporal / epistemic detail
    ↓ optional
validation / generated projections
```

The system should not require every stage for every object.

## What appears genuinely left to solve

After crediting existing approaches, the strongest remaining cross-cutting gap is approximately:

> A low-friction project-memory layer that connects heterogeneous engineering artifacts through stable semantic identity, scoped authority, provenance, temporal and epistemic evolution, claim-relative evidence, and explicit relationships, then exposes that model through current-state, historical, impact, retrieval, and narrative views without replacing the native tools that created the information.

This is more precise than the original “engineering knowledge graph” hypothesis and materially narrower than “store all project information.”

## What should happen next

The next discovery pass should perform **evidence-to-capability derivation**.

For every proposed capability, require a trace of the form:

```text
Corpus case(s)
    ↓
Failure mode(s)
    ↓
User job(s)
    ↓
Existing approach coverage
    ↓
Residual gap
    ↓
Candidate capability
```

Each capability should be labeled as one of:

- **REUSE** — existing mechanism already solves it;
- **INTEGRATE** — Project Knowledge needs to connect an existing mechanism into project memory;
- **EXTEND** — existing semantics are useful but need engineering-specific additions; or
- **NEW** — the corpus supports behavior not adequately supplied by surveyed mechanisms.

Only after that trace exists should the project promote candidate capabilities into requirements.
