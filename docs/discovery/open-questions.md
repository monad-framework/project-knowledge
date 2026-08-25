# Discovery Open Questions

These questions are intentionally unresolved. They should guide additional corpus collection, research, capability derivation, and requirements work.

## Identity

1. What needs a stable semantic identity distinct from file, issue, commit, or tool identity?
2. When do several representations refer to one logical object, and when are they meaningfully separate objects?
3. How should identity survive rename, move, regeneration, projection, split, merge, and supersession?

`PKC-0010` strengthens the rename/move portion of question 3: semantic identity can survive relocation when the source domain supplies a stable identifier. It does not answer how identity should work for artifacts that lack one.

Existing-approach research adds an important constraint: native tool identities should generally be preserved. Project Knowledge semantic identity, if required, should coordinate across Git objects, paths, issue IDs, URLs, execution IDs, and other native identifiers rather than replacing them.

## Time

4. Which engineering statements need explicit valid-time semantics rather than relying on Git history?
5. How should current truth and historical truth be presented together without ambiguity?
6. Which project assertions need the distinction between when something was valid/effective and when the project learned or recorded it?

`PKC-0008` adds a low-complexity example of question 5: an old phase statement remains historically true while becoming invalid as a present-tense summary.

Existing temporal and bitemporal systems demonstrate that valid/effective time and system/recorded time are mature, distinct concepts. The remaining question is no longer whether the distinction exists; it is **where the distinction creates enough engineering value to justify explicit modeling**.

## Authority

7. Is authority best understood as scoped to properties or claims rather than attached globally to artifacts?
8. How should conflicting authoritative claims be represented when disagreement is genuine rather than stale projection?
9. Can authority be derived from project policy, or must it be recorded with each relevant relationship?

Existing tools reinforce rather than solve this problem: Git, ADRs, issue trackers, standards, and generated artifacts can each be locally authoritative for different concerns. No surveyed mechanism supplies a project-wide scoped-authority model by itself.

## Provenance and evidence

10. Which engineering provenance concepts map directly to W3C PROV, and which require engineering-specific extension?
11. What exactly should an evidence record say it supports?
12. How should evidence validity respond to changes that are relevant to one claim but irrelevant to another?
13. How should later corrections to inaccurate provenance preserve both the original record and the corrected understanding?
14. Should evidence-support relationships be modeled separately from ordinary derivation/provenance relationships?

The existing-approaches pass substantially narrows question 10. W3C PROV already distinguishes Entity, Activity, Agent, generation, usage, derivation, attribution, revision, primary source, alternate representation, specialization, and qualified relationships. Project Knowledge should test those concepts before inventing generic provenance primitives.

The EOSV corpus still exposes a residual gap around **claim-relative evidence**: provenance can explain where evidence came from without necessarily defining the proposition it validates or which later changes invalidate that support.

## Context

15. Which contexts materially affect interpretation: repository, checkout, branch, commit, worktree, execution, host, tool version, environment, lifecycle state?
16. Which context should be captured automatically, and which should require explicit human declaration?
17. What historical execution context must remain reconstructable after temporary worktrees and environments disappear?
18. Which contextual locators should be preserved as historical facts even when they are unsuitable as durable reconstruction identities?

Git's distinction between immutable objects and contextual/mutable refs strengthens the need to separate exact source-state identity from locators such as `HEAD`, branch names, relative paths, and worktree paths.

## Classification and capture

19. How does the system distinguish canonical source, derived representation, coordination projection, transient control artifact, external source, and incidental environment state?
20. How much classification can be inferred safely before automation becomes a source of false structure?
21. What is the minimum-friction capture path for an unstructured observation or question?
22. How can the model become richer over time without forcing users to fully classify information at capture time?
23. When should the system deliberately decline to capture or enrich information because the expected retrieval value is too low?

`PKC-0011` makes questions 22 and 23 central. Existing linked-note systems and W3C PROV's layered vocabulary reinforce progressive formalization as a serious design constraint rather than merely a UI convenience.

## Epistemic evolution

