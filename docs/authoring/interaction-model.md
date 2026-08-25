# Interaction Model

## Participants

### Human author

Owns semantic decisions. The human may provide them interactively or through an authoring-intent document.

### Capture Planner

A read-only component that:

- loads the existing S2 corpus;
- discovers existing Subjects and Representations;
- inspects selected native Git paths/objects;
- generates mechanical record fields;
- detects ambiguity and missing semantic input;
- constructs a reviewable Capture Plan; and
- records the origin of material plan fields.

The Planner does not write S2 records.

### Capture Applier

A mutation component that:

- verifies plan preconditions;
- constructs the prospective complete S2 corpus;
- runs schema, semantic, and cross-reference validation before finalizing the write set;
- writes new records to their standard locations; and
- returns the created paths and resulting validation state.

### Existing compiler/resolver

Remains unchanged in conceptual responsibility. Capture is upstream authoring assistance, not a parallel semantic engine.

## State machine

```text
START
  │
  ▼
COLLECT_INTENT
  │
  ├── semantic ambiguity ──► REQUIRE_EXPLICIT_CHOICE
  │                              │
  │                              └──► COLLECT_INTENT
  ▼
RESOLVE_EXISTING_REFERENCES
  │
  ├── zero matches when existing required ─► ERROR
  ├── multiple plausible matches ──────────► REQUIRE_EXPLICIT_CHOICE
  ▼
OBSERVE_RELEVANT_NATIVE_STATE
  │
  ▼
BUILD_PLAN
  │
  ▼
REVIEW
  │
  ├── reject/edit ─► COLLECT_INTENT / ABORT
  ▼
APPLY_REQUESTED
  │
  ▼
RECHECK_RELEVANT_PRECONDITIONS
  │
  ├── changed ─► STALE_PLAN
  ▼
VALIDATE_PROSPECTIVE_CORPUS
  │
  ├── invalid ─► ERROR_WITHOUT_SEMANTIC_WRITE
  ▼
WRITE_RECORDS
  │
  ▼
VALIDATE_RESULT
  │
  ▼
COMPLETE
```

## Field-origin classes

Every material field in a Capture Plan must be classifiable as one of:

- **authored** — explicitly provided or confirmed by the human;
- **generated** — mechanical structure created by `pk`, such as UUID or destination path;
- **observed** — copied from an external/native source observation, such as current Git blob identity;
- **suggested** — proposed by the tool but not yet human-confirmed.

A `suggested` field that affects semantics may not be applied until it becomes `authored` through explicit confirmation.

## Interaction modes

### Guided interactive mode

Designed for direct human use. It should present semantic questions in domain language rather than asking the user to edit JSON or provide UUIDs.

Examples:

- “Use an existing Subject or create a new one?”
- “Which file represents this Subject?”
- “What role does this Representation play?”
- “What concern does this Claim address?”
- “What value is being claimed?”
- “Does this Representation have authority for that concern?”
- “What is the basis of that authority?”
- “When did this become valid?”
- “Which Claim does this evidence evaluate?”

The wizard may display candidate existing records, but ambiguity must be resolved by explicit selection.

### Non-interactive mode

Consumes a compact Authoring Intent document. The document uses local aliases to avoid UUID plumbing but still requires semantic fields.

If required semantic input is absent, non-interactive mode fails rather than selecting a semantic default.

## Review requirements

Before apply, human-readable review must show at minimum:

- records to be created;
- existing records to be referenced;
- Subjects and Representations involved;
- Claims and their values;
- authority scope and basis;
- valid-time intervals that were explicitly supplied;
- evidence target Claim and evidence inputs;
- generated identifiers and destination paths;
- relevant native states captured;
- warnings/ambiguities; and
- the authored/generated/observed origin distinction.

`--json` must expose the same plan structure for programmatic review.

## No project-wide transaction assumption

A Capture Plan concerns one authored semantic bundle. It must not assume ownership of the whole project-memory corpus.

That rule follows directly from DF-002, where a DF-001 test incorrectly assumed the repository would forever contain only its original ten records.
