# Policy Exception Lifecycle

## Overview

In enterprise environments, enforcing a strict architectural policy is critical to avoiding structural degradation. However, enforcing rigid boundaries can often block critical hotfixes or migration paths.

Batonel manages this reality through **Policy Overrides**. To prevent these overrides from becoming permanent technical debt, Batonel supports an exception lifecycle bound by strict expiration dates (`expires_at`). 

## Managing Overrides

Overrides allow architects to temporarily suppress specific rules against specific targets, preventing them from failing mechanical CI `batonel audit` checks. 

### Adding an Expiration Date

Whenever an override is introduced in an org, team, or project level policy (`.batonel/org.policy.yaml`, `policy.profile.yaml`, etc.), operators should define an `expires_at` date using the standard ISO 8601 (`YYYY-MM-DD`) format.

```yaml
version: 1
overrides:
  - rule_id: artifact-module-defined
    targets:
      - artifact:legacy_auth_service
    reason: "Pending migration to the new identity module. Sprint 42 deliverable."
    expires_at: "2026-12-31"
```

### Expiration Behavior

The Batonel core resolution engine parses the expiration date against the current UTC date when loading policies.

- **Active Overrides**: If the current date is *before or equal* to the `expires_at` date, the override suppresses the audit rule. 
- **Expired Overrides**: If the current date is *after* the `expires_at` date, Batonel structurally discards the override from its resolution. The audit rule immediately resurfaces.
  - Expired overrides are visibly tagged with `[EXPIRED]` when diagnosing policies using `batonel policy-resolve`.
  - Next time `batonel audit` runs in CI, the build will break securely, forcing the team to address the structural debt or renegotiate the architectural contract.

## Workflow Integration

We recommend the following flow for governance teams (members defined as `architect` or `policy_admin` in your RBAC configurations):

1. **Issue Identified**: `batonel audit` blocks a pull request due to an architectural deviation.
2. **Review & Negotiation**: The team submits a pull request proposing a temporary override in `policy.profile.yaml` setting `expires_at` to the expected remediation date.
3. **Approval**: An `architect` explicitly approves the override using `batonel fix-rollout-approve` or direct pull request review.
4. **Resolution**: The team removes the deviation inside the expiration timeframe and cleans up the override.
5. **Expiration Trap**: If forgotten, `batonel audit` ensures the system fails safe immediately upon the expiration boundary.
