# Batonel Prompt Generation: Usage Examples

This document provides typical commands to initialize and verify prompt generation across the different example architectures provided in this repository.

## Prerequisites
Ensure the binary is built and available. You can run it via `cargo run` from the project root.

Official CLI distribution/update operations are documented in:

- `docs/release-operations.md`

---

## CLI Distribution Quick Start

For users, use the official installer script (Linux/macOS).

```bash
curl -fsSL https://raw.githubusercontent.com/Arcflect/batonel/main/scripts/install-batonel.sh | bash
```

Install a pinned version:

```bash
curl -fsSL https://raw.githubusercontent.com/Arcflect/batonel/main/scripts/install-batonel.sh | bash -s -- vX.Y.Z
```

For CI, prefer pinned binary installation with checksum verification.

```bash
BATONEL_VERSION=vX.Y.Z
if [[ "$(uname -m)" == "aarch64" || "$(uname -m)" == "arm64" ]]; then
  BATONEL_TARGET=aarch64-unknown-linux-gnu
else
  BATONEL_TARGET=x86_64-unknown-linux-gnu
fi

curl -fsSL -o batonel.tar.gz "https://github.com/Arcflect/batonel/releases/download/${BATONEL_VERSION}/batonel-${BATONEL_VERSION}-${BATONEL_TARGET}.tar.gz"
curl -fsSL -o batonel.tar.gz.sha256 "https://github.com/Arcflect/batonel/releases/download/${BATONEL_VERSION}/batonel-${BATONEL_VERSION}-${BATONEL_TARGET}.tar.gz.sha256"
sha256sum -c batonel.tar.gz.sha256
tar -xzf batonel.tar.gz
chmod +x batonel
./batonel --version
```

CI guideline:

- Pin `BATONEL_VERSION` explicitly (do not install floating latest)
- Cache by `${BATONEL_VERSION}-${BATONEL_TARGET}`
- Validate with `batonel --version` before running pipeline steps

---

## Preset Bootstrap: `batonel init`

Use `batonel init` as the minimal startup command.

Default initialization (no preset):

```bash
cargo run -- init
```

Preset-based initialization:

```bash
# Generic layered starter
cargo run -- init --preset generic-layered

# Rust clean/hexagonal starter
cargo run -- init --preset rust-clean-hexagonal

# Dry-run preview (no files are written)
cargo run -- init --preset generic-layered --project-name my-app --dry-run
```

Optional immediate override:

```bash
cargo run -- init --preset generic-layered --project-name my-app
```

Generated files in current directory:

- `project.baton.yaml`
- `placement.rules.yaml`
- `contracts.template.yaml`
- `artifacts.plan.yaml` (when included by the chosen preset)
- `policy.profile.yaml`
- `guard.sidecar.yaml`

Behavior notes:

- existing files are skipped (not overwritten)
- unknown preset id fails with an error and available preset ids
- empty/whitespace `--project-name` fails with an explicit error
- deeper customization is done by editing generated files after init

Onboarding e2e check for core presets:

```bash
bash scripts/onboarding_e2e_init_plan.sh --preset generic-layered --project-name e2e-generic-service
bash scripts/onboarding_e2e_init_plan.sh --preset rust-clean-hexagonal --project-name e2e-rust-service
```

The check validates:

- `init --dry-run` does not write files
- `init` generates the expected root config files
- `plan` output is stable across identical repeated runs

---

日本語メモ（Onboarding e2e）:

- `scripts/onboarding_e2e_init_plan.sh` は、core preset に対する `init -> plan` の最小e2e検証です。
- `--dry-run` の非破壊性、初期ファイル生成、`plan` 出力の再現性を確認します。

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

---

## Minimal CI Example: `batonel verify`

Use the workflow file below as a minimal GitHub Actions example:

- `.github/workflows/batonel-verify-example.yml`

This example runs `batonel verify` for each bundled example fixture:

- `examples/minimal/batonel`
- `examples/generic-layered/batonel`
- `examples/rust-clean-hexagonal/batonel`

Each matrix run also uploads the execution log as a workflow artifact:

- `batonel-verify-log-examples-minimal-batonel`
- `batonel-verify-log-examples-generic-layered-batonel`
- `batonel-verify-log-examples-rust-clean-hexagonal-batonel`

