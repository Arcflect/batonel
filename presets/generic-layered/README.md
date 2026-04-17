# generic-layered preset

Language-agnostic layered starter preset for Batonel.

## Purpose

This preset provides reusable defaults for teams that want layered boundaries
without committing to one language ecosystem.

## Included files

Required files:

- `preset.yaml`
- `project.baton.yaml`
- `placement.rules.yaml`
- `contracts.template.yaml`
- `README.md`

Optional starter file:

- `artifacts.plan.yaml`

## Alignment

This preset aligns with the `examples/generic-layered` direction and keeps the
same role boundaries:

- `entity`
- `service`
- `repository_interface`
- `controller`
- `gateway`

## How to start

1. Copy `project.baton.yaml`, `placement.rules.yaml`, and `contracts.template.yaml`
   into your target project root.
2. Optionally copy `artifacts.plan.yaml` as a starter artifact inventory.
3. Adjust module names, artifact names, and `file_extension` values for your ecosystem.

## Notes

- Default `file_extension` values follow the current `examples/generic-layered` defaults (`rs`) for alignment.
- Teams can override `file_extension` immediately for their ecosystem after bootstrap.
- This preset is a broad starter package, not a framework-specific template.
