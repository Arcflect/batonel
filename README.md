# Arcflect Batonel

**The architecture-to-execution bridge for AI-assisted development.**

Arcflect Batonel turns design decisions into artifact-level contracts —
so that both humans and AI coding tools know exactly what to build, where, and under what constraints.

---

## The problem

AI coding tools are fast at writing code.
They struggle when structure is unclear.

Even when a team has agreed on an architecture, implementation keeps hitting the same wall:

- Where does this file belong?
- What is this component allowed to depend on?
- What must it never touch?
- How do I hand off this artifact to a smaller model without losing architectural intent?

Without explicit contracts, AI-generated code drifts. Architecture becomes a memory, not a rule.

---

## What Batonel does

Batonel converts architecture into an executable project scaffold.

Given a design intention, it generates:

- directory and file structure
- per-artifact contracts (role, responsibilities, forbidden behavior, allowed dependencies)
- AI handoff prompts derived directly from those contracts
- verification targets for structural consistency checks

Architecture stops being a diagram. It becomes a set of contracts that implementation tools can act on.

---

## Quickstart

**Install** (Linux / macOS):

```bash
curl -fsSL https://raw.githubusercontent.com/Arcflect/batonel/main/scripts/install-batonel.sh | bash
```

Pin a specific version:

```bash
curl -fsSL https://raw.githubusercontent.com/Arcflect/batonel/main/scripts/install-batonel.sh | bash -s -- v1.6.0
```

**Run the Primary Workflow:**

```bash
# 1. Initialize a project from a preset
batonel init --preset generic-layered --project-name my-service

# 2. Plan the architecture
batonel plan

# 3. Scaffold the directories and contracts
batonel scaffold

# 4. Verify project consistency against contracts
batonel verify
```

Two presets are included out of the box:

| Preset | When to use |
|--------|------------|
| `generic-layered` | Language-agnostic starting point, layered boundaries |
| `rust-clean-hexagonal` | Rust projects with domain/application/adapter separation |

→ See [docs/preset-onboarding.md](./docs/preset-onboarding.md) for the full onboarding flow.
→ See [docs/release-operations.md](./docs/release-operations.md) for CI pinned-version install.

---

## Command Hierarchy

Batonel features a unified **Golden Path** alongside distinct advanced operations.
Run `batonel --help` to see all commands grouped by category.

### 1. Primary Workflow
The core loop of converting an architectural preset into an executable reality.
- `init` - Initialize project configurations
- `plan` - Calculate definitions based on contracts
- `scaffold` - Generate structure into directories
- `verify` - Check compliance against strict rules

### 2. Advanced Usage
Operations meant to bridge the gap with AI tooling and triage workflows.
- `prompt` - Generate AI handoff context
- `triage` - Create remediation prioritization

### 3. Governance and Trust
Operations that maintain and enforce long-term compliance policies across teams.
- `audit`, `guard`, `compliance-report` - CI/CD and cross-repo audit checks
- `fix`, `fix-rollout-plan`, `fix-rollout-approve`, `fix-rollout-apply` - Managed remediations
- `policy-resolve` - Test merged organization policies

### 4. Preset Management
Local distribution for architecture packages.
- `preset-install`, `preset-publish`, `preset-verify`
- `preset-migration-plan`, `preset-migration-apply`

---

## Core concept: Contracts for your files

Batonel treats your project as a collection of **artifacts** (files or directories).
Instead of just being empty files, every artifact follows a **contract**—an explicit set of rules defining its role and boundaries.

```yaml
# example: src/domain/auth.contract.yaml
role: domain
responsibilities:
  - Validate authentication credentials
  - Enforce session invariants
must_not:
  - Import from adapter layer
  - Access HTTP context directly
allowed_dependencies:
  - domain/user
  - ports/auth
```

These contracts serve as the single source of truth for:

- **scaffold** — generating the actual directories and files
- **prompt** — providing clear implementation instructions to AI
- **verify / audit** — checking if the code structure actually follows the rules

---

## Why "architecture-to-execution bridge"

Most tools stop at one of two points:

- **Documentation tools** — architecture lives in diagrams or markdown, decoupled from implementation
- **Scaffolding tools** — directory templates with no semantic content

Batonel sits between them. It makes architectural intent executable.

The contract model is precise enough for a CI audit gate. It is also structured enough to generate a context-aware prompt for a lightweight AI coding model — without losing the design boundary.

---

## Trust & Quality Model

