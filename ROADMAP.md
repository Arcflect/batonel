# Batonel Roadmap

For a more practical breakdown of each phase, see [docs/roadmap-detail.md](./docs/roadmap-detail.md).

Related documents:
- [docs/schema-guide.md](./docs/schema-guide.md)
- [docs/architecture-flow.md](./docs/architecture-flow.md)
- [docs/presets.md](./docs/presets.md)
- [docs/acceptance-criteria.md](./docs/acceptance-criteria.md)
- [docs/decisions/README.md](./docs/decisions/README.md)

---

## Strategic Vision

> **Batonel** is the open-source CLI foundation of **Arcflect**. 
> While Batonel provides the ecosystem for structural contracts (init, plan, scaffold, verify), our primary commercial product focus is delivering the **Arcflect Handoff MVP** using Batonel as its execution engine.
> 
> The roadmap below reflects the stabilization of the Batonel OSS foundation and its progression toward enabling the Handoff use-case, followed by broader governance and enterprise capabilities.

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
Define the minimum stable concepts of Batonel.

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
- `batonel init`
- `batonel plan`
- `batonel scaffold`

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
- `batonel prompt`
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
- `batonel verify`
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
Make Batonel easier to adopt in real projects.

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

### Phase 6: OSS completion and foundation for continuous adoption

Goal:
Stabilize first-run experience and make architecture contracts explicit.

Scope:
- stabilize deterministic behavior for `init` and `plan`
- standardize `project.baton.yaml` as contract source
- expand docs, examples, and onboarding e2e tests

