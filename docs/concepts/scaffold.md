# Scaffold

## Overview

In Archflow, a **Scaffold** is the generated structural output created from architectural definitions.

A scaffold is the practical result of turning project context, placement rules,
artifact plans, and contracts into concrete implementation-ready structure.

Scaffolding may include directories, placeholder files, contract files,
prompt files, and other supporting outputs.

---

## Purpose

The purpose of a scaffold is to turn architectural intent into a usable starting structure.

A scaffold answers questions such as:

- What should exist in the repository after planning?
- Which files and directories should be created?
- Which artifact sidecar files should be generated?
- What should be ready before full implementation begins?

Without scaffolding, architecture may be documented,
but contributors and AI tools still have to reconstruct structure manually.

---

## Responsibilities

A scaffold is responsible for producing:

- concrete directory structure
- placeholder implementation files
- artifact contract files
- artifact prompt files
- predictable output paths
- implementation-ready structural context

A scaffold is not responsible for inventing architecture.
It reflects the architecture already defined through project files,
placement rules, artifact plans, and contracts.

A scaffold is also not the same thing as a completed implementation.
It is a prepared execution surface.

---

## Core elements

Typical scaffold outputs may include:

- directories
- placeholder implementation files
- `*.contract.yaml`
- `*.prompt.md`
- optional metadata files
- optional generated summaries

In practice, scaffold output usually reflects:

- resolved artifact paths
- role-based structure
- sidecar-first architectural data

---

## Example

A scaffold may produce output such as:

```text
src/
├─ domain/
│  └─ entities/
│     └─ user.rs
├─ application/
│  └─ usecases/
│     └─ create_user.rs
└─ .archflow/
   ├─ contracts/
   │  ├─ user.contract.yaml
   │  └─ create_user.contract.yaml
   └─ prompts/
      ├─ user.prompt.md
      └─ create_user.prompt.md
```

In this example, the scaffold does not only create file locations.
It also creates the sidecar files that preserve architectural intent.

---

## Relationship to other concepts

A scaffold is downstream from several other concepts.

The relationship is:

- the project defines the architectural frame
- the module organizes functional areas
- the role classifies each artifact
- the placement rule resolves where artifacts should live
- the artifact plan defines what should exist
- the contract defines boundaries
- the prompt defines AI handoff
- the scaffold materializes all of this into repository structure

This makes scaffold one of the main operational outputs of Archflow.

## Why scaffold matters in Archflow

Scaffolding matters because Archflow is not only a documentation system.

It is meant to help users move from:

- architecture
- to structure
- to implementation

That transition becomes much easier when the repository already contains:

- the right directories
- the right placeholder files
- the right sidecar files
- the right artifact names
- the right implementation boundaries

Scaffolding reduces ambiguity at the point where implementation begins.

## Design principles

A scaffold should be:

- predictable
- easy to inspect
- aligned with architectural intent
- useful before code is complete
- stable enough to support verification later

A good scaffold should make the next implementation step obvious.

## What a scaffold should not do

A scaffold should not:

- replace contracts as the source of truth
- invent responsibilities that are not defined elsewhere
- hide architectural intent behind generated complexity
- assume that generated code is complete production code
- become so magical that contributors cannot understand what happened

Scaffolding should make architecture clearer, not more opaque.

## Scaffold vs template

A scaffold is not exactly the same as a template.

- a **template** is usually a reusable pattern or starting file shape
- a **scaffold** is the generated structural result for one concrete project state

Archflow may use templates internally,
but the scaffold is the actual output created from the current project configuration.

This means scaffolding is project-specific and context-aware.

## Scaffold vs contract

A scaffold is not the same as a contract.

- a **contract** defines responsibilities and constraints
- a **scaffold** creates the structural output that includes or reflects those contracts

Contracts define meaning.
Scaffolds create structure.

The two work together, but they are not interchangeable.

## Scaffold vs implementation

A scaffold is not the final implementation.

- a **scaffold** prepares the structure for implementation
- an **implementation** fills that structure with working code or logic

This distinction is important because Archflow is designed to be useful
even before production code exists.

## Types of scaffold output

Depending on the project and future CLI behavior,
scaffold output may include different levels of richness.

### Minimal scaffold

A minimal scaffold may include only:

- directories
- empty or placeholder implementation files

### Contract-aware scaffold

A contract-aware scaffold may include:

- directories
- placeholder implementation files
- generated contract files

### AI-handoff-aware scaffold

A more complete scaffold may include:

- directories
- placeholder implementation files
- contracts
- prompts

This progression aligns with the broader Archflow roadmap.

## Why scaffold is important for AI-assisted development

In AI-assisted workflows, implementation often starts best when the unit of work is:

- small
- explicit
- bounded
- already placed correctly

Scaffolding helps create that state.

Instead of asking an AI model to both invent structure and implement logic,
Archflow can first provide:

- the target file
- the target role
- the target contract
- the target prompt

This improves implementation quality, especially for smaller models.

## Future directions

In the future, scaffolding may also support:

- richer placeholder content
- role-specific starter snippets
- preset-aware scaffold packages
- partial re-scaffolding
- scaffold diffing
- scaffold validation hooks
- ecosystem-specific output conventions

Even with those extensions, the core purpose stays the same:

a scaffold turns architectural definitions into implementation-ready structure.