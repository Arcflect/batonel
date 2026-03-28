# 0004 Sidecar files are first-class

- Status: accepted
- Date: 2026-03-28

## Context

Archflow needs a way to preserve architectural intent even before full implementation exists.

Possible storage strategies included:

- comments inside implementation files
- repository-wide instruction files only
- separate contract and prompt files
- code-aware extraction only

A code-only strategy would make early design and language-agnostic workflows harder.
A comment-only strategy would depend too much on implementation language and file state.

## Decision

Sidecar files are treated as **first-class artifacts** in Archflow.

Important examples include:

- `*.contract.yaml`
- `*.prompt.md`

These files are not secondary documentation.
They are operational parts of the Archflow model.

This supports workflows where:

- code does not exist yet
- placeholder files exist but implementation is incomplete
- architecture needs to remain explicit outside source code

## Consequences

What becomes easier:
- language-agnostic modeling
- pre-implementation design workflows
- prompt generation
- contract-centered verification
- artifact-level architectural memory

What becomes harder:
- relying only on inline code comments
- assuming source files are the only meaningful artifacts

This decision also supports future multi-language use.

## Alternatives considered

### Comments inside implementation files as the main storage layer

Not chosen because this is too dependent on language and implementation timing.

### Repository-wide instruction files only

Not chosen because they are too broad for artifact-level boundaries.

### Code-aware extraction as the main approach

Not chosen because it would make early-phase usage and non-code-first workflows weaker.

## Notes

This decision does not forbid inline comments.
It only says that sidecar files are first-class and must be treated seriously.