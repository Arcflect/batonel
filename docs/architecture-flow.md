# Architecture Flow

This document explains how Archflow works as a flow from architectural intent
to implementation scaffolding and AI handoff.

The goal of this document is to show:

- what Archflow takes as input
- how Archflow interprets that input
- what Archflow produces as output
- how its core concepts connect together

This is a conceptual flow description, not an implementation spec.

---

## Overview

Archflow is an architecture-to-execution bridge.

It starts with structured architectural definitions and turns them into:

- planned artifact placement
- artifact-level contracts
- AI handoff prompts
- scaffolded implementation units
- future verification targets

The central idea is simple:

**design first, resolve structure, define boundaries, then hand off implementation**

---

## High-level flow

At a high level, Archflow follows this sequence:

1. define project context
2. define placement rules
3. define planned artifacts
4. apply contract templates
5. resolve artifact paths
6. generate artifact contracts
7. generate AI handoff prompts
8. scaffold implementation structure
9. verify consistency over time

This flow is meant to preserve architectural intent before and during implementation.

---

## Flow inputs

Archflow begins from structured input files.

The main input files are:

- `project.arch.yaml`
- `placement.rules.yaml`
- `artifacts.plan.yaml`
- `contracts.template.yaml` (optional but recommended)

These files define the architecture before code is written.

### 1. Project context

The project definition establishes the architectural frame.

It tells Archflow:

- what kind of project this is
- which architectural style it follows
- which modules exist
- what language orientation is assumed

Without project context, the rest of the model has no stable frame.

### 2. Placement rules

Placement rules define where artifacts of each role should live.

They tell Archflow:

- where `entity` artifacts belong
- where `usecase` artifacts belong
- where `controller` artifacts belong
- what file extension or naming pattern may apply

Without placement rules, artifacts cannot be resolved into structure.

### 3. Artifact plan

The artifact plan defines what should exist.

It tells Archflow:

- which artifacts to prepare
- which module each artifact belongs to
- which role each artifact has
- optional inputs and outputs

Without an artifact plan, Archflow has no execution units to work with.

### 4. Contract templates

Contract templates provide reusable defaults by role.

They tell Archflow:

- what responsibilities are typical for a role
- what behaviors are forbidden for a role
- what dependency boundaries usually apply

Without templates, contracts can still exist, but consistency becomes weaker.

---

## Core interpretation flow

Once the input files are available, Archflow interprets them in layers.

### Step 1. Read project context

Archflow first reads the project definition.

This establishes the global context in which all later interpretation happens.

At this stage, Archflow identifies:

- architecture style
- module space
- language orientation
- optional workspace or structural settings

### Step 2. Load placement rules

Next, Archflow loads placement rules and creates a role-to-path map.

At this stage, Archflow can answer:

- if an artifact is a `usecase`, where should it go?
- if an artifact is a `repository_port`, where should it go?
- if an artifact is a `controller`, where should it go?

This step resolves structure intent, but not behavioral intent.

### Step 3. Load artifact plan

Then Archflow loads the artifact list.

At this stage, Archflow knows the concrete implementation units that should exist.

Each artifact becomes an execution target with:

- name
- module
- role
- optional inputs
- optional outputs
- optional status

This is the point where architecture becomes actionable.

### Step 4. Resolve artifact paths

Using placement rules and artifact roles, Archflow resolves where each artifact should live.

For example:

- `entity` -> `src/domain/entities/`
- `usecase` -> `src/application/usecases/`
- `controller` -> `src/interfaces/controllers/`

If an artifact defines an explicit path override, that may replace the default mapping.

At this stage, Archflow can determine the expected scaffold location of every artifact.

### Step 5. Apply contract templates

If contract templates exist, Archflow applies role-based defaults to each artifact.

This fills in common defaults such as:

- responsibilities
- forbidden behaviors
- allowed dependencies
- forbidden dependencies
- implementation size

This step creates the first draft of artifact boundaries.

### Step 6. Produce artifact contracts

Archflow then turns artifact identity + resolved path + template defaults
into artifact-specific contracts.

A contract represents the behavioral boundary of one artifact.

It typically includes:

- artifact identity
- role
- module
- resolved path
- responsibilities
- must-not rules
- dependency boundaries
- inputs and outputs
- implementation scope
- status

