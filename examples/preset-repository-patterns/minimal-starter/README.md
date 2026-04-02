# Pattern: Minimal Starter Repository

Preset:

- default `archflow init` output (minimal baseline)

## Repository shape (small but realistic)

```text
my-minimal-app/
  src/
    application/
      usecases/
    domain/
      entities/
  archflow/
    project.arch.yaml
    placement.rules.yaml
    contracts.template.yaml
    artifacts.plan.yaml
  README.md
```

## Mapping from baseline defaults

- role `usecase` -> `src/application/usecases/`
- role `entity` -> `src/domain/entities/`
- default artifacts:
  - `create_user` (usecase)
  - `user` (entity)

## Practical adaptation notes

- Start with default minimal roles, then add new roles only when boundaries are clear.
- Keep paths simple at first; move to layered or workspace patterns only when complexity grows.
- Keep contract templates concise and focused on architectural intent.

## Minimal override example

```yaml
# project.arch.yaml (repo naming override)
project:
  name: my-minimal-app
  architecture_style: simple
  language: generic
```

This pattern is designed for quick onboarding before adopting larger preset structures.
