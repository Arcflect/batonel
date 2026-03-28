# Project

## Overview

In Archflow, a **Project** is the top-level definition of architectural intent.

A project describes the overall structure that Archflow should use when generating
placement rules, artifacts, contracts, and AI handoff prompts.

It is the highest-level unit in the Archflow model.

---

## Purpose

The purpose of a project is to define the architectural frame in which all lower-level
elements are interpreted.

A project answers questions such as:

- What kind of system is this?
- What architectural style does it follow?
- What modules exist?
- What conventions should be applied?
- What language or environment is assumed?

Without a project definition, Archflow cannot consistently interpret artifact placement
or responsibility boundaries.

---

## Responsibilities

A project is responsible for defining:

- the project name
- the architectural style
- the language orientation
- the module list
- optional workspace or structural settings
- the context in which artifacts and contracts are generated

A project is not responsible for defining detailed behavior of individual artifacts.
That belongs to artifacts and contracts.

---

## Core fields

Typical fields include:

- `name`
- `architecture_style`
- `language`
- `modules`
- `workspace` (optional)
- `metadata` (optional)

---

## Example

```yaml
project:
  name: sample-app
  architecture_style: clean-hexagonal
  language: rust

workspace:
  enabled: true
  members:
    - crates/domain
    - crates/application
    - crates/adapters/http
    - crates/adapters/db

modules:
  - name: user
    features:
      - create_user
      - user_entity
```

---

## Relationship to other concepts

A project contains or frames:

- modules
- placement rules
- artifact plans
- contract templates
- prompts

The project does not replace those concepts.
Instead, it provides the context in which they are interpreted.

---

## Design principles

A project definition should be:

- explicit
- stable
- human-readable
- minimal but sufficient
- independent from implementation details where possible

A project should describe architectural intent, not business logic.

---

## What a project should not do

A project should not:

- define the internal code of one artifact
- replace contracts
- encode framework-specific behavior unless necessary
- mix architectural intent with low-level implementation detail

---

## What a project should not do

A project should not:

- define the internal code of one artifact
- replace contracts
- encode framework-specific behavior unless necessary
- mix architectural intent with low-level implementation detail

---

## Why it matters

In AI-assisted development, the project definition gives a shared architectural frame
to both humans and tools.

It helps Archflow answer:

- where an artifact belongs
- how a role should be interpreted
- what structure should be scaffolded
- which examples or presets best fit the repository

Without a clear project concept, the rest of the model becomes inconsistent.