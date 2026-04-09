# Archflow Architecture Rules

## Purpose

This document defines the mandatory architecture and coding rules for Archflow.

Archflow is a CLI tool, but it must be treated as an application with clear boundaries.
The project adopts a **Hexagonal Architecture (Ports and Adapters)** style on top of a **Modular Monolith** structure.

This document is written for both humans and generative AI.
When adding, editing, or refactoring code, these rules must be followed.

For review and day-to-day implementation:

- Current alignment note: `docs/architecture/current-state.md`
- Lightweight review checklist: `docs/architecture/refactor-checklist.md`
- PR review template: `.github/pull_request_template.md`

---

## 1. Architecture Decision

Archflow adopts the following architecture:

- **Style**: Hexagonal Architecture (Ports and Adapters)
- **Deployment shape**: Modular Monolith
- **Interaction model**: CLI entrypoints mapped to Application UseCases
- **Primary goal**: keep architecture decisions, planning logic, validation logic, and generation logic independent from I/O and framework details

Archflow is not a thin CLI wrapper.
It is an architecture decision engine exposed through a CLI.

---

## 2. Core Principles

### 2.1 Mandatory principles

The codebase MUST follow these principles:

1. **Domain-first**
   - Core business logic must live in the domain layer.
   - Domain logic must not depend on CLI, filesystem, network, or external services.

2. **Dependency inward**
   - Dependencies must point inward toward the domain.
   - Outer layers may depend on inner layers.
   - Inner layers must never depend on outer layers.

3. **Explicit boundaries**
   - File I/O, Git execution, template rendering, LLM calls, and console output must be isolated behind ports/interfaces.

4. **UseCase-oriented application flow**
   - Each CLI command must invoke an application-level UseCase.
   - CLI must not contain business logic.

5. **Modular growth**
   - New features must be added as modules with clear ownership and responsibilities.
   - Shared code must be minimized.

---

## 3. Layer Definitions

Archflow code must be separated into the following layers.

### 3.1 `cli/`

Responsibility:
- parse arguments
- map command-line input to application input
- call UseCases
- print or serialize results

Rules:
- MUST NOT contain business rules
- MUST NOT read or write files directly unless clearly limited to bootstrapping concerns
- MUST NOT call infrastructure code directly when a UseCase exists
- SHOULD remain thin

### 3.2 `app/`

Responsibility:
- orchestrate application behavior
- coordinate domain services and ports
- define UseCases
- transform input/output DTOs

Rules:
- MAY coordinate multiple domain objects and ports
- MUST NOT contain low-level I/O implementation details
- MUST NOT become a dumping ground for business rules that belong in domain
- SHOULD express workflows clearly

### 3.3 `domain/`

Responsibility:
- core concepts
- decision logic
- rules
- validations
- planning behavior
- invariants

Rules:
- MUST be independent from `clap`, `tokio`, `std::fs`, `reqwest`, git command execution, and terminal rendering
- MUST NOT know where data comes from or where results are written
- MUST contain the most important business logic in Archflow

### 3.4 `ports/`

Responsibility:
- define interfaces to the outside world

Examples:
- filesystem access
- git access
- llm client
- template rendering
- output rendering
- repository-like abstractions if needed

Rules:
- MUST be traits or equivalent abstractions
- MUST describe capability, not implementation
- MUST be stable and minimal

### 3.5 `infra/`

Responsibility:
- concrete implementations of ports
- actual filesystem access
- actual git execution
- actual API clients
- actual template engines
- actual console formatters

Rules:
- MUST implement ports
- MUST NOT contain core architecture judgment logic
- SHOULD be replaceable in tests

### 3.6 `shared/`

Responsibility:
- truly cross-cutting and stable primitives only

Rules:
- MUST remain small
- MUST NOT become a hidden dependency bucket
- MUST NOT be used as a place for unrelated utility functions

---

## 4. Required Directory Structure

The recommended structure is:

```text
src/
├─ main.rs
├─ cli/
│  ├─ mod.rs
│  ├─ args.rs
│  └─ commands/
├─ app/
│  ├─ mod.rs
│  ├─ dto/
│  └─ usecase/
├─ domain/
│  ├─ mod.rs
│  ├─ project/
│  ├─ preset/
│  ├─ planning/
│  ├─ validation/
│  └─ generation/
├─ ports/
│  ├─ mod.rs
│  ├─ filesystem.rs
│  ├─ git.rs
│  ├─ llm.rs
│  ├─ template.rs
│  └─ output.rs
├─ infra/
│  ├─ mod.rs
│  ├─ filesystem/
│  ├─ git/
│  ├─ llm/
│  ├─ template/
│  └─ output/
└─ shared/
   ├─ error.rs
   └─ result.rs
```

