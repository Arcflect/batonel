# Artifact

## Overview

In Archflow, an **Artifact** is the smallest planned implementation unit.

An artifact represents a concrete thing that should exist in the project structure,
such as a file, module boundary, handler, use case, entity, repository port, or gateway.

Artifacts are the primary execution units that Archflow scaffolds, documents, and prepares
for human or AI implementation.

---

## Purpose

The purpose of an artifact is to bridge architecture and implementation.

An artifact answers questions such as:

- What should be created?
- Where should it live?
- What role does it play?
- What module does it belong to?
- What should be handed to an implementer?

Artifacts make architecture operational.

---

## Responsibilities

An artifact is responsible for defining:

- its identity
- its role
- its module membership
- its planned location
- its expected inputs and outputs
- its implementation status

An artifact is not responsible for describing all behavioral rules in detail.
That belongs to the contract.

---

## Core fields

Typical fields include:

- `name`
- `module`
- `role`
- `path` (resolved or generated)
- `inputs` (optional)
- `outputs` (optional)
- `status` (optional)

---

## Example

```yaml
artifacts:
  - name: create_user
    module: user
    role: usecase
    inputs:
      - CreateUserCommand
    outputs:
      - CreateUserResult
```

---

## Relationship to other concepts

An artifact is shaped by:

- the project context
- placement rules
- role templates
- contracts
- prompts

An artifact is the unit that connects those concepts together.

Typical flow:

1. the project defines the architectural context
2. placement rules define where a role should live
3. the artifact identifies one concrete implementation unit
4. the contract defines its responsibilities and constraints
5. the prompt turns that contract into implementation handoff context

---

## Design principles

An artifact should be:

- concrete
- small enough to implement in isolation
- meaningful in the architecture
- stable enough to track through planning and execution
- easy to hand off to a human or lightweight model

Artifacts should avoid being too large or too vague.

---

## What an artifact should not do

An artifact should not:

- encode full business policy by itself
- replace the project definition
- replace the contract
- mix multiple unrelated responsibilities
- become so broad that it stops being a useful implementation unit

---

## Why it matters

Archflow is centered on artifact-level execution.

This is important because AI coding tools often perform best when the task is:

- narrow
- explicit
- bounded
- context-rich

Artifacts give Archflow a practical unit for:

- placement
- scaffolding
- contract generation
- prompt generation
- verification

---

## Examples of artifacts

Examples of artifacts include:

- user
- create_user
- user_repository
- create_user_handler
- postgres_user_repository

These are not only file names.
They are architectural execution units.

---

## Future directions

In the future, artifacts may also support:

- grouping or dependency references
- ownership metadata
- lifecycle state transitions
- artifact splitting recommendations
- mapping to generated code or existing repository files