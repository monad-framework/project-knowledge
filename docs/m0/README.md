# M0 — Executable Project-Memory Kernel

M0 is the first executable vertical slice of Project Knowledge.

It exists to falsify or validate the architecture selected in `ADR-0001`, not to implement the eventual product surface.

## M0 question

> Can a small local executable preserve the S1/S2/S3 boundary, compile portable semantic records into disposable derived state, and satisfy the eight architecture-entry scenarios without imposing semantic ceremony on a native-only project?

## Selected concrete stack

- **Language/runtime:** Rust 1.98 / edition 2024
- **Portable S2 serialization:** JSON
- **Portable schema:** JSON Schema Draft 2020-12
- **Stable generated identifiers:** UUIDv4
- **Derived S3 read model:** SQLite through `rusqlite`, bundled for the executable
- **Native VCS adapter:** invocation of the installed `git` executable
- **CLI:** `clap`
- **Time parsing:** RFC3339 timestamps parsed with Jiff

These are M0 implementation decisions, not irreversible product constraints.

## Repository shape

```text
Cargo.toml
rust-toolchain.toml
schemas/
  v1/
    record.schema.json
src/
  lib.rs
  model.rs
  records.rs
  git.rs
  compiler.rs
  store.rs
  resolver.rs
  main.rs
tests/
  acceptance.rs
  schema_validation.rs
```

The first implementation deliberately uses one Cargo package with a library and a `pk` binary. Module boundaries correspond to architectural responsibilities but are not separate crates yet.

## State boundaries

### S1 — native source state

M0 ships one source adapter: Git.

The adapter observes native state and returns source observations. It never writes Git history and never declares imported observations to be globally authoritative.

### S2 — portable semantic records

Project Knowledge-owned semantic records live under:

```text
.pk/records/<kind>/<uuid>.json
```

Each record:

- declares `schema: "pk/v1"`;
- has a stable UUID;
- is validated against the checked-in JSON Schema;
- remains human-inspectable and source-control friendly; and
- contains only Project Knowledge-owned cross-system semantics.

A project does not need `.pk/records/` merely to be observed by `pk status`.

### S3 — derived read model

The default derived database is:

```text
.pk/cache/read-model.sqlite3
```

It is disposable. `pk rebuild` deletes and reconstructs it from S1 observations plus S2 records.

## M0 commands

```text
pk init
pk validate
pk compile
pk rebuild
pk status
pk resolve --subject <uuid> --concern <concern>
pk freshness --representation <uuid>
pk evidence --evaluation <uuid>
```

All commands accept `--root <path>` and `--json`.

## M0 record kinds

The first schema supports:

- `subject`
- `representation`
- `claim`
- `assertion`
- `authority`
- `relationship`
- `activity`
- `context`
- `evidence_evaluation`

This is a vertical-slice vocabulary. It is intentionally smaller than the complete domain model and remains extensible.

## Current-state resolver

M0 resolves one `(Subject, concern)` at a requested valid-time/context from:

```text
Claim
  + source-bound Assertion
  + applicable Authority Assignment
  + valid-time window
  + optional Context
```

Valid outcomes are:

- `resolved`
- `compatible`
- `conflict`
- `unknown`

No search score, record count, import order, or representation role is a tie-breaker.

## Freshness and evidence

Derived freshness is based only on declared Activity inputs.

Evidence freshness is based only on declared Evidence Evaluation inputs. For Git-backed inputs M0 can use blob identity, so changing an unrelated file does not invalidate evidence bound to a different blob.

This is the executable form of proposition/input-relative freshness from the requirements.

## Acceptance gate

`tests/acceptance.rs` implements:

1. minimal Markdown + Git without S2 records;
2. Subject continuity across relocation;
3. scoped authority versus stale projection;
4. historical correction;
5. context-dependent source state;
6. claim-relative evidence;
7. derived projection freshness;
8. first-class unknown resolution;
9. clean-room S3 rebuild equivalence.

M0 is complete only when CI passes these scenarios plus schema-validation tests.

## Deliberate exclusions

M0 does not implement:

- a production UI;
- network/server operation;
- remote connectors;
- collaboration/consensus workflows;
- a graph database;
- RAG/embeddings/LLMs;
- generalized inference;
- automatic conflict resolution;
- generalized policy language;
- optimized query schemas; or
- production authorization enforcement.

Those remain later work unless M0 proves the architecture itself insufficient.
