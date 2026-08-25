# DF-001 — Recovering ADR-0001 Current Status

## Recovery question

> What is the current status of ADR-0001, what was its status before adoption, and what evidence supports continuing to treat the architecture as accepted?

## Why this is a real project-memory problem

`docs/decisions/ADR-0001-federated-portable-core.md` still contains the historical header:

> **Status:** Proposed for acceptance with Architecture PR

That wording is useful historical evidence, but it is not sufficient by itself to recover current project truth. Architecture PR #8 merged on `2026-08-25T15:24:13Z`, `docs/architecture/selected-architecture.md` states that Project Knowledge **will use** the federated portable-core architecture, and M0 later completed without falsifying that architecture within its tested scope.

A reader who opens only the ADR can therefore mistake a historically accurate representation for current state.

## Semantic slice

DF-001 deliberately leaves the native documents unchanged and models the cross-document semantics in S2.

The minimum useful footprint is:

| Record kind | Count | Purpose |
| --- | ---: | --- |
| Subject | 1 | Stable identity for the ADR-0001 architecture decision |
| Representation | 2 | ADR decision record + current selected-architecture definition |
| Claim | 2 | `decision_status = proposed` and `decision_status = accepted` |
| Assertion | 2 | Bind each Claim to the Representation that supports it in valid time |
| Authority | 2 | Scope which Representation controls `decision_status` before/after adoption |
| Evidence Evaluation | 1 | Bind the accepted Claim to the successful M0 falsification result |
| **Total** | **10** | One narrow recovery thread |

This count is itself a dogfooding measurement. The model can express the thread without new vocabulary, but ten authored records for one narrow question is enough ceremony that capture tooling deserves observation in later experiments.

## Valid-time interpretation

The adoption boundary is the Architecture PR #8 merge time:

```text
before 2026-08-25T15:24:13Z
    decision_status → proposed

after 2026-08-25T15:24:13Z
    decision_status → accepted
```

The old ADR wording is not deleted or rewritten to manufacture a timeless document. Instead, its assertion and authority are bounded historically while the selected-architecture representation becomes authoritative for the current concern.

## Evidence interpretation

The accepted Claim has one claim-relative Evidence Evaluation tied to the Git blob for `docs/m0/closure.md`.

The evidence says only that M0 did not falsify the architecture within the tested scope. It does **not** prove that:

- the architecture is universally correct;
- the M0 schema is final;
- Project Knowledge is production-ready; or
- every later use case will fit the architecture.

This is intentionally narrower than treating a successful prototype as blanket proof.

## Executable check

`tests/dogfood_project_state.rs` runs the real compiler/resolver against the repository's own `.pk/records` and verifies:

- the repository is enriched rather than native-only;
- a pre-adoption as-of query resolves `proposed`;
- a post-adoption as-of query resolves `accepted`;
- the accepted Claim's M0 evidence is currently fresh; and
- both native documents are recognized as Representations of the same Subject.

## Initial findings

### The semantic model is sufficient for this thread

DF-001 required no schema or resolver change. Subject/Representation separation, scoped Authority, valid-time Assertions, and claim-relative Evidence were enough to represent the real recovery problem.

### Current truth can differ from a document's visible header without erasing history

This is precisely the distinction the architecture was intended to preserve. The ADR remains historically legible while the resolver can answer the current-state question from the wider project record.

### Capture ceremony is already visible

Ten records is a large manual footprint for a single narrow recovery question. Nothing in DF-001 proves that all ten should disappear; several encode genuinely different semantics. But manually constructing UUIDs, cross-references, timestamps, source bindings, and validity windows is not a plausible default authoring experience.

### Safe automation boundary is becoming clearer

Several fields are candidates for deterministic tooling rather than semantic inference:

- UUID generation;
- record file placement;
- current Git blob identity;
- RFC3339 timestamp capture;
- selection of an existing Subject/Representation by ID;
- structural cross-reference validation.

By contrast, the following should not be inferred silently from merge activity alone:

- that a merge means an ADR is accepted;
- which representation becomes authoritative;
- the exact concern governed by that authority; or
- whether a passing experiment is sufficient evidence for a broader Claim.

## Product implication status

**No M1 feature is authorized by DF-001 alone.**

The strongest candidate emerging from this experiment is a low-friction authored-capture command that generates deterministic structural boilerplate while requiring the human to declare semantic intent. That hypothesis should be tested against additional dogfood cases before it becomes a product requirement.
