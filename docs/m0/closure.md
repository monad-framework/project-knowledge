# M0 Closure — Executable Project-Memory Kernel

## Status

**M0 complete, subject to merge of the closure-hardening change set.**

M0 was defined as an architecture falsification experiment, not as the first production release. Its purpose was to determine whether the selected federated portable-core architecture could be made executable while preserving the S1/S2/S3 state boundary and the domain invariants exercised by the architecture-entry scenarios.

## Closure decision

M0 **did not falsify** the selected architecture.

The implementation demonstrates that a small local executable can:

- observe an ordinary Git project without requiring Project Knowledge-owned semantic records;
- retain portable Project Knowledge-owned semantics separately from native source authority;
- compile those semantics and source observations into disposable SQLite-derived state;
- resolve scoped current state without truth-by-presence or truth-by-repetition;
- preserve historical valid-time semantics;
- retain context that differs from current source state;
- bind evidence and freshness to declared relevant inputs;
- return `unknown` when authority is insufficient; and
- delete and reconstruct S3 without changing the tested semantic result.

This validates the architecture **for the M0 scope**. It does not establish that the current model, schema, storage choices, or interfaces are sufficient for all future Project Knowledge use cases.

## Acceptance evidence

The clean-run acceptance suite covers the scenarios defined in `acceptance-plan.md`:

| Scenario | Result | What it demonstrates |
| --- | --- | --- |
| S-1 — Minimal project | PASS | Markdown + Git can participate with zero S2 semantic records. |
| S-2 — Identity continuity | PASS | Subject identity can survive Representation relocation. |
| S-3 — Scoped authority | PASS | Explicit scoped authority defeats stale projection state without role/repetition heuristics. |
| S-4 — Historical correction | PASS | Old and corrected Claims can resolve correctly at different valid times without erasure. |
| S-5 — Context-dependent observation | PASS | Historical Context remains distinct from the current Git observation. |
| S-6 — Claim-relative evidence | PASS | Evidence is bound to C1 and its declared input; unrelated change does not invalidate it, relevant change does. |
| S-7 — Derived freshness | PASS | Projection freshness follows declared Activity inputs. |
| S-8 — Unknown | PASS | Insufficient authority returns `unknown` rather than manufactured truth. |
| S-9 — Clean-room rebuild | PASS | Deleting and rebuilding SQLite S3 preserves the tested semantic resolution. |

The clean runner also passes:

- `cargo fmt --check`;
- `cargo clippy --locked --all-targets --all-features -- -D warnings`;
- `cargo test --locked --all-targets --all-features`;
- JSON Schema validation;
- rejection of unsupported/unknown record kinds through the schema/validation tests; and
- the full M0 acceptance suite.

## State-boundary result

M0 preserves the architecture's three state classes in executable form:

```text
S1 — native authoritative state
       Git / repository artifacts
             │ observations
             ▼
S2 — portable Project Knowledge semantic state
       .pk/records/**/*.json
             │ compile
             ▼
S3 — disposable derived state
       .pk/cache/read-model.sqlite3
```

The SQLite database can be destroyed and rebuilt. It is not the sole authoritative home of Project Knowledge semantics. The Git adapter observes native state but does not acquire authority merely by importing it. Portable S2 records remain inspectable and source-control friendly.

## Defects discovered during closure

The post-merge audit of PR #9 was important because merge status alone did not satisfy the M0 exit criterion.

The audit found and corrected four classes of implementation/process defect:

1. **Temporary CI repair behavior leaked into the merged branch.** The workflow had `contents: write`, ran `cargo fmt` rather than `cargo fmt --check`, and retained a one-off diagnostic workflow. The closure change restores read-only CI and removes diagnostic automation.
2. **Clippy lint in freshness ordering.** `sort_by` was replaced with the equivalent `sort_by_key` form required by the strict lint gate.
3. **CLI ownership bug.** Command dispatch partially moved `cli.command`, preventing later borrowing of `cli`. Dispatch now borrows the command and copies only Copy values where needed.
4. **Unpinned dependency resolution.** The executable originally lacked `Cargo.lock`. M0 now checks in the resolved graph and CI verifies with `--locked`.

These defects did not require changing the S1/S2/S3 architecture or the semantic model. They were implementation and verification defects exposed by the closure discipline.

## What M0 has not proven

M0 deliberately does not establish production readiness. In particular, it does not yet validate:

- production authorization and cross-source access enforcement;
- remote or heterogeneous source connectors beyond Git;
- multiple collaborating authors, review, consensus, or concurrent semantic edits;
- large-scale incremental synchronization or query performance;
- retention, deletion, privacy, and sensitive-context policy;
- production-grade provenance breadth;
- graph, semantic, or full-text retrieval at project scale;
- automatic capture/enrichment ergonomics;
- rich epistemic workflows for competing hypotheses and semantic evolution;
- a production UI, IDE integration, wiki projection, service API, or hosted operation; or
- whether real users find the semantic model worth the capture burden.

Those remain future evidence questions rather than implied M0 successes.

## Architecture conclusion

The principal M0 conclusion is therefore narrower and stronger than “the architecture works”:

> The federated portable-core architecture is internally coherent enough to implement, rebuild, and exercise against its first falsification scenarios without violating the tested domain invariants or imposing semantic records on the minimal-project case.

That is sufficient to continue the project without reopening the architecture merely because it was previously theoretical.

## Recommended next phase — dogfooding

The next step should **not** be immediate feature expansion.

Project Knowledge should first use M0 on Project Knowledge itself and then on representative Monad material. The goal is to discover whether the executable model solves the original recovery problem under real capture and retrieval pressure.

The next phase should test questions such as:

- Which real project facts are worth promoting from ordinary artifacts into S2?
- How much authoring friction do Subject, Representation, Claim, Authority, Context, and Evidence records create?
- Which records can be inferred or suggested safely without fabricating structure?
- Which current-state and historical recovery queries are actually useful in daily engineering work?
- Does the same Subject naturally accumulate representations across docs, Git, work tracking, and implementation?
- Where does the M0 record vocabulary feel too coarse, too verbose, or missing semantics?
- Can Project Knowledge detect or explain its own documentation/projection drift?
- What information remains difficult to recover even after using M0?

Only evidence from that dogfooding pass should authorize the next capability expansion.

## Closure rule

M0 is considered closed when this closure-hardening branch is merged with its final strict CI run green. Any later discoveries may reopen individual architecture or domain decisions through normal evidence and ADR processes, but they do not retroactively erase the M0 result.
