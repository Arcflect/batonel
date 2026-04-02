# GitHub Workflow Examples for Preset-Based Projects

These workflows are minimal examples for repositories that started from an
Archflow preset.

They are intentionally small and illustrative.

## Included examples

- `verify-preset-project.yml`
- `plan-scaffold-prompt-preview.yml`

## Assumed repository shape

The examples assume Archflow config files are under `archflow/`:

- `archflow/project.arch.yaml`
- `archflow/placement.rules.yaml`
- `archflow/contracts.template.yaml`
- `archflow/artifacts.plan.yaml`

If your repository keeps config files in a different location, adjust
`working-directory` accordingly.

## Installation step note

The workflows include a minimal Archflow installation step:

```bash
cargo install archflow --locked || cargo install --git https://github.com/Arcflect/archflow --locked
```

If your team uses a fixed binary cache or pinned release strategy, replace this
step with your standard installation method.
