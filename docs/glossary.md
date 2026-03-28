# Glossary

This glossary defines the core terms used in Archflow.

The goal is to keep terminology stable across examples, documentation,
schemas, and future CLI behavior.

---

## Project

The top-level definition of architectural intent.

A project describes the overall frame in which Archflow interprets modules,
placement rules, artifacts, contracts, and prompts.

Typical examples:
- a Rust clean / hexagonal application
- a generic layered service
- a modular monolith with multiple bounded modules

---

## Module

A named architectural area within a project.

A module groups related artifacts and features.
It is usually aligned with a business capability or cohesive technical area.

Examples:
- `user`
- `auth`
- `billing`

A module is not necessarily a package or directory by itself,
though it may map to those in a specific project.

---

## Role

The architectural role assigned to an artifact.

A role helps Archflow decide:

- where an artifact should live
- what template should apply
- what responsibilities are typical
- what constraints should be generated

Examples:
- `entity`
- `usecase`
- `service`
- `repository_port`
- `repository_interface`
- `controller`
- `handler`
- `gateway`
- `repository_impl`

---

## Artifact

The smallest planned implementation unit in Archflow.

An artifact is a concrete thing that should exist in the project structure,
such as a file, handler, entity, service, repository boundary, or adapter unit.

Artifacts are the main units Archflow scaffolds, constrains, and hands off
to humans or AI systems.

Examples:
- `user`
- `create_user`
- `user_repository`
- `create_user_handler`

---

## Placement Rule

A rule that maps a role to a location in the project structure.

Placement rules help Archflow determine where an artifact should live.

Examples:
- `entity` -> `src/domain/entities/`
- `usecase` -> `src/application/usecases/`
- `controller` -> `src/interfaces/controllers/`

Placement rules define location, not behavior.

---

## Artifact Plan

A structured list of artifacts that should be created for a project.

An artifact plan usually includes artifact names, roles, modules,
and optionally inputs and outputs.

It is one of the main inputs Archflow uses for scaffold generation.

---

## Contract

The definition of an artifact’s responsibilities and constraints.

A contract describes:

- what an artifact should do
- what it must not do
- what dependencies are allowed
- what dependencies are forbidden
- what inputs and outputs are expected
- how focused the implementation should be

Contracts are the main way Archflow preserves architectural intent
during implementation.

---

## Contract Template

A reusable rule set for generating contracts by role.

A contract template provides default responsibilities, constraints,
and implementation guidance for a given role.

Examples:
- a default `entity` template
- a default `usecase` template
- a default `controller` template

Artifact-specific contracts may extend or refine these templates.

---

## Prompt

The AI handoff representation of an artifact contract.

A prompt turns project context, artifact identity, and contract boundaries
into a format that can be directly given to an AI coding tool.

Prompts are derived from contracts.
They are not the primary source of truth.

---

## Scaffold

The generated structural output produced by Archflow.

Scaffolding may include:

- directories
- placeholder files
- contract files
- prompt files
- metadata files

Scaffolding is meant to make implementation easier and more consistent.

---

## Verify

The process of checking whether project structure and artifact definitions
remain consistent with Archflow rules.

Verification may include checking:

- required contract fields
- placement consistency
- artifact status consistency
- contract/prompt presence
- future optional code-aware checks

---

## Preset

A reusable configuration package for a common architectural style or ecosystem.

A preset may include:

- project defaults
- placement rules
- contract templates
- example artifacts
- role conventions

Examples:
- Rust clean / hexagonal preset
- generic layered preset

---

## AI Handoff

The act of passing an artifact to an AI system for implementation.

In Archflow, AI handoff is based on:

- project context
- artifact definition
- contract
- generated prompt

The goal is to make implementation clearer, smaller in scope,
and less likely to drift from architecture.

---

## Sidecar File

A file that accompanies an implementation artifact but is not itself
the implementation.

In Archflow, sidecar files commonly include:

- `*.contract.yaml`
- `*.prompt.md`

Sidecar files are important because they allow Archflow to work
even before production code exists.

---

## Status

The lifecycle state of an artifact or contract.

Status helps track where an artifact is in the workflow.

Examples:
- `planned`
- `scaffolded`
- `implementing`
- `reviewing`
- `done`

Status is especially useful for AI-assisted workflows and future verification.

---

## Architecture-to-Execution Bridge

A short way to describe what Archflow is.

It means Archflow sits between:

- architecture design
- structural planning
- AI handoff
- implementation scaffolding
- future verification

Archflow does not stop at documentation.
It turns design intent into executable implementation context.