# Prompt

## Overview

In Archflow, a **Prompt** is the AI handoff representation of an artifact contract.

A prompt translates project context, artifact identity, and contract constraints
into a form that can be directly given to an AI coding tool or lightweight model.

The prompt is not the source of truth.
The contract is the source of truth.
The prompt is a delivery format derived from that contract.

---

## Purpose

The purpose of a prompt is to make an artifact directly implementable by an AI system.

A prompt answers questions such as:

- What should the model implement?
- What is the role of this artifact?
- What must it do?
- What must it avoid?
- What dependencies are allowed?
- What should count as complete?

Prompts reduce the need for a human to restate the same architectural context every time.

---

## Responsibilities

A prompt is responsible for:

- packaging artifact context for implementation
- restating responsibilities clearly
- restating constraints clearly
- presenting expected inputs and outputs
- defining completion expectations
- making artifact contracts usable by AI systems

A prompt is not responsible for inventing architecture.
It should reflect the contract, not replace it.

---

## Core fields or sections

Typical prompt sections include:

- artifact name
- role
- module
- responsibilities
- must not
- allowed dependencies
- forbidden dependencies
- inputs
- outputs
- completion criteria

Prompts may be represented as Markdown, plain text, or structured output.

---

## Example

```md
# Artifact Prompt: create_user

Implement the `create_user` artifact.

## Role
usecase

## Module
user

## Responsibilities
- Execute one application use case
- Coordinate domain behavior
- Persist through an abstract repository boundary

## Must not
- Write SQL directly
- Return transport-specific responses

## Allowed dependencies
- domain
- application

## Forbidden dependencies
- interfaces
- infrastructure

## Inputs
- CreateUserCommand

## Outputs
- CreateUserResult

## Completion criteria
- The artifact has one clear responsibility
- The implementation respects architectural boundaries
- No infrastructure-specific logic appears in this artifact

---

## Relationship to other concepts

A prompt is derived from:

- the project context
- the artifact identity
- the artifact contract

The relationship is:

- project defines the architectural frame
- artifact defines the execution unit
- contract defines the boundary
- prompt delivers that boundary to an AI implementer

Prompts should be treated as generated interfaces, not primary architectural records.

---

## Design principles

A prompt should be:

- concise
- explicit
- implementation-oriented
- faithful to the contract
- easy for an AI model to follow
- easy for a human to inspect before use

A good prompt should reduce ambiguity without adding noise.

---

## What a prompt should do well

A strong prompt should:

- clearly identify the target artifact
- clearly identify allowed scope
- clearly identify forbidden behavior
- clearly identify expected result
- avoid unnecessary prose
- remain usable by smaller or cheaper models

This is especially important for artifact-level implementation workflows.

---

## What a prompt should not do

A prompt should not:

- introduce new rules not present in the contract
- drift away from the artifact definition
- contain large unrelated architectural explanations
- try to solve multiple artifacts at once
- become so long that lightweight models lose focus

---

## Why it matters

In many AI-assisted workflows, the bottleneck is not code generation ability.
It is context precision.

Repository-wide instructions help, but they are often too broad.

Prompts allow Archflow to hand over implementation in smaller, clearer units.

This makes them useful for:

- lightweight model workflows
- editor-based AI assistance
- human review before implementation
- reproducible artifact generation

---

## Prompt quality guidelines

A strong prompt usually has:

- one artifact target
- one clear role
- explicit responsibilities
- explicit constraints
- explicit completion criteria

A weak prompt usually has:

- vague intent
- hidden assumptions
- mixed responsibilities
- too much repository-wide context
- too much freeform prose

---

## Future directions

In the future, prompts may also support:

- role-specific prompt presets
- model-specific output modes
- compact and detailed prompt variants
- prompt validation against contracts
- editor and agent integration