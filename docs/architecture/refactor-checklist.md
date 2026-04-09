# Architecture Refactor Checklist

Use this checklist during implementation, review, and AI-assisted changes when a PR
touches architecture-sensitive code.

It is intentionally lightweight.

## PR review checklist

- [ ] The change keeps command flow thin: `cli -> app -> domain/ports`
- [ ] New business rules were added to `domain` or clearly justified in `app`
- [ ] The change does not introduce a new generic bucket such as `helpers`, `common`, `services`, `manager`, or `processor`
- [ ] New I/O, process execution, or rendering concerns are kept in `infra` or behind `ports`
- [ ] The PR does not create a forbidden dependency direction such as `domain -> cli` or `domain -> infra`
- [ ] If legacy `commands/config/generator/model` code was touched, the change reduced leakage or documented why it could not yet be moved
- [ ] Structured outputs, typed errors, and tests were updated where the behavior changed

## Developer / AI checklist

Before editing:

- [ ] Identify which layer owns the change before writing code
- [ ] Check whether a matching UseCase, domain module, port, or adapter already exists
- [ ] Prefer extending an existing responsibility-focused module over adding a new bucket

While editing:

- [ ] Keep CLI parsing and presentation logic out of domain
- [ ] Keep file access, shell execution, and external integrations out of domain
- [ ] Avoid adding new direct dependencies from `app` into legacy command executors unless the change is explicitly transitional
- [ ] Name modules by ownership and capability, not mechanism or leftovers

Before review:

- [ ] Re-read `ARCHITECTURE_RULES.md` if the change crosses more than one layer
- [ ] Update `docs/architecture/current-state.md` when the boundary picture materially changes
- [ ] Call out any temporary architecture debt in the PR summary