24. Which knowledge-state transitions need explicit semantics beyond generic version history?
25. How should refinement, narrowing, strengthening, weakening, rejection, correction, deprecation, and supersession differ?
26. How should simultaneous competing hypotheses be represented before evidence resolves them?
27. When does a hypothesis become a decision, requirement, invariant, or accepted project fact?
28. Are there mature epistemic/evidence models worth reusing before defining engineering-specific state semantics?

`PKC-0009` demonstrates refinement without wholesale replacement but does not yet answer questions 26 or 27.

The existing-approaches pass leaves epistemic evolution as one of the strongest potentially Project Knowledge-specific gaps and identifies question 28 as a targeted research need if capability derivation requires it.

## Relationships and causality

29. Which relationships deserve explicit structure rather than remaining ordinary links or prose?
30. How should causal statements differ from dependency, chronology, correlation, rationale, provenance, or influence?
31. How can the system avoid creating a dense graph that is technically complete but cognitively unusable?
32. Which relationship semantics can be reused from PROV, ADR practices, issue dependencies, and other mature models?

Existing approaches strongly suggest reusing established relation semantics where they fit instead of creating a universal Project Knowledge relationship vocabulary from scratch.

## Projections and views

33. What makes a projection trustworthy: lineage, generation time, source identity, synchronization status, deterministic regeneration, or some combination?
34. Which views should be generated and which should remain deliberately authored narratives?
35. How should the system present stale projections that are still valuable historical evidence?
36. How fine-grained should freshness be: artifact, section, assertion, field, dependency, or another unit?
37. How should search indexes, chunks, embeddings, and generated summaries identify their canonical source and extraction context?

`PKC-0008` suggests that whole-document freshness can be too coarse. Search/RAG research adds another projection family whose chunk/index identity must not be confused with semantic project identity.

## Scope and generality

38. Which findings are properties of engineering knowledge generally, and which are artifacts of Monad's unusually governed process?
39. What cases from simpler projects, team projects, incident response, exploratory prototyping, and non-software technical work would falsify or refine the emerging model?
40. Which needs should Project Knowledge solve by reuse, integration, extension, or new behavior?
41. At what point would a composition of existing tools be sufficient enough that Project Knowledge should remain only a convention/integration layer rather than a distinct application?

`PKC-0011` supplies one simpler-project counterexample, but one project is not enough to generalize from.

Question 40 replaces the broader earlier question “what can existing tools solve?” The initial existing-approaches pass now gives enough evidence to classify candidate capabilities explicitly as **REUSE**, **INTEGRATE**, **EXTEND**, or **NEW**.

## Cognitive burden

42. What amount of structure actually reduces human context-recovery cost?
43. When does additional metadata cost more to author and maintain than it returns in retrieval value?
44. Can the system support progressive formalization: capture first, enrich only when value becomes clear?
45. How should a project select a retention depth appropriate to its scale, risk, collaboration model, and learning goals?
46. Which enrichment can be suggested or derived automatically while still requiring human confirmation before semantic assertions become authoritative?

## Next evidence needed

The corpus should still deliberately seek:

- a decision that was explicitly superseded by another decision;
- an unresolved question with competing hypotheses;
- a research/experiment path that changed architecture;
- an implementation failure whose lesson later affected unrelated work;
- a concept or term whose meaning changed over the project;
- a case where the same underlying information is reorganized into both expert reference and educational narrative;
- another small or low-ceremony project with materially different characteristics;
- a case where excessive capture or metadata became a burden; and
- a multi-person collaboration case with genuine disagreement or concurrent knowledge creation.

## Next discovery work

The broad existing-approaches survey is now complete enough for capability derivation.

The next pass should trace each candidate capability through:

```text
Corpus case(s)
    ↓
Failure mode(s)
    ↓
User job(s)
    ↓
Existing approach coverage
    ↓
Residual gap
    ↓
Candidate capability
```

Targeted additional research should be triggered by specific residual gaps rather than another broad tool survey. Likely candidates include epistemic-state/evidence models and collaboration/disagreement semantics if those survive capability derivation.
