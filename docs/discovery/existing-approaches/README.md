# Existing Approaches

This discovery track evaluates established tools, practices, standards, and data models against the engineering-memory problems observed in the Project Knowledge corpus.

The goal is **not** to prove that existing tools are inadequate, and it is not to select an implementation stack. The goal is to identify:

1. which parts of the problem are already solved well;
2. which semantics are mature enough to reuse rather than reinvent;
3. which approaches compose naturally;
4. which gaps remain after composition; and
5. which candidate Project Knowledge capabilities are actually evidence-backed.

## Evaluation baseline

The primary evaluation inputs are:

- [`../user-jobs/initial-user-jobs.md`](../user-jobs/initial-user-jobs.md) — recovery and understanding jobs `UJ-001` through `UJ-014`;
- [`../failure-modes/initial-failure-mode-catalog.md`](../failure-modes/initial-failure-mode-catalog.md) — observed failure modes `FM-001` through `FM-016`;
- [`../corpus/coverage-matrix.md`](../corpus/coverage-matrix.md) — current evidence coverage and known gaps;
- [`../observations/initial-corpus-observations.md`](../observations/initial-corpus-observations.md); and
- [`../observations/second-pass-observations.md`](../observations/second-pass-observations.md).

The comparison method is documented in [`evaluation-method.md`](evaluation-method.md).

## Approaches in this pass

This pass evaluates families of mechanisms rather than vendors as products:

1. Git and repository history;
2. docs-as-code and wikis;
3. Architecture Decision Records;
4. issue/work tracking;
5. linked-note / PKM systems;
6. event sourcing;
7. temporal and bitemporal data models;
8. provenance models, especially W3C PROV;
9. RDF-style graph representation and explicit semantic relationships; and
10. lexical, semantic, and hybrid search / RAG-style retrieval.

The individual assessments are grouped under [`approaches/`](approaches/).

## Provisional result

The evidence does **not** support replacing these systems with one universal store or application.

Instead, the strongest current interpretation is compositional:

- **Git** is already an excellent immutable history and exact-state substrate for repository-native artifacts.
- **Docs-as-code and wikis** are already strong human-authored explanation and narrative mechanisms.
- **ADRs** are a proven pattern for preserving significant decision context, alternatives, status, and supersession.
- **Issue trackers** are strong coordination systems for work, discussion, hierarchy, dependencies, and current workflow state.
- **Linked-note systems** demonstrate low-friction progressive structure, backlinks, emergent relationships, aliases, and lightweight properties.
- **Event sourcing** demonstrates the value of immutable event history plus derived current-state projections.
- **Temporal/bitemporal models** directly address the distinction between what was true/effective and what the system knew or recorded at a particular time.
- **W3C PROV** provides a mature vocabulary for entities, activities, agents, derivation, revision, attribution, usage, generation, and qualified provenance relationships.
- **Graph representations** are useful for explicit many-to-many semantic relationships and cross-domain traversal.
- **Hybrid retrieval** combines exact lexical matching with semantic similarity and is a strong discovery/access mechanism.

None of these mechanisms alone solves the corpus's full recovery problem. In particular, none by itself provides a project-wide, cross-tool model of semantic identity, scoped authority, temporal truth, provenance, epistemic evolution, evidence scope, causal explanation, and multiple human views while retaining low-friction ordinary-project operation.

That remaining integration/problem-modeling gap is the strongest current justification for Project Knowledge as a distinct project.

## Important constraint

This track reinforces the principle discovered in `PKC-0011`:

> Do not require richer structure when ordinary files, links, search, and Git already make the project intelligible.

Any future Project Knowledge design should therefore support **progressive structure**. A project should be able to begin with normal files and history, then add semantic identity, relationships, provenance, temporal semantics, or other structure only where those additions reduce recovery cost.

## Outputs

- [`evaluation-method.md`](evaluation-method.md) — comparison criteria and rating semantics;
- [`source-notes.md`](source-notes.md) — primary research sources used in this pass;
- [`approach-capability-matrix.md`](approach-capability-matrix.md) — compact comparison across observed needs;
- [`composition-findings.md`](composition-findings.md) — what appears reusable, composable, and still missing; and
- [`approaches/`](approaches/) — focused assessments by mechanism family.

## Scope discipline

These documents are discovery findings, not architecture decisions.

They do not select:

- a database;
- a graph engine;
- RDF or OWL as the canonical representation;
- an event store;
- a search engine;
- a wiki or PKM product;
- an AI/RAG architecture; or
- a final Project Knowledge domain model.

A mature standard can contribute semantics without dictating storage technology, and a useful tool pattern can be adopted conceptually without being embedded as a product dependency.
