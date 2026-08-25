# M0 Technology Decisions

These choices optimize for architecture validation, portability, low operational burden, and reversibility.

## Rust 1.98 / edition 2024

Rust is selected for the executable kernel because M0 needs:

- a local single executable;
- strong type boundaries around semantic records;
- predictable error handling;
- straightforward SQLite integration;
- direct native-process integration with Git; and
- a credible path to a reusable library plus CLI.

M0 uses one package rather than a multi-crate workspace. Splitting modules into crates before the seams prove independently useful would add repository structure without increasing semantic confidence.

## JSON as the first portable S2 syntax

JSON is selected over YAML/TOML/custom syntax for M0 because:

- the data model is unambiguous;
- JSON Schema is mature and implementation-neutral;
- virtually every future implementation language can consume it;
- records remain inspectable and diffable;
- Serde support is mature; and
- schema validation does not depend on Rust types.

The semantic model is not defined by Rust serialization. The checked-in schema is the portable contract.

M0 does not claim JSON is the only future authoring syntax. A future YAML or richer authoring layer may compile into the same logical S2 record model.

## JSON Schema Draft 2020-12

A checked-in schema provides a language-neutral validation boundary before Rust deserialization.

Validation occurs in two stages:

1. structural JSON Schema validation;
2. semantic/cross-reference validation in the core library.

This keeps schema syntax responsible for document shape while domain invariants remain explicit code and tests.

## UUIDv4 identifiers

UUIDv4 is used for Project Knowledge-owned stable IDs because M0 needs uniqueness without central allocation and does not need ordering semantics in identifiers.

Using UUIDv4 avoids making creation time part of semantic identity. Native identifiers remain preserved separately.

## SQLite for disposable S3

SQLite is selected only for the derived read model.

It is not the S2 source of truth.

The M0 database intentionally begins with a generic records table plus source-observation table. This minimizes persistence assumptions while still proving:

- local operation;
- deletion/rebuild;
- query-time semantic resolution; and
- separation between portable canonical records and optimized derived state.

Normalization/index specialization should follow measured query pressure, not precede it.

## Native Git process adapter

M0 invokes the installed `git` executable rather than embedding a Git implementation.

Reasons:

- Git remains the native authority;
- M0 can use the same object identity users and CI already use;
- blob identity (`HEAD:path`) gives a precise primitive for relevant-input freshness;
- no competing checkout/history semantics are introduced; and
- the adapter remains small enough to audit.

Later architecture may add libgit-based acceleration if it preserves identical source semantics.

## RFC3339 timestamps

Portable records use RFC3339 strings.

The Rust implementation parses them into precise timestamps for comparison but does not expose an implementation-specific binary time representation in S2.

## Dependency policy

M0 keeps dependencies focused on one architectural responsibility each:

- `serde` / `serde_json` — portable records;
- `jsonschema` — schema validation;
- `uuid` — generated IDs;
- `jiff` — timestamp validation/comparison;
- `rusqlite` — disposable read model;
- `clap` — CLI parsing;
- `thiserror` — library errors;
- `tempfile` — isolated acceptance fixtures.

Dependency count is not minimized at the expense of reimplementing mature infrastructure, but dependencies that would introduce a new architectural subsystem are deferred.

## Reproducible dependency resolution

M0 is an executable application, so the resolved Rust dependency graph is checked in as `Cargo.lock`.

`Cargo.toml` continues to express the direct dependency requirements, while `Cargo.lock` pins the concrete direct and transitive versions used by the repository build. CI runs Clippy and tests with `--locked`, so an unnoticed dependency-resolution change cannot silently alter an otherwise identical M0 verification run.

Lockfile changes are therefore intentional reviewable changes. They should be regenerated and committed when dependencies are deliberately updated, not recreated implicitly during ordinary CI.
