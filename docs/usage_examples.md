# Batonel Prompt Generation: Usage Examples

This document provides typical commands to initialize and verify prompt generation across the different example architectures provided in this repository.

## Prerequisites
Ensure the binary is built and available. You can run it via `cargo run` from the project root.

---

## 1. Minimal Example
A flat architecture with simple domain and application layers.

```bash
# Navigate to the batonel configuration directory
cd examples/minimal/batonel

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
cd examples/generic-layered/batonel

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
cd examples/rust-clean-hexagonal/batonel

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
