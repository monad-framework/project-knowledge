# Architecture Validation and M0 Vertical Slice

Architecture is not accepted because the diagrams are plausible. It must preserve the domain invariants under executable scenarios.

## Validation layers

### V1 — Portable-record validation

Checks:

- syntax/serialization;
- schema/version identity;
- reference shape;
- reserved-vocabulary rules;
- extension namespace rules.

### V2 — Referential/integration validation

Checks:

- Source System exists/configured;
- Native Reference resolves when expected;
- immutable-vs-mutable identity declarations are coherent;
- Subject/Representation bindings are not malformed;
- source access partition is preserved;
- portable records do not claim unsupported adapter semantics.

### V3 — Domain invariant validation

Checks invariants such as:

- assertion != truth;
- role != authority;
- derived repetition retains lineage;
- authority scope is explicit enough;
- evidence references a Claim;
- projection lineage is present;
- inferred relations are labeled.

### V4 — Resolver scenario validation

Golden scenarios exercise semantic outputs, including valid `unknown` and conflict states.

### V5 — Rebuild validation

Delete derived state, rebuild from authoritative inputs, and compare semantic outputs.

### V6 — Adapter conformance

An adapter fixture suite checks native identity, source-state identity, relocation behavior, access metadata, change tokens, unavailable states, and reconstruction semantics.

---

## Architecture-entry scenarios

The domain phase defined eight architecture-entry tests. The selected architecture maps them as follows.

### S-1 — Minimal project

**Input:** ordinary Markdown + Git, no semantic records.

**Expected:** repository adapter can index/discover according to policy, but no Subject IDs, Claims, or typed relationships are required.

**Pass condition:** native project remains useful even if all Project Knowledge derived state is deleted.

### S-2 — Identity continuity

**Input:** one ADR with stable Subject across two file locations/commits.

**Expected:** portable Subject/Representation binding preserves continuity; Git adapter preserves old/new native state and relocation evidence.

**Negative case:** same-content unrelated file must not be auto-merged solely by similarity.

### S-3 — Authority conflict

**Input:** canonical lifecycle source says `Verified`; coordination projection says `In Progress`.

**Expected:** authority resolver identifies lifecycle source as authoritative for lifecycle concern if policy states so; projection is stale/historical, not independent authority.

**Negative case:** remove authority policy and resolver returns unknown/unresolved rather than guessing.

### S-4 — Historical correction

**Input:** prior Assertion recorded wrong status; later correction records accurate source state.

**Expected:** current resolver yields corrected Claim; historical/as-of view retains earlier Assertion and correction lineage.

### S-5 — Context-dependent observation

**Input:** worktree-local observation differs from canonical control source.

**Expected:** both observations retain Context; resolver can explain context difference; local path is not treated as durable state identity.

### S-6 — Claim-relative evidence

**Input:** validator/method proves C1 but does not test broader C2.

**Expected:** Evidence Evaluation supports C1 only. Querying support for C2 returns unsupported/unknown.

### S-7 — Derived freshness

**Input:** generated summary depends on A and B; A changes.

**Expected:** summary becomes stale/partial stale according to declared dependency semantics. Unrelated C change does not invalidate it when relevance is known.

### S-8 — Unknown resolution

**Input:** two plausible identity or authority candidates with insufficient basis.

**Expected:** resolver returns unknown/unresolved with candidates and basis; import order/search rank is ignored.

---

# M0 — Smallest Useful Vertical Slice

M0 exists to falsify the architecture, not to demonstrate every future feature.

## M0 goals

Prove that one implementation can:

1. leave an ordinary repository untouched and usable;
2. add a small number of portable semantic records;
3. observe Git/native files through an adapter;
4. compile into a disposable local read model;
5. resolve current state with scoped authority;
6. preserve conflict/unknown outcomes;
7. track one derived projection's lineage/freshness;
8. model one proposition-scoped Evidence Evaluation;
9. traverse one cross-object relationship path;
10. delete/rebuild all derived state and reproduce semantic results.

## M0 explicitly excludes

- production web UI;
- SaaS service;
- multi-user collaboration;
- production GitHub/Jira/Slack connectors;
- embeddings/RAG;
- automatic ontology extraction;
- general causal inference;
- universal temporal history;
- graph visualization;
- plugin marketplace;
- full policy language;
- generalized workflow engine.

## M0 fixture repository

Create a deterministic test fixture containing:

```text
fixture/
├── README.md                        # L0 native-only
├── docs/
│   ├── adr/
│   │   ├── 0042-old.md             # historical location
│   │   └── 0042-current.md         # current location
│   ├── lifecycle.md                 # authoritative concern source
│   └── coordination-summary.md      # stale derived/projection source
└── .project-knowledge/              # provisional default; detailed design may rename
    └── ... portable records ...
```

The fixture should include synthetic Git history so relocation and immutable source-state identity can be tested.

## M0 semantic fixture

At minimum:

- Subject `ADR-0042`;
- old/current Representation bindings;
- lifecycle status Claim/Assertion;
- stale coordination Assertion;
- Authority Assignment for `lifecycle.status` concern;
- one derived summary Activity/Relationship lineage;
- one Evidence Evaluation proving C1;
- one broader C2 not proven;
- one intentionally unresolved identity/authority case.

## M0 command/API capabilities

The exact CLI names are detailed design, but implementation must expose operations equivalent to:

```text
validate portable records
sync/compile project
inspect Subject
resolve current state for Subject + Concern
show representation/source lineage
show freshness of derived projection
show evidence support for Claim
traverse relationships/impact
rebuild from scratch
```

## M0 read-model technology criteria

The first read-model implementation should favor:

- embedded/local operation;
- zero external service dependency;
- transactional updates;
- indexed relational querying;
- recursive relationship traversal sufficient for the fixture;
- easy deletion/rebuild;
- broad tooling/library support.

These criteria make an embedded relational database a strong first implementation candidate, but the architecture decision here remains about the **read-model role**, not permanent product dependence on one engine.

## M0 portable-format criteria

The first portable serialization should favor:

- deterministic parsing;
- human reviewability;
- source-control diffs;
- cross-language libraries;
- schema validation;
- no ambiguous implicit typing;
- explicit version fields.

The detailed-design/M0 bootstrap PR should select the concrete format and schema standard.

## M0 acceptance threshold

M0 is successful only if:

- S-1 through S-8 pass;
- derived read model can be deleted/rebuilt;
- no fixture requires importing native source content into S2 merely for convenience;
- unknown/conflict results survive intact;
- all resolver outputs expose basis/source lineage;
- adding enrichment to ADR-0042 does not require enriching README.md;
- no search/index ranking participates in authority resolution.

## Falsification triggers

The selected architecture should be reconsidered if M0 shows that:

1. portable S2 records cannot express necessary semantics without duplicating native sources extensively;
2. rebuildable S3 cannot support required queries without becoming de facto canonical state;
3. access boundaries cannot be propagated safely through derived state;
4. incremental compilation requires a fundamentally event-sourced model to remain coherent;
5. the minimal project incurs substantial mandatory ceremony; or
6. cross-source semantic ownership cannot be kept distinct from native authority in practice.