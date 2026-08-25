# Existing-Approach Research Sources

This document records the primary sources used during the initial existing-approaches pass. It is a research ledger, not an endorsement list.

Sources were checked on **2026-08-25**.

## Git

### Git core data model

- Git, **gitdatamodel — Git's core data model**  
  https://git-scm.com/docs/gitdatamodel.html

Relevant observations:

- Git objects are immutable after creation.
- Object identity is content-derived.
- Git distinguishes objects, references, the index, and reflogs.
- Exact historical object contents are recoverable while the object remains available.

### Git revisions

- Git, **gitrevisions / revisions**  
  https://git-scm.com/docs/gitrevisions  
  https://git-scm.com/docs/revisions

Relevant observations:

- commit IDs identify immutable revisions;
- symbolic refs such as branch names and `HEAD` are contextual pointers rather than immutable state identities; and
- revision expressions can identify commits, trees, and blobs.

These semantics map directly to corpus cases where `HEAD`, branch names, paths, and immutable commit identity must not be conflated.

## GitHub Issues and wikis

- GitHub Docs, **About issues**  
  https://docs.github.com/en/issues/tracking-your-work-with-issues/learning-about-issues/about-issues
- GitHub Docs, **Adding sub-issues**  
  https://docs.github.com/en/issues/tracking-your-work-with-issues/using-issues/adding-sub-issues
- GitHub Docs, **Creating issue dependencies**  
  https://docs.github.com/en/issues/tracking-your-work-with-issues/using-issues/creating-issue-dependencies
- GitHub Docs, **About wikis**  
  https://docs.github.com/en/communities/documenting-your-project-with-wikis/about-wikis

Relevant observations:

- issue systems explicitly model work metadata, hierarchy, dependencies, discussion, and links to development activity;
- wikis provide long-form project documentation and their own revision history; and
- these capabilities are coordination/documentation oriented rather than a general semantic project-memory model.

## Docs as code

- Write the Docs, **Docs as Code**  
  https://www.writethedocs.org/guide/docs-as-code/

Relevant observations:

- docs-as-code deliberately reuses developer mechanisms: version control, plain text, issue trackers, code review, and automated testing;
- documentation can therefore participate directly in repository review/history workflows; and
- this is strong evidence for keeping ordinary version-controlled text as a first-class Project Knowledge input rather than replacing it with a proprietary authoring model.

## Architecture Decision Records

- MADR, **Decisions** and ADR resources  
  https://adr.github.io/madr/decisions/  
  https://adr.github.io/

Relevant observations:

- ADR practices preserve decision context and outcome in durable records;
- common ADR models explicitly represent lifecycle/status and relationships to other decisions; and
- the pattern is intentionally scoped to significant decisions rather than all project knowledge.

Monad's own ADR practice also provides corpus evidence for stable decision identity surviving repository relocation (`PKC-0010`).

## Linked-note / PKM systems

- Obsidian Help, **Backlinks**  
  https://obsidian.md/help/Plugins/Backlinks
- Obsidian Help, **Outgoing links**  
  https://obsidian.md/help/Plugins/Outgoing%2Blinks
- Obsidian Help, **Properties**  
  https://obsidian.md/help/properties

Relevant observations:

- plain-text notes can gain backlinks, outgoing-link views, aliases, and lightweight typed properties;
- unlinked mentions can suggest relationships not explicitly authored yet; and
- properties are intentionally small, atomic pieces of human- and machine-readable information.

This family is useful evidence for progressive formalization and low-friction relationship discovery.

## Event sourcing

- Microsoft Azure Architecture Center, **Event Sourcing pattern**  
  https://learn.microsoft.com/en-us/azure/architecture/patterns/event-sourcing
- Martin Fowler, **Event Sourcing**  
  https://martinfowler.com/eaaDev/EventSourcing.html

Relevant observations:

- event sourcing stores an append-only sequence of events rather than only mutating current state;
- current state can be materialized from the event history; and
- full audit history can support reconstruction and compensating behavior.

Event sourcing is therefore strong evidence for preserving state transitions and deriving projections, but an event log does not automatically encode decision rationale, authority, claim semantics, or valid-time corrections.

## Temporal and bitemporal data

- Microsoft Learn, **Temporal tables**  
  https://learn.microsoft.com/en-us/sql/relational-databases/tables/temporal/overview
- XTDB, **Time in XTDB**  
  https://docs.xtdb.com/about/time-in-xtdb.html
- XTDB, **What is XTDB?**  
  https://docs.xtdb.com/intro/what-is-xtdb.html

Relevant observations:

- system-versioned temporal tables preserve previous row versions and enable point-in-time reconstruction;
- system time alone records when a version was present in the database;
- valid time separately represents when a fact is considered effective in the modeled world; and
- bitemporal models combine system/transaction time with valid time, enabling questions resembling both “what do we now believe was true then?” and “what did we believe then?”

This distinction closely matches Project Knowledge's observed need to separate current truth, historical truth, and later correction.

## Provenance

- W3C Recommendation, **PROV-O: The PROV Ontology**  
  https://www.w3.org/TR/prov-o/

Relevant observations:

PROV-O defines a reusable model around:

- `Entity`;
- `Activity`;
- `Agent`;
- `wasGeneratedBy`;
- `used`;
- `wasDerivedFrom`;
- `wasAttributedTo`;
- `wasRevisionOf`;
- `hadPrimarySource`;
- `specializationOf`;
- `alternateOf`; and
- qualified relationships carrying additional attributes.

PROV-O explicitly supports incremental use: simple provenance descriptions can be elaborated with richer terms when needed. That is unusually well aligned with Project Knowledge's progressive-structure principle.

## RDF / graph representation

- W3C, **RDF 1.2 Concepts and Abstract Data Model**  
  https://www.w3.org/TR/rdf12-concepts/

Relevant observations:

- RDF provides a graph-oriented model of identified resources and relations;
- graph representation is naturally suited to many-to-many relationships and traversal; and
- graph expressiveness does not itself define Project Knowledge concepts such as scoped authority, evidence validity, temporal truth, or cognitive presentation.

RDF is therefore evidence that mature graph representation machinery exists; it is not evidence that Project Knowledge should use RDF as its storage format.

## Search, semantic retrieval, and RAG-style access

- Elastic Docs, **Hybrid search**  
  https://www.elastic.co/docs/solutions/search/hybrid-search
- Elastic Docs, **Vector search**  
  https://www.elastic.co/docs/solutions/search/vector
- Elastic Docs, **Ranking and reranking**  
  https://www.elastic.co/docs/solutions/search/ranking

Relevant observations:

- lexical search remains valuable for exact names, identifiers, and terms;
- vector/semantic search retrieves conceptually similar information when wording differs;
- hybrid search can combine lexical and semantic retrieval into one ranked result set; and
- reranking can improve candidate ordering.

This is strong evidence for hybrid retrieval as an eventual access capability. Retrieval does not itself establish provenance, current authority, temporal validity, or semantic identity, so search should not become the truth model.

## Research caution

Several sources above describe concrete products. Project Knowledge should extract the **semantics and proven patterns** relevant to the corpus, not infer that those products must become dependencies.