Core command pattern used in CI:

```bash
cd examples/minimal/batonel
cargo run --manifest-path ../../../Cargo.toml -- verify
```

Expected behavior:

- exit code `0`: verification succeeded (with or without warnings)
- exit code `1`: verification failed (at least one `Fail` check)

This is intentionally minimal and demonstrates the automation path without
introducing a full CI platform design.

---

## PR Gate Example: `batonel audit --strict`

Use the workflow file below as a baseline PR gate setup:

- `.github/workflows/batonel-audit-pr-gate.yml`

This example runs `batonel audit --strict` for each bundled example fixture:

- `examples/minimal/batonel`
- `examples/generic-layered/batonel`
- `examples/rust-clean-hexagonal/batonel`

Audit output includes rule-level diagnostics:

- `rule_id`
- `severity` (`error` or `warn`)
- remediation hints

Policy baseline behavior:

- loads `policy.profile.yaml` when present
- falls back to built-in minimum policy profile when missing
- supports project-level exceptions via explicit `overrides`

Core command pattern used in CI:

```bash
cd examples/minimal/batonel
cargo run --manifest-path ../../../Cargo.toml -- audit --strict
```

Expected behavior:

- exit code `0`: no errors and no warnings
- exit code `1`: at least one error, or warnings when `--strict` is enabled

---

## Multi-Repo Compliance Export: `batonel compliance-report`

`compliance-report` aggregates audit outcomes across multiple repositories and exports machine-readable metrics.

### JSON export

```bash
cargo run -- compliance-report \
  --repos examples/minimal/batonel \
  --repos examples/generic-layered/batonel \
  --format json \
  --output compliance-report.json
```

JSON report includes:

- per-repository status and error/warning counts
- organization-level totals (`total_errors`, `total_warnings`, `failed_count`)
- rule-level summary (`rule_summaries`) aggregated across all repositories

### CSV export

```bash
cargo run -- compliance-report \
  --repos examples/minimal/batonel \
  --repos examples/generic-layered/batonel \
  --format csv \
  --output compliance-report.csv
```

CSV contains normalized sections:

- `summary`: global totals
- `repo`: one row per repository
- `rule`: one row per rule-level aggregate
- `trend` / `trend_rule`: delta metrics when a baseline report is provided

### Repos file mode

You can pass repository targets via a newline-delimited file:

```bash
cargo run -- compliance-report \
  --repos-file repos.txt \
  --format json \
  --output compliance-report.json
```

`repos.txt` rules:

- one repository path per line
- blank lines ignored
- lines starting with `#` treated as comments

### Severity trends and rule deltas

Use `--baseline-json` to compute trend deltas vs a previous JSON export:

```bash
cargo run -- compliance-report \
  --repos-file repos.txt \
  --format json \
  --output compliance-report-current.json \
  --baseline-json compliance-report-previous.json
```

Trend metrics include:

- `error_delta`, `warning_delta`, `finding_delta`
- `failed_repo_delta`, `passed_repo_delta`
- per-rule deltas (`rule_trends`) with error/warning/total change

---

## Conservative Remediation: `batonel fix`

`batonel fix` introduces conservative automation boundaries.

Policy in this phase:

- auto-fix only low-risk structural gaps (currently: missing root config files)
- keep semantic or architectural decisions as review-required
- include patch previews for review-required findings

Dry-run preview (recommended default workflow):

```bash
cargo run -- fix --dry-run
```

Limited apply mode (low-risk fixes only):

```bash
cargo run -- fix --apply
```

Current classification examples:

- auto-fixable:
	- missing `project.baton.yaml` / `placement.rules.yaml` / `artifacts.plan.yaml` / `contracts.template.yaml`
- review-required:
	- artifact references undefined module
	- artifact uses undefined role
	- explicit artifact path deviates from role default path

`--apply` never auto-applies review-required findings. Instead it emits patch-oriented
previews and exits non-zero so human review remains mandatory.

---

## Preset Registry Prototype: `preset-publish` / `preset-install` / `preset-verify`

Batonel provides a prototype local registry workflow for preset sharing and alignment verification.

### Verify preset alignment: `preset-verify`

Before publishing or sharing a preset, verify that it preserves contract-first and sidecar-first behavior:

