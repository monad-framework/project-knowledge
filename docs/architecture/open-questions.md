# Architecture Open Questions

The architectural shape is selected, but detailed implementation choices remain intentionally open.

## Portable serialization

1. Which concrete text format should M0 use for portable semantic records?
2. Which schema language/version should validate records?
3. What canonicalization rules are required for deterministic diffs/hashes?
4. Should one semantic record live per file, or may record sets be grouped?
5. What default directory/layout should repository-local projects use?

Current direction: deterministic structured text with explicit schema/version and source-control-friendly files.

## Identifier encoding

6. What concrete encoding should Project, Subject, record, Activity, Relationship, and Evidence Evaluation IDs use?
7. Should project-local IDs be the default, with optional globally unique namespaces?
8. How should stable native IDs such as ADR IDs participate—as aliases, derived IDs, or bindings?

The architecture requires semantics/stability, not a UUID/URI choice yet.

## Embedded read model

9. Which embedded database/index should M0 use?
10. Should graph traversal use recursive relational queries initially or a separate graph index?
11. What migration/version discipline applies to disposable read-model schemas?
12. Should the read model be one file/database per Project or support a workspace containing many Projects?

Architecture requirement: S3 must remain disposable/rebuildable.

## Compiler/synchronization

13. What is the canonical dependency-key model for incremental invalidation?
14. How are source change tokens persisted without making them authoritative history?
15. Which compiler phases are deterministic and independently cacheable?
16. How are partial adapter failures committed to a consistent read-model generation?
17. Do we need a small local change journal for performance, and if so how do we prevent it from becoming mandatory event sourcing?

## Authority policy

18. What portable syntax expresses Authority Assignments and policy rules?
19. What precedence model applies to project defaults versus Subject-specific exceptions?
20. How are ambiguous overlapping policies detected?
21. How much historical policy state must M0 retain for as-of resolution?

The domain permits unresolved authority; architecture must not invent default winners.

## Claims and native structured state

22. Which native fields may be adapted directly as Assertions without portable Claim duplication?
23. How are Claims normalized sufficiently for authority/evidence comparison without requiring a universal predicate ontology?
24. What constitutes semantic equality between two simple value Claims in M0?
25. How are opaque prose Claims handled?

## Context/time

26. Which Context dimensions should be standardized in M0?
27. How are uncertain/partial valid-time intervals encoded?
28. When can Git history supply recorded/system time rather than duplicating timestamps?
29. What redaction model preserves reconstructability without storing sensitive environment data?

## Evidence

30. How are Claim-relevant dependencies declared in the first implementation?
31. Should evidence freshness be `unknown` unless explicit dependency relevance exists?
32. What validator/method descriptors are standardized versus opaque extension values?
33. How are multiple Evidence Evaluations displayed without creating a generic confidence/truth score?

## Retrieval

34. Which exact full-text/index mechanism should M0 use, if any?
35. Is semantic/vector retrieval deferred entirely beyond M0?
36. What citation structure should all retrieval/projection APIs expose?

Current direction: exact ID + structured lookup is enough for M0; full retrieval product can follow after semantic kernel validation.

## Security

37. What is the first concrete access-partition representation?
38. For local single-user operation, is filesystem/native access sufficient as the initial authorization boundary?
39. What conformance tests are required before remote/multi-user federation is allowed?
40. How are derived facts whose provenance crosses access partitions labeled/enforced?

## API and process model

41. Should M0 be a CLI process that opens the embedded read model directly?
42. When does a long-lived local daemon become useful?
43. Which contracts should be library APIs versus command protocols?
44. Is a language-neutral interchange/API specification needed before the first implementation?

## Implementation stack

45. Which implementation language best fits:

- filesystem/Git integration;
- deterministic parsing/validation;
- embedded database access;
- CLI distribution;
- future service operation;
- strong type/domain invariant enforcement?

46. Which package/build/runtime choices keep the first kernel small and portable?

Technology selection belongs to the **M0 detailed-design/bootstrap pass** immediately after this architecture is accepted.

## Product surface

47. Which first human-facing projection best proves context recovery: CLI inspect, generated Markdown current-state page, or both?
48. When should Wiki.js become an output/integration target rather than a hand-authored experiment?
49. What stable API should allow future web/IDE/wiki clients to consume the same resolver outputs?

## Deferred production architecture

The following remain intentionally outside M0:

- multi-tenant hosting;
- distributed synchronization;
- high-availability service topology;
- production OAuth/SSO;
- fine-grained collaborative authoring;
- real-time webhook/event ingestion;
- distributed search/index clusters;
- vector/LLM infrastructure;
- enterprise retention/legal-hold systems.

They should not shape the initial kernel unless later evidence shows a foundational constraint.