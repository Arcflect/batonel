# Current State: Module Boundaries and Dependency Direction

Date: 2026-04-09
Target issue: #191

## Purpose

This note captures the current structure, dependency direction, and major boundary violations before refactoring.
It is based on the current code in `src/` and aligned against `docs/ARCHITECTURE_RULES.ja.md`.

## 1. Current module map

### CLI entry and command routing

- `src/main.rs`
  - Parses CLI args and dispatches to command functions.
- `src/cli.rs`
  - Defines command options and subcommands via `clap`.

### Command layer (currently mixed application + domain + I/O)

- `src/commands/plan.rs`
- `src/commands/scaffold.rs`
- `src/commands/verify.rs`
- `src/commands/audit.rs`
- `src/commands/fix.rs`
- `src/commands/fix_rollout.rs`
- `src/commands/triage.rs`
- `src/commands/guard.rs`
- `src/commands/prompt.rs`
- `src/commands/policy_resolve.rs`
- `src/commands/compliance_report.rs`
- `src/commands/preset_registry.rs`
- `src/commands/preset_verify.rs`
- `src/commands/preset_migrate.rs`
- `src/commands/init.rs`

### Config loading/parsing

- `src/config/project.rs`
- `src/config/placement.rs`
- `src/config/artifact.rs`
- `src/config/contract.rs`
- `src/config/policy.rs`
- `src/config/guard.rs`
- `src/config/override_policy.rs`
- `src/config/error.rs`

### Generation / path resolution

- `src/generator/resolver.rs`
- `src/generator/scaffold.rs`

### Domain/model and validation logic

- `src/model/project.rs`
- `src/model/placement.rs`
- `src/model/artifact.rs`
- `src/model/contract.rs`
- `src/model/prompt.rs`
- `src/model/verify.rs`
- `src/model/contract_validation.rs`
- `src/model/prompt_validation.rs`
- `src/model/scaffold_validation.rs`
- `src/model/status_validation.rs`

## 2. Current dependency direction (observed)

High-level observed direction today:

1. `main -> cli`
2. `main -> commands::*`
3. `commands::* -> config::*`
4. `commands::* -> generator::*`
5. `commands::* -> model::*`
6. `commands::* -> std::fs/std::path/std::process`
7. `generator::* -> model::*` and `config::placement`
8. `config::* -> model::*`
9. `model::* -> std::fs` in validation modules (`contract_validation`, `prompt_validation`)
10. `model::prompt -> cli::OutputMode`

This means dependency flow is not purely inward; cross-layer references exist both ways.

## 3. Boundary violations against architecture rules

Major violations found (with concrete evidence):

1. Domain depending on CLI type
- Evidence: `src/model/prompt.rs` uses `crate::cli::OutputMode`.
- Why violation: domain/model depends on outer interface type.

2. Domain validation modules perform direct filesystem I/O
- Evidence: `src/model/contract_validation.rs` and `src/model/prompt_validation.rs` use `std::fs` and read files.
- Why violation: domain logic is coupled to file access implementation.

3. Command layer contains orchestration + rule decisions + rendering + process exits
- Evidence: `src/commands/verify.rs`, `src/commands/audit.rs`, `src/commands/fix.rs`, `src/commands/scaffold.rs` all mix:
  - config loading,
  - business validation/decision logic,
  - console output,
  - `std::process::exit`.
- Why violation: CLI/application concerns and domain decisions are not separated.

4. Command layer directly depends on generator internals
- Evidence: calls to `crate::generator::resolver::resolve_artifact_path` and `resolve_sidecar_path` from `plan`, `verify`, `guard`, `audit`, `prompt`.
- Why violation: missing stable application/domain service boundary; multiple commands bind to low-level resolver functions.

5. Missing explicit ports/infra split for I/O
- Evidence: file reads/writes and output rendering are called directly from command/domain-adjacent modules.
- Why violation: filesystem/output are not behind ports/adapters, making boundary enforcement and testing harder.

## 4. Refactor targets (short)

Recommended refactor order to reduce risk:

1. Introduce layer skeleton (`cli`, `app`, `domain`, `ports`, `infra`) without behavior changes.
2. Move command execution logic into app use-cases; keep command files as thin adapters.
3. Extract filesystem and output ports; route all file and console operations via infra adapters.
4. Move rule/validation/planning logic into domain services and value objects.
5. Remove domain->cli dependency (`model::prompt` mode type), define domain-local presentation mode or app DTO.
6. Consolidate path resolution behind one domain/app service boundary; stop direct resolver calls from many commands.

## 5. Immediate high-impact refactor candidates

1. `verify` use-case split
- Separate: input loading, domain checks, output formatting.

2. `audit` + `triage` + `fix` family consolidation
- Centralize shared policy/finding model in domain.
- Keep rollout/approval workflow in app layer.

3. Prompt generation boundary cleanup
- Move output mode conversion to app/cli.
- Keep `model::prompt` independent of CLI types.

## 6. Done criteria status for this audit task

- [x] current modules/files identified for CLI/config/planning/validation/generation/I/O
- [x] dependency direction mapped
- [x] boundary violations listed against architecture rules
- [x] short refactor targets note prepared
