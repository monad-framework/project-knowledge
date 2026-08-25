# Retrieval, Security, and Operational Boundaries

This document defines how discovery/search, authorization boundaries, local operation, and failure recovery fit around the semantic core.

## Retrieval boundary

Retrieval is a convenience/access subsystem over project memory.

It may index:

- native text/content permitted by policy;
- portable semantic records;
- derived semantic metadata;
- relationship neighborhoods;
- current/historical state labels;
- provenance/freshness metadata;
- generated projection text.

It does not establish semantic truth.

## Retrieval modes

A conforming implementation may provide any combination of:

### Exact/native lookup

- Subject ID;
- native issue/work ID;
- file path;
- commit/source-state ID;
- URI/reference;
- record ID.

### Lexical/content search

Traditional full-text search across authorized indexed material.

### Structured filtering

Examples:

- Source System;
- Subject;
- Representation role;
- relationship type;
- current/historical status;
- authority state;
- time interval;
- evidence/freshness state.

### Semantic retrieval

Optional embeddings/vector/LLM-assisted retrieval.

If present, semantic ranking is a discovery signal only.

## Retrieval result contract

A useful result should be able to expose, where available:

```text
native source identity
native locator
source-state identity
Subject identity
Representation role
current/historical/freshness context
access partition
retrieval reason/score
```

When a synthesized answer is produced, cited sources remain distinguishable from generated text.

## Semantic guardrails

Retrieval must not:

- decide Subject equivalence solely from similarity;
- decide authority from ranking;
- turn repeated derivatives into corroboration;
- hide unresolved conflict to produce a cleaner answer;
- infer causality from co-occurrence;
- present generated synthesis as canonical fact.

## Access-boundary model

The architecture preserves source/policy authorization boundaries across integration and derivation.

### Required behavior

1. adapters attach sufficient access-partition metadata to observations;
2. compiler propagates effective restrictions to derived records/index entries;
3. query layer applies authorization before returning source content or derived semantics that reveal restricted information;
4. multi-source views use the intersection/appropriate composed visibility rules rather than the weakest source restriction;
5. caches/indexes are partitioned or filtered sufficiently to prevent unauthorized leakage.

## Derived-information leakage

Even if raw restricted text is not returned, a derived fact may reveal protected information.

Examples:

- the existence of a confidential Subject;
- a relationship to a secret project;
- a summary generated from restricted evidence;
- a search snippet;
- a current-state resolution relying on inaccessible source.

Therefore access lineage applies to semantic/derived results, not only raw documents.

Detailed information-flow/security policy remains a later security specification.

## Local-first operating profile

The initial architecture supports a fully local profile:

```text
local repository
 + portable semantic records
 + local Git adapter
 + embedded/disposable read model
 + local CLI/API
```

No remote database or account is required for this profile.

## Federated/service profile

A later deployment may add:

- remote source adapters;
- shared synchronization;
- hosted read models;
- multiple users;
- centralized access policy enforcement;
- remote search indexes.

The semantic contracts remain unchanged.

## Offline behavior

When operating offline:

- local native sources remain available;
- local portable records remain available;
- previously built derived state may be queried subject to age/freshness labels;
- remote-source-dependent current truth may be `last_known` or `unknown`;
- the system must not silently claim remote currentness.

## Backup and recovery

### Authoritative backup scope

Back up:

1. native sources according to their own policies; and
2. portable Project Knowledge-owned semantic records.

Derived read models/indexes are rebuildable and need not be part of the authoritative backup set, though backing them up may improve recovery time.

### Disaster recovery test

A strong conformance test is:

```text
Delete all Project Knowledge derived databases/indexes.
Restore/access native sources + portable records.
Rebuild.
Verify semantic equivalence of supported resolutions/projections.
```

## Corruption handling

### Corrupt portable record

- report exact record/schema/invariant diagnostic;
- quarantine/skip affected semantics according to policy;
- dependent resolution becomes unknown/error rather than guessed.

### Corrupt derived read model

- discard/rebuild;
- no authoritative semantics are lost.

### Corrupt adapter cache

- re-observe native source where possible;
- preserve diagnostic history where useful.

## Observability

Runtime should expose health for:

- adapter synchronization;
- portable-record validation;
- compiler status;
- read-model generation;
- index freshness;
- unresolved references;
- source access failures;
- stale/unknown projections.

Operational telemetry is not itself canonical project knowledge unless deliberately promoted/retained.

## Privacy and retention

Architecture supports selective retention by keeping native observation caches separate from durable semantic records.

This allows a project to:

- avoid copying unnecessary source content;
- expire caches independently;
- retain semantic references while deleting sensitive cached content when policy permits;
- retain only material Context fields.

Detailed legal/privacy deletion semantics remain open.

## AI boundary

AI may later assist with:

- suggested identity matches;
- relationship suggestions;
- classification;
- summarization;
- semantic retrieval;
- narrative drafting.

Any consequential AI output enters the same provenance/inference discipline:

```text
AI suggestion != authoritative semantic record
```

Promotion requires inspectable source/basis and whatever review policy the project defines.