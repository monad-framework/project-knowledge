# Dogfooding and Real-Project Validation

Phase 6 uses the executable M0 kernel on real Project Knowledge and Monad material before authorizing broad feature expansion.

Each experiment begins with a concrete recovery question, uses the smallest semantic slice that can answer it, runs through the normal compiler/resolver and CI gates, and records both recovery value and capture burden.

## Experiments

| Experiment | Recovery shape | Result | Primary signal |
| --- | --- | --- | --- |
| [DF-001](DF-001-adr-status-recovery.md) | historical/current decision status + evidence | PASS | M0 semantics sufficient; manual capture burden visible |
| [DF-002](DF-002-serialization-choice.md) | unresolved question → alternatives → selected answer → implementation evidence | Pending CI | Tests `unknown`, alternative preservation, provenance, and repeated capture burden |

## Promotion discipline

A dogfood case may expose a product hypothesis without immediately authorizing implementation.

Prefer promotion when:

1. the recovery problem is real rather than synthetic;
2. the signal recurs across materially different cases;
3. the proposed capability reduces burden without weakening domain invariants; and
4. deterministic assistance is separated from semantic judgment.

DF-001 alone did not authorize capture tooling. If DF-002 independently reproduces the same authoring burden, low-friction authored capture/scaffolding becomes eligible for detailed design.
