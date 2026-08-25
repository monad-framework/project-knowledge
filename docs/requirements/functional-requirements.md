# Functional Requirements

These requirements define technology-neutral behavior for Project Knowledge.

## RF-1 — Native interoperability and progressive adoption

### FR-101 — Preserve native artifacts

**Scope:** Universal  
**Trace:** CAP-001, CAP-018

Project Knowledge **MUST** operate without requiring destructive migration of repository files, source code, ADRs, work items, or other supported native engineering artifacts into a proprietary source of truth.

Where a native system remains authoritative for an artifact or property, Project Knowledge **MUST** preserve that authority boundary.

**Verification direction:** Demonstrate a project whose native repository artifacts remain usable, reviewable, and authoritative with Project Knowledge enabled and after Project Knowledge tooling is unavailable.

### FR-102 — Preserve native identity and source-state references

**Scope:** Universal  
**Trace:** CAP-001, CAP-002

For integrated artifacts, Project Knowledge **MUST** retain enough native identity to identify the source system and native object, and **SHOULD** retain immutable source-state identity when the native system supplies one.

Mutable locators such as branch names, aliases, paths, or URLs **MUST NOT** be silently represented as immutable state identities.

### FR-103 — Federate heterogeneous artifact systems

**Scope:** Universal  
**Trace:** CAP-002

Project Knowledge **MUST** be able to compose project memory from more than one artifact system or representation family without requiring those systems to share a storage model.

Federation **MUST** preserve source-system boundaries and native identifiers.

### FR-104 — Support admission and exclusion policy

**Scope:** Conditional  
**Trace:** CAP-013

When automatic discovery or ingestion is used, Project Knowledge **MUST** provide a means to distinguish included project knowledge from excluded, transient, administrative, or incidental artifacts according to project policy.

Discovery **MUST NOT** treat mere filesystem presence, parseability, indexability, or API visibility as proof that an artifact is canonical project knowledge.

### FR-105 — Support progressive enrichment

**Scope:** Universal  
**Trace:** CAP-018

Project Knowledge **MUST** allow useful participation at multiple levels of structure.

A project **MUST** be able to begin with ordinary native artifacts and introduce stronger semantic identity, structured relationships, provenance, temporal metadata, evidence semantics, or projections only where useful.

### FR-106 — Preserve useful simple organization

**Scope:** Universal  
**Trace:** CAP-001, CAP-018

Project Knowledge **MUST NOT** require replacement of useful hierarchy, ordering, filenames, directories, authored documentation, or native work-tracking structures merely because richer cross-cutting relationships are available.

### FR-107 — Make enrichment reversible or non-destructive

**Scope:** Universal  
**Trace:** CAP-001, CAP-018

Adding Project Knowledge metadata or relationships **SHOULD** be non-destructive to the underlying native artifact and **MUST NOT** make the native artifact unusable by its ordinary tool solely because Project Knowledge semantics were added.

---

## RF-2 — Semantic identity, representation, and relationships

### FR-201 — Support semantic identity where continuity matters

**Scope:** Conditional  
**Trace:** CAP-003

Project Knowledge **MUST** support a stable semantic identity distinct from native file, issue, URL, branch, commit, or tool identity when a project needs to preserve continuity of one logical engineering subject across multiple representations or locations.

The system **MUST NOT** require a semantic identifier for every artifact.

### FR-202 — Preserve semantic identity across representation relocation

**Scope:** Conditional  
**Trace:** CAP-003

When an identified engineering subject is moved, renamed, or represented at a new native locator without changing its logical identity, Project Knowledge **MUST** be able to preserve that semantic continuity.

### FR-203 — Bind native representations to semantic subjects

**Scope:** Conditional  
**Trace:** CAP-003, CAP-004

Project Knowledge **MUST** support explicit bindings between a semantic subject and the native artifacts that represent, describe, implement, evidence, project, or otherwise concern it.

A binding **MUST** retain the native identity of the representation.

