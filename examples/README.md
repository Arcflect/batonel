# Examples

This directory contains example Archflow inputs and expected outputs.

The goal of these examples is to show how Archflow turns architecture into executable implementation scaffolding for AI-assisted development.

Each example is organized in the same way:

- `archflow/`: input files for Archflow
- `expected/`: example output structure after planning or scaffolding

These examples are not meant to be full applications.
They are designed to make the Archflow model easy to understand.

They are documentation-first assets.
Preset implementation should be derived from stabilized example defaults,
not the other way around.

---

## What these examples show

Across all examples, Archflow is expected to help define:

- where artifacts should live
- what role each artifact has
- what responsibilities each artifact owns
- what each artifact must not do
- what prompts can be handed to AI coding tools

This means the examples focus not only on directory structure,
but also on:

- artifact contracts
- AI handoff prompts
- architectural boundaries
- implementation intent before code exists

---

## Included examples

### 1. `minimal`

The smallest possible Archflow example.

Use this when you want to understand the basic Archflow flow with the least amount of structure.

What it emphasizes:

- minimum project definition
- minimum placement rules
- small artifact plan
- smallest contract/prompt set

Best for:

- first-time readers
- quick onboarding
- understanding the core idea

Directory focus:

- `src/application/usecases/`
- `src/domain/entities/`

Artifacts included:

- `create_user`
- `user`

---

### 2. `rust-clean-hexagonal`

A Rust-oriented example using a clean / hexagonal style layout.

Use this when you want to see how Archflow can support projects
with stronger architectural boundaries and workspace-style structure.

What it emphasizes:

- Rust workspace-oriented organization
- separation of domain, application, and adapters
- explicit outbound port and adapter implementation boundaries
- artifact contracts for Rust-friendly architectural roles

Best for:

- Rust projects
- clean architecture / hexagonal architecture users
- teams that care about strong module boundaries

Preset path:

- `presets/rust-clean-hexagonal/`

Directory focus:

- `crates/domain/`
- `crates/application/`
- `crates/adapters/http/`
- `crates/adapters/db/`

Artifacts included:

- `user`
- `create_user`
- `user_repository`
- `create_user_handler`
- `postgres_user_repository`

---

### 3. `generic-layered`

A language-agnostic layered architecture example.

Use this when you want clear architectural separation without tying the example to Rust or another specific language.

What it emphasizes:

- generic layered structure
- separation between domain, application, interfaces, and infrastructure
- neutral file examples
- broad applicability across languages and frameworks

Best for:

- language-agnostic understanding
- layered architecture users
- teams exploring Archflow before choosing a language-specific preset

Preset path:

- `presets/generic-layered/`

Directory focus:

- `src/domain/`
- `src/application/`
- `src/interfaces/`
- `src/infrastructure/`

Artifacts included:

- `user`
- `create_user_service`
- `user_repository`
- `create_user_controller`
- `user_repository_gateway`

---

## Comparison

| Example | Main purpose | Architecture style | Language orientation | Complexity | Best starting point for |
|---|---|---|---|---|---|
| `minimal` | Show the smallest Archflow flow | simple | generic | low | understanding the core idea quickly |
| `rust-clean-hexagonal` | Show strong boundaries in a Rust-style layout | clean / hexagonal | Rust-oriented | medium | Rust users and architecture-focused teams |
| `generic-layered` | Show broad layered architecture support | layered | language-agnostic | medium | teams wanting a neutral example |

---

## Example-to-preset mapping

The current examples map to future preset directions as follows:

| Example | Preset direction | Preset id | Reusable defaults | Illustrative-only parts |
|---|---|---|---|---|
| `minimal` | minimal starter preset | `minimal` | simple project defaults, small role set, minimal contract defaults | onboarding-oriented artifact naming and tutorial narration |
| `generic-layered` | language-agnostic layered preset | `generic-layered` | layered role map, neutral placement defaults, reusable layered contract defaults | explanatory layering walkthrough and demo-specific naming |
| `rust-clean-hexagonal` | Rust clean/hexagonal preset | `rust-clean-hexagonal` | Rust-oriented role map, workspace-aware placement defaults, clean/hexagonal contract defaults | Rust example storytelling and sample-specific adapter naming |

This mapping clarifies direction only.
It does not mean all presets are fully implemented now.

Guardrail decision for implementation sequencing:

- [docs/decisions/0022-guard-examples-first-behavior-during-preset-implementation.md](docs/decisions/0022-guard-examples-first-behavior-during-preset-implementation.md)

### Transition rules

An example should be promoted to a supported preset only when:

- naming is stable and matches the preset id
- role and placement defaults are internally consistent
- contract template defaults are broadly reusable
- the package can be represented as a self-contained preset directory

---

## Recommended reading order

If you are new to Archflow, read the examples in this order:

1. `minimal`
2. `generic-layered`
3. `rust-clean-hexagonal`

Why this order:

- `minimal` explains the core model with the least noise
- `generic-layered` shows a more realistic but still language-neutral structure
- `rust-clean-hexagonal` shows how Archflow can express stricter boundaries in a Rust-oriented project

If you already know you care about Rust, you can start with `rust-clean-hexagonal`.

---

## Common structure

Every example follows the same high-level pattern:

### `archflow/`
Input files that describe architecture and intent.

Typical files:

- `project.arch.yaml`
- `placement.rules.yaml`
- `contracts.template.yaml`
- `artifacts.plan.yaml`

### `expected/`
Example scaffold output after planning or generation.

Typical contents:

- planned file structure
- `.archflow/contracts/*.contract.yaml`
- `.archflow/prompts/*.prompt.md`

This reflects the core Archflow idea:

**design first, scaffold clearly, implement with explicit boundaries**

---

## Why the examples include contracts and prompts

Archflow is not only about deciding where files go.

It is also about turning architecture into artifact-level execution context.

That is why the examples include:

- scaffolded artifact files
- artifact contracts
- AI handoff prompts

This helps show how Archflow can support both humans and lightweight coding models during implementation.

---

## Notes

These examples intentionally keep implementation files minimal.

The purpose is to highlight:

- placement
- role
- responsibility
- contract
- prompt
- boundary

rather than full business logic.

As Archflow evolves, these examples may expand to include:

- additional roles
- more presets
- verification examples
- CLI usage examples

---

## Preset Adoption Repository Patterns

For small but realistic onboarding patterns that show how presets map to
repository structures, see:

- `examples/preset-repository-patterns/`

Direct pattern links:

- `examples/preset-repository-patterns/minimal-starter/`
- `examples/preset-repository-patterns/generic-layered-service/`
- `examples/preset-repository-patterns/rust-clean-hexagonal-workspace/`

These pattern guides are not full applications.
They are repository-shape onboarding aids for preset-based startup.