```bash
# Verify a preset directory
cargo run -- preset-verify --preset-dir presets/generic-layered

# Fail on warnings as well as errors
cargo run -- preset-verify --preset-dir presets/generic-layered --strict
```

**Contract-first checks:**

| Rule ID | Severity | Description |
|---------|----------|-------------|
| `preset-contracts-template-in-required` | error | `contracts.template.yaml` must be in `includes.required`, not optional |
| `preset-placement-rules-in-required` | error | `placement.rules.yaml` must be in `includes.required` |
| `preset-contracts-template-has-role-templates` | error | `contracts.template.yaml` must define at least one `role_templates` entry |
| `preset-role-template-has-responsibilities` | warn | Each role template should define at least one responsibility |
| `preset-placement-role-in-template` | warn | Each placement role should have a matching `role_templates` entry in `contracts.template.yaml` |

**Sidecar-first checks:**

| Rule ID | Severity | Description |
|---------|----------|-------------|
| `preset-guard-sidecar-present` | warn | `guard.sidecar.yaml` should appear in preset includes |
| `preset-guard-ci-hook-enabled` | warn | `hooks.ci` must be `true` in `guard.sidecar.yaml` |
| `preset-guard-checks-require-contracts-template` | warn | `checks.require_contracts_template` must be `true` |
| `preset-guard-checks-enforce-sidecar-suffixes` | warn | `checks.enforce_sidecar_suffixes` must be `true` |
| `preset-placement-file-extension-present` | warn | Each placement role should define `file_extension` for sidecar suffix resolution |

These checks are also run automatically during `preset-publish`. A publish attempt with alignment errors will be rejected.

### Publish a preset package

```bash
cargo run -- preset-publish --preset-dir presets/generic-layered --registry-dir .batonel/registry
```

Publish validation checks:

- `preset.yaml` must parse and `name` must match directory name
- `package.version` and compatibility versions must be semver (`x.y.z`)
- `package.visibility` must be `public` or `private`
- required includes must contain and resolve to existing files
- duplicate `(id, version)` entries are rejected
- contract-first and sidecar-first alignment checks run automatically (errors block publish, warnings are reported)

### Install a preset package

```bash
# latest compatible version
cargo run -- preset-install --preset generic-layered --registry-dir .batonel/registry --destination-dir presets

# explicit version
cargo run -- preset-install --preset generic-layered --preset-version 0.1.0 --registry-dir .batonel/registry --destination-dir presets
```

Install validation checks:

- preset id/version must exist in registry index
- compatibility range must include current Batonel version
- project/policy schema compatibility must match current runtime support
- destination preset directory must not already exist

Registry index format:

- registry root: `<registry-dir>`
- index file: `<registry-dir>/index.yaml`
- package files: `<registry-dir>/packages/<preset-id>/<version>/...`

---

## Sidecar Guard Checks: `batonel guard`

Guard checks provide sidecar-first policy enforcement with hook points for:

- `init`
- `plan`
- `ci`

Run guard explicitly:

```bash
# CI-oriented enforcement (warnings fail in strict mode)
cargo run -- guard --hook ci --strict

# Local checks aligned with plan hook
cargo run -- guard --hook plan
```

Diagnostics format is aligned with `audit` output:

- `rule_id`
- `severity` (`error` or `warn`)
- `target`
- remediation hint

Fallback behavior when guard rules are unavailable:

- if `guard.sidecar.yaml` is missing, guard emits `guard-rules-unavailable` as a warning
- fallback defaults are applied automatically to keep checks running
- if `guard.sidecar.yaml` exists but is invalid, guard still falls back and reports the warning

---

## Preset Migration: `preset-migration-plan` / `preset-migration-apply`

Batonel provides preset versioning and migration tooling to help projects upgrade to a newer preset version while preserving architecture guarantees and identifying conflicts early.

### Generate a migration plan

```bash
cargo run -- preset-migration-plan \
  --preset generic-layered \
  --from-version 0.1.0 \
  --to-version 0.2.0 \
  --registry-dir .batonel/registry \
  --project-dir .
```

The migration plan compares each file from the old preset version against the target and classifies every change:

