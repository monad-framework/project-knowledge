# DF-003 — CA-1 Evidence-to-Implementation Traceability

## Recovery question

> Why does CA-1 exist, which project evidence motivated it, which design decision governs it, and what evidence shows that the capability was actually implemented and validated?

## Why this is a different dogfood shape

DF-001 tested historical/current truth. DF-002 tested unresolved alternatives followed by a selected answer. DF-003 tests a causal/traceability recovery path:

```text
observed friction
    ↓
capability promotion
    ↓
detailed design
    ↓
architecture decision
    ↓
implementation
    ↓
verification
```

It is also the first experiment required to use CA-1 itself instead of hand-authoring the final S2 records.

## Authoring input

The human semantic input is retained at [`DF-003-ca1-traceability.intent.json`](DF-003-ca1-traceability.intent.json).

The intent explicitly chooses:

- one Subject: `CA-1 — Guided Capture and Scaffolding capability`;
- five Representation roles;
- one `capability_status = implemented_and_validated` Claim;
- one `evidence_driven_capability_delivery` Activity;
- the Assertion source;
- the Authority concern and basis;
- four semantic Relationships;
- the exact evidence target, method, result, and input artifacts.

The intent does **not** contain Project Knowledge UUIDs, final S2 paths, Git blob hashes, `schema`/`kind` envelopes, or ordinary `recorded_at` timestamps.

## Plan → review → apply execution

DF-003 exercised ADR-0003's transaction boundary literally.

### 1. Plan only

A read-only clean runner executed:

```text
pk capture plan \
  --intent docs/dogfooding/DF-003-ca1-traceability.intent.json \
  --out df003-plan.json
```

The generated immutable plan had:

- plan ID `4ba3c35a-5e2a-4f81-a2c8-164784e99bf0`;
- 15 create-record operations;
- 21 relevant preconditions;
- 0 warnings; and
- 0 blockers.

The plan visibly presented all generated IDs, destination paths, native paths, and observed Git blob identities before mutation.

### 2. Review

The plan was reviewed before apply. Its semantic shape matched the intended recovery thread:

| Record kind | Count | Purpose |
| --- | ---: | --- |
| Subject | 1 | Stable identity for CA-1 as a capability |
| Representation | 5 | DF-001, DF-002, authoring design, ADR-0003, CA-1 closure |
| Claim | 1 | `capability_status = implemented_and_validated` |
| Activity | 1 | Evidence-driven capability delivery provenance |
| Assertion | 1 | Closure asserts the completed status |
| Authority | 1 | Closure governs `capability_status` in this slice |
| Relationship | 4 | Two `motivates`, one `governs_design_of`, one `verifies` |
| Evidence Evaluation | 1 | Claim-relative CA-1 acceptance/closure evidence |
| **Total** | **15** | One evidence-to-implementation recovery thread |

### 3. Apply the exact reviewed plan

The apply runner downloaded the exact plan artifact produced above rather than replanning, then executed:

```text
pk capture apply --plan df003-plan.json --yes
pk validate
pk resolve --subject f8adff3e-11ad-43a9-9275-7d30fc9d1973 --concern capability_status
cargo test --locked --all-targets --all-features
```

Apply created exactly the 15 reviewed S2 files and reported `valid: true`.

After apply:

- repository S2 count increased from 24 to 39;
- `capability_status` resolved successfully;
- the applicable Claim is `a033a49d-1675-41d0-aacc-00cae1f2d724`;
- the scoped Authority is `14d840f2-146a-474a-aed5-d759dbdf23d7`;
- the claim-relative Evidence Evaluation is `91cc9483-aa06-4f13-b7e3-209d30cdb30b`; and
- all pre-existing M0, CA-1, DF-001, DF-002, and schema-validation tests remained green.

The temporary workflows used to execute the dogfood plan/apply transaction were removed after the generated records were committed.

## Executable recovery check

`tests/dogfood_ca1_traceability.rs` verifies the durable repository state rather than the transient Capture Plan.

