# PKC-0011 — Ordered authoritative documents can be enough

## Case type

Counterexample; lower-ceremony project; specification progression; proportionality.

## Source project

`thomascarter613/frontend` — Maximum Workspace.

## Observed situation

The repository implements a ten-pass product/design/engineering specification stored as `docs/0000.md` through `docs/0009.md`.

The root README explicitly identifies those documents as authoritative and summarizes the current implementation slice. The tenth document itself recaps the preceding passes as a deliberate progression:

1. maximum workspace visual architecture;
2. production design system and component architecture;
3. interaction, docking, splitting, keyboard, and workspace behavior;
4. product architecture and view archetypes;
5. canonical resource/object/entity architecture;
6. workflows and task architecture;
7. collaboration, permissions, governance, security, and administration;
8. extensibility, customization, integrations, and configuration;
9. reliability, concurrency, offline, recovery, performance, and hardening;
10. frontend engineering contract, implementation architecture, state, testing, and developer handoff.

Implementation then proceeded through a small number of ordinary Git commits and pull requests.

## Why this case matters

This project contains substantial design knowledge, but its repository organization is intentionally simple:

- a numbered specification sequence;
- a README summary;
- implementation source;
- tests; and
- Git history.

There is no evidence that the project requires a rich governance graph, explicit temporal database, or fine-grained knowledge-object registry in order to function at its present scale.

That is important negative evidence for Project Knowledge.

A general-purpose engineering knowledge system would fail if adopting it required every project to decompose documents into dozens of typed entities before the team received value.

## What the simple model does well

### Sequence is explicit

The numbered files make the intended reading and design progression immediately visible.

### Authority is understandable

The README names the pass documents as authoritative, while implementation source represents the realized software state.

### The current slice is easy to communicate

The README compresses a large specification corpus into a useful operational summary.

### Git provides adequate event history for many questions

For straightforward implementation evolution, commits and PRs provide a usable chronological record without another event system.

## Where pressure may still appear

The specification documents are large and cumulative. A future reader may still have difficulty answering cross-cutting questions such as:

- which exact pass introduced a particular invariant;
- how a concept changed between passes;
- which later statements refine earlier ones;
- why a particular design choice was made rather than merely what the final specification says; or
- which implemented behavior traces to which passage across ten large documents.

Those pressures are plausible retrieval problems, but they should not be assumed to require a heavy solution until they are observed in actual use.

## Counter-hypothesis

For many projects, the most useful knowledge system may initially be little more than:

- durable Markdown;
- clear authority conventions;
- good naming and ordering;
- links;
- search;
- Git history; and
- selective structured metadata only when a concrete recovery problem appears.

## Pressure on the eventual system

Project Knowledge should support **progressive structure**.

A credible design should allow a project to begin with ordinary documents and gain richer semantics incrementally. The cost of capture and modeling must remain proportional to the information-management problem being solved.

The system should therefore be evaluated not only on how much complexity it can represent, but also on how little complexity it can impose.