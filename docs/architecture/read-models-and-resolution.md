# Read Models and Semantic Resolution

Derived read models make the project-memory kernel usable at project scale without becoming the authoritative semantic source.

## Principle

```text
Canonical/native inputs + portable semantic records
                  ↓
            derived read model
                  ↓
        semantic resolvers / queries
```

The derived model may be deleted and rebuilt.

## Logical read-model partitions

The implementation may physically combine or split these partitions.

### RM-1 — Source observations

Normalized adapter observations keyed by Source System + Native Reference + source-state identity.

### RM-2 — Identity and representation

- Subjects;
- Native References;
- Representation bindings;
- aliases/locators;
- identity-resolution status;
- binding provenance.

### RM-3 — Claims and assertions

- Claims;
- source-bound Assertions;
- assertion origin;
- temporal qualifiers;
- Context references;
- native field/materialization lineage.

### RM-4 — Authority

- Authority Assignments;
- policy-derived authority rules;
- concern/property scope;
- applicability qualifiers;
- precedence/ambiguity diagnostics.

### RM-5 — Relationships and provenance

- typed Relationships;
- Relationship origin;
- Activities/Agents;
- derivation/revision/use/generation links;
- reverse adjacency.

### RM-6 — Evidence

- Evidence Evaluations;
- supported Claim;
- method/result;
- evaluated source state;
- dependency relevance;
- freshness status/basis.

### RM-7 — Synchronization/access lineage

- source observation tokens;
- adapter health;
- access partitions;
- last-known state;
- derivation dependencies.

## Semantic resolvers

Resolvers are pure/inspectable semantic operations over read-model state plus policy.

### R-1 — Subject resolution

Input:

- native reference or Subject ID.

Output examples:

- exact Subject;
- several candidate Subjects;
- no Subject required/assigned;
- inferred candidate requiring confirmation;
- unknown.

Must not false-merge by similarity alone.

### R-2 — Representation resolution

Answers:

- which native representations concern this Subject;
- their roles;
- source-state identities;
- current/missing/stale availability;
- derivation relationships.

### R-3 — Authority resolution

Input:

```text
Subject + Concern + optional time/context
```

Output:

```text
resolved(authority source/basis)
compatible(multiple non-conflicting authorities)
unresolved_conflict(candidates + basis)
unknown(insufficient policy/state)
```

Authority must never default to latest import, most repeated assertion, or search rank.

### R-4 — Current-state resolution

Input:

```text
Subject + Concern + current/as-of context
```

Process conceptually:

1. select relevant Assertions/Claims;
2. apply validity/recorded-time constraints;
3. apply Context applicability;
4. resolve authority;
5. account for corrections/refinements where modeled;
6. preserve conflicts and unknowns;
7. return supporting source/lineage.

Output:

- authoritative Claim;
- compatible Claims;
- unresolved conflict;
- unknown/insufficient information.

### R-5 — Historical/as-of resolution

Same semantic logic as current state, parameterized by historical effective/recorded context and the authority policy applicable to that history where retained.

A present-day authority rule must not silently rewrite historical interpretation if historical policy matters and is retained.

### R-6 — Contradiction classification

Given disagreeing Assertions, classify where justified:

- historical/current difference;
- stale projection;
- authority-scope difference;
- Context difference;
- provenance/source error;
- semantic identity mismatch;
- genuine unresolved conflict;
- unknown.

Classification includes evidence/basis, not only a label.

### R-7 — Derivation freshness

For a derived Representation/Projection:

```text
current
stale
partial_stale
source_unavailable
unknown
```

computed/explained from lineage and known relevant input state.

### R-8 — Evidence status

For an Evidence Evaluation:

```text
supports_as_recorded
invalidated_by_relevant_change
source_unavailable
method_scope_insufficient
unknown_relevance
```

The resolver must not generalize the evaluation beyond its Claim/method scope.

### R-9 — Impact traversal

Traverse explicit/derived dependency relationships to produce **impact candidates**, not guaranteed causal effects.

Results preserve:

- relationship type;
- origin;
- direction/path;
- confidence/inference status where applicable;
- access visibility.

## Resolver explanation contract

Every materially consequential resolver output should be explainable through a structure conceptually equivalent to:

```text
result
basis:
  input references
  applicable policy/rules
  relevant relationships
  temporal/context filters
  authority decision
  diagnostics / unresolved factors
```

The exact API shape is deferred.

## Caching

Resolver outputs may be cached.

A cache entry must track enough dependencies to know when it is invalid or unknown.

Cached results are S3 and may not become canonical S2 records merely because they were materialized.

## Materialized current-state projections

Current-state documents/views may be generated for humans.

They must include or retain machine-accessible lineage/freshness sufficient to distinguish:

- freshly generated from current dependencies;
- partially stale;
- last-known due to unavailable source;
- unknown.

A generated view may be version-controlled as historical evidence, but its materialization does not grant it independent authority.

## Query implementation freedom

The logical operations above can be implemented using:

- relational joins;
- graph traversal;
- recursive queries;
- document indexes;
- in-memory structures;
- hybrid approaches.

Architecture conformance is determined by semantics and explanation, not storage/query syntax.

## M0 resolver scope

M0 should implement only enough resolution to validate the core architecture:

1. Subject/Representation lookup;
2. scoped authority resolution;
3. current-state resolution with `resolved | conflict | unknown`;
4. derivation freshness for one generated projection;
5. evidence evaluation that distinguishes C1 from broader C2;
6. relationship traversal sufficient for one impact/recovery path.

This slice exercises the hard semantics without building the full product.