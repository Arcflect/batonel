# Pattern: Rust Clean/Hexagonal Workspace Repository

Preset:

- `rust-clean-hexagonal`

## Repository shape (small but realistic)

```text
my-rust-service/
  crates/
    domain/
      src/entities/
    application/
      src/usecases/
      src/ports/outbound/
    adapters/
      http/
        src/handlers/
      db/
        src/repositories/
  Cargo.toml
  Cargo.lock
  archflow/
    project.arch.yaml
    placement.rules.yaml
    contracts.template.yaml
    artifacts.plan.yaml
```

## Mapping from preset defaults

- role `entity` -> `crates/domain/src/entities/`
- role `usecase` -> `crates/application/src/usecases/`
- role `repository_port` -> `crates/application/src/ports/outbound/`
- role `http_handler` -> `crates/adapters/http/src/handlers/`
- role `repository_impl` -> `crates/adapters/db/src/repositories/`

## Practical adaptation notes

- Keep workspace crate boundaries stable before increasing artifact count.
- Add adapter crates (e.g., messaging) by introducing new roles and paths, not by overloading existing roles.
- Keep contracts at architecture level; avoid framework-specific policy in templates.

## Minimal override example

```yaml
# project.arch.yaml (repo naming override)
project:
  name: my-rust-service
  architecture_style: clean-hexagonal
  language: rust
```

This keeps the preset boundary model while adopting repository-specific naming.
