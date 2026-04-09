# Current State: Architecture Rule Alignment

Date: 2026-04-10
Target issue: #202
Reference: `ARCHITECTURE_RULES.md`

## Purpose

This note verifies the current `src/` structure against the architecture rules and
records the remaining gaps that should still guide refactoring and review.

It replaces the earlier pre-refactor audit from issue #191 with a shorter
"what is aligned now / what is still transitional" view.

## 1. Current structure snapshot

Current top-level source directories:

- `src/cli`
- `src/app`
- `src/domain`
- `src/ports`
- `src/infra`
- `src/commands`
- `src/config`
- `src/generator`
- `src/model`

The first five directories match the target architecture shape from
`ARCHITECTURE_RULES.md`.
The last four remain as transitional legacy areas and should continue shrinking.

## 2. What is aligned with the rules

The following rule expectations are already reflected in the implementation:

1. Layer skeleton exists
- `cli`, `app`, `domain`, `ports`, and `infra` are present in `src/`.

2. CLI entrypoint is thin
- `src/main.rs` only boots the CLI via `cli::run()`.

3. UseCase-oriented command flow exists
- `src/cli/commands` dispatches command input into application use cases.
- `src/app/usecase/*` now exists as the main orchestration layer.

4. Core planning / validation / generation logic has named domain areas
- `src/domain/project`
- `src/domain/preset`
- `src/domain/planning`
- `src/domain/validation`
- `src/domain/generation`

5. Ports and adapters are explicit
- `src/ports/*` defines capability boundaries.
- `src/infra/*` provides concrete adapters and renderers.

6. Structured application errors exist
- Application-facing error types now exist instead of only ad hoc string errors.

## 3. Remaining gaps against the rules

The architecture is improved, but not fully converged yet.

### 3.1 Transitional legacy modules still exist

The following directories still hold behavior that should gradually move behind the
new boundaries:

- `src/commands`
- `src/config`
- `src/generator`
- `src/model`

This is the main reason the current structure is "aligned in direction" but not
yet fully aligned in final shape.

### 3.2 App layer still calls legacy command/config code

Observed examples:

- `app/usecase/init_project.rs` still delegates to `commands::init::execute`
- `app/usecase/validate_project.rs` still delegates to `commands::verify::execute`
- `app/usecase/generate_artifacts.rs` still delegates to `commands::scaffold::execute`
- `app/usecase/plan_architecture.rs` still depends on legacy config loading and guard types

This means some use cases are still wrappers around old execution paths rather than
the final application orchestration boundary described in the rules.

### 3.3 Raw config loading is still coupled to legacy config modules

`app/usecase/*` still loads configuration through `crate::config::*::load(...)`.

This is acceptable as an intermediate state, but the review expectation should be:

- parsing stays outside domain
- raw config APIs do not leak arbitrarily across the codebase
- new features should prefer app/domain-facing boundaries instead of adding more
  direct calls into legacy config modules

### 3.4 Target directory shape is not complete yet

`ARCHITECTURE_RULES.md` describes `shared/` as the place for truly stable cross-cutting
primitives.

That directory does not exist yet, so contributors should avoid inventing new
cross-cutting buckets until there is a clear, documented need.

## 4. Review interpretation for current PRs

Until the migration is complete, reviewers should treat architecture alignment as:

1. New behavior should prefer `cli -> app -> domain/ports` flow.
2. New business decisions should not be added to `cli`.
3. New generic buckets should not be introduced under `commands`, `generator`, or `model`.
4. Legacy modules may be touched for compatibility, but changes should reduce or at least
   not increase boundary leakage.
5. If a rule cannot be followed yet, the PR should state the temporary reason explicitly.

## 5. Practical refactor priority

The next architecture-alignment wins are:

1. Move remaining command execution logic out of legacy `commands::*` into app/usecase flows.
2. Replace direct legacy config access with narrower app-facing loaders or ports where helpful.
3. Keep shrinking `model` and `generator` toward domain/infra ownership.
4. Introduce `shared/` only if a primitive is truly stable and cross-cutting.

## 6. Status summary

- [x] current structure checked against `ARCHITECTURE_RULES.md`
- [x] aligned areas identified
- [x] remaining gaps identified
- [x] review interpretation recorded for future PRs
