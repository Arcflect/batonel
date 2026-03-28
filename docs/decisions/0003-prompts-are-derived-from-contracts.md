# 0003 Prompts are derived from contracts

- Status: accepted
- Date: 2026-03-28

## Context

Archflow includes prompts because AI-assisted implementation is a major use case.

However, prompts can easily drift.
They may become:

- manually edited
- model-specific
- overly verbose
- inconsistent across roles
- disconnected from architectural intent

The project needs a clear rule for how prompts relate to contracts.

## Decision

Prompts are treated as **derived outputs** from contracts.

This means:

- prompts should reflect contract data
- prompts should not introduce new architectural rules
- prompts should not replace contract files
- prompt generation should remain traceable to artifact boundaries

A prompt is a delivery format for implementation handoff.
It is not the authoritative architectural record.

## Consequences

What becomes easier:
- stable AI handoff behavior
- future prompt regeneration
- consistency across examples
- contract-centered workflow design

What becomes harder:
- freeform manual prompt drift
- prompt-specific rules that bypass the contract model

This also supports multiple future prompt formats,
such as compact, detailed, or role-specific outputs.

## Alternatives considered

### Prompt as a co-equal source of truth

Not chosen because it creates ambiguity about where architectural rules truly live.

### Prompt as manually authored primary artifact

Not chosen because it weakens repeatability and consistency.

### No prompt generation at all

Not chosen because artifact-level AI handoff is one of the key reasons Archflow exists.

## Notes

Future model-specific formatting may still exist,
but it should remain downstream from the contract.