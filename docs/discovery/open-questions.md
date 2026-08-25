# Discovery Open Questions

These questions are intentionally unresolved. They should guide additional corpus collection, research, and requirements derivation.

## Identity

1. What needs a stable semantic identity distinct from file, issue, commit, or tool identity?
2. When do several representations refer to one logical object, and when are they meaningfully separate objects?
3. How should identity survive rename, move, regeneration, projection, split, merge, and supersession?

## Time

4. Which engineering statements need explicit valid-time semantics rather than relying on Git history?
5. How should current truth and historical truth be presented together without ambiguity?
6. Do we need to distinguish when something was true from when the system learned or recorded that it was true?

## Authority

7. Is authority best understood as scoped to properties or claims rather than attached globally to artifacts?
8. How should conflicting authoritative claims be represented when disagreement is genuine rather than stale projection?
9. Can authority be derived from project policy, or must it be recorded with each relevant relationship?

## Provenance and evidence

10. Which provenance concepts must remain distinct: producer, derivation source, requested origin, resolved origin, observed state, recorded claim, execution location, or others?
11. What exactly should an evidence record say it supports?
12. How should evidence validity respond to changes that are relevant to one claim but irrelevant to another?
13. How should later corrections to inaccurate provenance preserve both the original record and the corrected understanding?

## Context

14. Which contexts materially affect interpretation: repository, checkout, branch, commit, worktree, execution, host, tool version, environment, lifecycle state?
15. Which context should be captured automatically, and which should require explicit human declaration?
16. What historical execution context must remain reconstructable after temporary worktrees and environments disappear?

## Classification and capture

17. How does the system distinguish canonical source, derived representation, coordination projection, transient control artifact, external source, and incidental environment state?
18. How much classification can be inferred safely before automation becomes a source of false structure?
19. What is the minimum-friction capture path for an unstructured observation or question?
20. How can the model become richer over time without forcing users to fully classify information at capture time?

## Relationships and causality

21. Which relationships deserve explicit structure rather than remaining ordinary links or prose?
22. How should causal statements differ from dependency, chronology, correlation, or rationale?
23. How can the system avoid creating a dense graph that is technically complete but cognitively unusable?

## Projections and views

24. What makes a projection trustworthy: lineage, generation time, source identity, synchronization status, deterministic regeneration, or some combination?
25. Which views should be generated and which should remain deliberately authored narratives?
26. How should the system present stale projections that are still valuable historical evidence?

## Scope and generality

27. Which findings are properties of engineering knowledge generally, and which are artifacts of Monad's unusually governed process?
28. What cases from simpler projects, team projects, incident response, exploratory prototyping, and non-software technical work would falsify or refine the emerging model?
29. What can existing tools already solve well enough that Project Knowledge should integrate rather than reimplement?

## Cognitive burden

30. What amount of structure actually reduces human context-recovery cost?
31. When does additional metadata cost more to author and maintain than it returns in retrieval value?
32. Can the system support progressive formalization: capture first, enrich only when value becomes clear?

## Next evidence needed

The next corpus expansion should deliberately seek:

- a decision that was explicitly superseded by another decision;
- an unresolved question with competing hypotheses;
- a research/experiment path that changed architecture;
- an implementation failure whose lesson later affected unrelated work;
- a concept or term whose meaning changed over the project;
- a case where the same information is useful in both expert reference and educational narrative;
- a small-project case without Monad-style governance; and
- a multi-person collaboration case with genuine disagreement or concurrent knowledge creation.
