# Batonel Roadmap

For a more practical breakdown of each phase, see [docs/roadmap-detail.md](./docs/roadmap-detail.md).

Related documents:
- [docs/schema-guide.md](./docs/schema-guide.md)
- [docs/architecture-flow.md](./docs/architecture-flow.md)
- [docs/presets.md](./docs/presets.md)
- [docs/acceptance-criteria.md](./docs/acceptance-criteria.md)
- [docs/quality-model.md](./docs/quality-model.md)
- [docs/decisions/README.md](./docs/decisions/README.md)

---

## Strategic Vision

> **Batonel** is the open-source CLI foundation of **Arcflect**. 
> While Batonel provides the ecosystem for structural contracts (init, plan, scaffold, verify), our primary commercial product focus is delivering the **Arcflect Handoff MVP** using Batonel as its execution engine.
> 
> The roadmap below reflects the stabilization of the Batonel OSS foundation and its progression toward enabling the Handoff use-case, followed by broader governance and enterprise capabilities.

---

## English

### Active Priorities
These phases represent the core focus of current repository investments.

#### Phase 6: OSS completion and foundation for continuous adoption
**Goal**: Stabilize deterministic onboarding and make architecture contracts explicit.
- stabilize `init` and `plan` behavior
- standardize `project.baton.yaml` as contract source
- expand docs, examples, and onboarding e2e tests

#### Phase 7: Audit / Policy / CI integration
**Goal**: Move from one-time generation to continuous governance.
- implement `audit` baseline and PR gate integration
- define safe `fix` boundaries with dry-run first
- apply minimum policy profile (required files, naming, forbidden dependencies)

#### Phase 8: Preset Registry & Guard
**Goal**: Scale reusable presets while preserving contract-first and sidecar-first behavior.
- prototype registry publish/install flow
- introduce Guard sidecar checks in runtime and CI
- verify preset alignment with architectural intent

#### Phase 12: Ecosystem & Marketplace
**Goal**: Scale trusted preset ecosystem operations across internal and external contributors.
- define preset signature and trust verification pipeline
- build partner preset submission and review flow
- publish ecosystem compliance maturity benchmark model

### Backlog & Aspirational
These phases outline future capabilities and enterprise scale.

#### Phase 13: Handoff MVP Execution Layer
**Goal**: Transition from generating prompts to actively orchestrating AI-driven code generation.
- introduce `batonel handoff` orchestration workflow
- define AI response schemas and parsing boundaries
- integrate local verification loop for AI outputs

#### Phase 14: Continuous Alignment & Agentic Observability
**Goal**: Track the quality, accuracy, and drift of AI-generated implementations over time.
- implement `batonel drift` for implementation-to-contract comparison
- create a telemetry model for AI success rates
- automate continuous feedback loop in CI

#### Phase 15: Generative Architecture Discovery (Reverse Engineering)
**Goal**: Automatically derive Batonel contracts and placement rules from existing codebases.
- introduce `batonel discover` for architecture reverse-engineering
- AI-assisted mapping of legacy code to architectural roles
- interactive migration workflow for legacy adoption

#### Phase 16: Multi-Agent Collaboration Workflow
**Goal**: Orchestrate concurrent AI agents building full-stack features via shared structural contracts.
- implement `batonel orchestrate` for multi-agent role assignments
- define cross-boundary contract resolution
- introduce state-locking mechanisms for concurrent generation

#### Phase 17: IDE & Real-Time Editor Integration
**Goal**: Move the Batonel governance and handoff layer directly into the developer's IDE canvas.
- build Language Server Protocol implementation
- implement official extensions for real-time drift detection
- interactive "Handoff to AI" UI within the editor



#### Phase 9: Migration / Org-level Control / Reporting
**Goal**: Provide enterprise-scale evolution and compliance visibility.
- preset versioning and migration tooling
- org/team override precedence model
- multi-repo compliance reporting exports

#### Phase 10: Autonomous Governance
**Goal**: Automate governance operations while preserving safe human review boundaries.
- standardize `batonel` CLI distribution and release operations
- automate violation triage and remediation prioritization
- introduce approval-gated `fix` rollout workflow

#### Phase 11: Enterprise Control Plane
**Goal**: Establish enterprise-grade control ownership and operational accountability.
- deliver RBAC role model for governance operations
- implement expiring override lifecycle and policy exceptions
- strengthen audit evidence retention and extraction workflow

### Historical Foundation (Completed)
These phases represent the completed capabilities that Batonel already supports today.

- **Phase 0: Repository bootstrap**: Core README, CONTRIBUTING, issue forms, labels, and initial examples directory.
- **Phase 1: Core design model**: Definition of project, module, role, artifact, contract, and prompt schemas.
- **Phase 2: Minimal CLI**: The core `batonel init`, `plan`, and `scaffold` workflows.
- **Phase 3: AI handoff layer**: The `batonel prompt` command and contract-to-prompt conversion.
- **Phase 4: Verification**: The `batonel verify` command for checking structural and contract consistency.
- **Phase 5: Presets and ecosystem fit**: Built-in Rust and generic presets, plus GitHub workflows.

