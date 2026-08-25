# Dogfooding and Real-Project Validation

Phase 6 uses Project Knowledge on its own repository and then representative Monad material before authorizing broad post-M0 feature work.

The purpose is not to maximize the number of semantic records. It is to discover which pieces of project knowledge are worth formalizing, how much authoring burden that creates, and whether the resulting recovery value justifies the structure.

## Method

Each dogfood experiment should:

1. start from a real recovery question encountered in the project;
2. use the existing M0 vocabulary before proposing new model features;
3. capture the smallest S2 record set that can answer the question correctly;
4. exercise the records through the actual compiler/resolver and normal CI gates;
5. distinguish model defects from capture/tooling friction;
6. record the semantic footprint and manual steps required; and
7. derive product changes only from repeated or high-value evidence.

## Experiments

| Experiment | Recovery shape | Result | Primary signal |
| --- | --- | --- | --- |
| [DF-001 — ADR-0001 current status](DF-001-adr-status-recovery.md) | historical/current decision status + evidence | PASS | M0 semantics sufficient; manual capture burden visible |
| [DF-002 — Serialization choice](DF-002-serialization-choice.md) | unresolved question → alternatives → selected answer → implementation evidence | Pending CI | Tests `unknown`, alternative preservation, provenance, and repeated capture burden |

## Promotion discipline

A dogfood inconvenience is evidence, not automatically a feature request.

Prefer promotion when:

1. the recovery problem is real rather than synthetic;
2. the signal recurs across materially different cases or clearly blocks useful adoption;
3. the proposed capability reduces burden without weakening domain invariants; and
4. deterministic assistance is separated from semantic judgment.

DF-001 alone did not authorize capture tooling. If DF-002 independently reproduces the same authoring burden, low-friction authored capture/scaffolding becomes eligible for detailed design.
