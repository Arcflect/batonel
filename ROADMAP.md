# Archflow Roadmap

For a more practical breakdown of each phase, see [docs/roadmap-detail.md](./docs/roadmap-detail.md).

Related documents:
- [docs/schema-guide.md](./docs/schema-guide.md)
- [docs/architecture-flow.md](./docs/architecture-flow.md)
- [docs/presets.md](./docs/presets.md)
- [docs/decisions/README.md](./docs/decisions/README.md)

---

## English

### Phase 0: Repository bootstrap

Goal:
Establish a clear open source foundation.

Scope:
- README
- CONTRIBUTING
- issue forms
- labels
- roadmap
- initial examples directory

Related docs:
- `examples/README.md`
- `docs/contributing-areas.md`

### Phase 1: Core design model

Goal:
Define the minimum stable concepts of Archflow.

Scope:
- project definition model
- placement rule model
- artifact definition model
- contract definition model
- prompt definition model

Deliverables:
- schema draft
- sample config files
- terminology glossary

Related docs:
- `docs/concepts/project.md`
- `docs/concepts/artifact.md`
- `docs/concepts/contract.md`
- `docs/concepts/prompt.md`
- `docs/glossary.md`
- `docs/schema-guide.md`
- `docs/architecture-flow.md`

### Phase 2: Minimal CLI

Goal:
Provide the first usable command-line flow.

Scope:
- `archflow init`
- `archflow plan`
- `archflow scaffold`

Deliverables:
- config initialization
- structure generation
- artifact sidecar generation

Related docs:
- `docs/roadmap-detail.md`
- `docs/decisions/README.md`

### Phase 3: AI handoff layer

Goal:
Make each artifact directly usable by lightweight coding models.

Scope:
- `archflow prompt`
- prompt templates
- contract-to-prompt conversion

Deliverables:
- artifact prompt generation
- example prompts
- role-based prompt presets

Related docs:
- `docs/concepts/prompt.md`
- `docs/decisions/README.md`

### Phase 4: Verification

Goal:
Check structural and contract consistency.

Scope:
- `archflow verify`
- required contract checks
- path rule checks
- status checks

Deliverables:
- local verification
- CI example workflow

Related docs:
- `docs/decisions/0006-verify-starts-with-structure-and-contract-consistency.md`
- `docs/roadmap-detail.md`

### Phase 5: Presets and ecosystem fit

Goal:
Make Archflow easier to adopt in real projects.

Scope:
- Rust preset
- generic preset
- example repositories
- GitHub workflow examples

Related docs:
- `docs/presets.md`
- `examples/README.md`

### Longer-term directions

- editor integration
- GitHub Action
- import from existing repo structure
- optional lightweight code-aware checks
- multi-language presets

---

## 日本語

### Phase 0: リポジトリ初期整備

目標:
OSS としての土台を整える。

対象:
- README
- CONTRIBUTING
- issue forms
- labels
- roadmap
- examples ディレクトリの初期化

関連ドキュメント:
- `examples/README.md`
- `docs/contributing-areas.md`

### Phase 1: コア設計モデル

目標:
Archflow の最小概念を安定化する。

対象:
- project definition model
- placement rule model
- artifact definition model
- contract definition model
- prompt definition model

成果物:
- schema draft
- sample config files
- 用語集

関連ドキュメント:
- `docs/concepts/project.md`
- `docs/concepts/artifact.md`
- `docs/concepts/contract.md`
- `docs/concepts/prompt.md`
- `docs/glossary.md`
- `docs/schema-guide.md`
- `docs/architecture-flow.md`

### Phase 2: 最小 CLI

目標:
最初の実用的なコマンドフローを作る。

対象:
- `archflow init`
- `archflow plan`
- `archflow scaffold`

成果物:
- config 初期化
- 構造生成
- artifact sidecar 生成

関連ドキュメント:
- `docs/roadmap-detail.md`
- `docs/decisions/README.md`

### Phase 3: AI handoff レイヤ

目標:
各 artifact を軽量モデルへ直接渡せる状態にする。

対象:
- `archflow prompt`
- prompt template
- contract から prompt への変換

成果物:
- artifact prompt 生成
- prompt サンプル
- role ごとの prompt preset

関連ドキュメント:
- `docs/concepts/prompt.md`
- `docs/decisions/README.md`

### Phase 4: Verify

目標:
構造と contract の整合を検査できるようにする。

対象:
- `archflow verify`
- contract 必須項目チェック
- path rule チェック
- status チェック

成果物:
- ローカル verify
- CI 用サンプル workflow

関連ドキュメント:
- `docs/decisions/0006-verify-starts-with-structure-and-contract-consistency.md`
- `docs/roadmap-detail.md`

### Phase 5: Preset と導入しやすさ

目標:
実プロジェクトへ導入しやすくする。

対象:
- Rust preset
- generic preset
- example repository
- GitHub workflow examples

関連ドキュメント:
- `docs/presets.md`
- `examples/README.md`

### 長期的な方向性

- エディタ統合
- GitHub Action
- 既存 repo からのルール逆生成
- optional な軽量コードチェック
- 多言語 preset
