# Linked Notes and Progressive Structure

Personal-knowledge-management and linked-note systems are relevant to Project Knowledge not because engineering projects should become personal notebooks, but because these systems explore a design problem that conventional documentation often handles poorly:

> How can information be captured with low friction first and become more structured through links, backlinks, aliases, properties, and later organization?

Obsidian is used here as a concrete, well-documented example of this mechanism family.

## Useful established patterns

### Bidirectional navigation

Ordinary authored links create outgoing relationships, while backlinks expose incoming references without requiring the target note to maintain a manual reverse index.

This is highly relevant to `UJ-013` (impact discovery) and `FM-014` (relationship burden exceeds working memory).

### Emergent link discovery

Unlinked mentions can surface text that matches another note's name or alias, suggesting potential relationships that have not been explicitly authored.

This is a useful precedent for **assisted enrichment** rather than requiring perfect structure at capture time.

### Lightweight properties

Note properties demonstrate that free-form Markdown can coexist with small pieces of structured metadata such as:

- dates;
- aliases;
- links;
- tags;
- numbers; and
- lists.

Properties remain human-visible and machine-readable rather than forcing the entire note body into a schema.

### Stable-enough aliases

Aliases help separate the name by which users refer to a concept from the exact storage filename. This is not full semantic identity, but it is evidence that human-facing naming and storage naming need not be identical.

## Strong fit for Project Knowledge

Linked-note patterns are strongest for:

- `C-04` explicit/lightweight relationships — **Partial to Strong**, depending on link typing;
- `C-09` human narrative — **Strong**;
- `C-10` discovery — **Strong** through links, backlinks, search, and mentions; and
- `C-11` progressive structure — **Strong**.

This family directly reinforces the second-pass finding that structure should be earned by recovery value.

## Important limits

A backlink tells us that `A` mentions `B`; it does not inherently tell us whether:

- A implements B;
- A contradicts B;
- A supersedes B;
- A was derived from B;
- A provides evidence for B; or
- A merely mentions B in passing.

Similarly, flexible properties do not automatically establish shared semantics across projects. A property named `source`, for example, can still collapse the distinct provenance concepts identified in `O-005` and `FM-010`.

Typical linked-note systems also do not natively provide:

- scoped authority;
- bitemporal truth;
- claim-relative evidence validity;
- executable derivation lineage;
- exact repository-state identity; or
- a formal distinction between canonical and projected representations.

## Design lesson, not product choice

The strongest lesson is not “use Obsidian.” It is:

> Rich project memory can emerge incrementally from ordinary authored content when the system makes linking, reverse-link discovery, aliasing, and lightweight metadata cheap.

This suggests several capabilities worth carrying forward into requirements discovery:

1. capture should not require complete classification;
2. typed relationships can be added after initial capture;
3. the system can suggest candidate relationships without silently asserting them;
4. semantic identity should tolerate multiple human names/aliases;
5. backlinks/impact views should be derived automatically; and
6. human-readable source should remain useful even when richer structured projections exist.

## Failure mode to avoid

A Project Knowledge system could easily become worse than ordinary linked notes if every captured thought requires choosing among dozens of object types and relationship predicates.

Therefore any future ontology or schema should be evaluated against a hard question:

> Does this additional structure reduce later recovery cost enough to justify the capture burden?
