# Discovery Open Questions

These questions are intentionally unresolved. They should guide additional corpus collection, research, and requirements derivation.

## Identity

1. What needs a stable semantic identity distinct from file, issue, commit, or tool identity?
2. When do several representations refer to one logical object, and when are they meaningfully separate objects?
3. How should identity survive rename, move, regeneration, projection, split, merge, and supersession?

`PKC-0010` strengthens the rename/move portion of question 3: semantic identity can survive relocation when the source domain supplies a stable identifier. It does not answer how identity should work for artifacts that lack one.

## Time

4. Which engineering statements need explicit valid-time semantics rather than relying on Git history?
5. How should current truth and historical truth be presented together without ambiguity?
6. Do we need to distinguish when something was true from when the system learned or recorded that it was true?

`PKC-0008` adds a low-complexity example of question 5: an old phase statement remains historically true while becoming invalid as a present-tense summary.

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
21. When should the system deliberately decline to capture or enrich information because the expected retrieval value is too low?

`PKC-0011` makes questions 20 and 21 central. Progressive formalization is no longer only a convenience hypothesis; it is necessary counterpressure against over-modeling.

## Epistemic evolution

22. Which knowledge-state transitions need explicit semantics beyond generic version history?
23. How should refinement, narrowing, strengthening, weakening, rejection, and supersession differ?
24. How should simultaneous competing hypotheses be represented before evidence resolves them?
25. When does a hypothesis become a decision, requirement, invariant, or accepted project fact?

`PKC-0009` demonstrates refinement without wholesale replacement but does not yet answer questions 24 or 25.

## Relationships and causality

26. Which relationships deserve explicit structure rather than remaining ordinary links or prose?
27. How should causal statements differ from dependency, chronology, correlation, or rationale?
28. How can the system avoid creating a dense graph that is technically complete but cognitively unusable?

## Projections and views

29. What makes a projection trustworthy: lineage, generation time, source identity, synchronization status, deterministic regeneration, or some combination?
30. Which views should be generated and which should remain deliberately authored narratives?
31. How should the system present stale projections that are still valuable historical evidence?
32. How fine-grained should freshness be: artifact, section, assertion, field, dependency, or another unit?

`PKC-0008` suggests that whole-document freshness can be too coarse.

## Scope and generality

33. Which findings are properties of engineering knowledge generally, and which are artifacts of Monad's unusually governed process?
34. What cases from simpler projects, team projects, incident response, exploratory prototyping, and non-software technical work would falsify or refine the emerging model?
35. What can existing tools already solve well enough that Project Knowledge should integrate rather than reimplement?

`PKC-0011` supplies one simpler-project counterexample, but one project is not enough to generalize from.

## Cognitive burden

36. What amount of structure actually reduces human context-recovery cost?
37. When does additional metadata cost more to author and maintain than it returns in retrieval value?
38. Can the system support progressive formalization: capture first, enrich only when value becomes clear?
39. How should a project select a retention depth appropriate to its scale, risk, collaboration model, and learning goals?

## Next evidence needed

The next corpus expansion should deliberately seek:

- a decision that was explicitly superseded by another decision;
- an unresolved question with competing hypotheses;
- a research/experiment path that changed architecture;
- an implementation failure whose lesson later affected unrelated work;
- a concept or term whose meaning changed over the project;
- a case where the same underlying information is reorganized into both expert reference and educational narrative;
- another small or low-ceremony project with materially different characteristics;
- a case where excessive capture or metadata became a burden; and
- a multi-person collaboration case with genuine disagreement or concurrent knowledge creation.

## Next research needed

The next major discovery track should investigate existing approaches against the observed jobs and failure modes rather than as a generic tool survey. In particular, research should ask which parts of the problem are already handled well by Git, ADR practice, issue trackers, wikis/documentation systems, PKM/backlink systems, event sourcing, temporal databases, provenance models, knowledge graphs, search/RAG systems, and documentation-as-code practices.
