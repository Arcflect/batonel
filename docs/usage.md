# Archflow Prompt Generation: Usage Examples

This document provides typical commands to initialize and verify prompt generation across the different example architectures provided in this repository.

## Prerequisites
Ensure the binary is built and available. You can run it via `cargo run` from the project root.

---

## Preset Bootstrap: `archflow init`

Use `archflow init` as the minimal startup command.

Default initialization (no preset):

```bash
cargo run -- init
```

Preset-based initialization:

```bash
# Generic layered starter
cargo run -- init --preset generic-layered

# Rust clean/hexagonal starter
cargo run -- init --preset rust-clean-hexagonal
```

Optional immediate override:

```bash
cargo run -- init --preset generic-layered --project-name my-app
```

Generated files in current directory:

- `project.arch.yaml`
- `placement.rules.yaml`
- `contracts.template.yaml`
- `artifacts.plan.yaml` (when included by the chosen preset)

Behavior notes:

- existing files are skipped (not overwritten)
- unknown preset id fails with an error and available preset ids
- deeper customization is done by editing generated files after init

---

## 1. Minimal Example
A flat architecture with simple domain and application layers.

```bash
# Navigate to the archflow configuration directory
cd examples/minimal/archflow

# Step A: Generate missing contract and source placeholders
cargo run --manifest-path ../../../Cargo.toml -- scaffold

# Step B: Generate a prompt for an Entity
cargo run --manifest-path ../../../Cargo.toml -- prompt user

# Step C: Generate a prompt for a Usecase
cargo run --manifest-path ../../../Cargo.toml -- prompt create_user
```

## 2. Generic Layered Example
A traditional N-tier layered architecture.

```bash
cd examples/generic-layered/archflow

# Generate sidecars
cargo run --manifest-path ../../../Cargo.toml -- scaffold

# Generate a prompt for an HTTP Controller (Handler)
cargo run --manifest-path ../../../Cargo.toml -- prompt create_user_controller

# Generate a prompt for a Repository Port
cargo run --manifest-path ../../../Cargo.toml -- prompt user_repository
```

## 3. Rust Clean Hexagonal Example
A sophisticated Hexagonal (Ports & Adapters) architecture with crate isolation.

```bash
cd examples/rust-clean-hexagonal/archflow

# Generate sidecars
cargo run --manifest-path ../../../Cargo.toml -- scaffold

# Generate a prompt for a Port Implementation (Infrastructure)
cargo run --manifest-path ../../../Cargo.toml -- prompt postgres_user_repository

# Generate a prompt for an HTTP Handler Adapter
cargo run --manifest-path ../../../Cargo.toml -- prompt create_user_handler
```

---

## Output Options

### Compact Mode
Optimized for smaller LLM context windows or lightweight models, stripping metadata headers and list spacing.
```bash
cargo run --manifest-path [PATH_TO_CARGO_TOML] -- prompt [ARTIFACT] --mode compact
```

### Standard Mode (Default)
Human-readable Markdown with clear headers and full context.
```bash
cargo run --manifest-path [PATH_TO_CARGO_TOML] -- prompt [ARTIFACT] --mode standard
```

---

## Minimal CI Example: `archflow verify`

Use the workflow file below as a minimal GitHub Actions example:

- `.github/workflows/archflow-verify-example.yml`

This example runs `archflow verify` for each bundled example fixture:

- `examples/minimal/archflow`
- `examples/generic-layered/archflow`
- `examples/rust-clean-hexagonal/archflow`

Each matrix run also uploads the execution log as a workflow artifact:

- `archflow-verify-log-examples-minimal-archflow`
- `archflow-verify-log-examples-generic-layered-archflow`
- `archflow-verify-log-examples-rust-clean-hexagonal-archflow`

Core command pattern used in CI:

```bash
cd examples/minimal/archflow
cargo run --manifest-path ../../../Cargo.toml -- verify
```

Expected behavior:

- exit code `0`: verification succeeded (with or without warnings)
- exit code `1`: verification failed (at least one `Fail` check)

This is intentionally minimal and demonstrates the automation path without
introducing a full CI platform design.

---

## Preset-Based Workflow Examples

For small workflow examples aimed at preset-based repositories, see:

- `examples/preset-repository-patterns/workflows/README.md`
- `examples/preset-repository-patterns/workflows/verify-preset-project.yml`
- `examples/preset-repository-patterns/workflows/plan-scaffold-prompt-preview.yml`

These files illustrate where `plan`, `scaffold`, `prompt`, and `verify` can fit
in normal repository automation without introducing a large CI framework.
