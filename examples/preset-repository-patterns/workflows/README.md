# GitHub Workflow Examples for Preset-Based Projects

These workflows are minimal examples for repositories that started from an
Batonel preset.

They are intentionally small and illustrative.

## Included examples

- `verify-preset-project.yml`
- `plan-scaffold-prompt-preview.yml`

## Assumed repository shape

The examples assume Batonel config files are under `batonel/`:

- `batonel/project.baton.yaml`
- `batonel/placement.rules.yaml`
- `batonel/contracts.template.yaml`
- `batonel/artifacts.plan.yaml`

If your repository keeps config files in a different location, adjust
`working-directory` accordingly.

## Installation step note

The workflows include a pinned Batonel binary installation step:

```bash
BATONEL_VERSION="v0.1.0"
BATONEL_TARGET="x86_64-unknown-linux-gnu"
curl -fsSL -o batonel.tar.gz "https://github.com/Arcflect/batonel/releases/download/${BATONEL_VERSION}/batonel-${BATONEL_VERSION}-${BATONEL_TARGET}.tar.gz"
curl -fsSL -o batonel.tar.gz.sha256 "https://github.com/Arcflect/batonel/releases/download/${BATONEL_VERSION}/batonel-${BATONEL_VERSION}-${BATONEL_TARGET}.tar.gz.sha256"
sha256sum -c batonel.tar.gz.sha256
tar -xzf batonel.tar.gz
chmod +x batonel
sudo mv batonel /usr/local/bin/batonel
batonel --version
```

Use a fixed version in CI and rotate intentionally. See also:

- `docs/release-operations.md`
