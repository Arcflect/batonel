# Contract

## Overview

In Archflow, a **Contract** defines the responsibilities and constraints of an artifact.

A contract explains what an artifact is supposed to do, what it must not do,
what dependencies it may rely on, and what outputs it is expected to provide.

The contract is the main mechanism Archflow uses to preserve architectural intent
during implementation.

---

## Purpose

The purpose of a contract is to make artifact behavior explicit before implementation begins.

A contract answers questions such as:

- What is this artifact for?
- What are its responsibilities?
- What must it never do?
- What dependencies are allowed?
- What boundaries must be preserved?
- What does “done” mean for this artifact?

This makes the architecture usable in day-to-day implementation.

---

## Responsibilities

A contract is responsible for defining:

- responsibilities
- prohibited behavior
- allowed dependencies
- forbidden dependencies
- expected inputs and outputs
- implementation size or scope
- completion intent
- status

A contract is not responsible for deciding where the artifact lives.
That belongs to placement rules and the artifact definition.

---

## Core fields

Typical fields include:

- `name`
- `module`
- `role`
- `path`
- `responsibilities`
- `must_not`
- `allowed_dependencies`
- `forbidden_dependencies`
- `inputs` (optional)
- `outputs` (optional)
- `implementation_size` (optional)
- `status`

---

## Example

```yaml
name: create_user
module: user
role: usecase
path: src/application/usecases/create_user.rs

responsibilities:
  - "Execute one application use case"
  - "Coordinate domain behavior"
  - "Persist through an abstract repository boundary"

must_not:
  - "Write SQL directly"
  - "Return transport-specific responses"

allowed_dependencies:
  - "domain"
  - "application"

forbidden_dependencies:
  - "interfaces"
  - "infrastructure"

inputs:
  - "CreateUserCommand"

outputs:
  - "CreateUserResult"

implementation_size: "small"
status: "planned"
```

---

## Relationship to other concepts

A contract belongs to an artifact.

- the project provides context
- the artifact identifies the execution unit
- the contract defines its behavioral boundary
- the prompt turns the contract into AI-ready implementation context

The contract is the most important architectural safeguard in Archflow.

---

## Design principles

A contract should be:

- explicit
- specific
- small enough to be actionable
- understandable by both humans and AI tools
- stable enough to guide implementation
- independent from unnecessary framework details

A good contract should reduce ambiguity, not add it.

---

## What a contract should contain

A good contract should describe:

- what the artifact is responsible for
- what it must not do
- which layer or dependencies it may interact with
- what kind of inputs and outputs are expected
- how large or focused the implementation should be

---

## What a contract should not do

A contract should not:

- turn into a full design document
- include unrelated architectural discussion
- contain detailed source code
- encode every possible implementation decision
- become so vague that it stops constraining behavior

---

## Why it matters

Without contracts, architecture often exists only in:

- diagrams
- docs
- team memory
- review comments

That makes implementation drift likely, especially when using lightweight AI models.

Contracts give Archflow a way to preserve architecture at the artifact level.

They are useful for:

- scaffold generation
- human onboarding
- AI handoff
- review alignment
- future verification

---

## Contract quality guidelines

A strong contract usually has:

- 1 to 5 clear responsibilities
- explicit forbidden behavior
- clear dependency boundaries
- a realistic implementation scope
- language that can be turned into a prompt without major rewriting

A weak contract usually has:

- vague statements
- overlapping responsibilities
- no real constraints
- hidden assumptions
- too much implementation detail

---

## Future directions

In the future, contracts may also support:

- machine-readable validation rules
- required acceptance checks
- optional verification hints
- contract inheritance from templates
- repository-specific policy extensions