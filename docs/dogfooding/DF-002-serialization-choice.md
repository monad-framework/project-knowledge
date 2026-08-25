# DF-002 — Open Question to Resolved Serialization Choice

## Recovery question

> Before M0 detailed design, what portable serialization format had Project Knowledge selected, which alternatives were considered, what was eventually selected, and what evidence shows that selection is actually implemented?

## Why this is a different dogfood shape

DF-001 exercised historical-to-current decision status across multiple representations. DF-002 begins from an explicitly unresolved architecture question.

`docs/architecture/open-questions.md` asked:

> Which concrete text format should M0 use for portable semantic records?

That question was recorded at `2026-08-25T15:19:57Z`. The later M0 technology decision, committed at `2026-08-25T15:42:21Z`, selected JSON over YAML, TOML, and custom syntax.

The original open-question document remains historically correct. DF-002 therefore asks whether Project Knowledge can preserve the period of uncertainty, retain rejected alternatives, and recover the later decision without rewriting the earlier document.

## Semantic slice

DF-002 adds:

| Record kind | Count | Purpose |
| --- | ---: | --- |
| Subject | 1 | Stable identity for the serialization-choice question/decision |
| Representation | 2 | Open-question document + M0 technology-decision document |
| Claim | 4 | JSON, YAML, TOML, and custom as possible values of `serialization.format` |
| Assertion | 1 | The technology-decision document asserts JSON after selection |
| Authority | 1 | The M0 technology decision governs the selected format after its decision time |
| Relationship | 3 | Preserve YAML/TOML/custom as alternatives considered in the decision |
| Activity | 1 | Preserve the open-question → technology-selection provenance step |
| Evidence Evaluation | 1 | Confirm selected JSON is reflected in repository implementation artifacts |
| **Total** | **14** | One unresolved-question-to-resolution thread |

Together with DF-001, this brings the self-dogfood corpus to 24 portable records across two narrow recovery threads.

## Temporal behavior

Before the M0 technology decision, there is intentionally no authoritative Assertion for `serialization.format`:

```text
2026-08-25T15:30:00Z
serialization.format → unknown
```

After the decision boundary:

```text
2026-08-25T16:00:00Z
serialization.format → json
```

The system therefore does not need to manufacture a special value such as `undecided`. Absence of sufficient authoritative information remains first-class `unknown`.

## Alternatives

The technology-decision document explicitly says JSON was selected over YAML, TOML, and custom syntax.

M0 has no executable `EpistemicAnnotation` record kind, even though epistemic semantics exist in the broader domain model. DF-002 therefore represents the rejected possibilities as unasserted Claims linked to the technology-decision Representation by authored `alternative_considered_in` Relationships.

This works, but it is indirect. The model can preserve the alternatives without pretending they were ever true, yet a future richer epistemic model may offer a more natural representation if repeated real cases justify it.

## Investigation/provenance

An Activity records that the M0 technology-selection step used the open-question artifact and generated the technology-decision Representation.

This does not claim that a Git commit mechanically proves all reasoning in the decision. It preserves a minimal provenance edge between the previously unresolved question and the later decision artifact.

## Evidence interpretation

The selected JSON Claim has a claim-relative Evidence Evaluation using the current Git blobs for:

- `docs/m0/technology-decisions.md`; and
- `schemas/v1/record.schema.json`.

The evaluation confirms that JSON was selected and that the repository implements the portable contract as JSON Schema. It does not prove that JSON must remain the permanent authoring syntax; the technology decision itself explicitly leaves room for future YAML or richer authoring layers that compile into the same logical S2 model.

## Expected executable checks

`tests/dogfood_serialization_choice.rs` must verify:

- the pre-selection query returns `unknown`;
- the post-selection query resolves JSON;
- YAML, TOML, and custom remain recoverable as alternatives rather than competing authoritative truths;
- the selection Activity links the open question to the decision Representation; and
- the implementation evidence remains current.

## Findings to evaluate

### Unknown may be sufficient for unresolved questions

DF-002 does not yet require a first-class `Question` record. The Subject plus unresolved authoritative state can preserve the operational fact that no answer had been selected.

That does **not** mean questions are never first-class knowledge objects. It only means this case does not force that schema expansion.

### Alternative/hypothesis semantics are less natural than final-state semantics

Representing candidates as unasserted Claims plus typed Relationships is semantically safe but somewhat indirect. This is the first self-dogfood signal that the M0 executable vocabulary may be thinner around epistemic process than around authority/current truth.

One case is insufficient to add an epistemic record kind.

### Capture burden is now a repeated signal

DF-001 required 10 authored S2 records. DF-002 requires 14 more for a different recovery shape. Both require manual UUIDs, cross-references, source bindings, timestamps, and validity semantics.

The repeated friction is not that these semantics are meaningless; most encode distinct facts. The repeated friction is that humans should not have to hand-author deterministic structural boilerplate to express them.

## Product implication status

DF-002 provides the second independent self-dogfood signal supporting **low-friction authored capture/scaffolding** as a real post-M0 capability candidate.

This is now strong enough to promote that capability into detailed design after DF-002 merges, while preserving a strict boundary:

- tooling may generate deterministic structure;
- tooling may inspect native source state;
- tooling may validate and suggest;
- tooling must not silently decide authority, semantic equivalence, evidence breadth, or epistemic meaning.

The epistemic-vocabulary question remains open pending more cases.
