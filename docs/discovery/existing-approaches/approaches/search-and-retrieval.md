# Search, Semantic Retrieval, and RAG-Style Access

Project Knowledge exists partly because users cannot afford to reread an entire project every time they lose context. Retrieval is therefore essential—but the discovery corpus shows that **retrieval and truth are different problems**.

## Lexical search

Lexical/full-text search is especially strong for exact engineering tokens:

- stable IDs (`ADR-0005`, `WP-MVP-0001`);
- function/type names;
- filenames and paths;
- diagnostic strings;
- exact terminology; and
- quoted text.

This is often more precise and explainable than semantic similarity for identifier-heavy engineering work.

## Semantic/vector search

Semantic retrieval can find conceptually related material even when terminology differs.

Examples:

- “why did evidence become invalid?” may find documents discussing fingerprint freshness without those exact words;
- an older term can still retrieve a later concept after terminology evolves; and
- a user returning after months away can search by remembered meaning rather than repository location.

This strongly supports `UJ-011` and can reduce the navigation burden behind `FM-014`.

## Hybrid search

Modern search systems combine lexical and vector retrieval, often with rank fusion or reranking.

This is particularly attractive for engineering knowledge because the corpus contains both:

- exact symbolic identities; and
- fuzzy conceptual questions.

A good retrieval layer should not force a choice between them.

Conceptually:

```text
exact lexical candidates
          +
semantic candidates
          ↓
combined / reranked results
```

## RAG-style answer generation

Retrieval-augmented generation can summarize or synthesize retrieved project material into a response.

That could be valuable for questions such as:

- What changed while I was away?
- Why does this component exist?
- What decisions affect this subsystem?
- What remains unresolved?

But generation adds another layer whose claims need provenance back to the retrieved material.

## Critical limitation: retrieval is not authority

Suppose search returns three documents:

1. an old accepted design;
2. a stale GitHub projection; and
3. the current superseding decision.

A relevance score alone cannot safely tell the user which is authoritative.

Likewise, a vector model may retrieve two contradictory assertions because they are semantically similar.

Therefore:

```text
relevance ≠ truth
similarity ≠ identity
retrieval rank ≠ authority
```

This directly relates to `UJ-001`, `UJ-009`, `FM-001`, `FM-002`, and `FM-015`.

## Retrieval needs structured filters/context

Search becomes more useful when results can be constrained or annotated by Project Knowledge semantics such as:

- semantic object identity;
- current versus historical;
- artifact/representation role;
- authority scope;
- lifecycle or epistemic state;
- valid time;
- source repository/tool;
- provenance; and
- relation to the current work context.

This suggests a division of responsibility:

```text
Project-memory semantics
        ↓
retrieval constraints + result context
        ↓
lexical / semantic / hybrid search
        ↓
optional synthesis
```

rather than asking embeddings or an LLM to infer the entire truth model from prose.

## Search as a projection/index

Search indexes are naturally derived representations.

That implies they should carry enough lineage to answer:

- what source was indexed?
- at what source revision?
- when was the index built?
- which extraction/chunking process produced this record?
- is the result stale relative to canonical source?

This connects search directly to `UJ-010` and `FM-006`.

## Chunking caution

Semantic/RAG systems often operate on chunks rather than whole artifacts. Chunk identity should not be confused with semantic project identity.

A paragraph extracted from an ADR is:

- a retrievable representation of part of an ADR;
- not automatically an independent decision; and
- not automatically authoritative outside its parent artifact and temporal context.

This is another instance of the representation-versus-semantic-identity distinction in `O-001` and `O-002`.

## Provisional reuse direction

Strong candidates:

1. lexical search for exact engineering identity and terminology;
2. semantic retrieval for conceptual/context recovery;
3. hybrid ranking rather than vector-only search;
4. structured filters over semantic/temporal/authority metadata;
5. provenance from search hits back to canonical artifacts; and
6. generated answers that cite/reveal their supporting project sources.

## What not to do

Project Knowledge should not treat RAG as the underlying knowledge model.

Embedding all project documents and asking an LLM questions would improve retrieval, but it would leave the hardest corpus failures unresolved:

- stale authority;
- semantic identity;
- temporal truth;
- provenance distinction;
- claim-relative evidence;
- projection lineage; and
- explicit correction history.

Search is likely a powerful **access layer over project memory**, not project memory itself.
