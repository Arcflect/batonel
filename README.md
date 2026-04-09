# Archflow

Turn architecture into executable scaffolding for AI-assisted development.

Archflow is an open source tool that converts design decisions into artifact-level implementation contracts.
Instead of only documenting architecture, Archflow helps define:

- where code should live
- what each artifact is responsible for
- what it must not do
- what context should be handed to an AI coding tool

Archflow is designed for teams that define the overall architecture first, then implement file by file with humans or lightweight AI models.

For concrete example layouts and generated outputs, see [examples/README.md](./examples/README.md).

For deeper project documentation, see the [Docs guide](#docs--ドキュメント).

---

## English

### Why Archflow

AI coding tools are good at local implementation, but they often fail when structure is unclear.
Even when a team agrees on the architecture, development still gets stuck on questions like:

- Where should this file go?
- What is this file allowed to do?
- What must it never depend on?
- How can we hand this file to a smaller model without losing architectural intent?

Archflow fills that gap.

### What Archflow does

Archflow turns architecture into executable project scaffolding.

It can generate:

- directory and file structure
- artifact contracts
- responsibility definitions
- implementation constraints
- AI handoff prompts
- verification targets

This makes architecture usable during implementation, not just during planning.

### Core idea

Architecture should not stop at diagrams, folder trees, or markdown docs.
It should become a set of artifact contracts that both humans and AI tools can execute against.

Archflow is centered on artifact-level contracts such as:

- placement
- role
- responsibilities
- forbidden behavior
- allowed dependencies
- implementation size
- status

### Positioning

Archflow is not just a spec tool.
It is not just an agent instruction format.
It is not just an architecture linter.

It is an **architecture-to-execution bridge**.

## Examples

Archflow includes example inputs and expected outputs to show how architecture
is translated into placement rules, artifact contracts, and AI handoff prompts.

Start here:
- [examples/README.md](./examples/README.md)

Included examples:
- [`minimal`](./examples/minimal/README.md)
- [`generic-layered`](./examples/generic-layered/README.md)
- [`rust-clean-hexagonal`](./examples/rust-clean-hexagonal/README.md)

## Quickstart (init -> plan)

Use one of the core presets, then run `plan`:

```bash
# Generic layered path
cargo run -- init --preset generic-layered --project-name my-service
cargo run -- plan

# Rust clean/hexagonal path
cargo run -- init --preset rust-clean-hexagonal --project-name my-rust-service
cargo run -- plan
```

Preview before writing files:

```bash
cargo run -- init --preset generic-layered --project-name my-service --dry-run
```

Run baseline audit checks (strict mode fails on warnings):

```bash
cargo run -- audit --strict
```

## CLI Install and Update

Archflow CLI distribution is standardized through GitHub Releases.

- Release operation guide: [docs/release-operations.md](./docs/release-operations.md)
- Release workflow: [`.github/workflows/archflow-release-cli.yml`](./.github/workflows/archflow-release-cli.yml)

Quick install references:

- Binary install from GitHub Releases (recommended for users and CI)
- `archflow --version` should report the pinned version after install
- For CI, pin a fixed `vX.Y.Z` and cache by `${version}-${target}`

Export multi-repo compliance metrics (JSON/CSV):

```bash
# JSON export
cargo run -- compliance-report \
  --repos examples/minimal/archflow \
  --repos examples/generic-layered/archflow \
  --format json \
  --output compliance-report.json

# CSV export with trend comparison against a previous JSON baseline
cargo run -- compliance-report \
  --repos examples/minimal/archflow \
  --repos examples/generic-layered/archflow \
  --format csv \
  --output compliance-report.csv \
  --baseline-json compliance-report.json
```

Preview conservative remediation candidates:

```bash
cargo run -- fix --dry-run
```

Prototype preset registry workflow:

```bash
# Verify preset contract-first and sidecar-first alignment before publishing
cargo run -- preset-verify --preset-dir presets/generic-layered

# Publish local preset package into local registry index (alignment check runs automatically)
cargo run -- preset-publish --preset-dir presets/generic-layered --registry-dir .archflow/registry

# Install latest compatible preset from local registry index
cargo run -- preset-install --preset generic-layered --registry-dir .archflow/registry --destination-dir presets

# Run sidecar guard checks (CI-style)
cargo run -- guard --hook ci --strict
```

Preset versioning and migration workflow:

```bash
# Generate a migration plan (patch previews + conflict detection)
cargo run -- preset-migration-plan \
  --preset generic-layered \
  --from-version 0.1.0 \
  --to-version 0.2.0 \
  --registry-dir .archflow/registry

# Apply safe changes (backups created automatically; conflicts never auto-applied)
cargo run -- preset-migration-apply \
  --preset generic-layered \
  --from-version 0.1.0 \
  --to-version 0.2.0 \
  --registry-dir .archflow/registry
```

Org/team override precedence:

```bash
# Show effective policy after applying org → team → project precedence chain
cargo run -- policy-resolve
```

Onboarding e2e check script:

```bash
bash scripts/onboarding_e2e_init_plan.sh --preset generic-layered --project-name e2e-generic-service
```

## Docs / ドキュメント

If you want to understand the model, roadmap, and design decisions in more detail, start here:

- [ROADMAP.md](./ROADMAP.md)
- [ARCHITECTURE_RULES.md](./ARCHITECTURE_RULES.md)
- [docs/roadmap-detail.md](./docs/roadmap-detail.md)
- [docs/architecture/current-state.md](./docs/architecture/current-state.md)
- [docs/architecture/refactor-checklist.md](./docs/architecture/refactor-checklist.md)
- [docs/schema-guide.md](./docs/schema-guide.md)
- [docs/architecture-flow.md](./docs/architecture-flow.md)
- [docs/presets.md](./docs/presets.md)
- [docs/preset-onboarding.md](./docs/preset-onboarding.md)
- [docs/contributing-areas.md](./docs/contributing-areas.md)
- [docs/glossary.md](./docs/glossary.md)
- [docs/decisions/README.md](./docs/decisions/README.md)

Core concepts:
- [docs/concepts/project.md](./docs/concepts/project.md)
- [docs/concepts/module.md](./docs/concepts/module.md)
- [docs/concepts/role.md](./docs/concepts/role.md)
- [docs/concepts/placement-rule.md](./docs/concepts/placement-rule.md)
- [docs/concepts/artifact.md](./docs/concepts/artifact.md)
- [docs/concepts/contract.md](./docs/concepts/contract.md)
- [docs/concepts/prompt.md](./docs/concepts/prompt.md)
- [docs/concepts/handoff.md](./docs/concepts/handoff.md)
- [docs/concepts/scaffold.md](./docs/concepts/scaffold.md)
- [docs/concepts/verify.md](./docs/concepts/verify.md)

Usage:
- [docs/usage.md](./docs/usage.md)
- [docs/release-operations.md](./docs/release-operations.md)
- [`.github/workflows/archflow-audit-pr-gate.yml`](./.github/workflows/archflow-audit-pr-gate.yml)

Schema drafts:
- [schemas/project.schema.yaml](./schemas/project.schema.yaml)
- [schemas/placement-rules.schema.yaml](./schemas/placement-rules.schema.yaml)
- [schemas/contracts-template.schema.yaml](./schemas/contracts-template.schema.yaml)
- [schemas/artifacts-plan.schema.yaml](./schemas/artifacts-plan.schema.yaml)
- [schemas/contract.schema.yaml](./schemas/contract.schema.yaml)
- [schemas/prompt.schema.yaml](./schemas/prompt.schema.yaml)
- [schemas/policy-profile.schema.yaml](./schemas/policy-profile.schema.yaml)
- [schemas/guard-sidecar.schema.yaml](./schemas/guard-sidecar.schema.yaml)
- [schemas/preset-package.schema.yaml](./schemas/preset-package.schema.yaml)
- [schemas/preset-registry-index.schema.yaml](./schemas/preset-registry-index.schema.yaml)

Recommended reading order:
1. [examples/README.md](./examples/README.md)
2. [docs/schema-guide.md](./docs/schema-guide.md)
3. [docs/architecture-flow.md](./docs/architecture-flow.md)
4. [docs/concepts/project.md](./docs/concepts/project.md)
5. [docs/concepts/module.md](./docs/concepts/module.md)
6. [docs/concepts/role.md](./docs/concepts/role.md)
7. [docs/concepts/placement-rule.md](./docs/concepts/placement-rule.md)
8. [docs/concepts/artifact.md](./docs/concepts/artifact.md)
9. [docs/concepts/contract.md](./docs/concepts/contract.md)
10. [docs/concepts/prompt.md](./docs/concepts/prompt.md)
11. [docs/concepts/handoff.md](./docs/concepts/handoff.md)
12. [docs/concepts/scaffold.md](./docs/concepts/scaffold.md)
13. [docs/concepts/verify.md](./docs/concepts/verify.md)
14. [docs/usage.md](./docs/usage.md)
15. [docs/decisions/README.md](./docs/decisions/README.md)

---

## 日本語

### Archflow とは

Archflow は、設計で決めた内容を、AI 開発時代に使える**実装用の骨組み**へ変換するための OSS です。

アーキテクチャを文章や図で残すだけではなく、次のような情報を **artifact 単位の契約** として扱います。

- どこに配置するか
- 何を責務とするか
- 何をしてはいけないか
- どの依存を許可するか
- AI に何を渡して実装させるか

具体的なレイアウト例や生成された出力については、[examples/README.md](./examples/README.md)を参照してください。

より詳しい設計資料は、下の [Docs / ドキュメント](#docs--ドキュメント) を参照してください。

### なぜ必要か

生成 AI は局所的な実装は得意ですが、構造が曖昧だと誤った配置や責務逸脱を起こしやすくなります。

たとえば、次のような迷いが日常的に発生します。

- このファイルはどこに置くべきか
- このファイルは何をしてよいのか
- 何をしてはいけないのか
- 軽量モデルにどう渡せば設計意図を保てるのか

Archflow は、この曖昧さを減らすことを目的としています。

### 目指していること

Archflow は、設計を以下へ変換することを目指します。

- ディレクトリ構造
- 空ファイルや雛形
- `*.contract.yaml` のような責務契約
- `*.prompt.md` のような AI 実装指示
- verify 対象となる構造ルール

つまり、**設計を実装可能な単位まで下ろす**ための橋渡しです。

### ポジション

Archflow は、単なる仕様管理ツールでも、単なる AI 向け instruction ファイルでも、単なる lint ツールでもありません。

**設計から実装への橋渡しを行う OSS** です。

AI への正確な実装指示（AI Handoff）については、[docs/concepts/handoff.md](./docs/concepts/handoff.md) を参照してください。

## 例

Archflowには、アーキテクチャが配置ルール、アーティファクト契約、およびAIへの引き継ぎプロンプトにどのように変換されるかを示すための、入力例と期待される出力が含まれています。

まずはこちらから:
- [examples/README.md](./examples/README.md)

含まれる例:
- [`minimal`](./examples/minimal/README.md)
- [`generic-layered`](./examples/generic-layered/README.md)
- [`rust-clean-hexagonal`](./examples/rust-clean-hexagonal/README.md)

## クイックスタート（init -> plan）

core preset のどちらかを選び、`plan` まで実行します。

```bash
# Generic layered
cargo run -- init --preset generic-layered --project-name my-service
cargo run -- plan

# Rust clean/hexagonal
cargo run -- init --preset rust-clean-hexagonal --project-name my-rust-service
cargo run -- plan
```

生成前に確認したい場合:

```bash
cargo run -- init --preset generic-layered --project-name my-service --dry-run
```

オンボーディングe2e検証スクリプト:

```bash
bash scripts/onboarding_e2e_init_plan.sh --preset generic-layered --project-name e2e-generic-service
```

---

## Current status / 現在のステータス

Archflow is currently in early design and repository bootstrap stage.

現在の Archflow は、初期設計とリポジトリ整備の段階です。
最初の公開ゴールは次のとおりです。

- design file の読み込み
- placement rules の定義
- scaffold の生成
- artifact contract の生成
- AI handoff prompt の生成
- verify の最小実装

より詳しい段階分けは、次を参照してください。

- [ROADMAP.md](./ROADMAP.md)
- [docs/roadmap-detail.md](./docs/roadmap-detail.md)

---

## Planned commands / 想定コマンド

```bash
archflow init
archflow plan
archflow scaffold
archflow prompt
archflow verify
```

---

## Community / コミュニティ

Please use GitHub Issues for bugs, feature requests, and architecture rule proposals.
For open-ended exploration, use GitHub Discussions when available.

バグ報告、機能提案、アーキテクチャルール提案は GitHub Issues を利用してください。
広めの議論は GitHub Discussions を想定しています。

Contributors may also find these documents useful:

- [CONTRIBUTING.md](./CONTRIBUTING.md)
- [docs/contributing-areas.md](./docs/contributing-areas.md)
- [docs/decisions/README.md](./docs/decisions/README.md)

---

## License / ライセンス

Apache License 2.0.

A short Japanese summary is available in docs/LICENSE.ja.md.

Apache License 2.0 を採用します。
日本語の参考サマリーは [docs/LICENSE.ja.md](./docs/LICENSE.ja.md) にあります。