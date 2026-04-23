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

## 2. Release Gating

A release is considered "Gated" by these criteria. No official release of Batonel shall be published if any of the following workflows fail:

1. **Batonel Verify Example** (G1)
2. **Onboarding Init-Plan E2E** (G2)
3. **Batonel Verify Parity** (G3)

For more details on the release process, see [docs/release-operations.md](./release-operations.md).

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
