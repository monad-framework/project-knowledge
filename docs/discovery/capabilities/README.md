# Evidence-to-Capability Derivation

This directory is the boundary between Discovery evidence and later product requirements.

Its purpose is not to produce a feature wish list. Its purpose is to determine, from the evidence already collected, which capabilities are justified, which should be delegated to mature existing mechanisms, which require Project Knowledge-specific semantics, and which attractive ideas remain under-evidenced.

## Required trace

Every candidate capability must have an auditable path of the form:

```text
corpus case(s)
    ↓
observed failure mode(s)
    ↓
user recovery job(s)
    ↓
existing-approach coverage
    ↓
residual gap
    ↓
candidate capability
```

A capability with no evidence trace is not admitted merely because it would be useful or architecturally elegant.

## Disposition classes

Each capability receives exactly one primary disposition:

- **REUSE** — a mature existing mechanism already provides the needed semantics/behavior; Project Knowledge should rely on it rather than duplicate it.
- **INTEGRATE** — existing tools already provide the capability locally, but Project Knowledge must connect or expose it across project memory.
- **EXTEND** — an existing model supplies a strong semantic base, but the engineering corpus requires additional semantics or behavior.
- **NEW** — the corpus supports behavior for which the surveyed approaches do not provide an adequate cross-project-memory model.

Disposition describes novelty, not implementation technology.

## Confidence

Capability confidence is based on evidence strength:

- **High** — repeated support across materially different cases, or one severe case plus strong independent approach analysis.
- **Medium** — meaningful evidence exists, but important counterexamples or generality questions remain.
- **Low / HOLD** — the idea is plausible but the corpus is not sufficient for requirement promotion.

## Scope levels

Candidate capabilities are also grouped by responsibility:

### Foundation dependency

The overall solution needs the behavior, but Project Knowledge should normally delegate it to a native or mature mechanism.

### Project-memory integration

Project Knowledge must connect existing identities, artifacts, histories, relationships, search, or views without replacing their native source systems.

### Project-memory semantics

Project Knowledge appears to need a distinct cross-cutting semantic model because no surveyed mechanism supplies the required project-wide behavior.

### Human-facing projection

The capability turns project memory into a form that supports current-state recovery, history, impact analysis, search, diagnosis, or authored learning.

## Progressive-structure constraint

No capability in this directory implies that every project or artifact must use it.

The evidence requires a progressive model in which ordinary files, native tool identities, links, Git history, and search may remain sufficient. Richer structure is justified only where it improves a concrete recovery job.

Conceptually:

```text
native artifact
    ↓ optional
semantic identity / aliases
    ↓ optional
explicit relationships
    ↓ optional
provenance / time / authority / epistemic detail
    ↓ optional
evidence and derived projections
```

## Files

- [`derivation-method.md`](derivation-method.md) — decision rules used during derivation.
- [`candidate-capabilities.md`](candidate-capabilities.md) — capability registry and detailed rationale.
- [`trace-matrix.md`](trace-matrix.md) — compact evidence-to-capability trace.
- [`promotion-boundary.md`](promotion-boundary.md) — which candidates are mature enough to enter requirements and which remain on hold.

## Non-goals of this pass

This pass does not:

- choose storage technology;
- define the final domain ontology;
- select a graph, temporal, provenance, or search implementation;
- write product requirements prematurely;
- define an MVP architecture; or
- treat all discovered dimensions as mandatory metadata.

The output is a justified capability boundary, not a system design.
