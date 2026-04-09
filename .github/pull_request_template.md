## Summary

- What changed?
- Why was it needed?

## Checklist

- [ ] PR title follows: `type: summary`
- [ ] Architecture rules still hold, or I documented the temporary deviation
- [ ] New behavior follows `cli -> app -> domain/ports` flow where practical
- [ ] I did not introduce a generic bucket such as `helpers`, `common`, `services`, `manager`, or `processor`
- [ ] I added or updated tests if needed
- [ ] I updated docs if needed
- [ ] I checked release note impact

If this PR changes architecture-sensitive code, review against:

- `ARCHITECTURE_RULES.md`
- `docs/architecture/current-state.md`
- `docs/architecture/refactor-checklist.md`

For architecture-sensitive PRs, complete the architecture checklist items above or explain the temporary deviation in the summary.

## Examples of valid PR titles

- `feat: add policy-resolve command for org/team override diagnostics`
- `feat: support aarch64 target in release workflow`
- `fix: avoid binary version mismatch when tag differs from Cargo.toml`
- `fix: preset-install rejects incompatible archflow version range`
- `refactor: extract reusable audit report API from audit command`
- `perf: reduce YAML parse overhead in plan command`
- `docs: update release-operations with release-drafter flow`
- `test: add migration conflict detection coverage`
- `build: pin MSRV to 1.85`
- `ci: add verify-tag-version workflow on tag push`
- `chore: bump version to 0.2.0`
- `security: reject malformed preset registry index entries`

## Breaking changes

Use `!` when the change is breaking.

Examples:
- `feat!: change project.arch.yaml schema_version to 2`
- `feat!: rename preset-migration-plan to preset-upgrade-plan`