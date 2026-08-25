# Dogfooding

Phase 6 uses Project Knowledge on its own repository before authorizing broad post-M0 feature work.

The purpose is not to maximize the number of semantic records. It is to discover which pieces of project knowledge are worth formalizing, how much authoring burden that creates, and whether the resulting recovery value justifies the structure.

## Method

Each dogfood experiment should:

1. start from a real recovery question encountered in the project;
2. use the existing M0 vocabulary before proposing new model features;
3. capture the smallest S2 record set that can answer the question correctly;
4. exercise the records through the actual compiler/resolver;
5. distinguish model defects from capture/tooling friction;
6. record the semantic footprint and manual steps required; and
7. derive product changes only from repeated or high-value evidence.

## Experiments

- [DF-001 — Recovering ADR-0001 current status](DF-001-adr-status-recovery.md)

## Rule

A dogfood inconvenience is evidence, not automatically a feature request. Prefer fixing the workflow only after the same friction recurs or clearly blocks useful adoption.
