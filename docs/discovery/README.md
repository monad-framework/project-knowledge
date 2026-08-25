# Discovery

Discovery turns the inception problem statement into evidence-backed requirements.

This phase should resist the temptation to design the system too early. Its job is to understand the information we need to preserve, the workflows in which that information appears, the failure modes of current practices, and the minimum capabilities required to improve them.

## Current status

Discovery is active.

### First evidence pass

The first evidence-driven pass established:

- [`corpus/README.md`](corpus/README.md) — corpus method and case index;
- [`corpus/case-template.md`](corpus/case-template.md) — neutral case-study structure;
- `corpus/cases/PKC-0001` through `PKC-0007` — seed cases from real Monad engineering history;
- [`observations/initial-corpus-observations.md`](observations/initial-corpus-observations.md) — cross-case findings;
- [`user-jobs/initial-user-jobs.md`](user-jobs/initial-user-jobs.md) — context-recovery jobs exposed by the cases;
- [`failure-modes/initial-failure-mode-catalog.md`](failure-modes/initial-failure-mode-catalog.md) — failure patterns exposed by the cases; and
- [`open-questions.md`](open-questions.md) — unresolved questions and evidence targets.

### Second evidence pass

The second pass deliberately introduced dogfood cases and counterexamples rather than adding only more Monad governance cases:

- `PKC-0008` — Project Knowledge's own root-status summary drift;
- `PKC-0009` — a working hypothesis refined by evidence without being wholly rejected;
- `PKC-0010` — an ADR retaining semantic identity across repository relocation;
- `PKC-0011` — a lower-ceremony frontend project where ordered authoritative documents and Git provide a useful project-memory model;
- [`observations/second-pass-observations.md`](observations/second-pass-observations.md) — findings about assertion-scoped freshness, epistemic refinement, relocation, progressive structure, ordering, selective capture, and negative evidence; and
- [`corpus/coverage-matrix.md`](corpus/coverage-matrix.md) — explicit support levels and missing case types.

The corpus is no longer Monad-only, but it remains too narrow to establish a general domain model.

## Planned discovery tracks

### 1. Real information corpus

Collect representative project information from actual engineering work. The corpus should include both difficult cases and counterexamples where ordinary repository practices are already sufficient.

Important case types include:

- unresolved questions;
- competing hypotheses;
- decisions with alternatives;
- superseded decisions;
- requirements and constraints;
- experiments and evidence;
- implementation artifacts;
- failures and corrections;
- terminology changes;
- work and milestone context;
- narrative explanations; and
- information that legitimately belongs to several views.

The coverage matrix should be used to distinguish observed evidence from attractive but untested concepts.

### 2. User jobs and recovery scenarios

Describe what a person is trying to accomplish when they need project memory. Examples include returning after a long absence, understanding why a component exists, reviewing a changed decision, onboarding, investigating a regression, and reconstructing a milestone.

### 3. Existing approaches

Study how current tools and practices address portions of the problem: source control, issue trackers, wikis, ADRs, architecture documentation, notebooks, PKM systems, search, graph systems, provenance systems, temporal models, and related approaches.

The goal is not to prove that existing tools are inadequate. The goal is to identify what already works, what can be composed, and where gaps actually remain.

This is the next major discovery track after the counterexample pass.

### 4. Failure modes

Record concrete ways project knowledge becomes difficult to use: duplication, drift, stale authority, lost rationale, weak provenance, context fragmentation, hierarchy mismatch, unbounded capture, excessive structure, or retrieval without understanding.

### 5. Requirements derivation

Derive functional and quality requirements from the corpus, user jobs, existing-approach analysis, and observed failure modes. Requirements should be traceable back to evidence wherever practical.

## Key validation question

> Can the emerging model represent the messy reality of engineering work without forcing information into unnatural categories or imposing more cognitive burden than it removes?

A second, equally important question is now explicit:

> Can a project remain simple when simple files, links, ordering, search, and Git are already enough?

## Discovery outputs

Expected outputs include:

- representative corpus and case studies;
- corpus coverage and counterexamples;
- user jobs and use cases;
- existing-approach analysis;
- failure-mode catalog;
- capability requirements;
- quality attributes;
- constraints;
- unresolved questions; and
- candidate domain concepts for the subsequent domain-modeling phase.

No implementation architecture is an expected discovery output unless a requirement necessarily constrains it.

## Exit discipline

Discovery should not be considered complete merely because a plausible ontology or architecture has emerged. Before requirements are treated as stable, the project should have:

1. tested findings against cases outside the initial Monad corpus;
2. investigated existing approaches that may already solve portions of the problem;
3. connected important user jobs to concrete failure modes and evidence;
4. distinguished general engineering-knowledge needs from Monad-specific governance conventions;
5. preserved negative evidence showing where richer structure is unnecessary; and
6. demonstrated that proposed capabilities reduce rather than increase context-management burden.
