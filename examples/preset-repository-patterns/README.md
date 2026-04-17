# Preset Repository Patterns

This directory provides onboarding-focused repository patterns that show how
Batonel presets fit realistic project layouts.

These are not full applications.
They are intentionally small mapping guides from preset defaults to repository
structure.

## Included patterns

- `minimal-starter/`
- `generic-layered-service/`
- `rust-clean-hexagonal-workspace/`

## How to use these patterns

1. Pick a preset and initialize a project.
2. Compare your target repository layout with the pattern guide.
3. Adjust generated config files to fit naming and module boundaries.

Example bootstrap commands:

```bash
# Minimal baseline
batonel init --project-name my-minimal-app

# Language-agnostic layered baseline
batonel init --preset generic-layered --project-name my-service

# Rust clean/hexagonal baseline
batonel init --preset rust-clean-hexagonal --project-name my-rust-service
```

Each pattern includes:

- a realistic top-level repository layout
- mapping notes from preset defaults to directories
- minimal adaptation hints

---

## GitHub Workflow Examples

For minimal GitHub Actions examples that show where Batonel commands can fit,
see:

- `examples/preset-repository-patterns/workflows/`

Included examples:

- `examples/preset-repository-patterns/workflows/verify-preset-project.yml`
- `examples/preset-repository-patterns/workflows/plan-scaffold-prompt-preview.yml`
