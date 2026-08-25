# Evidence-to-Capability Trace Matrix

This matrix is the compact audit trail for the candidate capability registry.

It should be read with [`candidate-capabilities.md`](candidate-capabilities.md), which contains the detailed rationale and counterpressure for each row.

| Capability | Corpus evidence | Failure / constraint | User jobs | Existing coverage credited | Residual disposition |
| --- | --- | --- | --- | --- | --- |
| `CAP-001` Preserve native artifact authority/history | PKC-0001, 0008, 0010, 0011 | FM-002, 004, 016 | UJ-002, 003, 014 | Git, ADRs, docs, issues | **REUSE** |
| `CAP-002` Federate heterogeneous native artifacts | PKC-0001, 0002, 0003, 0011 | FM-003, 014 | UJ-003, 006, 008, 011, 013 | Native APIs, links, integrations | **INTEGRATE** |
| `CAP-003` Stable semantic identity | PKC-0001, 0010 | FM-004, 012, 014 | UJ-001, 003, 006, 013 | Native IDs, ADR IDs, graph identity | **NEW** |
| `CAP-004` Representation roles/bindings | PKC-0001, 0002, 0004, 0008, 0010 | FM-001, 006, 015 | UJ-001, 006, 010 | PROV alternate/specialization/derivation | **EXTEND** |
| `CAP-005` Claim/property-scoped authority | PKC-0001, 0002, 0003, 0008 | FM-001, 002, 011, 015 | UJ-001, 006, 009, 013 | Local tool authority only | **NEW** |
| `CAP-006` Structured provenance | PKC-0003, 0004, 0006, 0007 | FM-006, 008, 009, 010 | UJ-005, 007, 010, 014 | W3C PROV | **EXTEND** |
| `CAP-007` Valid vs recorded time | PKC-0002, 0005, 0007, 0008, 0009 | FM-002, 007, 009, 016 | UJ-001, 002, 004, 014 | Bitemporal models, Git history | **EXTEND** |
| `CAP-008` Material observation/execution context | PKC-0003, 0004, 0006, 0007 | FM-003, 009, 010, 012 | UJ-005, 007, 008, 009 | Git/CI/execution/provenance metadata | **EXTEND** |
| `CAP-009` Epistemic state/evolution | PKC-0009 | FM-002, 013, 016 | UJ-002, 004, 009, 014 | ADR lifecycle, versions, PROV revision | **NEW / Medium** |
| `CAP-010` Claim-relative evidence | PKC-0005, 0006, 0007 | FM-007, 008, 009, 010 | UJ-005, 007, 009, 014 | Test records + provenance | **NEW** |
| `CAP-011` Derivation lineage/freshness | PKC-0001, 0002, 0004, 0008 | FM-001, 006, 015 | UJ-001, 006, 010, 013 | PROV, projections, dependency invalidation | **EXTEND** |
| `CAP-012` Typed relationships/impact traversal | PKC-0001 + corpus-wide | FM-014 | UJ-003, 004, 008, 013 | Graphs, issues, ADR links, backlinks | **INTEGRATE** |
| `CAP-013` Source/admission classification | PKC-0004 (+ PKC-0011 constraint) | FM-005, 006 | UJ-005, 010, 011 | Ignore rules, manifests, repository policy | **EXTEND / Medium** |
| `CAP-014` Current + historical views | PKC-0002, 0008, 0009 | FM-001, 002, 013, 016 | UJ-001, 002, 004, 011, 014 | Git, temporal models, docs, projections | **INTEGRATE** |
| `CAP-015` Contradiction diagnosis | PKC-0002, 0003, 0006, 0007, 0008 | FM-001, 002, 003, 008, 011, 012 | UJ-009 | Diff/search only solve surface mismatch | **NEW** |
| `CAP-016` Hybrid semantic retrieval | PKC-0001, 0011 + corpus-wide | FM-014 | UJ-001–013, esp. UJ-011 | Lexical/semantic/hybrid search, RAG | **INTEGRATE** |
| `CAP-017` Traceable authored narrative | Corpus history + UJ-012 motivation | FM-013, 014, 016 | UJ-012 | Docs/wikis/linked notes | **INTEGRATE / Medium** |
| `CAP-018` Progressive formalization/selective retention | PKC-0008, 0011 | Counterpressure to FM-014 and overcapture | UJ-011, 012, 013 | Linked notes, optional properties, progressive PROV | **NEW constraint** |
| `CAP-019` Causal/recovery path reconstruction | PKC-0004, 0005, 0006, 0007 | FM-013, 016 | UJ-003, 004, 008, 012 | Graph traversal + history + ADR rationale | **EXTEND / Medium** |
| `CAP-020` Preserve correction without erasure | PKC-0006, 0008, 0009 | FM-002, 008, 016 | UJ-004, 005, 014 | Git + temporal + provenance + epistemics | **EXTEND** |