Related docs:
- [#127](https://github.com/Arcflect/batonel/issues/127)
- [#128](https://github.com/Arcflect/batonel/issues/128)
- [#129](https://github.com/Arcflect/batonel/issues/129)

### Phase 7: Audit / Policy / CI integration

Goal:
Move from one-time generation to continuous governance.

Scope:
- implement `audit` baseline and integrate with PR gate
- define safe `fix` boundaries with dry-run first
- apply minimum policy profile (required files, naming, forbidden dependencies)

Related docs:
- [#130](https://github.com/Arcflect/batonel/issues/130)
- [#131](https://github.com/Arcflect/batonel/issues/131)
- [#132](https://github.com/Arcflect/batonel/issues/132)

### Phase 8: Preset Registry & Guard

Goal:
Scale reusable presets while preserving contract-first and sidecar-first behavior.

Scope:
- prototype registry publish/install flow
- introduce Guard sidecar checks in runtime and CI
- verify preset alignment with architectural intent

Related docs:
- [#133](https://github.com/Arcflect/batonel/issues/133)
- [#134](https://github.com/Arcflect/batonel/issues/134)
- [#135](https://github.com/Arcflect/batonel/issues/135)

### Phase 9: Migration / Org-level Control / Reporting

Goal:
Provide enterprise-scale evolution and compliance visibility.

Scope:
- preset versioning and migration tooling
- org/team override precedence model
- multi-repo compliance reporting exports

Related docs:
- [#136](https://github.com/Arcflect/batonel/issues/136)
- [#137](https://github.com/Arcflect/batonel/issues/137)
- [#138](https://github.com/Arcflect/batonel/issues/138)

### Phase 10: Autonomous Governance

Goal:
Automate governance operations while preserving safe human review boundaries.

Scope:
- standardize `batonel` CLI distribution and release operations
- automate violation triage and remediation prioritization
- introduce approval-gated `fix` rollout workflow

Related docs:
- [#152](https://github.com/Arcflect/batonel/issues/152)
- [#153](https://github.com/Arcflect/batonel/issues/153)
- [#154](https://github.com/Arcflect/batonel/issues/154)

### Phase 11: Enterprise Control Plane

Goal:
Establish enterprise-grade control ownership and operational accountability.

Scope:
- deliver RBAC role model for governance operations
- implement expiring override lifecycle and policy exceptions
- strengthen audit evidence retention and extraction workflow

Related docs:
- [#155](https://github.com/Arcflect/batonel/issues/155)
- [#156](https://github.com/Arcflect/batonel/issues/156)
- [#157](https://github.com/Arcflect/batonel/issues/157)

### Phase 12: Ecosystem & Marketplace

Goal:
Scale trusted preset ecosystem operations across internal and external contributors.

Scope:
- define preset signature and trust verification pipeline
- build partner preset submission and review flow
- publish ecosystem compliance maturity benchmark model

Related docs:
- [#158](https://github.com/Arcflect/batonel/issues/158)
- [#159](https://github.com/Arcflect/batonel/issues/159)
- [#160](https://github.com/Arcflect/batonel/issues/160)

Tracking issues:
- [#127](https://github.com/Arcflect/batonel/issues/127) Phase6 Task 1: Stabilize init/plan deterministic onboarding
- [#128](https://github.com/Arcflect/batonel/issues/128) Phase6 Task 2: Standardize project.baton.yaml contract schema
- [#129](https://github.com/Arcflect/batonel/issues/129) Phase6 Task 3: Expand docs, examples, and onboarding e2e coverage (see [Acceptance Criteria](./docs/acceptance-criteria.md))
- [#130](https://github.com/Arcflect/batonel/issues/130) Phase7 Task 1: Implement audit baseline and PR gate integration
- [#131](https://github.com/Arcflect/batonel/issues/131) Phase7 Task 2: Define safe fix boundaries and conservative automation
- [#132](https://github.com/Arcflect/batonel/issues/132) Phase7 Task 3: Apply minimum policy profile across repositories
- [#133](https://github.com/Arcflect/batonel/issues/133) Phase8 Task 1: Prototype preset registry publish/install workflow
- [#134](https://github.com/Arcflect/batonel/issues/134) Phase8 Task 2: Introduce Guard sidecar policy checks
- [#135](https://github.com/Arcflect/batonel/issues/135) Phase8 Task 3: Verify contract-first and sidecar-first preset alignment
- [#136](https://github.com/Arcflect/batonel/issues/136) Phase9 Task 1: Deliver preset versioning and migration tooling
- [#137](https://github.com/Arcflect/batonel/issues/137) Phase9 Task 2: Implement org/team override precedence model
- [#138](https://github.com/Arcflect/batonel/issues/138) Phase9 Task 3: Build multi-repo compliance reporting exports
- [#152](https://github.com/Arcflect/batonel/issues/152) Phase10 Task 1: Standardize batonel CLI distribution and release operations
- [#153](https://github.com/Arcflect/batonel/issues/153) Phase10 Task 2: Automate violation triage and remediation prioritization
- [#154](https://github.com/Arcflect/batonel/issues/154) Phase10 Task 3: Introduce approval-gated fix rollout workflow
- [#155](https://github.com/Arcflect/batonel/issues/155) Phase11 Task 1: Deliver RBAC role model for governance operations
- [#156](https://github.com/Arcflect/batonel/issues/156) Phase11 Task 2: Implement expiring override lifecycle and policy exceptions
- [#157](https://github.com/Arcflect/batonel/issues/157) Phase11 Task 3: Strengthen audit evidence retention and extraction workflow
- [#158](https://github.com/Arcflect/batonel/issues/158) Phase12 Task 1: Define preset signature and trust verification pipeline
- [#159](https://github.com/Arcflect/batonel/issues/159) Phase12 Task 2: Build partner preset submission and review flow
- [#160](https://github.com/Arcflect/batonel/issues/160) Phase12 Task 3: Publish ecosystem compliance maturity benchmark model

---

## 日本語

### 戦略ビジョン

> **Batonel** は、**Arcflect** ブランドにおけるオープンソースの CLI 基盤（エンジン）です。
> Batonel 自体は構造や契約を支えるエコシステム（init, plan, scaffold, verify）を提供しますが、現在の私たちの主要な商用プロダクトの中心は、この Batonel を実行エンジンとして活用した **Arcflect Handoff MVP** の提供になります。
>
> 以下のロードマップは、この Batonel OSS 基盤の安定化と、Handoff ユースケースの実現に向けた前進、そしてその後のガバナンスおよびエンタープライズ機能への拡張を示しています。

---

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
Batonel の最小概念を安定化する。

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
- `batonel init`
- `batonel plan`
- `batonel scaffold`

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
- `batonel prompt`
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
- `batonel verify`
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

### Phase 6: OSS完成と継続利用基盤の土台

目標:
初回体験を安定化し、アーキテクチャ契約を明示的に扱えるようにする。

対象:
- `init` / `plan` の決定論的挙動を安定化
- `project.baton.yaml` を契約ソースとして標準化
- ドキュメント、サンプル、オンボーディングe2eテストを拡充

関連ドキュメント:
- [#127](https://github.com/Arcflect/batonel/issues/127)
- [#128](https://github.com/Arcflect/batonel/issues/128)
- [#129](https://github.com/Arcflect/batonel/issues/129)

### Phase 7: Audit / Policy / CI 統合

目標:
一度きりの生成から、継続的なガバナンスへ移行する。

対象:
- `audit` のベースライン実装とPRゲート統合
- `fix` の安全境界定義（dry-run優先）
- 最小ポリシープロファイル（必須ファイル、命名、禁止依存）適用

関連ドキュメント:
- [#130](https://github.com/Arcflect/batonel/issues/130)
- [#131](https://github.com/Arcflect/batonel/issues/131)
- [#132](https://github.com/Arcflect/batonel/issues/132)

### Phase 8: Preset Registry と Guard

目標:
contract-first / sidecar-first の思想を維持したまま、preset再利用を拡大する。

対象:
- registry の publish/install フローを試作
- 実行時とCIに Guard サイドカーを導入
- preset が設計意図に整合することを検証

関連ドキュメント:
- [#133](https://github.com/Arcflect/batonel/issues/133)
- [#134](https://github.com/Arcflect/batonel/issues/134)
- [#135](https://github.com/Arcflect/batonel/issues/135)

### Phase 9: Migration / Org-level Control / Reporting

目標:
組織導入を見据えた更新運用と準拠可視化を提供する。

対象:
- preset バージョニングと migration ツーリング
- org/team オーバーライド優先順位モデル
- 複数repo横断のコンプライアンスレポート

関連ドキュメント:
- [#136](https://github.com/Arcflect/batonel/issues/136)
- [#137](https://github.com/Arcflect/batonel/issues/137)
- [#138](https://github.com/Arcflect/batonel/issues/138)

### Phase 10: Autonomous Governance

目標:
安全なレビュー境界を維持しつつ、ガバナンス運用を自動化する。

対象:
- `batonel` CLI配布・リリース運用の標準化
- 違反トリアージと是正優先度付けの自動化
- 承認付き `fix` 段階適用ワークフローの導入

関連ドキュメント:
- [#152](https://github.com/Arcflect/batonel/issues/152)
- [#153](https://github.com/Arcflect/batonel/issues/153)
- [#154](https://github.com/Arcflect/batonel/issues/154)

### Phase 11: Enterprise Control Plane

目標:
組織運用に必要な統制責任と実行責任を明確化する。

対象:
- ガバナンス運用向けRBACロールモデルの提供
- 期限付きoverrideとポリシー例外のライフサイクル実装
- 監査証跡の保持・抽出フロー強化

関連ドキュメント:
- [#155](https://github.com/Arcflect/batonel/issues/155)
- [#156](https://github.com/Arcflect/batonel/issues/156)
- [#157](https://github.com/Arcflect/batonel/issues/157)

### Phase 12: Ecosystem & Marketplace

目標:
信頼可能なPresetエコシステム運用を内外のコントリビューターへ拡張する。

対象:
- Preset署名と信頼検証パイプラインの定義
- パートナーPreset投稿・審査フローの構築
- エコシステム準拠成熟度ベンチマークモデルの公開

関連ドキュメント:
- [#158](https://github.com/Arcflect/batonel/issues/158)
- [#159](https://github.com/Arcflect/batonel/issues/159)
- [#160](https://github.com/Arcflect/batonel/issues/160)

トラッキングIssue:
- [#127](https://github.com/Arcflect/batonel/issues/127) Phase6 Task 1: Stabilize init/plan deterministic onboarding
- [#128](https://github.com/Arcflect/batonel/issues/128) Phase6 Task 2: Standardize project.baton.yaml contract schema
- [#129](https://github.com/Arcflect/batonel/issues/129) Phase6 Task 3: Expand docs, examples, and onboarding e2e coverage
- [#130](https://github.com/Arcflect/batonel/issues/130) Phase7 Task 1: Implement audit baseline and PR gate integration
- [#131](https://github.com/Arcflect/batonel/issues/131) Phase7 Task 2: Define safe fix boundaries and conservative automation
- [#132](https://github.com/Arcflect/batonel/issues/132) Phase7 Task 3: Apply minimum policy profile across repositories
- [#133](https://github.com/Arcflect/batonel/issues/133) Phase8 Task 1: Prototype preset registry publish/install workflow
- [#134](https://github.com/Arcflect/batonel/issues/134) Phase8 Task 2: Introduce Guard sidecar policy checks
- [#135](https://github.com/Arcflect/batonel/issues/135) Phase8 Task 3: Verify contract-first and sidecar-first preset alignment
- [#136](https://github.com/Arcflect/batonel/issues/136) Phase9 Task 1: Deliver preset versioning and migration tooling
- [#137](https://github.com/Arcflect/batonel/issues/137) Phase9 Task 2: Implement org/team override precedence model
- [#138](https://github.com/Arcflect/batonel/issues/138) Phase9 Task 3: Build multi-repo compliance reporting exports
- [#152](https://github.com/Arcflect/batonel/issues/152) Phase10 Task 1: Standardize batonel CLI distribution and release operations
- [#153](https://github.com/Arcflect/batonel/issues/153) Phase10 Task 2: Automate violation triage and remediation prioritization
- [#154](https://github.com/Arcflect/batonel/issues/154) Phase10 Task 3: Introduce approval-gated fix rollout workflow
- [#155](https://github.com/Arcflect/batonel/issues/155) Phase11 Task 1: Deliver RBAC role model for governance operations
- [#156](https://github.com/Arcflect/batonel/issues/156) Phase11 Task 2: Implement expiring override lifecycle and policy exceptions
- [#157](https://github.com/Arcflect/batonel/issues/157) Phase11 Task 3: Strengthen audit evidence retention and extraction workflow
- [#158](https://github.com/Arcflect/batonel/issues/158) Phase12 Task 1: Define preset signature and trust verification pipeline
- [#159](https://github.com/Arcflect/batonel/issues/159) Phase12 Task 2: Build partner preset submission and review flow
- [#160](https://github.com/Arcflect/batonel/issues/160) Phase12 Task 3: Publish ecosystem compliance maturity benchmark model
