# Cross-Cutting Constraints

These constraints restrict acceptable interpretations and future designs.

## CON-001 — Native systems remain legitimate sources of truth

Project Knowledge **MUST NOT** assume that all project knowledge becomes authoritative only after import into a Project Knowledge-owned store.

Native repository, decision, work-tracking, documentation, external-standard, and execution systems may remain authoritative within their declared scopes.

## CON-002 — No mandatory maximum semantic model

A conforming project **MUST NOT** be required to assign semantic identity, typed relationships, provenance, valid-time metadata, evidence records, epistemic states, or generated projections to every artifact.

Enrichment is conditional on demonstrated value.

## CON-003 — No authority from retrieval relevance

Search rank, vector similarity, language-model output, backlink count, document frequency, or generated synthesis **MUST NOT** determine authoritative truth.

## CON-004 — No fabricated conflict resolution

The system **MUST NOT** silently resolve conflicting claims when the recorded authority, time, context, identity, and evidence do not justify a resolution.

Unresolved disagreement is a valid state.

## CON-005 — Preserve source/projection distinction

A generated, projected, cached, summarized, indexed, or replicated representation **MUST NOT** silently become an independent source of truth merely because it is materialized separately.

## CON-006 — Preserve immutable versus mutable identity distinctions

Mutable locators such as branch names, paths, symbolic refs, aliases, query results, and current URLs **MUST NOT** be represented as immutable historical state identities unless the underlying native system guarantees that property.

## CON-007 — Provenance semantics should extend mature foundations

The future domain model **SHOULD** evaluate W3C PROV-compatible concepts before introducing incompatible generic provenance primitives.

This constraint does **not** require RDF, OWL, PROV-O serialization, or a graph database.

## CON-008 — Temporal semantics do not imply mandatory temporal infrastructure

Requirements concerning valid/effective time and recorded/system time **MUST NOT** be interpreted as selecting a bitemporal database or requiring explicit bitemporal fields everywhere.

## CON-009 — Relationship requirements do not imply graph storage

Typed many-to-many relationships and traversal requirements **MUST NOT** be interpreted as selecting RDF, property graphs, graph databases, or any particular query language.

## CON-010 — Event/history requirements do not imply event sourcing

Preservation of change history, corrections, or projections **MUST NOT** be interpreted as requiring event sourcing as the persistence architecture.

## CON-011 — Retrieval requirements do not require AI

The system **MUST** be able to satisfy its authoritative semantic responsibilities without a language model.

AI-assisted retrieval, extraction, classification, summarization, relationship suggestion, or narrative assistance MAY be added later, but authoritative semantic claims require inspectable provenance and policy regardless of whether AI participated.

## CON-012 — Evidence validity is proposition-scoped

Future architecture **MUST NOT** reduce claim-relative evidence validity to generic whole-repository or whole-artifact freshness when the system has enough information to identify the relevant proposition and inputs.

## CON-013 — Historical preservation does not mean preserving everything

The requirement to preserve corrections and historical truth **MUST NOT** be interpreted as universal indefinite retention of every transient observation, tool artifact, chat message, generated output, or environment detail.

Retention policy remains project-scoped and selective.

## CON-014 — Human-authored narrative remains distinct from canonical facts

Authored learning and explanatory narratives MAY reorganize and interpret project-memory sources, but they **MUST NOT** silently become the authoritative source for underlying engineering claims merely because they are more readable.

## CON-015 — Domain vocabulary remains provisional until domain modeling

Terms used in this requirements phase—such as semantic subject, representation, claim, authority scope, evidence, and epistemic state—describe behavioral needs.

Their exact entity boundaries, lifecycle, cardinality, identity rules, and schemas **MUST** be settled in domain modeling rather than inferred from requirement wording alone.
