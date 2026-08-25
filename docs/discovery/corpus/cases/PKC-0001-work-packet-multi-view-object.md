# PKC-0001 — A work packet as a multi-view engineering object

## Source context

Project: Monad

Primary source:

- `engineering/work-packets/WP-MVP-0001.md`
- https://github.com/monad-framework/monad/blob/main/engineering/work-packets/WP-MVP-0001.md

Related source identities named by the packet include:

- `EPIC-002`
- `F-002-01`
- `PI-MVP-001`
- `WC-MVP-0001`
- `PG-001`
- `product/MVP-RELEASE-1.md`
- `product/product-requirements.md`
- `ADR-0002`
- `ADR-0004`
- `ADR-0005`
- `IFC-WORKSPACE-0001`

## Observed situation

`WP-MVP-0001` is a single named work packet, but understanding it requires information from many different engineering perspectives.

The packet records lifecycle state, objective, product hierarchy, governing architectural decisions, interface specification authority, dependencies, scope boundaries, acceptance criteria, validation procedures, implementation boundaries, and authorization semantics.

Its current status is `CLOSED`, while much of the file necessarily preserves statements describing earlier states such as Ready and not yet authorized.

The packet is therefore simultaneously:

- a planning object;
- a governance object;
- an execution boundary;
- a product decomposition node;
- an architecture/specification consumer;
- a validation contract;
- a historical record; and
- an entry point into implementation history.

## Information involved

- stable identities;
- lifecycle state;
- product hierarchy;
- work hierarchy;
- requirements;
- architecture decisions;
- interface specifications;
- scope and exclusions;
- dependencies;
- authorization conditions;
- acceptance criteria;
- validation commands;
- implementation boundaries;
- historical state statements; and
- completion state.

## Why this is difficult to organize

No single ordinary hierarchy adequately represents the packet.

Placing it under `work-packets/` is useful for one navigation path, but the packet also belongs naturally to views organized by feature, epic, program increment, work cycle, product goal, requirement, ADR, interface, lifecycle state, source component, validation method, and chronology.

Duplicating the packet into those structures would create synchronization and authority problems. Keeping only links preserves navigation but leaves many relationships implicit and difficult to query or reconstruct.

The same document also contains statements from different temporal perspectives: its current header says `CLOSED`, while later text describes what was true when the packet was Ready but inactive.

## Candidate relationships

Provisional relationships visible in the source include:

- work packet `belongs to` feature;
- feature `belongs to` epic;
- work packet `scheduled in` work cycle;
- work packet `scheduled in` program increment;
- work packet `contributes to` product goal;
- work packet `governed by` requirement;
- work packet `governed by` ADR;
- work packet `governed by` interface specification;
- work packet `depends on` accepted architectural decision;
- implementation path `authorized by` work packet;
- acceptance criterion `verified by` validation behavior;
- work packet `has lifecycle state` status;
- historical statement `was valid during` earlier lifecycle state.

These names are descriptive only; they are not a domain-model commitment.

## Time, authority, provenance, and context

### Time

The packet preserves both present and historical assertions. A reader must distinguish `status now` from `instruction that was valid before execution began`.

### Authority

The packet names several governing artifacts rather than containing all governing knowledge itself. Authority is distributed but explicitly referenced.

### Provenance

Acceptance criteria and implementation scope can be traced to product requirements, ADRs, and an interface specification, but that trace is human-readable rather than represented as independently queryable relations.

### Context

The packet means different things in planning, authorization, execution, verification, and historical-review contexts while retaining one stable identity.

## Recovery questions

A future engineer may need to ask:

- Why was this work packet created?
- Which product requirement did each part of it satisfy?
- Which ADRs constrained its implementation?
- What was explicitly excluded and why?
- What lifecycle state was it in on a particular date?
- When did it become authorized?
- Which implementation changes fulfilled it?
- Which tests or evidence demonstrated completion?
- Which later work depended on it?
- What did the packet say before it was closed?
- Which statements in the current document describe historical rather than current truth?

## Provisional observations

1. A useful project-memory unit can participate in many organizational views without having many independent identities.
2. Stable identity and hierarchical location are different concerns.
3. A current document can contain historically valid statements that are no longer current instructions.
4. Understanding governed work requires traversing relationships across product, architecture, specification, execution, and verification artifacts.
5. Folder placement is useful for storage but insufficient as the complete semantic organization of the information.

## Open questions

- Should relationships be extracted from artifacts, authored explicitly, derived, or some combination?
- How should the system distinguish a current assertion from a preserved historical instruction inside the same artifact?
- What constitutes the identity of the work packet across file moves, rewrites, projections, and external representations?
- How much relationship structure is useful before authoring overhead becomes worse than manual navigation?