At this stage, architecture is no longer only structural.
It becomes executable at the artifact level.

### Step 7. Generate prompts

From each artifact contract, Archflow generates AI handoff prompts.

A prompt packages the contract into an implementation-oriented format.

It usually contains:

- the target artifact
- its role
- its module
- its responsibilities
- its constraints
- its inputs and outputs
- completion criteria

This step makes the architecture directly usable by AI coding tools.

### Step 8. Generate scaffold output

Archflow can then scaffold the project structure.

This may include:

- directories
- placeholder implementation files
- contract files
- prompt files
- metadata files

At this point, implementation can begin with clear boundaries.

---

## Output model

Archflow produces outputs in several layers.

### Structural outputs

These describe where things should live.

Examples:
- resolved file paths
- generated directories
- scaffolded placeholder files

### Contract outputs

These describe what each artifact is supposed to do.

Examples:
- `create_user.contract.yaml`
- `user.contract.yaml`

### Prompt outputs

These describe how an artifact should be handed to an AI system.

Examples:
- `create_user.prompt.md`
- `user.prompt.md`

### Verification targets

These describe what should remain true over time.

Examples:
- required contract presence
- role-to-path consistency
- status consistency
- future dependency or code-aware checks

---

## Conceptual dependency chain

The internal dependency chain of Archflow looks like this:

```text
project
  -> placement rules
  -> artifact plan
     -> resolved artifact path
     -> contract template application
        -> artifact contract
           -> prompt
           -> scaffold
           -> verify target
```

This means:

- the project defines the frame
- placement rules define location
- artifacts define units of work
- contracts define boundaries
- prompts define AI handoff
- scaffold and verify operationalize the result

---

## Why this flow matters

Many tools help with one part of this process.

Some tools help define specs.
Some tools help instruct AI systems.
Some tools help lint code after implementation.

Archflow is focused on the flow in between.

Its main value is preserving architectural intent across the transition from:

- design
- to structure
- to artifact definition
- to implementation handoff

This is especially important when using lightweight models,
because smaller models need tighter boundaries and clearer context.

---

## Example flow

A simple example may look like this:

1. define a project with one `user` module
2. define `entity` and `usecase` placement rules
3. define two artifacts:
  - `user`
  - `create_user`
4. apply role-based defaults from contract templates
5. resolve paths:
  - `user` -> `src/domain/entities/user.rs`
  - `create_user` -> `src/application/usecases/create_user.rs`
6. generate:
  - `user.contract.yaml`
  - `create_user.contract.yaml`
7. generate:
  - `user.prompt.md`
  - `create_user.prompt.md`
8. scaffold placeholder implementation files

This turns architecture into concrete implementation context.

---

## Design principles behind the flow

Archflow’s flow is built on several principles.

### Architecture before implementation

Structure and responsibility should be defined before code generation begins.

### Artifact-level execution

The useful unit of implementation is not the whole repository.
It is the artifact.

### Contracts as architectural memory

Architecture should not live only in diagrams or team memory.
Contracts preserve it at the artifact level.

### Prompts are derived, not primary

The prompt is not the source of truth.
The contract is.

### Sidecar-first design

Important architectural data should remain usable even before full code exists.

---

## What this flow does not assume

Archflow does not require:

- full code parsing
- framework-specific assumptions
- one fixed architecture style
- one programming language
- one AI vendor or tool

The flow is meant to stay useful even in early design phases.

---

## Future evolution of the flow

In the future, this flow may extend to include:

- schema validation
- preset expansion
- project import from existing repositories
- optional lightweight code-aware checks
- editor integration
- CI verification pipelines
- role-specific prompt variants

Even as those grow, the core flow remains the same:

**project** -> **structure** -> **artifact** -> **contract** -> **prompt** -> **scaffold** -> **verify**

---

## Summary

Archflow works by turning architecture into progressively more executable forms.

It starts with project-level intent,
resolves that into structure,
turns structure into artifact boundaries,
turns those boundaries into contracts,
and turns contracts into implementation handoff.

If you remember only one thing, remember this:

**Archflow does not start from code.
It starts from architecture and turns it into executable implementation context.**