### Tracking issues:
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
- [#295](https://github.com/Arcflect/batonel/issues/295) Phase13 Task 1: Introduce `batonel handoff` orchestration workflow
- [#296](https://github.com/Arcflect/batonel/issues/296) Phase13 Task 2: Define AI response schemas and parsing boundaries
- [#297](https://github.com/Arcflect/batonel/issues/297) Phase13 Task 3: Integrate local verification loop for AI outputs
- [#298](https://github.com/Arcflect/batonel/issues/298) Phase14 Task 1: Implement `batonel drift` for implementation-to-contract comparison
- [#299](https://github.com/Arcflect/batonel/issues/299) Phase14 Task 2: Create a telemetry model for AI success rates
- [#300](https://github.com/Arcflect/batonel/issues/300) Phase14 Task 3: Automate continuous feedback loop in CI
- [#301](https://github.com/Arcflect/batonel/issues/301) Phase15 Task 1: Introduce `batonel discover` for architecture reverse-engineering
- [#302](https://github.com/Arcflect/batonel/issues/302) Phase15 Task 2: AI-assisted mapping of legacy code to architectural roles
- [#303](https://github.com/Arcflect/batonel/issues/303) Phase15 Task 3: Interactive migration workflow for legacy adoption
- [#304](https://github.com/Arcflect/batonel/issues/304) Phase16 Task 1: Implement `batonel orchestrate` for multi-agent role assignments
- [#305](https://github.com/Arcflect/batonel/issues/305) Phase16 Task 2: Define cross-boundary contract resolution
- [#306](https://github.com/Arcflect/batonel/issues/306) Phase16 Task 3: Introduce state-locking mechanisms for concurrent generation
- [#307](https://github.com/Arcflect/batonel/issues/307) Phase17 Task 1: Build Language Server Protocol implementation
- [#308](https://github.com/Arcflect/batonel/issues/308) Phase17 Task 2: Implement official extensions for real-time drift detection
- [#309](https://github.com/Arcflect/batonel/issues/309) Phase17 Task 3: Interactive "Handoff to AI" UI within the editor

---

## 日本語

### 戦略ビジョン

> **Batonel** は、**Arcflect** ブランドにおけるオープンソースの CLI 基盤（エンジン）です。
> Batonel 自体は構造や契約を支えるエコシステム（init, plan, scaffold, verify）を提供しますが、現在の私たちの主要な商用プロダクトの中心は、この Batonel を実行エンジンとして活用した **Arcflect Handoff MVP** の提供になります。
>
> 以下のロードマップは、この Batonel OSS 基盤の安定化と、Handoff ユースケースの実現に向けた前進、そしてその後のガバナンスおよびエンタープライズ機能への拡張を示しています。

---

### アクティブな優先事項 (Active Priorities)
現在のリポジトリ投資の核となるフェーズです。

#### Phase 6: OSS完成と継続利用基盤の土台
**目標**: 初回体験を安定化し、アーキテクチャ契約を明示的に扱えるようにする。
- `init` / `plan` の決定論的挙動を安定化
- `project.baton.yaml` を契約ソースとして標準化
- ドキュメント、サンプル、オンボーディングe2eテストを拡充

#### Phase 7: Audit / Policy / CI 統合
**目標**: 一度きりの生成から、継続的なガバナンスへ移行する。
- `audit` のベースライン実装とPRゲート統合
- `fix` の安全境界定義（dry-run優先）
- 最小ポリシープロファイル（必須ファイル、命名、禁止依存）適用

#### Phase 8: Preset Registry と Guard
**目標**: contract-first / sidecar-first の思想を維持したまま、preset再利用を拡大する。
- registry の publish/install フローを試作
- 実行時とCIに Guard サイドカーを導入
- preset が設計意図に整合することを検証

#### Phase 12: Ecosystem & Marketplace
**目標**: 信頼可能なPresetエコシステム運用を内外のコントリビューターへ拡張する。
- Preset署名と信頼検証パイプラインの定義
- パートナーPreset投稿・審査フローの構築
- エコシステム準拠成熟度ベンチマークモデルの公開

### バックログ & 将来構想 (Backlog & Aspirational)
将来の機能やエンタープライズ規模の運用を示すフェーズです。

#### Phase 13: Handoff MVP Execution Layer
**目標**: プロンプトの生成から、AIによるコード生成の積極的なオーケストレーションへ移行する。
- `batonel handoff` オーケストレーションワークフローの導入
- AIレスポンススキーマとパース境界の定義
- AI出力に対するローカル検証ループの統合

#### Phase 14: Continuous Alignment & Agentic Observability
**目標**: AIによって生成された実装の品質、精度、およびドリフトを継続的に追跡する。
- 実装と契約の比較のための `batonel drift` の実装
- AI成功率のためのテレメトリモデルの作成
- CIにおける継続的フィードバックループの自動化

#### Phase 15: Generative Architecture Discovery (Reverse Engineering)
**目標**: 既存のコードベースからBatonelの契約と配置ルールを自動的に導出する。
- アーキテクチャのリバースエンジニアリングのための `batonel discover` の導入
- レガシーコードからアーキテクチャの役割へのAI支援によるマッピング
- レガシー導入のためのインタラクティブな移行ワークフロー

#### Phase 16: Multi-Agent Collaboration Workflow
**目標**: 共有の構造的契約を介して、フルスタック機能を構築する並行AIエージェントをオーケストレーションする。
- マルチエージェントの役割割り当てのための `batonel orchestrate` の実装
- 境界を越えた契約解決の定義
- 並行生成のための状態ロック機構の導入

#### Phase 17: IDE & Real-Time Editor Integration
**目標**: Batonelのガバナンスとハンドオフレイヤを開発者のIDEキャンバスに直接移動する。
- Language Server Protocol 実装の構築
- リアルタイムなドリフト検出のための公式拡張機能の実装
- エディタ内のインタラクティブな「AIへのハンドオフ」UI



#### Phase 9: Migration / Org-level Control / Reporting
**目標**: 組織導入を見据えた更新運用と準拠可視化を提供する。
- preset バージョニングと migration ツーリング
- org/team オーバーライド優先順位モデル
- 複数repo横断のコンプライアンスレポート

#### Phase 10: Autonomous Governance
**目標**: 安全なレビュー境界を維持しつつ、ガバナンス運用を自動化する。
- `batonel` CLI配布・リリース運用の標準化
- 違反トリアージと是正優先度付けの自動化
- 承認付き `fix` 段階適用ワークフローの導入

#### Phase 11: Enterprise Control Plane
**目標**: 組織運用に必要な統制責任と実行責任を明確化する。
- ガバナンス運用向けRBACロールモデルの提供
- 期限付きoverrideとポリシー例外のライフサイクル実装
- 監査証跡の保持・抽出フロー強化

### 歴史的基盤 (Historical Foundation - Completed)
すでに実装され、現在の Batonel を支えている完了済みのフェーズです。

- **Phase 0: リポジトリ初期整備**: コアとなる README、CONTRIBUTING、issue テンプレート、初期 examples ディレクトリ。
- **Phase 1: コア設計モデル**: project, module, role, artifact, contract, prompt の各スキーマ定義。
- **Phase 2: 最小 CLI**: `batonel init`, `plan`, `scaffold` のコアワークフロー実装。
- **Phase 3: AI handoff レイヤ**: `batonel prompt` コマンドと、contract から prompt への変換機能。
- **Phase 4: Verify**: 構造と contract の整合性を検証する `batonel verify` コマンド。
- **Phase 5: Preset と導入しやすさ**: Rust および generic なプリセットの内蔵、GitHub Actions ワークフロー例。

### トラッキングIssue:
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
- [#295](https://github.com/Arcflect/batonel/issues/295) Phase13 Task 1: Introduce `batonel handoff` orchestration workflow
- [#296](https://github.com/Arcflect/batonel/issues/296) Phase13 Task 2: Define AI response schemas and parsing boundaries
- [#297](https://github.com/Arcflect/batonel/issues/297) Phase13 Task 3: Integrate local verification loop for AI outputs
- [#298](https://github.com/Arcflect/batonel/issues/298) Phase14 Task 1: Implement `batonel drift` for implementation-to-contract comparison
- [#299](https://github.com/Arcflect/batonel/issues/299) Phase14 Task 2: Create a telemetry model for AI success rates
- [#300](https://github.com/Arcflect/batonel/issues/300) Phase14 Task 3: Automate continuous feedback loop in CI
- [#301](https://github.com/Arcflect/batonel/issues/301) Phase15 Task 1: Introduce `batonel discover` for architecture reverse-engineering
- [#302](https://github.com/Arcflect/batonel/issues/302) Phase15 Task 2: AI-assisted mapping of legacy code to architectural roles
- [#303](https://github.com/Arcflect/batonel/issues/303) Phase15 Task 3: Interactive migration workflow for legacy adoption
- [#304](https://github.com/Arcflect/batonel/issues/304) Phase16 Task 1: Implement `batonel orchestrate` for multi-agent role assignments
- [#305](https://github.com/Arcflect/batonel/issues/305) Phase16 Task 2: Define cross-boundary contract resolution
- [#306](https://github.com/Arcflect/batonel/issues/306) Phase16 Task 3: Introduce state-locking mechanisms for concurrent generation
- [#307](https://github.com/Arcflect/batonel/issues/307) Phase17 Task 1: Build Language Server Protocol implementation
- [#308](https://github.com/Arcflect/batonel/issues/308) Phase17 Task 2: Implement official extensions for real-time drift detection
- [#309](https://github.com/Arcflect/batonel/issues/309) Phase17 Task 3: Interactive "Handoff to AI" UI within the editor
