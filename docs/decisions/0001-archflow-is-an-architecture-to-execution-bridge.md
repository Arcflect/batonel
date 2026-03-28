# 0001 Archflow is an architecture-to-execution bridge

- Status: accepted
- Date: 2026-03-28

## Context

Archflow could have been positioned in several different ways.

Possible interpretations included:

- a scaffold generator
- a prompt generation tool
- a repository template system
- an architecture linting tool
- a spec-driven planning tool

Each of these captures part of the project, but none captures the whole intended flow.

The core problem Archflow addresses is the gap between architecture and implementation.
That gap becomes even more important in AI-assisted development, where implementation may be handed to lightweight models that need explicit structure and constraints.

## Decision

Archflow is defined as an **architecture-to-execution bridge**.

This means Archflow is intended to connect:

- architectural intent
- structural planning
- artifact definition
- responsibility boundaries
- AI handoff
- future verification

Archflow is not defined primarily as a template repository, prompt toolkit, or linter,
even though it may include aspects of those.

## Consequences

This decision makes several things clearer.

What becomes easier:
- explaining the product direction
- deciding what belongs in scope
- evaluating whether a feature supports the core flow
- keeping concepts and implementation aligned

What becomes harder:
- narrowly optimizing for only one sub-problem
- positioning Archflow as a generic utility tool
- expanding into unrelated tooling too early

This decision supports the current documentation direction and the roadmap layering.

## Alternatives considered

### Archflow as a scaffold generator

Not chosen because it is too narrow.
Scaffolding is important, but Archflow also defines contracts and prompts.

### Archflow as a prompt generation tool

Not chosen because prompts are derived outputs, not the primary purpose.

### Archflow as an architecture linter

Not chosen because linting is a later operational layer, not the starting point.

### Archflow as a spec-driven planning system

Not chosen because Archflow focuses more specifically on artifact-level implementation handoff.

## Notes

This decision is foundational and should be treated as a framing decision for future scope discussions.