## Coverage by user job

| User job | Candidate capabilities that directly support it |
| --- | --- |
| `UJ-001` Recover current truth | CAP-003, 004, 005, 007, 011, 014, 016 |
| `UJ-002` Recover historical truth | CAP-001, 007, 009, 014, 020 |
| `UJ-003` Explain why artifact/implementation exists | CAP-001, 002, 003, 012, 019 |
| `UJ-004` Explain why something changed | CAP-007, 009, 012, 014, 019, 020 |
| `UJ-005` Reconstruct provenance | CAP-006, 008, 010, 013, 020 |
| `UJ-006` Compare representations | CAP-002, 003, 004, 005, 011, 012 |
| `UJ-007` Determine evidence validity | CAP-006, 008, 010 |
| `UJ-008` Reconstruct work episode | CAP-002, 008, 012, 019 |
| `UJ-009` Diagnose contradiction | CAP-005, 007, 008, 009, 010, 015 |
| `UJ-010` Trace derived artifact | CAP-004, 006, 011, 013 |
| `UJ-011` Return after context loss | CAP-002, 014, 016, 018 |
| `UJ-012` Learn process as narrative | CAP-017, 018, 019 |
| `UJ-013` Assess change impact | CAP-002, 003, 005, 011, 012, 016, 018 |
| `UJ-014` Preserve correction without erasure | CAP-001, 006, 007, 009, 010, 014, 020 |

## Coverage by failure mode

The trace reveals four useful clusters rather than twenty independent problems.

### Cluster A — Identity, representation, and authority

- `FM-001`, `FM-004`, `FM-011`, `FM-015`
- Primary capabilities: `CAP-002` through `CAP-005`, plus `CAP-012`

### Cluster B — Time, provenance, context, and evidence

- `FM-002`, `FM-003`, `FM-007` through `FM-010`, `FM-012`
- Primary capabilities: `CAP-006` through `CAP-011`, plus `CAP-020`

### Cluster C — Ingestion, derivation, and project-scale relationship burden

- `FM-005`, `FM-006`, `FM-014`
- Primary capabilities: `CAP-002`, `CAP-011` through `CAP-013`, `CAP-016`, `CAP-018`

### Cluster D — Explanation and preserved learning

- `FM-013`, `FM-016`
- Primary capabilities: `CAP-009`, `CAP-014`, `CAP-017`, `CAP-019`, `CAP-020`

## Important negative result

The matrix does **not** support a design in which all twenty capabilities become twenty independent subsystems.

Several user-facing capabilities are compositions of a smaller semantic kernel. For example:

```text
CAP-015 Contradiction diagnosis
    consumes
        CAP-003 semantic identity
        CAP-004 representation roles
        CAP-005 scoped authority
        CAP-006 provenance
        CAP-007 time
        CAP-008 context
        CAP-009 epistemic state
        CAP-010 evidence
```

Similarly:

```text
CAP-014 current/historical views
CAP-016 retrieval
CAP-017 narrative
CAP-019 recovery paths
CAP-020 correction views
```

are primarily projections over underlying project-memory semantics.

This is a key guardrail for later domain modeling and architecture.
