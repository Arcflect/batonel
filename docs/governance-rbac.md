# Governance & RBAC

## Overview

Batonel allows strict structural and architectural contracts across your organization. Once you deploy these rules, you must establish an operational boundary between standard development activities and governance operations (changing the rules or overriding the architecture).

Batonel's Role-Based Access Control (RBAC) maps operational actors (such as GitHub teams or users) to built-in Batonel governance roles.

## Core Governance Roles

The system supports the following built-in governance roles:

### 1. `policy_admin`
- **Responsibilities**: Modifying organization or team policies, locking rules, and adjusting the baseline configurations.
- **Allowed Operations**: `EditPolicy`, `RunAudit`, `ApproveOverride`, `FixRollout`.
- **Target Audience**: Core Architecture Team / Security Team.

### 2. `architect`
- **Responsibilities**: Overseeing the day-to-day architecture across repositories and evaluating temporary exceptions or deviations from policy.
- **Allowed Operations**: `ApproveOverride`, `FixRollout`, `RunAudit`.
- **Target Audience**: Team Leads and Domain Architects.

### 3. `auditor`
- **Responsibilities**: Viewing compliance reports and investigating architectural violations.
- **Allowed Operations**: `RunAudit`.
- **Target Audience**: General engineers, compliance officers, and automated CI pipelines.

> [!NOTE]
> Any unmapped actor falls back to the minimum permission set allowed, which allows them to view the `RunAudit` output but absolutely prevents policy tampering or override approvals.

---

## Configuring Governance Roles

You can map an actor to a role by adding `governance_roles` blocks in your policy layer files (`.batonel/org.policy.yaml`, `.batonel/team.policy.yaml`, etc.).

```yaml
version: 1
label: platform-tech
governance_roles:
  - role: policy_admin
    members:
      - "@platform-core"
  - role: architect
    members:
      - "@domain-a-leads"
      - "@domain-b-leads"
      - alice@example.com
```

### Precedence

Like required files or forbidden dependencies, governance roles follow a precedence chain: `org` → `team` → `project`.
Typically, you define your `policy_admin` in `org.policy.yaml` and delegate `architect` definitions at the `team.policy.yaml` level.

---

## Enforcement in CI vs Local

Because Batonel is primarily a CLI toolkit, an actor mapping doesn't inherently verify the cryptographic identity of the active user. Instead, operators structure their physical boundaries inside CI via `CODEOWNERS` files referencing the same names.

For example, a physical CI boundary involves generating a `.github/CODEOWNERS` configuration dynamically based on your `.batonel/org.policy.yaml`.

During pipeline executions (like merging PRs or rolling out fixes), Batonel can be invoked explicitly with the `--actor` argument to test simulated governance flows locally or apply strict enforcement rules:
```bash
batonel policy-resolve --actor "@alice"
```
```bash
# Example showing a rejection message:
[!] RBAC Denied: actor '@alice' is not authorized...
```