This structure is a rule, not a suggestion, unless there is a strong documented reason to deviate.

---

## 5. Dependency Rules

The following dependency rules are mandatory.

### Allowed directions

- `cli -> app`
- `app -> domain`
- `app -> ports`
- `infra -> ports`
- `infra -> domain` only when needed for data mapping or adapter integration
- `main -> cli`, `main -> app`, `main -> infra`

### Forbidden directions

- `domain -> app`
- `domain -> cli`
- `domain -> infra`
- `ports -> infra`
- `ports -> cli`
- `app -> cli`

### Important note

If a domain object needs data from the outside world, the application layer must obtain that data through a port and then pass it into the domain.
The domain must not fetch data by itself.

---

## 6. Feature Modeling Rules

Archflow features must be modeled around concepts, not technical mechanisms.

### Good module examples

- `project`
- `preset`
- `planning`
- `validation`
- `generation`

### Bad module examples

- `helpers`
- `common_utils`
- `services`
- `misc`
- `manager`
- `processor`

Generic names hide responsibility.
Responsibility must be visible in the module name.

---

## 7. Command Design Rules

Each top-level CLI command MUST map to a UseCase.

Examples:

- `archflow init` -> `InitProjectUseCase`
- `archflow plan` -> `PlanArchitectureUseCase`
- `archflow validate` -> `ValidateProjectUseCase`
- `archflow generate` -> `GenerateArtifactsUseCase`

### Mandatory rules

- A command handler MUST remain thin
- A command handler MUST only:
  - receive CLI args
  - convert them into input DTOs
  - call a UseCase
  - render output
- A command handler MUST NOT:
  - perform architecture decision logic
  - contain complex branching rules
  - directly manipulate external systems unless explicitly bootstrapping

---

## 8. Domain Rules

The domain layer is the heart of Archflow.

### Domain should contain

- project model
- preset model
- plan model
- validation rules
- architecture decisions
- recommendation rules
- invariants
- rule evaluation logic

### Domain must not contain

- terminal formatting
- JSON/YAML file reading logic
- HTTP calls
- shell command execution
- git process execution
- direct logging dependencies if avoidable
- framework-specific types unless there is no realistic alternative

### Domain coding rules

- Domain objects SHOULD be small and explicit
- Domain services SHOULD be deterministic where possible
- Domain logic SHOULD be testable without filesystem or network
- Hidden side effects are forbidden

---

## 9. Port and Adapter Rules

### Ports

Ports define required capabilities.

Examples:
- `FileSystem`
- `GitClient`
- `TemplateRenderer`
- `LlmClient`
- `OutputWriter`

### Port design rules

- A port MUST be focused
- A port MUST describe intent, not implementation detail
- A port MUST NOT expose adapter-specific details unless unavoidable
- Large “god ports” are forbidden

### Adapters

Adapters implement ports.

Examples:
- local filesystem adapter
- process-based git adapter
- OpenAI adapter
- local model adapter
- markdown renderer adapter
- console writer adapter

### Adapter rules

- An adapter MUST implement a port
- An adapter MUST NOT define business policy
- An adapter MAY translate third-party errors into internal errors
- An adapter SHOULD be swappable in tests

---

## 10. Configuration Rules

Archflow will likely read project configuration files such as `project.arch.yaml`.

### Mandatory configuration rules

- Parsing raw config files belongs outside domain
- Validation of structural format may happen near config loading
- Semantic interpretation belongs in domain or app, depending on responsibility
- The raw config format must not leak everywhere

Recommended pattern:

1. load raw file
2. parse into raw config struct
3. convert into domain-relevant config model
4. execute planning/validation logic

Do not mix parsing and business decisions in a single function.

---

## 11. Output Rules

Archflow may support multiple output formats such as text, markdown, and json.

### Mandatory rules

- Domain objects MUST NOT format themselves for CLI presentation
- Output formatting belongs in output adapters or presentation mapping code
- UseCases SHOULD return structured results, not pre-rendered terminal strings
- `--json` compatibility should be preserved where reasonable

Bad pattern:
- UseCase returns already-decorated terminal string

Good pattern:
- UseCase returns structured result
- output layer formats it for text/json/markdown

---

## 12. Error Handling Rules

### Mandatory rules

- Errors must be explicit
- Error boundaries should be clear
- Infrastructure errors should be translated into application-friendly errors
- Silent failure is forbidden

### Recommended approach

Use domain-specific and application-specific error types where it improves clarity.

Examples:
- `ConfigLoadError`
- `PresetResolutionError`
- `PlanBuildError`
- `ValidationError`
- `GenerationError`

