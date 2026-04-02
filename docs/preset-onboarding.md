# Preset Onboarding Guide

This guide helps new users start projects quickly from Archflow presets.

It focuses on practical onboarding, not advanced customization.

---

## What presets are available

Current implemented presets:

- `generic-layered`
- `rust-clean-hexagonal`

Preset packages live under:

- `presets/generic-layered/`
- `presets/rust-clean-hexagonal/`

Each preset includes reusable defaults for:

- project context (`project.arch.yaml`)
- role-to-path placement (`placement.rules.yaml`)
- role-based contract defaults (`contracts.template.yaml`)
- optional starter artifacts (`artifacts.plan.yaml`)

---

## Which preset to choose

Use `generic-layered` when:

- you want a language-agnostic starting point
- your team wants layered boundaries first
- you have not committed to a Rust-specific workspace structure

Use `rust-clean-hexagonal` when:

- your project is Rust-oriented
- you want explicit domain/application/adapter boundaries
- you plan to use a workspace-style crate layout

Simple rule of thumb:

- unsure -> start with `generic-layered`
- Rust-first architecture -> start with `rust-clean-hexagonal`

---

## Minimal bootstrap path

### 1. Initialize from a preset

```bash
# Generic layered baseline
archflow init --preset generic-layered --project-name my-service

# Rust clean/hexagonal baseline
archflow init --preset rust-clean-hexagonal --project-name my-rust-service
```

Generated files in the current directory:

- `project.arch.yaml`
- `placement.rules.yaml`
- `contracts.template.yaml`
- `artifacts.plan.yaml` (when included by the preset)

### 2. Continue standard flow

```bash
archflow plan
archflow scaffold
```

Use prompt or verify as needed:

```bash
archflow prompt <artifact>
archflow verify
```

---

## Immediate override scope

The first onboarding flow intentionally keeps overrides minimal.

Supported at init time:

- `--project-name <name>`

Not included at init time:

- per-role override flags
- interactive role/path wizard
- partial merge semantics across multiple preset files

For deeper customization, edit generated config files directly.

---

## How presets relate to examples

Examples are teaching assets.
Presets are reusable startup packages.

Current directional mapping:

- `examples/generic-layered` -> `generic-layered` preset
- `examples/rust-clean-hexagonal` -> `rust-clean-hexagonal` preset

Repository-shape onboarding aids:

- `examples/preset-repository-patterns/minimal-starter/`
- `examples/preset-repository-patterns/generic-layered-service/`
- `examples/preset-repository-patterns/rust-clean-hexagonal-workspace/`

These pattern guides show how preset defaults map into realistic repositories
without requiring full application implementations.

---

## Customization checklist after bootstrap

After `archflow init --preset ...`, review these in order:

1. `project.arch.yaml`
2. `placement.rules.yaml`
3. `contracts.template.yaml`
4. `artifacts.plan.yaml` (if present)

Recommended first edits:

- set module names to your domain language
- align path prefixes with your repository layout
- refine role template boundaries for your team conventions
- remove starter artifacts you do not need

Keep customization architecture-level first.
Avoid framework-specific detail in early contracts.

---

## Related docs

- [docs/presets.md](docs/presets.md)
- [docs/usage.md](docs/usage.md)
- [examples/README.md](examples/README.md)
- [examples/preset-repository-patterns/README.md](examples/preset-repository-patterns/README.md)
- [docs/decisions/0015-define-minimal-preset-model-for-phase-5.md](docs/decisions/0015-define-minimal-preset-model-for-phase-5.md)
- [docs/decisions/0016-define-preset-packaging-approach.md](docs/decisions/0016-define-preset-packaging-approach.md)
- [docs/decisions/0017-formalize-example-to-preset-mapping.md](docs/decisions/0017-formalize-example-to-preset-mapping.md)
- [docs/decisions/0018-design-minimal-project-bootstrap-flow-from-presets.md](docs/decisions/0018-design-minimal-project-bootstrap-flow-from-presets.md)