Batonel establishes a verifiable bridge between design and code through a layered **Quality & Trust Model**:

- **Structural Validity**: Core consistency check between plan and execution (`batonel verify`).
- **Example Parity**: Byte-for-byte synchronization between documentation and presets (`verify_parity.sh`).
- **Preset Integrity**: Cryptographic origin authenticity using **Ed25519 SSH signatures**.
- **Continuous Compliance**: Five-level maturity benchmark (L0–L4) enforced via CI gates.

→ See **[docs/quality-model.md](./docs/quality-model.md)** for the full trust framework.
→ See [docs/acceptance-criteria.md](./docs/acceptance-criteria.md) for release-gating definitions.

---

## Examples

| Example | What it shows |
|---------|--------------|
| [`minimal`](./examples/minimal/README.md) | Smallest possible Batonel project |
| [`generic-layered`](./examples/generic-layered/README.md) | Language-agnostic layered structure |
| [`rust-clean-hexagonal`](./examples/rust-clean-hexagonal/README.md) | Rust workspace with clean architecture |

→ [examples/README.md](./examples/README.md)

## Read this next

How you proceed depends on what you want to do:

**For Users & Developers:**
- **[CLI Reference Manual](./docs/usage.md)**: Deep dive into using every command.
- **[Preset Guide](./docs/presets.md)** & **[Onboarding](./docs/preset-onboarding.md)**: Learn how to manage architectural shapes.
- **[Schema Reference](./docs/schema-guide.md)**: How to write `.contract.yaml` files.

**For Project Contributors & Architects:**
- **[Roadmap](./ROADMAP.md)**: See what we are building next (Focus: Arcflect Handoff MVP).
- **[Architecture State](./docs/architecture/current-state.md)**: Understand the internal codebase structure and technical debt.
- **[Governance & RBAC](./docs/governance-rbac.md)**: How organizational policies are enforced.

Core concept docs:
[project](./docs/concepts/project.md) · [artifact](./docs/concepts/artifact.md) · [contract](./docs/concepts/contract.md) · [prompt](./docs/concepts/prompt.md) · [handoff](./docs/concepts/handoff.md) · [verify](./docs/concepts/verify.md)

---

## About Arcflect 

**Arcflect** is an overarching brand dedicated to designing responsibility, handoff, and progression structures for engineering organizations in the AI era.

**Batonel** (this repository) is the core open-source foundation of Arcflect. It acts as the execution engine that turns architectural intent into structured artifacts.

Currently, Batonel is focused on delivering its first major commercial use-case as the engine for **Arcflect Handoff**—a tool dedicated to making software repositories fully transferable to new developers and AI agents.

---

## Community

Bugs, feature requests, and architecture rule proposals: **GitHub Issues**

Contributors:
- [CONTRIBUTING.md](./CONTRIBUTING.md)
- [docs/contributing-areas.md](./docs/contributing-areas.md)
- [docs/decisions/README.md](./docs/decisions/README.md)

---

## License

Apache License 2.0 · [LICENSE](./LICENSE) · [日本語サマリー](./docs/LICENSE.ja.md)

---

---

## 日本語

# Arcflect Batonel

**AI 開発時代のための、設計から実装への橋渡しツール。**

Arcflect Batonel は、設計上の意思決定を artifact 単位の契約へ変換します。
人間も AI コーディングツールも、「何を」「どこに」「どんな制約のもとで」作ればよいかを明示的に把握できます。

---

## 解決する問題

AI コーディングツールはコードを書くのが速い。
しかし、構造が曖昧だと誤った実装を生み出しやすい。

チームがアーキテクチャを合意していても、実装フェーズで次の壁にぶつかり続けます：

- このファイルはどこに置くべきか
- このコンポーネントが依存してよい対象は何か
- 絶対に触れてはいけない対象は何か
- 軽量モデルに渡すとき、設計意図をどう保つか

明示的な契約がなければ、AI が生成するコードは設計から逸脱していきます。アーキテクチャは「記憶」になり、「ルール」ではなくなります。

---

## Batonel が行うこと

Batonel は、アーキテクチャを「実行可能なプロジェクト骨格」へ変換します。

設計意図が与えられると、次のものを生成します：

- ディレクトリ・ファイル構造
- artifact ごとの契約（役割・責務・禁止事項・許可された依存）
- その契約から直接導出される AI への handoff プロンプト
- 構造一貫性チェックのための verification ターゲット

アーキテクチャは図で終わらない。実装ツールが動ける契約になります。

