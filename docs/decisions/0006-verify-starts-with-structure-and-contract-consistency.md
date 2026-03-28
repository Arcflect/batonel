# 0006 Verify starts with structure and contract consistency

- Status: accepted
- Date: 2026-03-28

## Context

Archflow is expected to include a `verify` capability in the future.

That verification layer could grow in many directions, for example:

- required file presence checks
- role-to-path consistency checks
- contract completeness checks
- prompt presence and derivation checks
- dependency boundary checks
- code-aware import checks
- static analysis integration
- language-specific validation

Because verification can expand quickly, the project needs an early decision about
where verification should begin.

Without this decision, there is a risk that `verify` becomes too broad too early,
or that it drifts into code analysis before the core structural model is stable.

Archflow’s current maturity is centered on:

- project definitions
- placement rules
- artifact plans
- contracts
- prompts
- scaffold structure

That means the first verification layer should protect those assets first.

## Decision

Archflow verification begins with **structure and contract consistency**.

The first scope of `verify` should focus on questions such as:

- do required files exist?
- do planned artifacts have corresponding contract files?
- do roles align across input and generated files?
- do artifact paths match placement rules?
- do required contract fields exist?
- do prompt files exist where expected?
- do artifact statuses remain internally consistent?

In other words, `verify` begins by checking whether the architecture-defined structure
and artifact boundaries remain coherent over time.

It does **not** begin with deep code-aware checks.

## Consequences

What becomes easier:
- defining a clear first version of `verify`
- keeping verification aligned with the current Archflow model
- protecting architectural intent before code parsing exists
- making examples and future CLI behavior easier to validate
- supporting language-agnostic usage in early phases

What becomes harder:
- using `verify` as a full architecture linter from the start
- immediately supporting language-specific code inspection
- catching implementation drift that only appears inside source code

This is an intentional tradeoff.

The first job of `verify` is to protect the explicit architectural model,
not to fully understand all implementation details.

## Initial verification targets

The first version of `verify` should prioritize checks such as:

### 1. Required input file presence

Examples:
- `project.arch.yaml` exists
- `placement.rules.yaml` exists
- `artifacts.plan.yaml` exists

### 2. Artifact-to-contract consistency

Examples:
- every planned artifact has a contract
- contract names match artifact names
- contract roles match planned artifact roles
- contract modules match planned artifact modules

### 3. Role-to-path consistency

Examples:
- artifact roles exist in placement rules
- resolved contract paths match placement rules
- explicit path overrides are recognized consistently

### 4. Required contract field presence

Examples:
- `name`
- `module`
- `role`
- `path`
- `responsibilities`
- `must_not`
- `status`

### 5. Prompt presence and derivation consistency

Examples:
- expected prompt files exist
- prompt artifact names align with contracts
- prompt structure reflects contract-level intent

### 6. Status consistency

Examples:
- artifact and contract status do not conflict
- invalid status values are surfaced
- lifecycle state is present when required

## What verify should not start with

The first version of `verify` should not start with:

- compiler integration
- language-specific AST parsing
- import graph analysis
- full dependency graph enforcement
- framework-specific rule engines
- model-vendor-specific prompt validation

These may become useful later,
but they should remain downstream from structural and contract verification.

## Why this fits Archflow

This decision aligns with several earlier project decisions.

It supports the idea that:

- Archflow is an architecture-to-execution bridge
- contracts are the source of truth for artifact boundaries
- prompts are derived from contracts
- sidecar files are first-class
- examples come before more operational preset machinery

Because Archflow starts from architecture rather than code,
its first verification layer should also begin from architecture rather than code.

## Alternatives considered

### Start verification with code-aware checks

Not chosen because code-aware validation would introduce language coupling too early
and would outrun the current model maturity.

### Start verification with prompts only

Not chosen because prompts are derived artifacts, not the primary architectural source of truth.

### Delay verification until after CLI maturity

Not chosen because even early examples and generated structures benefit from consistency checking.

### Treat verification as only a future CI concern

Not chosen because local verification is useful even before formal CI integration exists.

## Notes

This decision does not reject code-aware verification in the future.

It only establishes ordering:

**verify should begin with structure and contract consistency, then expand later if needed.**