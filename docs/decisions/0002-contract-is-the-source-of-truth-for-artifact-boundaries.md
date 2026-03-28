# 0002 Contract is the source of truth for artifact boundaries

- Status: accepted
- Date: 2026-03-28

## Context

Archflow includes multiple representations of architectural intent:

- examples
- project definitions
- artifact plans
- contracts
- prompts
- scaffolded files

Without a clear source of truth, contributors may treat prompts or placeholder files as the primary definition of an artifact.
That would make architectural boundaries less stable.

The project needs one central place where artifact responsibilities and constraints are defined.

## Decision

The **contract** is the source of truth for artifact-level responsibilities and constraints.

This means the contract is the canonical place to define:

- what an artifact is responsible for
- what it must not do
- what dependencies are allowed
- what dependencies are forbidden
- what inputs and outputs are expected
- what implementation size is intended

Prompts are derived from contracts.
Scaffolded files may reference contracts.
But contracts remain primary.

## Consequences

What becomes easier:
- consistent prompt generation
- future verification design
- artifact-level boundary preservation
- contributor understanding of where rules live

What becomes harder:
- treating prompts as informal ad hoc instructions
- keeping architecture only in example files or team memory

This decision also strengthens the sidecar-file model.

## Alternatives considered

### Prompt as source of truth

Not chosen because prompts are delivery-oriented and may vary by usage context.

### Placeholder implementation file as source of truth

Not chosen because code-adjacent files are too dependent on language and repository layout.

### Artifact plan as source of truth

Not chosen because the artifact plan identifies units, but does not fully define behavioral boundaries.

## Notes

This decision should guide future verify behavior and prompt design.