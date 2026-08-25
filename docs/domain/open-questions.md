# Domain Open Questions

The first semantic kernel is sufficient to satisfy the current corpus and requirements, but several questions should remain explicit rather than being answered by assumption.

## Identity

1. What concrete identifier format should Subject use?
2. Should Subject identity be project-local by default, globally unique, URI-like, or layered?
3. When a native system already has a stable domain identifier such as an ADR ID, should Project Knowledge alias it, derive from it, or wrap it?
4. What review/confirmation policy is required before inferred identity bindings become authoritative?
5. How should split and merge identity transitions be represented once corpus evidence justifies them?

These are architecture/schema questions except where future evidence changes semantic requirements.

## Representation

6. Is Representation best modeled as its own persisted entity or as a binding over Native Reference + Subject?
7. How should addressable fragments of large documents be identified durably across edits?
8. Which representation roles deserve a small standard vocabulary versus project-defined extensions?

## Claims and assertions

9. What is the minimum useful structured form for a Claim?
10. Should Claims support opaque prose, predicate/value form, typed values, or all of these?
11. When should two semantically equivalent propositions share one Claim identity?
12. Does every authority-bearing property need explicit Claim/Assertion materialization, or can some be adapted directly from native structured state?

## Authority

13. How are Authority Assignments expressed: direct records, project policy rules, adapters, or a combination?
14. How are overlapping authority rules ordered or detected as ambiguous?
15. How should inherited/default authority interact with Subject-specific exceptions?
16. How much historical authority policy must be retained to reconstruct old current-state views correctly?

## Provenance

17. Which W3C PROV concepts map directly to kernel concepts and which engineering extensions are genuinely necessary?
18. Should provenance relationships be normalized into a shared Relationship model or exposed through a specialized API/view?
19. What is the minimum provenance detail required at each progressive-formalization level?

## Time

20. Which domain objects require temporal qualifiers in the first implementation slice?
21. Can native version history satisfy recorded/system time for repository-backed Assertions without duplicated metadata?
22. How should uncertain or partially known valid intervals be represented?
23. Do we need transaction-time correction semantics beyond ordinary recorded-at/history for the first MVP?

## Context

24. What standard Context dimensions belong in the portable kernel versus integration-specific extensions?
25. How should sensitive environment information be redacted or summarized while preserving reconstructability?
26. What is the canonical relationship between Context.source_state and Native Reference source-state identity?

## Evidence

27. What constitutes the identity of an Evidence Evaluation?
28. How should input relevance to a Claim be declared or computed?
29. Can evidence invalidation rules be deterministic enough for M0/M1, or should the first implementation expose freshness as `unknown` unless explicit dependencies exist?
30. How should several Evidence Evaluations combine without accidentally implementing a generalized probabilistic truth engine?

## Epistemics

31. Which minimal epistemic annotations should ship as examples rather than normative vocabulary?
32. How should competing hypotheses be represented once a real corpus case is captured?
33. Should refinement/correction/supersession be Relationship types, Activity kinds, epistemic annotations, or combinations?

## Relationships

34. Which Relationship types are required for the first vertical prototype?
35. Should Relationship identity itself be stable/addressable?
36. What cardinality and validation rules should be schema-level versus project-policy-level?
37. How should project-defined relationship vocabularies avoid semantic collision across integrations?

## Projections

38. Which projection is the smallest useful vertical slice for validating the kernel?
39. Should current-state resolution be a query-time computation, cached projection, materialized artifact, or architecture-dependent choice?
40. How should partial staleness be expressed for compound documents/summaries?
41. What should a retrieval result cite to preserve both native source identity and semantic context?

## Security and privacy

42. How are access boundaries represented when one Subject spans Representations with different permissions?
43. Is authorization entirely architectural/integration-specific, or does the domain need an explicit visibility/access-policy concept?
44. How does selective retention interact with legal/privacy deletion requirements while preserving references to historical facts?

These questions were already acknowledged as requirements-phase gaps and should be addressed before production architecture is considered complete.

## Collaboration and concurrency

45. How should simultaneous incompatible Assertions by different people/teams be represented beyond generic unresolved conflict?
46. Does collaboration require authoring transactions, review state, or consensus concepts in the kernel?

Current evidence is insufficient to add them.

## Architecture selection questions

The following must be answered by architecture rather than by extending the domain model prematurely:

- relational vs document vs graph vs hybrid persistence;
- embedded/local-first vs service-backed operation;
- canonical portable serialization;
- indexing/search engine;
- API boundaries;
- integration adapter model;
- incremental synchronization;
- cache/materialized-view strategy;
- query language;
- identifier encoding;
- validation/schema language;
- implementation language/runtime;
- UI/CLI/web surfaces.

## Suggested architecture-entry tests

Before selecting technology, candidate architectures should demonstrate that they can represent at least these domain scenarios cleanly:

1. **Minimal project:** ordinary Markdown + Git with almost no enrichment.
2. **Identity continuity:** one ADR Subject across two file locations.
3. **Authority conflict:** canonical lifecycle state vs stale coordination projection.
4. **Historical correction:** earlier wrong status retained while corrected current state is clear.
5. **Context-dependent observation:** worktree-local view differs from canonical control state.
6. **Claim-relative evidence:** validator supports C1 but not broader C2.
7. **Derived freshness:** generated summary exposes stale/unknown/current state from lineage.
8. **Unknown resolution:** insufficient information returns unknown rather than guessed identity/authority/truth.

An architecture that handles only the rich cases but makes case 1 cumbersome violates the model.