### FR-204 — Distinguish representation roles

**Scope:** Conditional  
**Trace:** CAP-004

Where role differences affect interpretation, Project Knowledge **MUST** distinguish materially different representation roles such as authoritative source, projection, generated derivative, historical representation, evidence, implementation, coordination representation, or external source.

The domain model may refine this vocabulary; this requirement does not mandate these exact labels.

### FR-205 — Prevent projected repetition from becoming false corroboration

**Scope:** Universal when projections are integrated  
**Trace:** CAP-004, CAP-011

When several representations derive from the same underlying source, Project Knowledge **MUST** preserve enough lineage to prevent them from being presented as independent corroborating sources merely because they are distinct artifacts.

### FR-206 — Support typed relationships where semantics matter

**Scope:** Conditional  
**Trace:** CAP-012

Project Knowledge **MUST** support explicit relationship semantics for relationships whose meaning is required for recovery, impact, provenance, authority, evidence, or explanation.

The system **MUST** also permit ordinary links or associations where typed semantics provide insufficient value.

### FR-207 — Traverse relationships across native boundaries

**Scope:** Conditional  
**Trace:** CAP-002, CAP-012

Users and consuming systems **MUST** be able to traverse supported relationships across heterogeneous native artifact boundaries without requiring all participating artifacts to reside in one native tool.

### FR-208 — Preserve relationship provenance

**Scope:** Conditional  
**Trace:** CAP-006, CAP-012

For relationships whose origin affects trust or interpretation, Project Knowledge **MUST** be able to distinguish an authored/asserted relationship from a generated, imported, inferred, or derived relationship.

---

## RF-3 — Authority and current truth

### FR-301 — Represent scoped authority

**Scope:** Universal for authority-bearing information  
**Trace:** CAP-005

Project Knowledge **MUST** represent authority at a scope fine enough to explain which source governs a relevant claim, property, role, or concern.

Authority **MUST NOT** be modeled solely as a global boolean attached to an entire artifact when the artifact is authoritative for only part of the subject.

### FR-302 — Explain authority basis

**Scope:** Universal where authority resolution is presented  
**Trace:** CAP-005

When Project Knowledge identifies one source or statement as authoritative, it **MUST** expose the basis for that determination, such as explicit project policy, native-system ownership, decision authority, governing source, or recorded relationship.

### FR-303 — Recover current authoritative state

**Scope:** Universal  
**Trace:** CAP-005, CAP-014

For information represented by multiple current or historical artifacts, Project Knowledge **MUST** provide a way to recover the currently authoritative state where the evidence and authority model permit that determination.

### FR-304 — Preserve unresolved authority conflicts

**Scope:** Universal  
**Trace:** CAP-005, CAP-015

When conflicting claims are genuinely unresolved or authority cannot be safely determined, Project Knowledge **MUST** preserve and expose the unresolved state rather than fabricate reconciliation.

### FR-305 — Present current and historical truth distinctly

**Scope:** Universal  
**Trace:** CAP-007, CAP-014, CAP-020

Project Knowledge **MUST** provide enough temporal and status context to prevent a historically valid statement from being presented indistinguishably as current truth.

Historical information **MUST** remain recoverable when retained.

### FR-306 — Diagnose disagreement categories

**Scope:** Universal when contradiction diagnostics are offered  
**Trace:** CAP-015

When statements disagree, Project Knowledge **MUST** expose enough available semantics to distinguish, where determinable, among causes such as:

- stale or unsynchronized projection;
- historical-versus-current difference;
- observation-context difference;
- authority-scope difference;
- provenance error;
- semantic or identity mismatch; and
- genuinely unresolved disagreement.

The system **MUST NOT** claim a diagnosis unsupported by recorded evidence.

### FR-307 — Preserve correction without historical erasure

**Scope:** Universal for managed corrections  
**Trace:** CAP-020

When project knowledge is corrected, Project Knowledge **MUST** allow the corrected state to become clearly current while preserving the earlier retained statement, its historical context, and the basis for correction where available.

