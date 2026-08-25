# Portable Semantic Records

Portable semantic records are the canonical durable representation of **Project Knowledge-owned semantics** (state class S2).

They are not a copy of every native artifact.

## Architectural role

The portable record layer exists because some cross-system semantics belong to Project Knowledge itself and must remain durable even when:

- an adapter implementation changes;
- a derived database is rebuilt;
- a search index is deleted;
- a UI is replaced;
- a remote service is unavailable; or
- a native representation moves.

## Canonical meaning versus physical serialization

The architecture selects a canonical **logical record model** with these required properties:

- deterministic parseability;
- explicit schema/version identity;
- stable record identity where the domain requires it;
- explicit references rather than implicit storage-location semantics;
- human inspectability;
- source-control friendly diffs;
- portable export/import;
- no dependence on an opaque database encoding.

The first implementation slice SHOULD use a text serialization compatible with standard structured-data tooling. Exact encoding and schema language are detailed-design decisions, but M0 must choose one and document canonicalization/version rules.

## Record families

The portable layer may contain records corresponding to domain concepts such as:

### Project / Source System configuration

Defines project identity, source-system aliases/configuration, adapter selection, and policy references.

### Subject

Exists only when semantic continuity across representations is useful.

### Representation Binding

Binds a Subject to a Native Reference with role and optional qualifiers.

### Relationship

Stores authored or retained cross-system relationships whose semantics matter.

### Claim / Assertion

Stores Project Knowledge-authored Claims/Assertions or retained source assertions when materialization is required for authority, history, or evidence.

Native structured fields do not have to be duplicated into portable Claim records when an adapter can faithfully expose them and durable reconstruction remains possible.

### Authority Assignment / policy

Stores explicit authority rules or exceptions not adequately owned by a native system.

### Activity / provenance record

Stores Project Knowledge-managed provenance activities and cross-source derivation relationships where native provenance is insufficient.

### Context

Stores only Context deliberately retained because it affects interpretation, reconstruction, or evidence.

### Evidence Evaluation

Stores proposition-scoped validation/evidence semantics.

### Epistemic Annotation

Optional and sparse.

## What portable records must not become

The record layer must not become an excuse to duplicate:

- every repository file;
- full Git history;
- all issue fields;
- every CI log;
- every search document;
- every chat message;
- every source-code AST;
- every native metadata field.

Those remain native facts or derived/indexed observations unless a specific requirement justifies durable Project Knowledge ownership.

## Reference model

Portable records reference native sources through `Native Reference` semantics rather than copied source content.

A Native Reference needs, conceptually:

```text
source_system
native_object_identity
locator(s)                    optional/mutable
source_state_identity         optional but preferred where reconstructability matters
fragment_identity             optional
observed_at                   when retained observation matters
access_partition              when required
```

Exact field names are detailed design.

## Project Knowledge record identity

Record identity serves a different purpose from Subject identity.

Examples:

- a Subject is semantic continuity;
- an Authority Assignment is an addressable policy/assertion;
- an Evidence Evaluation is an addressable evaluation occurrence;
- a Relationship may need stable identity when provenance/correction/history applies.

The architecture does not require globally unique semantic IDs for all native artifacts.

## Versioning

Portable record compatibility must distinguish:

1. **format/schema version** — how the record is encoded;
2. **semantic vocabulary version** — meaning of standardized types/fields if versioned;
3. **record revision history** — supplied by the surrounding durable store/VCS when available.

Schema migration must not silently change historical semantic meaning.

## Extension model

Domain categories are extensible (INV-032).

Portable records therefore need an extension mechanism supporting:

- project-defined relationship types;
- adapter-specific Context dimensions;
- representation roles;
- Activity kinds;
- epistemic annotations;
- domain-specific attributes.

Extension data must not redefine the meaning of reserved kernel fields.

## Storage topology

Portable records may be stored:

- in the project repository;
- in a dedicated project-memory repository;
- in another versioned filesystem/store that preserves portability guarantees.

The architecture does not require co-location with every integrated source.

For the minimal single-repository project, zero portable records is a valid state.

## Transactionality

Some semantic changes may span several records—for example creating a Subject and two Representation bindings.

The portable layer needs a way to detect incomplete or inconsistent sets, but this does not yet mandate a database transaction mechanism.

M0 may rely on atomic source-control commits plus validation for repository-backed records. Later service-backed authoring may add transactional APIs.

## Secrets and sensitive data

Portable records should contain stable references and minimum necessary Context, not credentials or raw sensitive environment values.

Secrets belong in native secret stores/configuration. Sensitive Context may be redacted, hashed, categorized, or referenced according to later security design.

## Canonicalization principle

The portable layer is canonical only for semantics Project Knowledge owns.

For example:

```text
Git commit content                    → Git remains canonical
Git file bound to Subject S1          → PK binding record is canonical for that binding
Issue workflow state                  → issue tracker remains canonical
Authority rule saying issue owns status concern → PK authority policy may be canonical
Generated current-state summary       → derived, not canonical unless explicitly authored as a separate narrative
```

## M0 implication

The first implementation should prove that a small set of portable records can be committed beside an ordinary project without changing the native files, then compiled into a disposable read model that can be deleted and rebuilt.