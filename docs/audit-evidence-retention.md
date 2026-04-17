# Audit Evidence Retention and Extraction

## Overview

In highly regulated environments (e.g., SOC2, ISO27001), it is not enough to simply block a pull request when an architectural rule is violated. Organizations must prove to auditors that architectural governance was consistently applied and resolved over time.

To facilitate this, Batonel provides an evidence extraction workflow. By emitting structural, timestamped JSON representations of the audit report, CI/CD pipelines can reliably archive the state of governance on every merge to a trunk branch.

## Generating Evidence

The `batonel audit` command supports a dedicated extraction flag: `--evidence-export`. 

When this flag is provided, Batonel processes the audit normally (including standard output for developers) and additionally serializes the entire `AuditReport` data structure to the requested file path.

```bash
batonel audit --evidence-export .batonel/audit-evidence.json
```

### JSON Structure

The resulting JSON artifact guarantees the inclusion of a timestamp alongside all violations:

```json
{
  "repository": ".",
  "findings": [
    {
      "rule_id": "module-name-policy",
      "severity": "Error",
      "target": "module:legacy_Account",
      "message": "module 'legacy_Account' does not satisfy naming rule 'KebabCase'",
      "remediation": "Rename the module to satisfy policy naming rules or add a targeted override in policy.profile.yaml."
    }
  ],
  "errors": 1,
  "warnings": 0,
  "timestamp": "2026-04-11T09:00:00+00:00"
}
```

## Continuous Integration Workflow (GitHub Actions)

To ensure this evidence is preserved without manual intervention, integrate the `evidence-export` mechanism directly into your CI pipeline rules. 

We recommend generating this evidence on every push to the `main` or `master` branch and uploading it as a CI artifact.

### Example: `.github/workflows/batonel-audit.yml`

```yaml
name: Batonel Governance Audit
on:
  push:
    branches: [ "main" ]

jobs:
  audit-and-retain:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6
      
      - name: Install Batonel
        run: cargo install --path . # or download release binary
        
      - name: Run Audit and Export Evidence
        run: |
          mkdir -p .batonel-evidence
          batonel audit --evidence-export .batonel-evidence/audit-report.json
          
      - name: Upload Evidence Artifact
        if: always() # Ensure evidence is uploaded even if the audit fails
        uses: actions/upload-artifact@v7
        with:
          name: batonel-audit-evidence
          path: .batonel-evidence/audit-report.json
          retention-days: 90
```

By guaranteeing an uploaded artifact exists for every merge, your organization automatically constructs an immutable ledger of compliance that can be provided directly to external auditors.
