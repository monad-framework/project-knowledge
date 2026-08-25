# PKC-0004 — Administrative metadata is misclassified as project source

## Source context

Project: Monad

Source:

- GitHub issue `#172` — `[Defect] sync-machine-docs indexes linked-worktree .git file`
- https://github.com/monad-framework/monad/issues/172

Observed during `WP-MVP-0001` / `EXEC-0001` in a linked Git worktree.

## Observed situation

Monad's machine-document synchronization excludes `.git` when it is a directory. In a linked Git worktree, however, `.git` is represented as a root-level file pointing to Git administrative state.

The discovery logic therefore classified that `.git` pointer as canonical UTF-8 project source and generated machine projections from it.

Equivalent project content produced different derived knowledge depending on the incidental filesystem representation of Git administrative metadata.

## Information involved

- source classification rules;
- filesystem representation;
- Git administrative metadata;
- canonical project content;
- derived machine projections;
- execution/worktree context;
- generated corpus/graph/manifest content; and
- validation results.

## Why this is difficult to organize

The case demonstrates that `what exists as a file` and `what belongs to project knowledge` are not equivalent.

Classification depends on semantic role, not merely format or location. A root-level text file may be project source in one case and tool metadata in another.

Derived representations amplify classification errors: once an incidental artifact is admitted as source, it can propagate into several machine projections and later appear authoritative simply because it has been indexed repeatedly.

## Candidate relationships

- filesystem artifact `has semantic role` administrative metadata;
- discovery rule `classifies` artifact;
- source artifact `produces` projection;
- projection `derived from` source artifact;
- worktree representation `differs from` canonical checkout representation;
- classification error `causes` projection contamination.

## Time, authority, provenance, and context

### Context

The decisive difference is linked-worktree representation: `.git` is a file rather than a directory.

### Provenance

Generated machine documents can be traced to the misclassified `.git` pointer, making provenance useful for explaining why contamination appeared.

### Authority

Generated projections should not create new semantic authority merely through replication.

## Recovery questions

- Why did `.git` appear in the machine corpus?
- Was it authored project content or administrative metadata?
- Which derived artifacts came from it?
- Why did the same synchronization behave differently in a normal checkout and a linked worktree?
- Which artifacts should be removed or regenerated after correcting classification?

## Provisional observations

1. Presence, parsability, and semantic membership are different properties.
2. Project-memory ingestion requires classification boundaries, not just file discovery.
3. Derived artifacts can magnify a single source-classification mistake.
4. Provenance can make contamination diagnosable, but provenance alone does not prevent bad classification.
5. Equivalent semantic project states should not diverge because of incidental tool-specific filesystem representation.

## Open questions

- How should source membership be declared, inferred, or excluded?
- Should project memory retain excluded administrative artifacts as contextual evidence without treating them as canonical source?
- How can derived knowledge retain lineage without allowing derivation count to imply authority?
