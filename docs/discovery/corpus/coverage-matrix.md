# Corpus Coverage Matrix

This matrix tracks what the current corpus actually demonstrates. It is a guardrail against promoting attractive concepts into requirements before sufficient evidence exists.

Confidence refers to corpus support, not implementation priority.

| Area | Supporting cases | Current reading | Confidence | Important gaps / counterevidence |
| --- | --- | --- | --- | --- |
| Multiple representations of one logical object | PKC-0001, PKC-0002, PKC-0008 | Real and operationally important in some projects | High | Simpler projects may have few projections |
| Semantic identity distinct from storage locator | PKC-0001, PKC-0010 | Strongly supported where continuity matters | High | Not every note/artifact needs an assigned stable ID |
| Scoped authority | PKC-0001, PKC-0002, PKC-0003, PKC-0008 | Authority can differ by property and context | High | Need cases outside repository/governance settings |
| Current truth plus historical truth | PKC-0002, PKC-0008, PKC-0009 | Both are needed; history should not masquerade as current state | High | Need longer-lived project examples |
| Temporal validity | PKC-0002, PKC-0005, PKC-0007, PKC-0008 | Strong pressure exists | High | Exact temporal model remains unknown |
| Provenance semantics | PKC-0003, PKC-0006, PKC-0007 | Generic `source` is too coarse for complex cases | High | Simpler provenance needs should remain cheap |
| Observation context | PKC-0003, PKC-0004, PKC-0007 | Context can materially change interpretation | High | Need non-worktree/non-CI examples |
| Derivation lineage | PKC-0001, PKC-0002, PKC-0004, PKC-0008 | Useful for projections and summaries | High | Need human-curated derivation examples |
| Evidence validity is claim-relative | PKC-0005, PKC-0006, PKC-0007 | Strongly supported in verification workflows | High | Need evidence cases outside software verification |
| Classification before ingestion | PKC-0004 | Clearly demonstrated | Medium | Only one direct source-classification failure so far |
| Causal explanation | PKC-0004, PKC-0005, PKC-0006, PKC-0007 | Useful for diagnosis and learning | Medium | Need cases where causality is uncertain or contested |
| Epistemic state | PKC-0009 | Hypothesis/refinement distinction is clearly useful | Medium | Need competing simultaneous hypotheses and explicit rejection |
| Supersession | ADR process in PKC-0010 discusses it, but no corpus case yet shows a complete real supersession event | Do not yet model from assumption | Low | Acquire an actual old-decision/new-decision chain |
| Terminology evolution | Indirectly suggested by project history but not yet captured as a case | Open | Low | Need one term whose meaning/name changes over time |
| Experiment-driven decision | Not yet represented | Open | None | Acquire a real experiment -> evidence -> decision -> implementation chain |
| Narrative / learning projection | Motivation is strong, but direct corpus evidence is limited | Open | Low | Need a case showing same facts reorganized for teaching or onboarding |
| Ordered hierarchy / sequence | PKC-0011 | Can be highly useful and sufficient | High | Must coexist with multi-view organization where needed |
| Progressive structure | PKC-0008, PKC-0011 | Strong counterpressure against mandatory heavy modeling | High | Need additional small-project cases |
| Capture selectivity / retention depth | PKC-0011 and discovery practice | Likely necessary | Medium | Need examples of overcapture causing retrieval burden |
| Summary freshness | PKC-0008, PKC-0011 | Curated summaries are valuable but can drift | High | Need to compare manual, derived, and mixed summaries |

## Current corpus balance

The corpus now contains:

- seven high-complexity Monad cases;
- two Project Knowledge dogfood cases;
- one Monad identity/relocation case; and
- one lower-ceremony frontend counterexample.

This is better than a Monad-only corpus but is still narrow. It should not yet support claims about engineering projects in general.

## Admission rule for requirements

A discovery observation should not become a general requirement solely because it appears in one pathological case.

Before requirement derivation, prefer one of the following:

1. repeated evidence across materially different cases;
2. one severe case plus a compelling user job and failure consequence;
3. a foundational constraint that follows directly from the project vision; or
4. explicit scoping that limits the requirement to projects/workflows exhibiting the relevant pressure.

## Next evidence targets

Priority additions are:

1. an actual decision supersession chain;
2. competing hypotheses resolved by evidence;
3. an experiment-driven engineering decision;
4. terminology evolution across real artifacts;
5. a teaching/onboarding narrative built from existing project state;
6. a small project where capture overhead itself became a problem; and
7. a non-software or adjacent technical project if Project Knowledge is expected eventually to generalize beyond software engineering.