It checks that:

- the CA-1 Subject's `capability_status` resolves;
- the resolved Claim is the reviewed `implemented_and_validated` Claim;
- its Evidence Evaluation remains current;
- the Subject has the expected motivating/design/decision/verification Representation roles;
- the provenance Activity uses both dogfood findings and identifies three generated downstream Representations; and
- the Activity-scoped Relationships preserve two `motivates` edges, `governs_design_of`, and `verifies`.

## Findings

### CA-1 materially reduces mechanical authoring burden

This is the first real-project confirmation of the capability that DF-001 and DF-002 requested.

For a 15-record final bundle, the author supplied no:

- UUIDs;
- destination record filenames;
- portable-record envelopes;
- Git blob identities; or
- ordinary capture timestamps.

CA-1 generated those values, exposed them for review, rechecked relevant preconditions, validated the prospective corpus, and applied the exact reviewed plan.

The structural-authoring problem identified by DF-001 and DF-002 is therefore materially reduced rather than merely moved into another hand-authored canonical format.

### The review boundary worked as intended

Planning was read-only. Apply used the exact reviewed plan artifact rather than generating a fresh set of UUIDs after review.

This confirms that plan immutability is not just a design statement; it is usable as an operational safety boundary.

### Semantic decisions remain explicit

The author still had to decide the meaning-bearing fields:

- what the Subject is;
- what role each Representation plays;
- what `capability_status` means;
- what Activity occurred;
- what sources motivate the design;
- which decision governs the design;
- what verifies the Claim;
- which Representation is authoritative for the concern; and
- exactly what the evidence evaluation supports.

CA-1 did not obtain its usability improvement by erasing these decisions.

### Raw Authoring Intent still contains interaction ceremony

CA-1 removes final-record boilerplate, but the JSON intent still requires operational syntax such as:

- local `as` aliases;
- repeated alias references;
- explicit endpoint `kind` values in Relationships; and
- nested `ref` objects.

Some of that is safely derivable from the session/catalog and does not itself represent semantic judgment. A richer guided interaction could reduce it further.

DF-003 is the first real-use signal for this remaining interaction burden. One case is not enough to authorize a broad conversational/AI authoring layer.

### Representation is ahead of retrieval

The most important new limitation is not authoring or the semantic model.

Project Knowledge can now preserve the causal/traceability chain, but the current CLI cannot directly recover it. `pk resolve` answers the scoped current-state question:

```text
capability_status → implemented_and_validated
```

There is no corresponding command today for questions such as:

```text
why does this capability exist?
what motivated this design?
which decision governs it?
what verifies this Claim?
show the related evidence/provenance chain
```

The DF-003 test can traverse `all_records()` programmatically to verify those edges, but an ordinary user should not need to write Rust or inspect raw JSON to recover reasoning that Project Knowledge already stores.

This is the first concrete self-dogfood case where **semantic representation succeeds but the retrieval surface blocks the full recovery job**.

## Product implication status

### Semantic model

**No semantic-model expansion is authorized by DF-003.**

Subject, Representation, Claim, Assertion, Authority, Activity, Relationship, and Evidence Evaluation were sufficient for the tested trace.

### Capture/authoring

**CA-1 is validated by real use.**

A richer guided authoring interaction is now a candidate for further evaluation, especially for safely derivable alias/kind/reference ceremony, but DF-003 alone does not authorize fuzzy identity inference, semantic auto-decision, or an AI authoring agent.

### Retrieval/traversal

DF-003 provides a strong requirements signal for a user-facing semantic traversal/explanation capability. The exact command/API shape should still be designed from recovery jobs rather than guessed from graph terminology.

The next question should be framed as:

> What is the smallest retrieval surface that lets a human recover the reasoning chain already represented in Project Knowledge without exposing raw storage mechanics?

This signal is stronger than a generic desire for a graph browser because it is tied to a real failed recovery path.