| Class | Meaning |
|-------|---------|
| `add` | File is new in the target preset and does not exist locally — safe to add |
| `update` | File changed in the target preset and local copy matches the old preset — safe to overwrite |
| `conflict` | File changed in both the project and the target preset, or is new in the target but already exists locally — manual review required |
| `unchanged` | File is the same between preset versions — no action needed |

The plan exits with code 1 if any conflicts are detected. All actionable steps include a patch preview.

### Apply a migration plan

```bash
# Preview only (no files written)
cargo run -- preset-migration-apply \
  --preset generic-layered \
  --from-version 0.1.0 \
  --to-version 0.2.0 \
  --registry-dir .batonel/registry \
  --project-dir . \
  --dry-run

# Apply safe changes (add + update only; conflicts are never auto-applied)
cargo run -- preset-migration-apply \
  --preset generic-layered \
  --from-version 0.1.0 \
  --to-version 0.2.0 \
  --registry-dir .batonel/registry \
  --project-dir .
```

Apply behavior:

- `add` and `update` steps are applied automatically
- `conflict` steps are **never** auto-applied — they must be resolved manually
- Before overwriting any existing file, a backup is created in `.batonel/migration-backup/<from-version>/`
- **Rollback**: restore files from the backup directory if the migration result is unsatisfactory

Apply aborts before writing anything if conflicts are present. Resolve conflicts first, then re-run apply.

---

## Org/Team Override Precedence Model: `policy-resolve`

Batonel implements a three-level policy override precedence chain:

```
org  →  team  →  project  →  default
(highest priority)      (lowest priority)
```

Each level contributes overrides, naming rules, forbidden-dependency policies, and required-file
requirements. The effective policy used by `batonel audit` is computed from all available layers.

### Layer files

| Level | Default path | Notes |
|-------|--------------|-------|
| org | `.batonel/org.policy.yaml` | Applies to all repositories in the organisation |
| team | `.batonel/team.policy.yaml` | Applies to all repositories owned by the team |
| project | `policy.profile.yaml` | Repository-local overrides (existing format) |

### Locking rules

Org and team layers can lock specific audit rules to prevent lower-priority layers from overriding them:

```yaml
# .batonel/org.policy.yaml
version: 1
label: acme-org
locked_rules:
  - module-name-policy
  - artifact-name-policy
overrides: []
```

A project override for a locked rule is silently ignored. Run `policy-resolve` to see which rules are locked.

### Show effective policy

```bash
# Use standard lookup paths automatically
cargo run -- policy-resolve

# Specify explicit paths for each layer (useful in CI or multi-tenant setups)
cargo run -- policy-resolve \
  --org-policy .batonel/org.policy.yaml \
  --team-policy .batonel/team.policy.yaml \
  --project-policy policy.profile.yaml
```

Output includes:
- Precedence chain
- Locked rules and their source level
- Effective naming rules with source attribution
- Union of all required files and where each comes from
- Effective overrides (deduplicated by rule_id + target; highest level wins)
- Summary of which rules cannot be overridden at project level

### Resolution rules

| Dimension | Resolution strategy |
|-----------|-------------------|
| Naming rules | Highest-priority level that defines naming wins |
| Required files | Union of all levels (additive) |
| Forbidden dependencies | Highest-priority level wins per role |
| Overrides | Highest-priority level wins per (rule_id, target) pair |
| Locked rules | Union of org + team locked rules; project cannot lock |

### Example: conflict behavior

If both org and project try to override `module-name-policy` for `module:LegacyModule`:
- org's entry wins (org has higher precedence)
- only one resolved entry appears in the output
- project's duplicate entry is discarded (not an error)

If org locks `module-name-policy` and project tries to override it:
- project's override is silently blocked
- `policy-resolve` shows the locked rule alongside the constraint
- audit respects the lock and applies the rule without exception

---

## Preset-Based Workflow Examples

For small workflow examples aimed at preset-based repositories, see:

- `examples/preset-repository-patterns/workflows/README.md`
- `examples/preset-repository-patterns/workflows/verify-preset-project.yml`
- `examples/preset-repository-patterns/workflows/plan-scaffold-prompt-preview.yml`

These files illustrate where `plan`, `scaffold`, `prompt`, and `verify` can fit
in normal repository automation without introducing a large CI framework.
