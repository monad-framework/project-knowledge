# Integration and Synchronization Architecture

Project Knowledge is federated: authoritative project information may remain in several native systems.

The integration architecture must observe those systems without erasing their identity, authority, access boundaries, or historical semantics.

## Adapter boundary

An adapter translates a native system into **source observations**, not into Project Knowledge truth.

The adapter contract conceptually exposes:

```text
SourceSystemDescriptor
NativeReference
SourceObservation
SourceStateIdentity
NativeRelationshipObservation
NativeChangeToken
AccessPartition
AdapterDiagnostic
```

Exact types belong to detailed design.

## Adapter responsibilities

An adapter MUST:

- preserve native object identity;
- distinguish mutable locator from stable/immutable state identity;
- expose source state/version tokens when available;
- expose enough structured fields to support configured mappings;
- preserve source access boundaries;
- label unavailable/unsupported semantics;
- identify observation time/context when material;
- avoid claiming semantic Subject equivalence unless supplied by explicit mapping/policy;
- avoid converting native workflow state into global Project Knowledge authority by default.

An adapter SHOULD:

- support incremental change enumeration;
- expose reconstructable historical state where the native source supports it;
- minimize copied source content;
- produce stable normalized observations for semantically unchanged source state.

## Observation versus record

A source observation is derived from a native source.

It may be cached in S3, but is not automatically an S2 portable semantic record.

Example:

```text
Git adapter observes:
  path = docs/adr/0042.md
  commit = abc123
  content_hash = ...

Portable PK record states:
  Subject ADR-0042
  represented_by NativeReference(git, docs/adr/0042.md)
```

The first can be regenerated from Git. The second is Project Knowledge-owned semantic continuity.

## Synchronization pipeline

```text
1. Load portable Project Knowledge records
2. Resolve configured Source Systems/adapters
3. Obtain source change/version observations
4. Resolve Native References needed by semantic records
5. Discover optional indexed native artifacts according to admission policy
6. Normalize observations into the derived model
7. Resolve dependencies/lineage
8. Recompute affected semantic projections/resolver inputs
9. Update retrieval indexes
10. Emit diagnostics/freshness/access metadata
```

## Full rebuild

A conforming implementation must support a full rebuild path:

```text
accessible native sources + portable semantic records
                         ↓
                    empty S3
                         ↓
                 deterministic compile
                         ↓
               semantically equivalent S3
```

Where remote/native historical state is no longer accessible, the rebuild may produce explicit unavailable/unknown state rather than silently reproducing stale cached observations.

## Incremental synchronization

Incremental operation is an optimization over the same semantics.

### Change detection

Adapters may use:

- immutable Git commit/object identity;
- filesystem metadata/content hashes;
- issue update timestamps/revision IDs;
- API cursors/webhooks;
- CI run IDs;
- external-source ETags/version IDs;
- other source-specific change tokens.

The architecture does not require one global event log.

### Dependency invalidation

Changes invalidate only affected derived dependencies where known.

Conceptually:

```text
Native change
   ↓
changed observations
   ↓
Representation / Assertion dependencies
   ↓
Authority / evidence / projection dependencies
   ↓
affected read-model partitions and indexes
```

When dependency scope is unknown, the compiler may conservatively invalidate a broader region while exposing that limitation.

## Synchronization state

For each source or source partition, the derived runtime should be able to distinguish states such as:

- current as of known source token;
- stale relative to known newer source state;
- last-known observation;
- source unavailable;
- authorization denied;
- invalid adapter output;
- unsupported historical reconstruction;
- unknown.

These are runtime synchronization states, not universal domain epistemic states.

## Source disappearance

When a source object disappears:

- do not automatically delete a Subject or semantic record;
- record/derive that the Representation is currently unresolved/missing if relevant;
- preserve retained historical bindings according to policy;
- distinguish deletion from temporary unavailability where the adapter can determine it.

## Relocation

When a native artifact moves but stable identity is known:

- update/observe the locator change;
- preserve Subject identity;
- preserve historical native references as needed;
- do not infer relocation solely from content similarity without labeling inference.

## Identity inference

Adapters or tooling may suggest that two native representations correspond to the same Subject using:

- stable native IDs;
- explicit embedded IDs;
- configured aliases;
- rename tracking;
- similarity/inference.

Only explicit or policy-approved bindings become authoritative semantic continuity. Inferred bindings remain labeled until confirmed according to project policy.

## Admission policy

Discovery/indexing is governed by project policy.

Policy may include:

- include/exclude globs;
- repository roots;
- file/media types;
- native object categories;
- generated directory exclusions;
- transient/admin exclusions;
- opt-in sources;
- retention rules.

Mere visibility or parseability is not admission.

## Access propagation

Every cached/derived observation needs sufficient source-access lineage to prevent data leakage.

A simple local public repository may effectively have one access partition.

A richer deployment may have per-source, per-repository, per-team, or per-object authorization partitions.

The architecture requires preservation of the boundary but defers concrete auth mechanism.

## Adapter failure isolation

One broken adapter must not corrupt unrelated project memory.

Compilation should:

- isolate diagnostics by source/record;
- retain unaffected read-model partitions;
- mark dependent views unknown/stale as appropriate;
- avoid substituting unrelated sources simply to produce an answer.

## Integration extensibility

New adapters must map into established kernel semantics without redefining them.

An adapter may introduce namespaced extension fields/types, but:

- Subject still means semantic continuity;
- Authority Assignment still means scoped authority;
- Activity still represents provenance occurrence;
- retrieval rank still cannot establish truth.

## M0 integration scope

M0 should begin with a **repository/Git adapter** because it can validate:

- native files;
- mutable paths;
- immutable commits;
- relocation/history;
- local/offline operation;
- deterministic source-state references.

A second synthetic/mock adapter should prove the architecture is genuinely federated without prematurely integrating a production issue tracker.