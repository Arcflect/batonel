# Pattern: Generic Layered Service Repository

Preset:

- `generic-layered`

## Repository shape (small but realistic)

```text
my-service-repo/
  apps/
    api/
      src/
        application/
          services/
        domain/
          entities/
        interfaces/
          controllers/
        infrastructure/
          gateways/
  docs/
  batonel/
    project.baton.yaml
    placement.rules.yaml
    contracts.template.yaml
    artifacts.plan.yaml
```

## Mapping from preset defaults

- preset role `entity` -> `src/domain/entities/`
- preset role `service` -> `src/application/services/`
- preset role `repository_interface` -> `src/application/interfaces/`
- preset role `controller` -> `src/interfaces/controllers/`
- preset role `gateway` -> `src/infrastructure/gateways/`

## Practical adaptation notes

- If source code lives under `apps/api/`, set paths in `placement.rules.yaml` to include that prefix.
- Keep role semantics intact even if directory names change.
- Keep `contracts.template.yaml` role boundaries broad and framework-neutral.

## Minimal override example

```yaml
# placement.rules.yaml (repo-specific override)
roles:
  service:
    path: "apps/api/src/application/services/"
    file_extension: ts
```

This keeps preset intent while adapting to repository conventions.