### FR-308 — Avoid authority by repetition or search rank

**Scope:** Universal  
**Trace:** CAP-004, CAP-005, CAP-016

Project Knowledge **MUST NOT** infer authority solely from the number of matching representations, backlink count, retrieval score, vector similarity, search rank, or frequency of repetition.

---

## RF-4 — Provenance, time, and context

### FR-401 — Represent structured provenance

**Scope:** Conditional  
**Trace:** CAP-006

Project Knowledge **MUST** support structured provenance sufficient to distinguish materially different concepts such as produced-by, used-input, derived-from, attributed-to, associated-with, revision-of, or primary-source relationships when required by a recovery job.

The design **SHOULD** reuse or remain compatible with mature provenance semantics where practical.

### FR-402 — Preserve derivation lineage

**Scope:** Universal for derived information  
**Trace:** CAP-006, CAP-011

Generated projections, summaries, indexes, reports, or other derived project-memory artifacts **MUST** retain lineage to the source inputs or derivation activity needed to explain their origin.

### FR-403 — Distinguish valid/effective time from recorded/system time

**Scope:** Conditional  
**Trace:** CAP-007

Where correctness or historical recovery depends on the distinction, Project Knowledge **MUST** be able to represent separately:

1. when a statement or condition was considered valid/effective; and
2. when the system learned, recorded, or stored that statement.

The system **MUST NOT** require both times for every artifact or assertion.

### FR-404 — Preserve material observation context

**Scope:** Conditional  
**Trace:** CAP-008

When interpretation depends on repository, commit, branch, checkout, worktree, execution, host, tool version, environment, lifecycle state, or another contextual dimension, Project Knowledge **MUST** be able to preserve that material context.

### FR-405 — Separate contextual locators from reconstructable state identity

**Scope:** Conditional  
**Trace:** CAP-008

Ephemeral paths, local worktree locations, mutable refs, or host-specific locators **MUST NOT** be treated as sufficient durable reconstruction identities when an immutable or reconstructable source-state identity is available or required.

### FR-406 — Expose derivation freshness

**Scope:** Universal for derived current-state views  
**Trace:** CAP-011

Project Knowledge **MUST** expose enough lineage and source-state information to determine, calculate, or explain whether a derived current-state representation is synchronized with its relevant inputs.

### FR-407 — Scope freshness to relevant inputs

**Scope:** Conditional  
**Trace:** CAP-010, CAP-011

Freshness or invalidation **SHOULD** be scoped to the source inputs relevant to the derived claim or view rather than to unrelated repository changes when that distinction is known.

### FR-408 — Preserve provenance corrections

**Scope:** Conditional  
**Trace:** CAP-006, CAP-020

If a previously recorded provenance statement is found to be inaccurate, Project Knowledge **MUST** allow the corrected provenance to become current without requiring deletion of the retained historical record of what was originally recorded.

---

## RF-5 — Evidence and epistemic evolution

### FR-501 — Associate evidence with an explicit proposition

**Scope:** Conditional  
**Trace:** CAP-010

Where evidence semantics are used, Project Knowledge **MUST** associate evidence with the proposition or claim it supports rather than treating evidence as generically valid for an entire artifact or project state.

### FR-502 — Record evidence evaluation context

**Scope:** Conditional  
**Trace:** CAP-006, CAP-008, CAP-010

Evidence records **MUST** be able to identify, where materially relevant:

- the proposition evaluated;
- the source state evaluated;
- the validation or observation method;
- the material execution/observation context; and
- the result.

### FR-503 — Bound evidence claims to what was actually evaluated

**Scope:** Conditional  
**Trace:** CAP-010

Project Knowledge **MUST NOT** present a verification result as establishing a proposition broader than the recorded method and inputs actually support.

### FR-504 — Support claim-relative evidence invalidation

**Scope:** Conditional  
**Trace:** CAP-010

