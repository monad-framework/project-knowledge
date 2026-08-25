# Dogfooding and Real-Project Validation

Phase 6 uses Project Knowledge on its own repository and then representative Monad material before authorizing broad post-M0 feature work.

The purpose is not to maximize the number of semantic records. It is to discover which pieces of project knowledge are worth formalizing, how much authoring burden that creates, and whether the resulting recovery value justifies the structure.

## Method

Each dogfood experiment should:

1. start from a real recovery question encountered in the project;
2. use the existing M0 vocabulary before proposing new model features;
3. capture the smallest S2 record set that can answer the question correctly;
4. exercise the records through the actual compiler/resolver and normal CI gates;
5. distinguish model defects from capture/tooling/retrieval friction;
6. record the semantic footprint and manual steps required; and
7. derive product changes only from repeated or high-value evidence.

Dogfood tests must also be compositional: an experiment may assert the semantics of its own slice, but must not assume that it owns the whole project-memory corpus.

## Experiments

| Experiment | Recovery shape | Result | Primary signal |
| --- | --- | --- | --- |
| [DF-001 — ADR-0001 current status](DF-001-adr-status-recovery.md) | historical/current decision status + evidence | PASS | M0 semantics sufficient; manual capture burden visible |
| [DF-002 — Serialization choice](DF-002-serialization-choice.md) | unresolved question → alternatives → selected answer → implementation evidence | PASS | `unknown` and final selection work; alternative semantics are indirect; capture burden repeated |
| [DF-003 — CA-1 traceability](DF-003-ca1-traceability.md) | evidence → promoted capability → design decision → implementation → verification | PASS | CA-1 materially reduces structural authoring burden; representation now exceeds user-facing retrieval capability |

## Promotion discipline

A dogfood inconvenience is evidence, not automatically a feature request.

Prefer promotion when:

1. the recovery problem is real rather than synthetic;
2. the signal recurs across materially different cases or clearly blocks useful adoption;
3. the proposed capability reduces burden without weakening domain invariants; and
4. deterministic assistance is separated from semantic judgment.

DF-001 and DF-002 independently reproduced the same structural-authoring burden across materially different recovery shapes. That evidence authorized CA-1 — Guided Capture and Scaffolding.

DF-003 then used CA-1 against Project Knowledge itself. The reviewed plan generated and safely applied 15 final S2 records without hand-authored UUIDs, record paths, Git blob identities, portable-record envelopes, or capture timestamps. **CA-1 is therefore validated by real use rather than only by acceptance fixtures.**

The strongest new DF-003 signal is different: Project Knowledge can represent the motivating/provenance/verification chain, but the current CLI cannot recover that chain directly. `pk resolve` can answer scoped current-state questions, while relationship/provenance traversal still requires programmatic access or raw-record inspection.

This does not automatically authorize a generic graph browser. It establishes a concrete recovery requirement to investigate:

> What is the smallest retrieval surface that lets a human recover the reasoning chain already represented in Project Knowledge without exposing raw storage mechanics?

A richer guided authoring interaction also remains a candidate because Authoring Intent still contains safely reducible alias/kind/reference ceremony. DF-003 alone does not authorize fuzzy identity resolution, semantic auto-decision, or an AI authoring agent.