### Forbidden patterns

- returning vague string errors everywhere
- swallowing errors
- panicking for expected failure cases
- mixing user-facing and internal diagnostic messages without structure

---

## 13. Testing Rules

Testing is mandatory because Archflow is a decision tool.

### Domain tests

Must verify:
- rule behavior
- planner decisions
- validator results
- preset resolution
- invariants

These tests MUST run without filesystem, network, or git dependencies.

### Application tests

Must verify:
- usecase orchestration
- port interaction
- result mapping

These tests SHOULD use fakes or mocks for ports.

### Integration tests

May verify:
- config loading
- real filesystem interaction
- real command behavior
- generated artifacts

### Mandatory testing principle

The more central the logic is to Archflow’s decision-making, the less it should depend on integration tests alone.

---

## 14. Naming Rules

### Modules

Use domain-oriented names.

Good:
- `planning`
- `validation`
- `preset`
- `project`

Bad:
- `utils`
- `stuff`
- `helper`
- `service_layer`

### Types

Type names must reveal responsibility.

Good:
- `ArchitecturePlanner`
- `PresetResolver`
- `ProjectConfig`
- `ValidationResult`

Bad:
- `Manager`
- `Processor`
- `Handler` when the action is unclear
- `Service` without precise meaning

### Functions

Function names should describe business intent.

Good:
- `build_plan`
- `resolve_preset`
- `validate_project`
- `generate_artifacts`

Bad:
- `handle`
- `run_all`
- `process_data`
- `do_work`

---

## 15. AI Coding Rules

The following rules are specifically for generative AI agents editing this repository.

### AI MUST

- preserve layer boundaries
- place business logic in domain
- place orchestration in app
- place I/O behind ports and infra adapters
- create small focused modules
- prefer explicit names over generic abstractions
- add or update tests when changing decision logic
- keep command handlers thin

### AI MUST NOT

- place business logic in `main.rs`
- place planning or validation rules inside CLI code
- add `utils.rs` unless there is a strong and specific reason
- directly call filesystem/network/process logic from domain
- introduce a new abstraction unless it solves a real boundary problem
- create “god objects” that know too much
- return terminal-decorated output from domain logic

### AI SHOULD

- prefer deterministic domain logic
- prefer structured return values
- separate parsing from interpretation
- keep traits minimal
- keep adapters replaceable
- document non-obvious boundary decisions

---

## 16. Anti-Patterns

The following are considered architecture violations.

### Forbidden anti-patterns

1. **Fat CLI**
   - command handlers doing real business decisions

2. **Anemic boundary collapse**
   - domain logic mixed with file parsing and writing

3. **Utility dumping ground**
   - growing `utils` or `common` modules with unclear ownership

4. **Adapter intelligence**
   - infra deciding business rules

5. **Stringly-typed flow**
   - passing raw strings everywhere instead of explicit domain types

6. **Premature framework lock-in**
   - designing domain around a specific library or external API

7. **Feature scattering**
   - one feature spread randomly across unrelated directories without clear module ownership

---

## 17. When Adding a New Feature

When implementing a new feature, use this sequence.

1. Define the feature in domain terms
2. Identify the UseCase
3. Define required ports
4. Implement domain logic
5. Implement application orchestration
6. Implement adapters
7. Add tests
8. Expose through CLI

### Example checklist

- What is the domain concept?
- Is this a new UseCase or part of an existing one?
- Does it require external I/O?
- If yes, is there already a port for it?
- If not, should a new port be added?
- Can the core decision logic be tested without real I/O?

---

## 18. Lightweight Decision Heuristics

Use these heuristics when uncertain.

### Put code in `domain/` when

- it answers “what is the correct architecture decision?”
- it validates rules
- it resolves presets
- it constructs plans
- it protects invariants

### Put code in `app/` when

- it answers “in what order should steps happen?”
- it coordinates ports with domain services
- it maps inputs and outputs

### Put code in `infra/` when

- it answers “how do we actually read/write/call this external thing?”

### Put code in `cli/` when

- it answers “how does a user invoke this from the terminal?”

---

## 19. Minimal Quality Gate

A change should not be merged unless all of the following are true:

- layer boundaries are preserved
- no new architecture violation is introduced
- command handler remains thin
- domain logic remains testable without real I/O
- naming is responsibility-driven
- new external access is abstracted appropriately
- tests are added or updated where decision logic changed

---

## 20. Final Rule

When there is a conflict between convenience and architecture consistency, prefer architecture consistency unless there is a documented reason not to.

Archflow is itself a tool for architecture thinking.
Its internal structure is part of the product’s credibility.
