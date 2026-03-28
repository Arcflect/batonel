# Decisions

This directory contains architectural and product decisions for Archflow.

The purpose of these records is to preserve important reasoning over time.

Archflow includes many design choices that may otherwise become unclear later, such as:

- what the core concepts mean
- what is considered source of truth
- how prompts relate to contracts
- how examples relate to presets
- how much Archflow should depend on code-aware analysis

These decision records help contributors understand not only **what** the project does,
but also **why** it was designed this way.

---

## Why this directory exists

Archflow is concept-heavy.

That means important project decisions are not only implementation details.
They also include:

- model boundaries
- terminology choices
- workflow priorities
- sequencing decisions
- tradeoffs between flexibility and strictness

Without decision records, these choices can drift into:

- scattered discussions
- implicit maintainer memory
- inconsistent documentation
- implementation guesses

This directory exists to reduce that drift.

---

## How to read these files

Each decision file should explain:

- the context
- the decision
- the consequences

Start with the earliest decision if you want to understand the project direction from the beginning.

Decision files are numbered in roughly chronological order.

---

## File naming convention

Use this pattern:

`NNNN-short-kebab-case-title.md`

Examples:

- `0001-archflow-is-an-architecture-to-execution-bridge.md`
- `0002-contract-is-the-source-of-truth-for-artifact-boundaries.md`

The number keeps ordering stable.
The title keeps the decision readable in repository views.

---

## Suggested status values

A decision file may include one of these status values:

- `proposed`
- `accepted`
- `superseded`
- `deprecated`

If a decision is replaced later, do not delete the old file.
Instead:

- keep the old file
- mark it as superseded
- add a note pointing to the newer decision

This preserves project history.

---

## When to create a new decision

Create a decision record when a change affects:

- project direction
- conceptual model
- source of truth
- workflow sequencing
- schema philosophy
- preset philosophy
- prompt behavior
- verification scope
- major implementation tradeoffs

Do not create a decision file for every minor edit.

A good rule is:

Create a decision record when future contributors are likely to ask,
“Why was it done this way?”

---

## Relationship to other docs

Decision files are different from:

- concept docs  
  These explain what a concept means.

- schema docs  
  These explain structure and fields.

- roadmap docs  
  These explain what should happen next.

Decision files explain why important choices were made.

---

## Current initial decisions

The initial set of decisions focuses on core Archflow direction:

- Archflow is an architecture-to-execution bridge
- Contracts are the source of truth for artifact boundaries
- Prompts are derived from contracts
- Sidecar files are first-class
- Examples come before presets

These decisions establish the early shape of the project.

---

## Summary

This directory helps Archflow preserve intent over time.

If you remember only one thing, remember this:

**documentation explains the model, but decision records explain why the model exists in its current form**