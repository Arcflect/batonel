# Batonel Acceptance Criteria

This document defines the product-level guarantees that **Batonel** must satisfy to be considered stable and release-ready. 
These criteria elevate core CI workflows from implementation details to documented product promises, rooted in the [Batonel Quality & Trust Model](./quality-model.md).

---

## 1. Product Guarantees

Batonel provides the following guarantees to users and contributors. These are automatically verified in CI for every pull request and release.

### G1: Core Verification Integrity
**Guarantee**: The `verify` command correctly identifies structural and contract consistency in standard architectural patterns.
- **Verification Workflow**: `.github/workflows/batonel-verify-example.yml`
- **Scope**: Covers `minimal`, `generic-layered`, and `rust-clean-hexagonal` examples.
- **Failure Condition**: Any `batonel verify` command returning a non-zero exit code on standard examples.

### G2: Onboarding Experience & Determinism
**Guarantee**: New projects can be initialized from presets, and the resulting `plan` output is deterministic.
- **Verification Workflow**: `.github/workflows/onboarding-init-plan-e2e.yml`
- **Scope**: 
    - `init --dry-run` must not write any files.
    - `init` must generate all required root configuration files.
    - `plan` must produce identical output when run multiple times on the same input.
- **Failure Condition**: Missing files after `init`, non-deterministic `plan` output, or unexpected side effects during dry-run.

### G3: Ecosystem Consistency (Parity)
**Guarantee**: Examples and Presets remain byte-for-byte identical for shared configurations, and generated prompts are synchronized.
- **Verification Workflow**: `.github/workflows/batonel-verify-parity.yml`
- **Scope**: 
    - Parity between `presets/` and `examples/*/batonel/`.
    - Synchronization of expected prompts in examples.
    - Schema version consistency (must use `schema_version: "1"`).
- **Failure Condition**: Any drift between presets and examples, or out-of-sync prompt artifacts.

---

## 2. Release Gating (The Release Contract)

A release is considered "Gated" by these criteria. No official release of Batonel shall be published if any of the following blocking workflows fail.

### Blocking Release Gates
These checks are mandatory. A failure here explicitly prevents a release artifact from being published or trusted:

1. **Tag Version Integrity** (`.github/workflows/verify-tag-version.yml`): Ensures the Git tag version strictly matches the `Cargo.toml` version.
2. **Preset Trust & Signing** (`.github/workflows/preset-trust-verification.yml`): Validates cryptographic Ed25519 signatures of core presets using `scripts/verify_trust.sh`.
3. **Core Verification Integrity** (`.github/workflows/batonel-verify-example.yml`): Verifies (G1).
4. **Onboarding Determinism** (`.github/workflows/onboarding-init-plan-e2e.yml`): Verifies (G2) using `scripts/onboarding_e2e_init_plan.sh`.
5. **Ecosystem Parity** (`.github/workflows/batonel-verify-parity.yml`): Verifies (G3) using `scripts/verify_parity.sh`.
6. **Rust Native Integration Tests** (`cargo test --test cli`): Ensures core commands (`init`, `plan`, `scaffold`, `verify`) execute correctly against a real filesystem.

### Non-Blocking / PR-Level Gates
These checks act as advisory quality gates during PR review. While they are expected to pass on `main` for repository health, they do not technically block the creation of release artifacts:

1. **Audit Baseline PR Gate** (`.github/workflows/batonel-audit-pr-gate.yml`): Verifies architectural compliance for the Batonel repository itself.
2. **Guard Sidecar Checks** (`.github/workflows/batonel-guard-sidecar.yml`): Executes sidecar verifications explicitly.

For more details on the release process and deliverables, see [docs/release-operations.md](./release-operations.md).

---

## 3. Identified Acceptance Gaps

The following areas are currently identified as "Missing Coverage" and are planned for future acceptance criteria:

- [ ] **Cross-platform Verification**: Currently, E2E flows only run on Linux. macOS and Windows verification is needed.
- [ ] **Scaffold E2E**: The `scaffold` command is not yet fully verified in an automated onboarding flow.
- [ ] **Prompt Quality Assertion**: While prompt *presence* is checked, prompt *content* effectiveness is not yet automatically measured.
- [ ] **Audit/Policy Gate**: Integration of the `audit` command as a mandatory PR gate for downstream projects (Phase 7).

---

## 4. Definition of Done (DoD) for New Features

Every new feature or architectural change must:
1. **Satisfy existing guarantees**: Must not break G1, G2, or G3.
2. **Update examples**: If behavior changes, the corresponding `examples/` must be updated to maintain parity.
3. **Extend criteria**: If a new core capability is added (e.g., `audit`), a corresponding acceptance criterion and workflow should be established.
