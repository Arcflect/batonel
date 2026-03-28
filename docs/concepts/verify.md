# Verify

## Overview

In Archflow, **Verify** is the process of checking whether architectural structure
and artifact definitions remain consistent over time.

Verification helps ensure that the project still matches the architecture
described by its project files, placement rules, artifact plans, contracts,
prompts, and scaffold expectations.

Verify is not only about detecting errors.
It is about preserving architectural intent as the project evolves.

---

## Purpose

The purpose of verify is to protect consistency between architectural definitions
and the project state.

Verify answers questions such as:

- Do the expected files exist?
- Do artifact roles remain aligned across files?
- Do resolved paths still match placement rules?
- Do contracts include the required fields?
- Do prompts exist where expected?
- Are artifact statuses internally consistent?

Without verification, architecture can drift into:

- missing files
- inconsistent naming
- contract gaps
- stale prompts
- structural mismatches
- unclear implementation state

---

## Responsibilities

Verify is responsible for checking whether the Archflow model remains coherent.

It is responsible for validating things such as:

- required file presence
- role consistency
- path consistency
- contract completeness
- prompt presence
- status consistency
- scaffold consistency

Verify is not responsible for inventing architecture.
It checks the architecture that has already been defined.

Verify is also not, at least initially, a full code-analysis engine.

---

## Core idea

The core idea of verify is simple:

**architecture should remain inspectable and enforceable after generation**

Archflow is not meant to stop at scaffold generation.
It should also help users detect when the repository drifts away
from its explicit architectural model.

This is especially important in AI-assisted workflows,
where implementation can happen quickly and drift can happen quietly.

---

## Relationship to other concepts

Verify is downstream from the rest of the Archflow model.

The relationship is:

- the project defines the architectural frame
- modules organize functional areas
- roles classify artifacts
- placement rules define structure
- artifact plans define what should exist
- contracts define boundaries
- prompts define AI handoff
- scaffolds materialize structure
- verify checks whether these remain consistent

This makes verify the main protective layer of the model.

---

## Why verify matters

Archflow is built around explicit architecture.

That explicit architecture should not live only at generation time.
It should continue to matter as the project changes.

Verify matters because it helps answer:

- Has the structure drifted?
- Are artifact contracts missing?
- Are prompts still aligned with artifacts?
- Does the project still reflect the intended architecture?

Without verify, the explicit model can slowly become decorative rather than operational.

---

## What verify should check first

In its earliest form, verify should focus on structure and contract consistency.

Examples of early checks include:

- required input files exist
- planned artifacts have corresponding contract files
- contract names match artifact names
- contract roles match artifact roles
- contract modules match artifact modules
- artifact paths match placement rules
- required contract fields are present
- expected prompt files exist
- status values are present and internally consistent

These checks are enough to make the architectural model operational
without requiring deep source code understanding.

---

## What verify should not start with

Verify should not begin with:

- compiler integration
- AST parsing
- import graph analysis
- framework-specific inspection
- deep language-specific static analysis
- full architecture linting from code alone

Those may become useful later,
but they should not replace the sidecar-first and contract-centered model too early.

Archflow begins from architecture,
so verify should begin from architecture too.

---

## Verify vs contract

Verify is not the same as a contract.

- a **contract** defines what an artifact is responsible for
- **verify** checks whether that contract exists, is complete, and remains aligned

Contracts define the intended boundary.
Verify protects the continued integrity of that boundary.

---

## Verify vs scaffold

Verify is not the same as scaffold.

- a **scaffold** creates structure
- **verify** checks whether that structure still matches the model

Scaffold is generative.
Verify is protective.

The two are closely related, but they serve different purposes.

---

## Verify vs code-aware analysis

Verify is broader than code-aware analysis, but also starts earlier.

- **early verify** focuses on structure and sidecar consistency
- **later verify** may optionally include code-aware checks

This distinction matters because Archflow is designed to be useful
before full implementation exists.

That means verify must remain meaningful even when:

- code is incomplete
- placeholders are still present
- implementation is only partially started

This is why verify starts from architectural files,
not from source code parsing.

---

## Design principles

Verify should be:

- explicit
- understandable
- aligned with the documented model
- useful before deep code analysis exists
- strict enough to catch drift
- simple enough to explain and trust

A good verify system should make architectural consistency visible
without becoming mysterious or overly heavy.

---

## What verify should not do

Verify should not:

- become the primary place where architecture is defined
- invent new rules not present in the documented model
- depend too early on one language ecosystem
- assume production code is already complete
- become so complex that contributors cannot understand what is being checked

Verification should strengthen clarity, not hide it.

---

## Examples of verification questions

Examples of useful verification questions include:

- Does every artifact in `artifacts.plan.yaml` have a corresponding contract?
- Does each contract path align with the role-to-path mapping?
- Does each prompt correspond to a real artifact contract?
- Are required contract fields present?
- Are status values valid?
- Are examples internally consistent with the documented schemas?

These are the kinds of checks that make Archflow operational over time.

---

## Why verify matters for AI-assisted development

In AI-assisted workflows, many changes may be made quickly across multiple artifacts.

That creates a risk that:

- files are created in the wrong place
- contracts are skipped
- prompts become stale
- naming drifts
- intended architectural boundaries weaken

Verify helps reduce that risk by checking the explicit model regularly.

This is especially useful when lightweight models are used,
because those workflows benefit from tighter guardrails.

---

## Future directions

In the future, verify may also support:

- optional code-aware checks
- import-pattern checks
- dependency boundary checks
- role-aware file inspections
- CI integration
- preset-aware verification defaults
- richer lifecycle validation
- project health summaries

Even if those features grow later, the basic purpose remains the same:

verify protects the consistency of the architecture-to-execution model.

---

## Summary

Verify is the consistency-checking layer of Archflow.

It exists to ensure that project structure, contracts, prompts,
and scaffold outputs remain aligned with architectural intent.

If you remember only one thing, remember this:

**scaffold creates the structure, verify protects it over time**