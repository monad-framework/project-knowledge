# PKC-0002 — Coordination projection diverges from canonical lifecycle state

## Source context

Project: Monad

Sources:

- GitHub issue `#21` — `[Feature] F-002-01 / WP-MVP-0001 — Repository identity/config`
- https://github.com/monad-framework/monad/issues/21
- canonical work packet `engineering/work-packets/WP-MVP-0001.md`

## Observed situation

The GitHub feature issue describes `WP-MVP-0001` as `READY — NOT AUTHORIZED` and explicitly states that GitHub is a coordination projection while canonical Git/EOS state remains authoritative.

The current canonical work packet, however, records `WP-MVP-0001` as `CLOSED`.

Both artifacts are useful. The GitHub issue preserves coordination history and the earlier operational state, while the canonical work packet represents the current governed state. Read without authority and time context, however, they appear contradictory.

## Information involved

- one logical work identity represented in multiple tools;
- canonical lifecycle state;
- projected lifecycle state;
- authority declaration;
- historical coordination content;
- synchronization state;
- timestamps or event ordering; and
- links between canonical and projected representations.

## Why this is difficult to organize

The problem is not simply duplication. The two representations serve different purposes and may have different update cadences.

Deleting the stale projection would discard history and coordination context. Treating both representations as equally current would create ambiguity. Treating only the canonical artifact as useful would discard externally visible project context.

A directory hierarchy cannot by itself express that two artifacts refer to the same logical work while having different authority, projection roles, and validity intervals.

## Candidate relationships

- GitHub issue `projects` canonical work item;
- projection `represents` logical work identity;
- canonical artifact `authoritative for` lifecycle state;
- projection state `derived from` canonical state at an earlier point;
- projection `became stale relative to` canonical state;
- historical statement `valid at` earlier lifecycle interval.

## Time, authority, provenance, and context

### Time

The apparent contradiction disappears if the two statements are interpreted at different valid times.

### Authority

The GitHub issue explicitly declares itself a coordination projection. The canonical work artifacts and EOS control state own the authoritative lifecycle state.

### Provenance

A reader should be able to determine which canonical state produced a projected statement and whether the projection has synchronized since.

### Context

A GitHub user may encounter the projection first and reasonably assume it is current unless authority and freshness are visible.

## Recovery questions

- What is the current lifecycle state of `WP-MVP-0001`?
- What did GitHub show when the work packet was Ready?
- Was the issue incorrect when written, or merely later superseded by lifecycle progress?
- Which representation owns lifecycle authority?
- When was the projection last synchronized?
- What other projections of the same logical work exist?
- Which projection statements are still current?

## Provisional observations

1. Multiple representations of one logical object are not necessarily duplicates; they may be projections with different purposes.
2. Authority is a property that project memory may need to preserve explicitly.
3. Apparent contradictions can be temporal rather than factual errors.
4. Projection freshness and canonical truth are distinct concepts.
5. Preserving history and providing current truth are both necessary and can conflict if validity is not modeled.

## Open questions

- Should projection freshness be computed, declared, or inferred?
- What minimum provenance is needed to connect a projection state to the canonical state from which it was produced?
- How should a human interface show useful stale history without presenting it as current truth?