When evidence freshness is managed, the system **MUST** support invalidating or questioning evidence when a change is relevant to the supported proposition, while avoiding invalidation solely because unrelated state changed when relevance is known.

### FR-505 — Represent epistemic roles when useful

**Scope:** Optional / Conditional  
**Trace:** CAP-009

Project Knowledge **SHOULD** support explicit epistemic roles or states—such as question, hypothesis, observation, claim, correction, or accepted result—when they materially improve recovery or correctness.

The system **MUST NOT** require one universal epistemic state machine for every project or knowledge object.

### FR-506 — Preserve refinement and correction semantics

**Scope:** Conditional  
**Trace:** CAP-009, CAP-020

When a project records that knowledge was refined, narrowed, strengthened, weakened, corrected, rejected, or superseded, Project Knowledge **SHOULD** preserve the nature and basis of that change rather than representing every evolution as generic replacement.

Only relationship types supported by evidence and the later domain model may become normative.

### FR-507 — Preserve uncertainty where unresolved

**Scope:** Conditional  
**Trace:** CAP-009, CAP-015

Project Knowledge **MUST NOT** convert uncertainty, competing hypotheses, or unresolved disagreement into an accepted fact merely to simplify a current-state view.

---

## RF-6 — Retrieval, impact, and explanation

### FR-601 — Provide project-memory retrieval across integrated sources

**Scope:** Universal  
**Trace:** CAP-016

Project Knowledge **MUST** make integrated project memory discoverable across supported native sources using one or more suitable retrieval mechanisms.

Retrieval **SHOULD** support exact native identifiers as well as semantic/content discovery where available.

### FR-602 — Expose semantic metadata to retrieval

**Scope:** Conditional  
**Trace:** CAP-005, CAP-007, CAP-016

Where available, retrieval **SHOULD** allow users or downstream systems to filter, rank, or contextualize results using semantic metadata such as authority, representation role, current/historical state, provenance, time, or relationship context.

### FR-603 — Keep relevance separate from truth semantics

**Scope:** Universal  
**Trace:** CAP-016

Retrieval score, semantic similarity, generated synthesis, or language-model confidence **MUST NOT** determine authority, validity, provenance correctness, or semantic identity.

### FR-604 — Support impact traversal

**Scope:** Conditional  
**Trace:** CAP-012

For recorded relationships, Project Knowledge **MUST** support traversal sufficient to identify potentially affected artifacts, claims, decisions, evidence, or projections before or after a relevant change.

Impact results **MUST** distinguish recorded dependency/relationship evidence from speculative inference where applicable.

### FR-605 — Support explainable recovery paths

**Scope:** Conditional  
**Trace:** CAP-019

Where sufficient relationships exist, Project Knowledge **SHOULD** support recovery paths connecting relevant problem/context, rationale, decisions, evidence, work, implementation, corrections, and resulting state.

The path **MUST** preserve source traceability.

### FR-606 — Do not invent causality

**Scope:** Universal  
**Trace:** CAP-019

Project Knowledge **MUST NOT** present chronology, correlation, dependency, or semantic similarity as causality unless a causal/rationale relationship has been asserted or is derived under an explicit, reviewable rule with sufficient evidence.

### FR-607 — Support traceable authored narratives

**Scope:** Optional  
**Trace:** CAP-017

Project Knowledge **SHOULD** support human-authored learning, onboarding, retrospective, or engineering narratives that reference project-memory sources without requiring the narrative itself to become the canonical source for the underlying engineering facts.

### FR-608 — Support context recovery after absence

**Scope:** Universal as an outcome; implementation mechanism open  
**Trace:** CAP-002, CAP-014, CAP-016, CAP-018

The system **MUST** make it possible for a user returning after loss of mental context to recover relevant current state, important historical changes, unresolved questions, and supporting sources without requiring exhaustive rereading of all project artifacts.

This requirement does not prescribe a dashboard, generated summary, chat interface, or AI system.
