# Portable Record Format v1

M0 persists Project Knowledge-owned semantic state as one JSON object per record.

## Location

```text
.pk/records/<kind>/<id>.json
```

The directory and filename are organizational locators, not semantic identity. Record identity comes from the `id` field.

## Envelope

Every v1 record contains:

```json
{
  "schema": "pk/v1",
  "kind": "subject",
  "id": "ed2c538d-9f2a-4f62-b5ef-863885c44d65"
}
```

`schema` identifies the logical record contract. `kind` selects the record variant. `id` is stable Project Knowledge identity for that record.

## Native references

Native source identity is never replaced by record identity:

```json
{
  "source_system": "git",
  "object_type": "blob",
  "locator": "src/model.rs",
  "state": "<git-blob-id>"
}
```

`locator` may be mutable. `state`, when present, identifies the immutable/reconstructable source state expected by the record.

## Claims and assertions

A Claim is a proposition independent of one source occurrence:

```json
{
  "schema": "pk/v1",
  "kind": "claim",
  "id": "...",
  "subject_id": "...",
  "concern": "lifecycle.status",
  "value": "closed"
}
```

An Assertion binds a source Representation to that Claim and may add valid-time/context/source-state semantics.

The separation is required so repeated/derived source occurrences do not become additional truth by repetition.

## Authority

Authority is explicit and scoped by `(subject_id, concern)` plus optional time/context.

Representation role does not imply authority.

If two applicable authority assignments lead to incompatible Claim identities, M0 resolves `conflict` rather than ranking them implicitly.

## Activities and freshness

Activity records declare native inputs and generated Representations.

Freshness compares only declared inputs with current source observations.

A missing input produces stale state when the previously identified input no longer exists. An unavailable/unsupported source produces unknown state.

## Evidence evaluations

Evidence records identify exactly one Claim and the native inputs the method evaluated.

A successful evaluation of C1 never creates evidence for C2 merely because the two Claims are related or stored near one another.

## Evolution

V1 is intentionally additive/extensible only through a new schema version or later namespaced extension design. Unknown top-level fields are rejected in M0 so accidental metadata does not silently acquire semantics.
