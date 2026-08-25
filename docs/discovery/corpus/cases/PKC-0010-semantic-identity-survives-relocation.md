# PKC-0010 — Semantic identity survives repository relocation

## Case type

Identity; locator change; provenance; architectural authority.

## Source project

Monad (`monad-framework/monad`).

## Observed situation

Monad moved its ADR material from a root-level `adrs/` location into `architecture/decisions/`.

A subsequent architecture-documentation commit made the intended semantics explicit:

- `architecture/decisions/` is the canonical ADR root;
- the former `adrs/` root is retired;
- moving an ADR changes its canonical repository location, not its historical identity, acceptance state, or decision meaning; and
- stable ADR identifiers must not be reused for a different decision.

The same change also states that stale projections referencing the retired path should migrate their locator while preserving the ADR identifier.

## Why this case matters

Filesystem identity is convenient but insufficient for long-lived project knowledge.

If `adrs/ADR-0001-...md` and `architecture/decisions/ADR-0001-...md` are treated as unrelated objects merely because the paths differ, the system fragments one historical decision into two entities.

If the path is treated as permanently intrinsic to the decision, normal repository refactoring becomes difficult or destroys continuity.

## Distinctions exposed

### Semantic identity versus locator

`ADR-0001` identifies the decision across repository movement. The path identifies a representation at a particular repository state.

### Current canonical locator versus historical locator

The old path remains important historical provenance even after it ceases to be the current canonical location.

### Move versus semantic change

A relocation event should not imply a new decision version. A materially changed decision would instead require a new ADR and an explicit lifecycle relationship.

### Reference repair versus identity repair

Downstream references may need updating after a move even though the referenced semantic object has not changed.

## Recovery task

A reader or tool should be able to answer:

1. Where is ADR-0001 now?
2. Where did it live previously?
3. Is the current file the same decision or a replacement decision?
4. Which historical or generated references still use the retired path?
5. Did the move alter status, authority, or meaning?

## Pressure on the eventual system

A useful model should avoid equating storage location with semantic identity.

It may need to preserve:

- stable semantic identifiers where the source domain provides them;
- representation locators with valid intervals;
- relocation events;
- canonical-locator status;
- alias or historical-locator resolution; and
- explicit distinction between relocation and semantic revision.

The case does not imply that every note needs a globally assigned ID. Stable identity should be introduced where continuity across movement or projection is valuable.