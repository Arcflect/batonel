# rust-clean-hexagonal preset

Rust-oriented clean/hexagonal starter preset for Archflow.

## Purpose

This preset provides reusable defaults for Rust projects that want stronger
architectural boundaries across domain, application, and adapters.

## Included files

Required files:

- `preset.yaml`
- `project.arch.yaml`
- `placement.rules.yaml`
- `contracts.template.yaml`
- `README.md`

Optional starter file:

- `artifacts.plan.yaml`

## Alignment

This preset aligns with the `examples/rust-clean-hexagonal` direction and keeps
its Rust-friendly architectural role boundaries:

- `entity`
- `usecase`
- `repository_port`
- `http_handler`
- `repository_impl`

## How to start

1. Copy `project.arch.yaml`, `placement.rules.yaml`, and `contracts.template.yaml`
   into your target project root.
2. Optionally copy `artifacts.plan.yaml` as a starter artifact inventory.
3. Adjust module names and artifact names while preserving role boundary intent.

## Notes

- This preset stays at architecture-style level.
- It intentionally avoids framework-specific conventions.
