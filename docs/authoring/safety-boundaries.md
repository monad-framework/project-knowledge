# Safety Boundaries

## Principle

Capture tooling exists to remove mechanical authoring burden, not to make semantic decisions invisible.

## Classification

| Operation | Tool may do automatically | Requires explicit human semantics | May be suggested but not silently applied |
| --- | --- | --- | --- |
| Generate UUID | yes | no | n/a |
| Choose `.pk/records/<kind>/<id>.json` path | yes | no | n/a |
| Add `schema: pk/v1` / record `kind` | yes | no | n/a |
| Capture ordinary `recorded_at` | yes | user may override where supported | n/a |
| Read Git blob/object identity | yes | user chooses the source artifact/input | n/a |
| Find candidate existing Subject | yes | user selects when identity matters | yes |
| Decide two artifacts share one Subject | no | yes | yes |
| Assign Representation role | no | yes | yes |
| Define Claim concern/value | no | yes | yes |
| Bind Assertion to Claim/Representation | tool resolves aliases | yes, relationship is authored | yes |
| Set valid-time meaning | no silent default | yes | yes |
| Create Authority | no | yes | yes |
| Define authority scope/concern | no | yes | yes |
| Define authority basis | no | yes | yes |
| Declare typed relationship meaning | no | yes | yes |
| Classify relationship as authored/imported/derived/inferred | no when interpretation matters | yes | yes |
| Select evidence target Claim | no | yes | yes |
| Capture current state of selected evidence inputs | yes | human selects relevant inputs | n/a |
| Widen evidence from C1 to related C2 | no | yes, as a separate evaluation | no automatic widening |
| Infer decision acceptance from PR merge | no | yes if project policy/author explicitly declares it | yes only as visible proposal |
| Assign epistemic meaning such as rejected/hypothesis | no | yes when vocabulary supports it | yes |
| Infer causality from chronology | no | yes | yes |

## Fail-closed cases

The Planner must stop and request explicit input when:

- semantic identity is ambiguous;
- authority would otherwise be guessed;
- valid time is required for the intended meaning but not supplied;
- evidence target or breadth is unclear;
- a relationship's type changes interpretation materially;
- a requested existing record selector matches multiple records; or
- a semantic suggestion remains unconfirmed.

## No silent convenience defaults

The following are specifically prohibited as hidden defaults:

```text
newest document = authoritative
merged PR = accepted decision
same title/path stem = same Subject
recorded_at = valid_from
nearby test = evidence for all related Claims
most linked source = canonical source
search result rank = authority
multiple copies = corroboration
chronologically prior event = cause
```

## Source mutation boundary

Capture may read native files and Git metadata. It must not rewrite those artifacts merely to make them easier to model.

The only writes in CA-1 are Project Knowledge-owned authoring artifacts and final S2 records under Project Knowledge-controlled paths.

## AI boundary

CA-1 does not require or authorize LLM/AI semantic extraction.

A future AI-assisted authoring layer must use the same authored/generated/observed/suggested distinction and may not bypass the plan/review boundary for materially consequential semantics.

## Security/access boundary

The first local implementation inherits the repository/filesystem access boundary already used by M0. Capture must not copy source content into S2 merely because it can read it. Native references and narrowly required observed state should be preferred over duplicating potentially sensitive content.
