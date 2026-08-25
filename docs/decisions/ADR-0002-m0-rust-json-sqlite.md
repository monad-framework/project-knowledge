# ADR-0002 — M0 Concrete Stack: Rust, JSON Schema, SQLite, Native Git

- **Status:** Accepted for M0
- **Date:** 2026-08-25
- **Supersedes:** none
- **Related:** ADR-0001 — Federated Portable Core

## Context

ADR-0001 selected a federated architecture with native authoritative state (S1), portable Project Knowledge semantic state (S2), and disposable derived state (S3), but intentionally deferred concrete technology.

M0 now needs the smallest executable implementation that can test the architecture-entry scenarios without causing implementation convenience to redefine the architecture.

## Decision

M0 will use:

- Rust 1.98, edition 2024;
- one Cargo package exposing a library plus `pk` CLI;
- JSON records for S2;
- JSON Schema Draft 2020-12 for portable structural validation;
- UUIDv4 for Project Knowledge-generated stable IDs;
- RFC3339 timestamps;
- SQLite as the disposable S3 read model;
- the native `git` executable as the first S1 adapter; and
- Rust tests as the executable architecture-conformance harness.

## Rationale

### Preserve S1 authority

The implementation calls Git rather than replacing repository semantics. Source object identity remains Git identity.

### Keep S2 portable

JSON + JSON Schema can be read and validated independently of Rust or SQLite. S2 survives deletion of the executable database.

### Keep S3 disposable

SQLite gives M0 a useful local query/runtime substrate while remaining easy to delete and rebuild.

### Keep M0 small

One package avoids a premature crate graph. Modules preserve logical boundaries until independent release/compile boundaries have demonstrated value.

### Make evidence relevance executable

Git blob identity allows M0 to prove that evidence can depend on a specific relevant input rather than an entire repository HEAD.

## Alternatives considered

### YAML for S2

Deferred. It is more pleasant for some hand authoring, but JSON has a smaller ambiguity surface and a direct standard schema story for the first executable contract.

### UUIDv7

Rejected for M0 IDs. Sortability is not a semantic requirement and time-ordered identity would unnecessarily encode creation time.

### Graph database for S3

Rejected. M0 relationship volume does not justify adding a canonical-looking graph substrate, and the architecture forbids graph storage from becoming semantic destiny.

### Event store

Rejected. M0 requires historical semantics, not an event-sourced persistence model.

### Embedded Git library

Deferred. Native Git is simpler for proving source-fidelity first.

### Bun/TypeScript

Viable for a UI-oriented prototype, but M0's primary problem is a local semantic compiler/resolver with an embedded derived store. Rust provides the tighter initial executable/library boundary for that slice.

## Consequences

Positive:

- local single-binary path;
- strongly typed semantic kernel;
- portable S2;
- disposable S3;
- exact Git object identity;
- direct acceptance-test mapping.

Costs:

- JSON is verbose for hand-authored records;
- invoking Git creates a process boundary;
- SQLite schema will initially be intentionally generic;
- a Rust toolchain is required for contributors building from source.

## Reconsideration triggers

Revisit this decision if M0 shows that:

- JSON authoring cost overwhelms progressive adoption;
- SQLite cannot support required semantic queries without awkward duplication;
- native Git invocation prevents necessary observation performance or fidelity;
- Rust materially slows integration/plugin work relative to value; or
- the single-package boundary becomes a source of coupling rather than simplicity.
