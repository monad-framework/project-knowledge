# Problem Statement

## Context

Complex software engineering projects generate a large and continuously changing body of information. That information includes source code, requirements, architecture, decisions, alternatives, experiments, evidence, failures, implementation notes, work history, unresolved questions, terminology, lessons, and the reasoning connecting them.

The volume alone is difficult to manage, but volume is not the central problem. The harder problem is that the same piece of information often belongs to several valid organizational perspectives at once.

A decision may simultaneously be:

- part of the chronological history of a project;
- part of the rationale for an architectural element;
- associated with a subsystem;
- motivated by one or more requirements;
- based on research or experiments;
- implemented by particular artifacts;
- produced during a milestone or work item;
- superseded later while remaining historically important; and
- useful as part of an educational explanation of how the system evolved.

Forcing such information into one primary hierarchy loses useful context. Duplicating it across several hierarchies creates drift and maintenance burden.

## Practical origin

The project was motivated by a recurring experience during development of a complex engineering system: even after information had been categorized and organized, the amount of material and the number of valid perspectives over it made it easy to lose orientation.

The problem was not merely remembering individual facts. It was preserving an intelligible model of:

- what is currently believed or decided;
- what was believed or decided previously;
- why the state changed;
- which evidence and constraints influenced it;
- how concepts and artifacts relate;
- what remains unresolved; and
- how another person could reconstruct and learn from the process.

## Limitations of fragmented tooling

Existing engineering and knowledge-management tools are individually useful, but they commonly optimize for one dominant information model:

- version control preserves files and commits;
- issue trackers preserve work items and workflow state;
- wikis preserve pages and navigation hierarchies;
- ADR collections preserve selected decisions;
- architecture documentation preserves system descriptions;
- notebooks preserve chronological or thematic notes;
- search systems retrieve text;
- graph systems represent relationships.

A real engineering project spans all of these concerns. When the project record is distributed among tools whose models do not compose cleanly, reconstructing context becomes a human integration task.

## Core problem

> There is no sufficiently useful, unified way to preserve and navigate the evolving knowledge state of a complex engineering project across multiple organizational perspectives while retaining chronology, provenance, reasoning, uncertainty, and relationships between information.

This statement intentionally describes the problem rather than prescribing a solution.

## Consequences

When the problem is not addressed well:

- engineers repeatedly reconstruct context they previously possessed;
- rationale becomes detached from implementation;
- obsolete information is difficult to distinguish from current information;
- historical reasoning is deleted or buried during documentation updates;
- decisions appear obvious in retrospect because alternatives and uncertainty disappear;
- the same information is copied into multiple locations and diverges;
- new contributors learn the current artifact but not how or why it evolved;
- project knowledge becomes dependent on individual memory;
- handoffs and resumptions become expensive; and
- the engineering process itself is difficult to study or teach from.

## Desired problem property

A successful solution should make the project easier to understand from several directions without requiring the underlying knowledge to be manually rewritten for every view.

Examples include asking:

- What is true now?
- What did we believe at a particular point in time?
- Why was this decision made?
- What evidence supports it?
- What superseded it?
- Which requirements led to this implementation?
- What changed during this milestone?
- Which open questions affect this subsystem?
- What path would teach a newcomer how this part of the system came to exist?

## Open question

It is not yet established whether the right solution is a new application, a data model, a protocol, an integration layer over existing tools, a set of conventions, or some combination of these. Discovery must determine that.