---

## クイックスタート

**インストール**（Linux / macOS）：

```bash
curl -fsSL https://raw.githubusercontent.com/Arcflect/batonel/main/scripts/install-batonel.sh | bash
```

バージョンを固定してインストール：

```bash
curl -fsSL https://raw.githubusercontent.com/Arcflect/batonel/main/scripts/install-batonel.sh | bash -s -- v1.6.0
```

**プリセットからプロジェクトを初期化：**

```bash
# 生成内容を確認（ファイルは書かない）
batonel init --preset generic-layered --project-name my-service --dry-run

# 初期化して artifact プランを生成
batonel init --preset generic-layered --project-name my-service
batonel plan

# 構造整合性の監査
batonel audit --strict
```

すぐ使える 2 つのプリセット：

| プリセット | 用途 |
|-----------|------|
| `generic-layered` | 言語非依存のレイヤードアーキテクチャ起点 |
| `rust-clean-hexagonal` | Rust ワークスペース向けクリーンアーキテクチャ |

→ 詳細は [docs/preset-onboarding.md](./docs/preset-onboarding.md) を参照してください。
→ CI でのバージョン固定インストールは [docs/release-operations.md](./docs/release-operations.md) を参照してください。

---

## コアコンセプト：ファイルごとの「役割」と「契約」

Batonel では、管理対象のファイルやディレクトリを **artifact** と呼びます。
単にファイルを置くだけでなく、個々の artifact に対して「何に責任を持つか（責務）」や「何をしてはいけないか（境界）」を **契約 (contract)** として定義します。

```yaml
# 例: src/domain/auth.contract.yaml
role: domain
responsibilities:
  - 認証クレデンシャルを検証する
  - セッション不変条件を強制する
must_not:
  - adapter レイヤーからインポートしない
  - HTTP コンテキストに直接アクセスしない
allowed_dependencies:
  - domain/user
  - ports/auth
```

この契約が、開発のあらゆるフェーズで「唯一の真実源」になります：

- **scaffold**（生成） — 実際のディレクトリやファイルを自動作成
- **prompt**（指示） — AI に対して、設計意図に基づいた的確な実装指示（handoff）を生成
- **verify / audit**（検証） — 実装された構造が、契約（ルール）に違反していないかをチェック

---

## なぜ「設計から実装への橋渡し」か

多くのツールは次の 2 点のどちらかで止まっています：

- **ドキュメントツール** — アーキテクチャが図やmarkdownに留まり、実装と切り離されている
- **スキャフォールドツール** — 意味のないディレクトリテンプレートで、設計の境界情報がない

Batonel はその中間にあります。設計意図を実行可能にします。

契約モデルは、CI 監査ゲートとして使えるほど精密で、かつ軽量 AI モデルへのコンテキスト付きプロンプトを生成できるほど構造化されています。

---

## 信頼と品質のモデル (Trust & Quality)

Batonel は、多層的な **Quality & Trust Model** を通じて、設計とコードの間に検証可能な橋を架けます：

- **構造的妥当性**: 計画と実装の整合性を自動検証 (`batonel verify`)
- **例示の整合性**: ドキュメントとプリセットの完全な同期を保証 (`verify_parity.sh`)
- **プリセットの完全性**: **Ed25519 SSH 署名**によるオリジンの真正性保証
- **継続的コンプライアンス**: 5 段階の成熟度ベンチマーク (L0–L4) によるガバナンス監査

→ 詳細は **[docs/quality-model.md](./docs/quality-model.md)** を参照してください。
→ リリース基準は [docs/acceptance-criteria.md](./docs/acceptance-criteria.md) を参照してください。

---

## Arcflect について

Arcflect Batonel は [Arcflect](https://github.com/Arcflect) プロジェクトの一部です。

Arcflect は、AI 支援開発における設計責務・継承・ガバナンスのためのツールを構築しています。
Batonel はそのオープンソース基盤であり、設計意図を実行可能な構造へ変換するエンジンです。

---

## コミュニティ

バグ報告・機能提案・アーキテクチャルール提案は **GitHub Issues** へ。

コントリビューター向け：
- [CONTRIBUTING.md](./CONTRIBUTING.md)
- [docs/contributing-areas.md](./docs/contributing-areas.md)
- [docs/decisions/README.md](./docs/decisions/README.md)

---

## ライセンス

Apache License 2.0 · [LICENSE](./LICENSE) · [日本語サマリー](./docs/LICENSE.ja.md)