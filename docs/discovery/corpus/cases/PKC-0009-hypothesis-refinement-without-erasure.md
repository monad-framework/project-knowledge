# PKC-0009 — Hypothesis refinement without erasure

## Case type

Epistemic evolution; hypothesis refinement; dogfood case.

## Source project

Project Knowledge (`monad-framework/project-knowledge`).

## Observed situation

The project root README records an explicit working hypothesis:

> A useful engineering knowledge system may need to preserve more than documents. It may need to represent knowledge objects, their relationships, provenance, temporal evolution, epistemic state, and multiple projections over the same underlying information.

The README also explicitly marks that statement as a hypothesis rather than an architectural commitment.

After examining `PKC-0001` through `PKC-0007`, `docs/discovery/observations/initial-corpus-observations.md` records a more specific finding: the idea that the problem could be solved principally as a graph problem is incomplete. The corpus exposed independent pressure around temporal semantics, scoped authority, provenance semantics, observation context, classification, evidence scope, derivation lineage, and causality.

## Why this case matters

The later finding does not make the original hypothesis simply `wrong`.

Several parts of the original hypothesis remain supported:

- relationships appear important;
- provenance appears important;
- temporal evolution appears important;
- epistemic state appears important; and
- multiple projections appear important.

What changed is the explanatory sufficiency of one framing. A graph-shaped relationship model may be useful, but it is no longer a sufficient description of the problem.

This is not clean replacement. It is **refinement under evidence**.

## Distinctions exposed

### Hypothesis versus decision

A hypothesis can remain open to revision without controlling implementation. Treating every recorded belief as a decision would overstate its authority.

### Refuted versus narrowed versus strengthened

Knowledge evolution needs more than `current/obsolete`.

A proposition may be:

- rejected;
- partially supported;
- narrowed;
- generalized;
- strengthened;
- weakened;
- replaced; or
- retained with additional conditions.

### Evidence linkage

The refinement is meaningful because it is traceable to concrete cases and observations rather than merely a later preference.

### Historical intelligibility

Deleting or silently rewriting the original hypothesis would make the project's reasoning look more linear and certain than it actually was.

Keeping both without an explicit relationship would force a future reader to infer how the thinking changed.

## Recovery task

A reader should be able to ask:

1. What did the project originally hypothesize?
2. Which evidence tested that hypothesis?
3. Which parts survived?
4. Which parts were weakened or expanded?
5. What is the current working interpretation?
6. Has any of this crossed the boundary from hypothesis into requirement or architectural decision?

## Pressure on the eventual system

The eventual model may need an epistemic lifecycle that is richer than document versioning and richer than binary supersession.

At minimum it should be possible to preserve:

- the proposition as originally stated;
- its epistemic role at that time;
- evidence considered;
- later propositions that refine or challenge it;
- the semantic relationship between old and new claims; and
- the current disposition without rewriting history.

This case argues for explicit knowledge evolution, not for a heavyweight scientific-claim ontology.