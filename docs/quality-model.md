# Batonel Quality & Trust Model

Batonel is designed to bridge architectural intent and technical execution. To ensure this bridge remains reliable, we apply a layered **Quality & Trust Model**. This model transforms architectural promises into operational safeguards.

---

## 1. The Five Layers of Quality

Batonel establishes trust through five progressive layers of verification and governance.

### Layer 1: Structural Validity (Execution Alignment)
**Goal**: Ensure the project structure matches its architectural definition.
- **Mechanism**: `batonel verify` command.
- **Safeguard**: Validates that all planned artifacts exist, contracts are complete, and paths follow placement rules.
- **Operational Link**: [docs/concepts/verify.md](./concepts/verify.md) | `.github/workflows/batonel-verify-example.yml`

### Layer 2: Example Parity (Documentation Integrity)
**Goal**: Ensure that documentation (examples) and implementation (presets) never drift.
- **Mechanism**: `scripts/verify_parity.sh`.
- **Safeguard**: Performs byte-for-byte comparisons between presets and their corresponding examples. Ensures generated prompts are synchronized.
- **Operational Link**: `scripts/verify_parity.sh` | `.github/workflows/batonel-verify-parity.yml`

### Layer 3: Preset Integrity (Origin Authenticity)
**Goal**: Protect users from malicious or accidental tampering with distribution assets.
- **Mechanism**: **SSH Ed25519 Signatures**.
- **Safeguard**: Verifies cryptographic signatures of preset bundles against a committed "Root of Trust" (`allowed_signers`).
- **Operational Link**: [docs/concepts/trust.md](./concepts/trust.md) | `scripts/verify_trust.sh` | `.github/workflows/preset-trust-verification.yml`

### Layer 4: Release Authenticity (Secure Distribution)
**Goal**: Ensure the CLI binaries and release assets are authoritative.
- **Mechanism**: Automated signing and checksum verification.
- **Safeguard**: Releases are signed with dedicated keys, and SHA256 checksums are verified during installation.
- **Operational Link**: [docs/release-operations.md](./release-operations.md) | `.github/workflows/sign-release-assets.yml`

### Layer 5: Continuous Compliance (Maturity Governance)
**Goal**: Maintain a high governance posture across the entire ecosystem.
- **Mechanism**: **Ecosystem Compliance Maturity Benchmark**.
- **Safeguard**: Assigns a maturity level (L0-L4) based on the presence of operational controls. Regressions block development.
- **Operational Link**: [docs/ecosystem-compliance-maturity.md](./ecosystem-compliance-maturity.md) | `scripts/check_compliance_level.sh`

---

## 2. Operational Safeguards Summary

| Control Area | Operational Script | CI/CD Workflow |
|--------------|-------------------|---------------|
| **Architecture** | `cargo run -- verify` | `batonel-verify-example.yml` |
| **Drift Prevention** | `verify_parity.sh` | `batonel-verify-parity.yml` |
| **Preset Trust** | `verify_trust.sh` | `preset-trust-verification.yml` |
| **Distro Trust** | `install-batonel.sh` (checksum) | `sign-release-assets.yml` |
| **Governance** | `check_compliance_level.sh` | (Maturity Gated) |

---

## 3. The Batonel Trust Promise

Users and partners can trust Batonel because:

1.  **Transparency**: Every rule, preset, and contract is defined in language-agnostic YAML or Markdown. There is no hidden logic.
2.  **Reproducibility**: `init` and `plan` operations are deterministic. The same input always results in the same architectural plan.
3.  **Auditability**: Public keys and governance roles are committed directly to the repository, providing a permanent, versioned audit trail of trust anchors.
4.  **Conservative Defaults**: Verification and trust checks fail-closed. If a signature is missing or a path drifts, the process stops.

---

## 4. Definition of Done (DoD) Reference

Any significant change to Batonel must be evaluated against this model. A feature is not "Done" until it has satisfied the relevant quality layer (e.g., adding a new preset requires establishing Layer 2 Parity and Layer 3 Integrity).

For the specific release-gating criteria, see [Acceptance Criteria](./acceptance-criteria.md).
