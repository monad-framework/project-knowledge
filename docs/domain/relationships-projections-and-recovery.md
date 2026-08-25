# Relationships, Projections, and Recovery

This document defines how Project Knowledge connects domain entities and turns shared semantics into human recovery views.

## 1. Relationship

A Relationship is an explicit semantic connection whose type is worth preserving.

Conceptually:

```text
Relationship
├── type
├── source
├── target
├── origin
├── provenance?
├── temporal qualifiers?
├── context?
└── confidence/epistemic qualifier?
```

The implementation representation remains open.

## 2. Relationship endpoints

A Relationship MAY connect:

- Subject → Subject;
- Subject → Representation;
- Claim → Claim;
- Assertion → Claim;
- Representation → Activity;
- Activity → Representation;
- Evidence Evaluation → Claim;
- Authority Assignment → Subject/Concern;
- other domain entities where the semantics are clear.

The domain does not require one universal edge type or one graph storage model.

## 3. Relationship type families

The model recognizes several broad semantic families because requirements depend on them.

### Representation/identity

Examples:

- represents;
- concerns;
- alternate-of;
- specialization-of.

### Derivation/provenance

Examples:

- derived-from;
- generated-by;
- used-input;
- revision-of;
- primary-source-of.

### Dependency/impact

Examples:

- depends-on;
- implements;
- satisfies;
- constrains;
- affected-by.

### Rationale/explanation

Examples:

- motivates;
- justified-by;
- caused-by;
- responds-to.

### Evidence

Examples:

- supports;
- refutes;
- evaluated-by.

### Evolution

Examples may include:

- corrects;
- refines;
- supersedes;
- replaces;
- splits-into;
- merges-from.

The exact vocabulary MUST remain evidence-driven. A broad family does not authorize every example as a normative relation type.

## 4. Relationship origin

Origin is required when it affects trust.

At minimum the domain can distinguish:

- `asserted` — deliberately authored by a human/project source;
- `imported` — supplied by an integrated native system;
- `derived` — deterministically produced from recorded inputs/rules;
- `inferred` — proposed or concluded from heuristics/model reasoning.

An inferred Relationship MUST NOT silently appear indistinguishable from an asserted fact.

## 5. Ordinary links remain ordinary links

Not every hyperlink needs to become a typed Relationship.

A typed Relationship is justified when its semantics are needed for one or more of:

- authority resolution;
- current truth;
- provenance;
- evidence;
- impact traversal;
- contradiction diagnosis;
- recovery/explanation.

Otherwise ordinary native links are sufficient.

## 6. Projection / View

A Projection/View is a query/composition over project memory for a specific recovery job.

Important view families include:

### Current-state view

Presents authoritative current Claims and unresolved states with traceability.

### Historical view

Presents retained Assertions and relevant authority/time/context at a selected historical point or interval.

### Representation view

Shows native Representations of one Subject and their roles/lineage.

### Provenance view

Shows Activities, sources, Agents, and derivation lineage.

### Evidence view

Shows what Claim was evaluated, with what state/method/context/result.

### Impact view

Traverses recorded dependencies/relationships and labels speculative/inferred results distinctly.

### Contradiction diagnostic

Explains why Assertions disagree where the semantics permit classification.

### Retrieval view

Finds candidates using native identifiers, text/content, semantic metadata, or external retrieval systems while keeping ranking separate from authority.

### Narrative view

Human-authored or assisted sequence that teaches/explains the project while citing underlying sources.

## 7. Projection authority

By default:

```text
Projection authority ≤ authority justified by its source lineage and project policy
```

Materialization does not increase authority.

A Projection may itself become canonical for a specific concern only through an explicit Authority Assignment or project policy.

## 8. Projection lineage

A derived Projection should retain:

- source inputs;
- source-state identities where relevant;
- generating Activity/rule/version;
- generated/recorded time;
- policy/configuration inputs that materially affect output.

This makes freshness and explanation possible.

## 9. Freshness

Freshness is a derived state over lineage.

A Projection may be:

- `current` — relevant inputs/policy remain current for its claims;
- `stale` — a relevant input changed;
- `partially-stale` — only part of a compound projection is affected;
- `unknown` — relevance/currentness cannot be determined safely;
- `historical` — intentionally represents a prior state;
- `not-applicable` — freshness is not meaningful for the artifact.

The exact enumeration may be refined later, but the domain must preserve the distinction between stale current-state material and intentional historical material.

## 10. Impact traversal

Impact is not binary truth. It is a traversal result.

Conceptually:

```text
changed entity
   ↓
recorded typed relationships
   ↓
reachable potentially affected entities
```

Each path should retain:

- relationship type;
- origin;
- provenance;
- direction;
- any inference status.

The system should distinguish:

- directly recorded dependency;
- transitive recorded dependency;
- inferred/suggested impact.

## 11. Recovery path

A Recovery Path is an explanation-oriented traversal selected for a user job.

Example:

```text
Problem / pressure
  → requirement
  → decision
  → work
  → implementation
  → evidence
  → correction
  → current state
```

The path is valid only to the extent that each semantic step is supported by recorded Relationships or an explicit derivation rule.

Chronological adjacency MUST NOT be silently converted into causal linkage.

## 12. Contradiction diagnostic as composition

Contradiction diagnosis consumes the shared kernel:

```text
Subject identity
+ Claims / Assertions
+ representation role
+ Authority Assignments
+ provenance
+ time
+ Context
+ evidence
+ derivation
→ diagnostic
```

This means contradiction diagnosis is not a separate truth database.

Valid output may be:

- explained;
- partially explained;
- unresolved;
- insufficient information.

## 13. Retrieval boundary

Retrieval systems may contribute:

- lexical matching;
- semantic similarity;
- graph traversal;
- filtering;
- ranking;
- synthesis.

But retrieval outputs are candidates/views over project memory.

They MUST NOT determine:

- Subject identity;
- authority;
- Claim truth;
- evidence validity;
- provenance correctness;

without separate inspectable semantic rules/evidence.

## 14. Narrative boundary

A narrative may:

- reorder events for teaching;
- omit irrelevant detail;
- synthesize explanations;
- cite several project-memory sources.

A narrative does not silently become canonical for those sources' underlying Claims.

Narrative assertions that add new engineering facts must either:

- remain clearly interpretive; or
- be promoted into project memory with appropriate provenance/authority semantics.

## 15. Requirement coverage

This model primarily satisfies:

- FR-205 through FR-208;
- FR-402, FR-406, FR-407;
- FR-601 through FR-608;
- FR-306;
- QA-004 through QA-006, QA-011, QA-013;
- CON-003, CON-005, CON-009, CON-011, CON-014.

It preserves NR-011, NR-015, NR-017, NR-021, and NR-023.