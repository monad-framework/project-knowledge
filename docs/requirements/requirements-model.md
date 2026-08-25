# Requirements Model and Traceability Discipline

This document defines how Project Knowledge requirements are written, classified, traced, and later verified.

## Normative language

The keywords **MUST**, **MUST NOT**, **SHOULD**, **SHOULD NOT**, and **MAY** are normative.

- **MUST / MUST NOT** — required for conformance.
- **SHOULD / SHOULD NOT** — expected unless a documented project-specific reason justifies deviation.
- **MAY** — optional capability or behavior.

## Requirement classes

### Functional requirements — `FR-NNN`

Externally meaningful system behavior.

### Quality attributes — `QA-NNN`

Properties of the system and its operation, such as portability, explainability, determinism, progressive adoption, and performance.

### Constraints — `CON-NNN`

Boundaries that restrict acceptable designs or interpretations of other requirements.

### Non-requirements — `NR-NNN`

Explicitly excluded or deferred behavior. These protect the project from architecture by assumption.

## Requirement family numbering

Functional requirements are grouped by the six evidence-derived families:

- `FR-1xx` — RF-1 Native interoperability and progressive adoption
- `FR-2xx` — RF-2 Semantic identity, representation, and relationships
- `FR-3xx` — RF-3 Authority and current truth
- `FR-4xx` — RF-4 Provenance, time, and context
- `FR-5xx` — RF-5 Evidence and epistemic evolution
- `FR-6xx` — RF-6 Retrieval, impact, and explanation

The numeric grouping is organizational only. It does not imply module or service boundaries.

## Required trace for every promoted functional requirement

Every `FR-*` requirement MUST trace to one or more promoted discovery capabilities.

Every promoted capability already traces through:

```text
Capability
  ↓
User job
  ↓
Failure mode / constraint
  ↓
Corpus evidence
  ↓
Existing-approach coverage
```

The requirements trace therefore becomes:

```text
Requirement
  ↓
Capability
  ↓
User job
  ↓
Failure mode / constraint
  ↓
Corpus evidence
```

Where a requirement is a foundational constraint rather than a direct response to one failure, that fact MUST be stated explicitly.

## Requirement anatomy

Each functional requirement should include, where useful:

- **Statement** — normative behavior.
- **Purpose** — why the behavior exists.
- **Primary trace** — promoted capability IDs.
- **Scope** — universal, conditional, or optional.
- **Failure prevented** — the main failure mode or ambiguity addressed.
- **Verification direction** — the kind of evidence a later implementation should produce.

Requirements SHOULD avoid embedding:

- storage schema;
- API shape;
- database technology;
- UI layout;
- framework choice;
- deployment topology; or
- internal service decomposition.

Those belong to later specifications unless the behavior cannot be stated without them.

## Scope classes

### Universal

Applies to every conforming Project Knowledge deployment or implementation.

### Conditional

Applies when the relevant project-memory feature is used or when the corresponding project condition exists.

Example: valid-time semantics are conditional because not every artifact needs explicit bitemporal metadata.

### Optional

A supported capability that a project may choose to use, such as authored learning narratives.

## Progressive-formalization interpretation

No requirement may be interpreted to force a project through the maximum semantic model merely because the system can represent it.

Examples:

- stable semantic identity is required **where cross-representation continuity matters**, not for every file;
- explicit valid time is required **where validity differs materially from recorded history**, not for every assertion;
- typed relationships are required **where their semantics support a recovery or impact job**, while ordinary links remain valid elsewhere;
- epistemic state is required only for workflows where it materially improves recovery or correctness.

## Requirement change discipline

Once accepted, a requirement SHOULD NOT be silently rewritten in a way that changes its meaning.

Material changes should preserve:

1. prior requirement identity/version or historical text;
2. the reason for change;
3. evidence or decision that caused the change; and
4. affected downstream domain/architecture/specification artifacts.

This requirement process intentionally dogfoods Project Knowledge's own correction-without-erasure principles.

## Promotion versus implementation

A requirement becoming normative does not authorize implementation.

The intended sequence remains:

```text
requirements
    ↓
domain model
    ↓
architecture
    ↓
specifications
    ↓
implementation planning
```
