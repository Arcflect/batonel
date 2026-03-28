# 0005 Examples precede presets

- Status: accepted
- Date: 2026-03-28

## Context

Archflow already has examples such as:

- minimal
- generic-layered
- rust-clean-hexagonal

A future preset system is likely,
but implementing presets too early would risk locking unstable concepts into reusable packages before the model is mature enough.

The project needs a sequencing decision.

## Decision

Examples come before presets.

This means:

- examples are the first step for teaching and exploring the model
- examples should stabilize naming, structure, and expectations
- presets should be introduced only after examples prove reusable patterns

Examples are descriptive first.
Presets are operational later.

## Consequences

What becomes easier:
- learning from concrete cases
- refining the model before operational packaging
- avoiding premature preset rigidity
- evolving role naming and contract defaults safely

What becomes harder:
- offering instant reusable preset bootstrap early
- optimizing for adoption speed before conceptual stability

This sequencing keeps the project aligned with its layered roadmap.

## Alternatives considered

### Build presets immediately

Not chosen because the concept model is still stabilizing.

### Ignore presets entirely

Not chosen because reusable starting points are likely to become important later.

### Treat examples and presets as the same thing

Not chosen because they serve different purposes.
Examples teach.
Presets operationalize.

## Notes

This decision supports the current documentation direction in `docs/presets.md`.