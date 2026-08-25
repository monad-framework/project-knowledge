# Explicit Non-Requirements and Deferred Concepts

These statements prevent discovery hypotheses, implementation conveniences, or attractive ideas from silently becoming requirements.

## NR-001 — Not a replacement for Git or source control

Project Knowledge is **not required** to implement version control, source checkout, merge, branching, commit storage, or repository synchronization.

It may integrate with Git or equivalent systems.

## NR-002 — Not a replacement for issue/work tracking

Project Knowledge is **not required** to implement a complete issue tracker, sprint planner, project-management suite, or workflow engine.

It may integrate work identity, state, relationships, and history from native work systems.

## NR-003 — Not a replacement for documentation or wiki authoring

Project Knowledge is **not required** to replace Markdown, docs-as-code, wikis, or other authored documentation systems.

## NR-004 — Not a replacement for ADR practice

Project Knowledge is **not required** to replace established decision-record practices. It may connect decisions into broader project memory.

## NR-005 — No universal ontology yet

This requirements phase does **not** define a universal engineering-knowledge ontology or fixed taxonomy of all object and relationship types.

## NR-006 — No semantic ID for every artifact

The system is **not required** to assign a Project Knowledge semantic identifier to every file, issue, commit, log entry, message, or source object.

## NR-007 — No universal supersession model yet

The system is **not required** to apply one generic supersession lifecycle to all engineering objects.

Decision supersession, artifact revision, correction, replacement, refinement, split, and merge may have different semantics and remain a domain-modeling question.

## NR-008 — No fixed universal epistemic state machine

The system is **not required** to force all knowledge through a single sequence such as question → hypothesis → experiment → decision → fact.

## NR-009 — No first-class experiment-management product yet

The system is **not required** to provide experiment design, execution, statistics, notebook, or laboratory-management workflows.

It may later integrate experiment artifacts or evidence.

## NR-010 — No automatic terminology-evolution machinery yet

Automatic tracking of renamed or semantically evolving project terminology is deferred pending stronger corpus evidence.

## NR-011 — No automatic causal inference requirement

Project Knowledge is **not required** to infer causality automatically from chronology, commits, links, embeddings, or dependency structure.

## NR-012 — No automatic authoritative conflict resolution

Project Knowledge is **not required** to decide unresolved human or organizational disagreements automatically.

## NR-013 — No mandatory conversation/chat capture

Projects are **not required** to capture every ChatGPT conversation, Slack message, email, meeting transcript, or informal discussion into project memory.

Selected conversational information may be promoted into durable artifacts when useful.

## NR-014 — No mandatory event sourcing

The requirements do **not** require an event-sourced persistence architecture.

## NR-015 — No mandatory graph database

The requirements do **not** require graph storage, RDF, a property-graph database, or a graph query language.

## NR-016 — No mandatory temporal database

The requirements do **not** require a system-versioned or bitemporal database.

## NR-017 — No mandatory AI or RAG

The requirements do **not** require an LLM, embedding model, vector database, RAG pipeline, autonomous agent, or AI provider.

## NR-018 — No mandatory hosted service

The requirements do **not** require Project Knowledge to be cloud-hosted, multi-tenant, SaaS, or dependent on an always-on remote service.

Deployment architecture remains open.

## NR-019 — No mandatory single canonical storage repository

The requirements do **not** require all integrated project-memory information to be physically copied into one repository or database.

## NR-020 — No universal indefinite retention

The system is **not required** to retain every historical or transient piece of project information forever.

Retention policy, archival depth, and privacy constraints require later design.

## NR-021 — No automatic truth from generated summaries

Generated summaries, synthesized answers, or AI-created narratives are **not** inherently authoritative project knowledge.

## NR-022 — No replacement of native build/test/CI execution

Project Knowledge is **not required** to become the system that compiles code, runs tests, executes CI pipelines, or produces primary validation evidence.

It may integrate the resulting evidence and context.

## NR-023 — No prescribed user interface

The requirements do **not** mandate a wiki, desktop app, web app, graph explorer, IDE extension, CLI, chat interface, or dashboard as the primary user experience.

## NR-024 — No prescribed programming language or framework

The requirements do **not** select any implementation language, runtime, package manager, frontend framework, backend framework, schema language, or deployment platform.

## Reconsideration rule

A non-requirement may be promoted later only when new evidence creates a traceable need.

Promotion should follow the same discipline used for the current requirements:

```text
new evidence
  ↓
blocked user job / failure mode
  ↓
existing approach gap
  ↓
candidate capability
  ↓
requirement
```
