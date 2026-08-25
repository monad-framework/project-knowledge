# Goals and Non-Goals

This document defines the current scope of Project Knowledge during inception. It should evolve as discovery produces evidence.

## Goals

### G1. Preserve project orientation

Make it easier for a person to recover the state, context, rationale, and important open questions of a complex engineering project after context has been lost.

### G2. Preserve evolving knowledge

Retain both current understanding and meaningful historical states without requiring obsolete reasoning to be deleted.

### G3. Represent multiple valid perspectives

Allow the same underlying information to participate in chronological, architectural, conceptual, work-oriented, causal, and educational views without uncontrolled duplication.

### G4. Preserve traceability

Support useful navigation among intent, requirements, constraints, evidence, decisions, implementation, verification, and outcomes.

### G5. Preserve provenance and status

Make it possible to know where important knowledge came from and whether it is tentative, accepted, superseded, verified, disputed, or otherwise qualified.

### G6. Reduce maintenance burden

Avoid a system in which every useful projection requires a separately maintained copy of the same information.

### G7. Support progressive structure

Permit quick capture of incomplete information and later refinement into richer, more strongly related knowledge.

### G8. Remain intelligible to humans

The system must improve human comprehension rather than merely produce a sophisticated machine model.

### G9. Remain usable by software

Important project knowledge should be addressable and queryable by tools without requiring a manually synchronized shadow representation.

### G10. Learn from real projects

Use real engineering material, initially including Project Knowledge itself and relevant Monad development history, to derive and test the model.

### G11. Preserve engineering process for learning

Allow others to study how a real system evolved, including uncertainty, alternatives, failure, evidence, revision, and implementation.

## Non-goals

### NG1. Replacing Git

The project is not intended to replace version control or reproduce Git's responsibilities.

### NG2. Replacing every project-management tool

The project does not initially aim to become a complete issue tracker, planning suite, chat system, or source-code host.

### NG3. Building a universal ontology of knowledge

We are solving an engineering project-memory problem, not attempting to model all human knowledge.

### NG4. Perfect automatic understanding

The initial project does not require an AI system that can infer every relationship or classify every artifact correctly without human input.

### NG5. Capturing every transient interaction

Not every chat message, keystroke, terminal command, or thought deserves permanent representation. Discovery must determine what information has durable value.

### NG6. Selecting the implementation stack during inception

No database, graph engine, application framework, schema technology, or hosting platform is considered part of the problem definition.

### NG7. Making public publication mandatory

Although preserving engineering process can enable build-in-public workflows, the underlying system should also be useful for private projects.

### NG8. Requiring perfect structure at capture time

The project should not make recording knowledge so expensive that people avoid recording it.

### NG9. Treating documents as obsolete

Documents and narratives remain valuable. The project is exploring how they can coexist with richer underlying structure, not seeking to eliminate prose documentation.

## Scope test

A proposed feature belongs in the early project only if it materially helps us understand or validate the core project-memory problem. Features that are primarily conveniences, integrations, or presentation polish should wait until the information model and core workflows are better understood.
