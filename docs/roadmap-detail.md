# Roadmap Detail

This document expands the high-level roadmap into a more practical working plan.

Its purpose is to make each phase easier to execute by clarifying:

- the goal of the phase
- the main work items
- the expected outputs
- the definition of done
- what should not be overbuilt too early

This document complements `ROADMAP.md`.
`ROADMAP.md` stays concise.
This file adds more operational detail.

---

## Overview

Batonel is being developed in phases.

The overall flow is:

- Phase 0: repository bootstrap
- Phase 1: core design model
- Phase 2: minimal CLI
- Phase 3: AI handoff layer
- Phase 4: verification
- Phase 5: presets and ecosystem fit
- Phase 6: OSS completion and foundation for continuous adoption
- Phase 7: Audit / Policy / CI integration
- Phase 8: Preset Registry & Guard
- Phase 9: Migration / Org-level Control / Reporting
- Phase 10: Autonomous Governance
- Phase 11: Enterprise Control Plane
- Phase 12: Ecosystem & Marketplace

The purpose of this phased approach is to keep the project focused.

Batonel should not start by trying to solve everything at once.
It should establish concepts first, then operationalize them gradually.

---

## Phase 0: Repository bootstrap

### Goal

Establish a clear open source foundation and make the project understandable before implementation grows.

### Why this phase matters

If the repository is unclear, future implementation will become harder to navigate.
Phase 0 creates the minimum structure needed for contributors and future users to understand what Batonel is trying to do.

### Main work items

- README
- CONTRIBUTING
- CODE_OF_CONDUCT
- LICENSE
- SECURITY
- issue forms
- labels
- roadmap
- examples directory bootstrap
- basic documentation structure

### Expected outputs

- root repository documentation is in place
- examples directory exists and is understandable
- core project positioning is documented
- contribution entry points are clear
- community health files exist

### Definition of done

Phase 0 is done when:

- a new visitor can understand what Batonel is
- a contributor can find how to participate
- examples exist for the main conceptual directions
- the repository structure no longer feels empty or ambiguous
- the initial roadmap and supporting docs exist

### What not to overbuild

Do not overbuild:

- plugin integration
- code-aware analysis
- full CLI behavior
- preset engine
- advanced CI

The focus here is repository clarity, not feature completeness.

---

## Phase 1: Core design model

### Goal

Define the minimum stable conceptual model of Batonel.

### Why this phase matters

Without a stable concept model, implementation will drift.
Phase 1 gives Batonel its vocabulary and internal architecture.

### Main work items

- define core concepts
- define glossary
- define schema drafts
- define schema guide
- define architecture flow
- define preset direction
- align examples with the concept model

### Core concepts to stabilize

The main concepts to stabilize are:

- project
- module
- role
- artifact
- placement rule
- contract
- prompt
- scaffold
- verify
- preset

### Expected outputs

- concept documents exist
- glossary exists
- schema drafts exist
- schema guide exists
- architecture flow document exists
- preset concept is documented
- examples are aligned with the terminology

### Definition of done

Phase 1 is done when:

- the core concepts no longer conflict with each other
- the repository has a stable shared vocabulary
- input and output file types are documented
- contributors can explain the model consistently
- future implementation work has a clear conceptual base

### What not to overbuild

Do not overbuild:

- strict formal validation
- deep schema enforcement
- complete preset machinery
- detailed runtime behavior
- all possible role types

The goal is conceptual stability, not maximal completeness.

---

## Phase 2: Minimal CLI

### Goal

Provide the first usable command-line flow.

### Why this phase matters

At this stage, Batonel should move from concept documentation to an actual operational tool.

The first CLI should be small but real.

### Main work items

- `batonel init`
- `batonel plan`
- `batonel scaffold`

### Recommended implementation order

1. parse project definition
2. parse placement rules
3. parse artifact plan
4. resolve paths
5. generate basic scaffold structure
6. optionally generate initial sidecar files

### Expected outputs

- CLI crate exists
- configuration loading works
- path resolution works
- scaffold generation works for documented examples
- example-based manual validation is possible

### Definition of done

Phase 2 is done when:

- users can initialize or prepare a project structure
- users can generate planned output from structured input
- examples can be mapped to real CLI behavior
- the core CLI flow is demonstrable end-to-end

### What not to overbuild

Do not overbuild:

- perfect UX
- many subcommands
- editor integration
- advanced validation
- every configuration edge case

The first CLI should prove the flow, not solve every future use case.

---

## Phase 3: AI handoff layer

### Goal

Make each artifact directly usable by lightweight coding models.

### Why this phase matters

This is where Batonel becomes clearly different from a generic scaffold tool.

The goal is not only to create files.
It is to create artifact-level implementation handoff.

### Main work items

- `batonel prompt`
- prompt generation from contract data
- role-based prompt defaults
- prompt output modes
- artifact-level completion criteria

### Expected outputs

- prompts can be generated from contracts
- prompt structure is consistent across examples
- lightweight AI-oriented usage becomes demonstrable
- prompt derivation is clearly tied to the contract model

### Definition of done

Phase 3 is done when:

- one artifact can be handed to an AI model with explicit constraints
- prompt generation is stable for the main example roles
- prompts are derived from contract information, not handwritten ad hoc
- the value of Batonel for AI-assisted development is visible

### What not to overbuild

Do not overbuild:

- model-specific integrations
- vendor-specific APIs
- agent protocol support
- overly complex prompt personalization
- automatic implementation generation inside Batonel itself

The focus is handoff quality, not model orchestration.

---

## Phase 4: Verification

### Goal

Check whether project structure and artifact definitions remain consistent over time.

### Why this phase matters

Without verification, contracts and prompts may drift away from actual structure.
Phase 4 protects the architectural memory of the project.

### Main work items

- `batonel verify`
- required contract checks
- placement consistency checks
- status checks
- scaffold consistency checks
- future CI example

### Verification scope for the first version

Start with checks such as:

- required files exist
- required fields exist
- role names align across files
- artifact paths match placement rules
- contract and prompt files are present for expected artifacts

### Expected outputs

- local verification command works
- verification output is understandable
- examples can be checked using the same rules
- CI usage becomes possible

### Definition of done

Phase 4 is done when:

- users can detect structural drift
- users can detect missing contracts or prompt files
- role/path mismatches are surfaced clearly
- verification can be demonstrated locally and in a basic CI example

### What not to overbuild

Do not overbuild:

- full static code analysis
- compiler integration
- deep dependency graph inspection
- advanced policy DSL
- heavy runtime coupling to one language

The first verify phase should focus on structure and contract consistency.

---

## Phase 5: Presets and ecosystem fit

### Goal

Make Batonel easier to adopt in real projects and more reusable across styles.

### Why this phase matters

Once the core flow exists, users will want faster starting points.
Presets and ecosystem-aware defaults reduce friction.

### Main work items

- define preset packaging approach
- formalize current example-to-preset evolution
- create Rust preset
- create generic preset
- add example repository patterns
- add GitHub workflow examples
- improve onboarding for new projects

### Expected outputs

- at least one reusable preset exists
- example structures map clearly to preset concepts
- project bootstrap becomes faster
- ecosystem-specific conventions become easier to apply

### Definition of done

Phase 5 is done when:

- users can start from a preset instead of defining everything manually
- examples and presets have a clear relationship
- at least one language-specific and one language-agnostic path exist
- Batonel feels easier to adopt in realistic projects

### What not to overbuild

Do not overbuild:

- too many presets too early
- deep per-framework specialization
- overly rigid preset locking
- automatic migration of all existing repositories
- complete plugin ecosystem

The focus is useful starting points, not maximum coverage.

---

## Cross-phase principles

Some principles apply across all phases.

### Keep concepts ahead of implementation

Implementation should follow stable concepts, not invent them on the fly.

### Prefer explicitness over magic

Batonel should be understandable by reading files, not only by running code.

### Keep the artifact as the main execution unit

The artifact is the center of planning, contracts, prompts, and future verification.

### Preserve architecture outside source code

Important architectural intent should not exist only in production code.

### Avoid premature ecosystem lock-in

Batonel can begin with Rust-friendly examples without becoming Rust-only.

---

## Phase 6: OSS completion and foundation for continuous adoption

### Goal

Stabilize deterministic onboarding and standardize architecture contracts.

### Main work items

- stabilize `init` / `plan` behavior
- standardize `project.baton.yaml`
- expand docs/examples/e2e coverage

### Definition of done

- `init` and `plan` are reproducible in local and CI contexts
- `project.baton.yaml` is the default contract source for planning and verification
- onboarding examples are validated by CI e2e checks

### What not to overbuild

- avoid broad schema expansion before core fields are stable
- avoid UI polish work that weakens deterministic behavior guarantees

### Linked execution tasks

- [#127](https://github.com/Arcflect/batonel/issues/127)
- [#128](https://github.com/Arcflect/batonel/issues/128)
- [#129](https://github.com/Arcflect/batonel/issues/129)

---

## Phase 7: Audit / Policy / CI integration

### Goal

Transition to continuous architecture governance.

### Main work items

- implement baseline `audit` checks
- integrate PR gate with CI
- define conservative `fix` boundaries
- apply minimum policy profile

### Definition of done

- PRs consistently run `audit` and surface actionable diagnostics
- `fix --dry-run` provides clear patch previews for safe remediations
- minimum policy profile can be reused across repositories

### What not to overbuild

- avoid deep static code analysis in this phase
- avoid policy DSL complexity before baseline adoption is stable

### Linked execution tasks

- [#130](https://github.com/Arcflect/batonel/issues/130)
- [#131](https://github.com/Arcflect/batonel/issues/131)
- [#132](https://github.com/Arcflect/batonel/issues/132)

---

## Phase 8: Preset Registry & Guard

### Goal

Scale preset reuse while preserving contract-first and sidecar-first behavior.

### Main work items

- prototype registry publish/install flow
- introduce Guard sidecar checks
- verify preset alignment with architecture intent

### Definition of done

- teams can publish/install presets with compatibility checks
- Guard checks run in runtime and CI paths
- preset packaging does not collapse to folder-only templates

### What not to overbuild

- avoid production-grade registry operations before package contracts are stable
- avoid deep semantic content comparison beyond identity/alignment checks

### Linked execution tasks

- [#133](https://github.com/Arcflect/batonel/issues/133)
- [#134](https://github.com/Arcflect/batonel/issues/134)
- [#135](https://github.com/Arcflect/batonel/issues/135)

---

## Phase 9: Migration / Org-level Control / Reporting

### Goal

Provide enterprise-scale update workflows and compliance visibility.

### Main work items

- preset versioning and migration tooling
- org/team override precedence model
- multi-repo compliance reporting

### Definition of done

- upgrades can be planned and applied with predictable outputs
- effective policy resolution is explicit across org/team/project scopes
- compliance posture can be exported and tracked over time

### What not to overbuild

- avoid full platform UI/dashboard before report semantics are stable
- avoid over-automation that bypasses review for risky migrations

### Linked execution tasks

- [#136](https://github.com/Arcflect/batonel/issues/136)
- [#137](https://github.com/Arcflect/batonel/issues/137)
- [#138](https://github.com/Arcflect/batonel/issues/138)

---

## Phase 10: Autonomous Governance

### Goal

Automate governance operations while preserving safe review boundaries.

### Main work items

- standardize `batonel` CLI distribution and release operations
- automate violation triage and remediation prioritization
- introduce approval-gated `fix` rollout workflow

### Definition of done

- CLI distribution and release workflow is documented and repeatable
- violation triage results can be prioritized automatically with explicit criteria
- risky fixes require approval before rollout, with clear audit logs

### What not to overbuild

- avoid full autonomous remediation for high-risk changes
- avoid release channel sprawl before core distribution flow is stable

### Linked execution tasks

- [#152](https://github.com/Arcflect/batonel/issues/152)
- [#153](https://github.com/Arcflect/batonel/issues/153)
- [#154](https://github.com/Arcflect/batonel/issues/154)

---

## Phase 11: Enterprise Control Plane

### Goal

Establish enterprise-grade control ownership and accountability.

### Main work items

- deliver RBAC role model for governance operations
- implement expiring override lifecycle and policy exceptions
- strengthen audit evidence retention and extraction workflow

### Definition of done

- role responsibilities for view/approve/apply operations are enforced
- override lifecycle includes expiry and exception controls with explicit traceability
- audit evidence can be retained and extracted within target SLA

### What not to overbuild

- avoid complex UI-first control plane work before policy operations are stable
- avoid broad integration expansion before evidence model is validated

### Linked execution tasks

- [#155](https://github.com/Arcflect/batonel/issues/155)
- [#156](https://github.com/Arcflect/batonel/issues/156)
- [#157](https://github.com/Arcflect/batonel/issues/157)

---

## Phase 12: Ecosystem & Marketplace

### Goal

Scale trusted preset ecosystem operations across internal and external contributors.

### Main work items

- define preset signature and trust verification pipeline
- build partner preset submission and review flow
- publish ecosystem compliance maturity benchmark model

### Definition of done

- signed preset verification is available in distribution and install flow
- partner submission/review workflow is documented and operational
- benchmark model can compare compliance maturity across participating repositories;
  see [docs/ecosystem-compliance-maturity.md](docs/ecosystem-compliance-maturity.md)
  for the five-level model (L0–L4) and self-assessment tooling

### What not to overbuild

- avoid broad marketplace automation before trust model is validated
- avoid overfitting benchmark criteria before baseline adoption data is collected

### Linked execution tasks

- [#158](https://github.com/Arcflect/batonel/issues/158)
- [#159](https://github.com/Arcflect/batonel/issues/159)
- [#160](https://github.com/Arcflect/batonel/issues/160)

### Related documents

- [docs/ecosystem-compliance-maturity.md](docs/ecosystem-compliance-maturity.md)
- [docs/partner-preset-review.md](docs/partner-preset-review.md)
- [docs/governance-rbac.md](docs/governance-rbac.md)
- [docs/decisions/0024-preset-signature-and-trust-verification.md](docs/decisions/0024-preset-signature-and-trust-verification.md)
- [docs/decisions/0025-ecosystem-compliance-maturity-benchmark-model.md](docs/decisions/0025-ecosystem-compliance-maturity-benchmark-model.md)

---

## Suggested practical milestone order

## Active phase issues (Phase6-Phase12)

Use these issues as the execution baseline for ongoing roadmap work.

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

A practical internal milestone sequence may look like this:

1. bootstrap repository and examples
2. stabilize concepts and glossary
3. stabilize schema drafts
4. document architecture flow and presets
5. implement minimal config loading
6. implement path resolution
7. implement scaffold generation
8. implement prompt generation
9. implement verification
10. evolve examples into reusable presets

This sequence keeps the work grounded and incremental.

---

## How to use this document

Use this file when deciding:

- what to work on next
- whether a phase is actually complete
- whether a feature is too early
- how to scope contributions
- how to explain project maturity to contributors

If `ROADMAP.md` says where the project is going,
this file explains how each stage should behave in practice.

---

## Summary

The roadmap should help Batonel stay focused.

The main idea is:

- first make the repository understandable
- then stabilize the model
- then make it operational
- then make it AI-useful
- then make it verifiable
- then make it reusable through presets

If you remember only one thing, remember this:

**Batonel should grow in layers: clarity first, then structure, then execution, then reuse.**

---

## 日本語

このドキュメントは、高レベルのロードマップをより実践的な作業計画へと展開します。

目的は、次の点を明確にすることで各フェーズを実行しやすくすることです。

- フェーズの目標
- 主な作業項目
- 期待される成果物
- 完了の定義
- 早期に作りすぎてはいけないもの

このドキュメントは `ROADMAP.md` を補完します。
`ROADMAP.md` は簡潔さを保ちます。
このファイルはより多くの運用上の詳細を追加します。

---

### 概要

Batonel はフェーズごとに開発されています。

全体的なフローは次のとおりです。

- Phase 0: リポジトリ初期整備
- Phase 1: コア設計モデル
- Phase 2: 最小 CLI
- Phase 3: AI ハンドオフレイヤ
- Phase 4: Verify
- Phase 5: Preset とエコシステムへの適合
- Phase 6: OSS完成と継続利用基盤の土台
- Phase 7: Audit / Policy / CI 統合
- Phase 8: Preset Registry と Guard
- Phase 9: Migration / Org-level Control / Reporting
- Phase 10: Autonomous Governance
- Phase 11: Enterprise Control Plane
- Phase 12: Ecosystem & Marketplace

このフェーズ型アプローチの目的は、プロジェクトに焦点を当て続けることです。

Batonel は最初からすべてを解決しようとすべきではありません。
まず概念を確立し、その後徐々にそれらを運用化すべきです。

---

### Phase 0: リポジトリ初期整備

#### 目標

明確なオープンソースの基盤を確立し、実装が成長する前にプロジェクトを理解可能にします。

#### このフェーズが重要な理由

リポジトリが不明瞭な場合、将来の実装はナビゲートが難しくなります。
Phase 0 は、コントリビューターと将来のユーザーが Batonel が何をしようとしているかを理解するために必要な最小限の構造を作成します。

#### 主な作業項目

- README
- CONTRIBUTING
- CODE_OF_CONDUCT
- LICENSE
- SECURITY
- issue フォーム
- ラベル
- ロードマップ
- examples ディレクトリのブートストラップ
- 基本的なドキュメント構造

#### 期待される成果物

- ルートリポジトリのドキュメントが整っている
- examples ディレクトリが存在し、理解可能である
- コアプロジェクトのポジショニングがドキュメント化されている
- コントリビューションのエントリポイントが明確である
- コミュニティヘルスファイルが存在する

#### 完了の定義

Phase 0 は以下の時に完了します。

- 新しい訪問者が Batonel が何であるかを理解できる
- コントリビューターが参加方法を見つけられる
- 主要な概念的方向性の examples が存在する
- リポジトリ構造が空または曖昧に感じられない
- 初期ロードマップとサポートドキュメントが存在する

#### 作りすぎてはいけないもの

作りすぎてはいけないもの：

- プラグイン統合
- コード認識分析
- 完全な CLI の動作
- preset エンジン
- 高度な CI

ここでの焦点はリポジトリの明確さであり、機能の完全性ではありません。

---

### Phase 1: コア設計モデル

#### 目標

Batonel の最小限の安定した概念モデルを定義します。

#### このフェーズが重要な理由

安定した概念モデルなしには、実装がずれます。
Phase 1 は Batonel に語彙と内部アーキテクチャを与えます。

#### 主な作業項目

- コアコンセプトの定義
- 用語集の定義
- スキーマドラフトの定義
- スキーマガイドの定義
- architecture flow の定義
- preset の方向性の定義
- 概念モデルに合わせた examples の整合

#### 安定させるコアコンセプト

安定させるべき主な概念は次のとおりです。

- project
- module
- role
- artifact
- placement rule
- contract
- prompt
- scaffold
- verify
- preset

#### 期待される成果物

- 概念ドキュメントが存在する
- 用語集が存在する
- スキーマドラフトが存在する
- スキーマガイドが存在する
- architecture flow ドキュメントが存在する
- preset の概念がドキュメント化されている
- examples が用語に合わせて整合されている

#### 完了の定義

Phase 1 は以下の時に完了します。

- コアコンセプトが互いに矛盾しない
- リポジトリに安定した共有語彙がある
- 入力と出力のファイルタイプがドキュメント化されている
- コントリビューターがモデルを一貫して説明できる
- 将来の実装作業に明確な概念的な基盤がある

#### 作りすぎてはいけないもの

作りすぎてはいけないもの：

- 厳格な形式的バリデーション
- 深いスキーマの強制
- 完全な preset 機構
- 詳細なランタイムの動作
- すべての可能なロールタイプ

目標は概念的な安定性であり、最大限の完全性ではありません。

---

### Phase 2: 最小 CLI

#### 目標

最初の実用的なコマンドラインフローを提供します。

#### このフェーズが重要な理由

この段階で、Batonel は概念ドキュメントから実際の運用ツールへと移行すべきです。

最初の CLI は小さいが本物であるべきです。

#### 主な作業項目

- `batonel init`
- `batonel plan`
- `batonel scaffold`

#### 推奨実装順序

1. プロジェクト定義を解析する
2. 配置ルールを解析する
3. artifact プランを解析する
4. パスを解決する
5. 基本的なスキャフォルド構造を生成する
6. 任意で初期 sidecar ファイルを生成する

#### 期待される成果物

- CLI クレートが存在する
- 設定の読み込みが機能する
- パス解決が機能する
- ドキュメント化された examples に対してスキャフォルド生成が機能する
- example ベースの手動バリデーションが可能

#### 完了の定義

Phase 2 は以下の時に完了します。

- ユーザーがプロジェクト構造を初期化または準備できる
- ユーザーが構造化された入力から生成した出力を生成できる
- examples を実際の CLI の動作にマッピングできる
- コア CLI フローがエンドツーエンドでデモンストレーション可能

#### 作りすぎてはいけないもの

作りすぎてはいけないもの：

- 完璧な UX
- 多くのサブコマンド
- エディタ統合
- 高度なバリデーション
- すべての設定エッジケース

最初の CLI はフローを証明すべきであり、すべての将来のユースケースを解決するべきではありません。

---

### Phase 3: AI ハンドオフレイヤ

#### 目標

各 artifact を軽量コーディングモデルによって直接使用可能にします。

#### このフェーズが重要な理由

ここで Batonel は汎用スキャフォルドツールとは明らかに異なるものになります。

目標はファイルを作成するだけではありません。
artifact レベルの実装ハンドオフを作成することです。

#### 主な作業項目

- `batonel prompt`
- contract データからの prompt 生成
- ロールベースの prompt デフォルト
- prompt 出力モード
- artifact レベルの完了基準

#### 期待される成果物

- contract から prompt を生成できる
- prompt 構造が examples 全体で一貫している
- 軽量な AI 向け使用がデモンストレーション可能になる
- prompt の導出が contract モデルに明確に結びついている

#### 完了の定義

Phase 3 は以下の時に完了します。

- 1 つの artifact が明示的な制約とともに AI モデルにハンドオフできる
- prompt 生成が主要な example ロールに対して安定している
- prompt は手書きのアドホックではなく、contract 情報から導出されている
- AI 支援開発における Batonel の価値が見えている

#### 作りすぎてはいけないもの

作りすぎてはいけないもの：

- モデル固有の統合
- ベンダー固有の API
- エージェントプロトコルのサポート
- 過度に複雑な prompt のパーソナライズ
- Batonel 内での自動実装生成

焦点はハンドオフの品質であり、モデルのオーケストレーションではありません。

---

### Phase 4: Verify

#### 目標

プロジェクト構造と artifact の定義が時間をかけて一貫したままかどうかをチェックします。

#### このフェーズが重要な理由

verify がなければ、contract と prompt は実際の構造からずれる可能性があります。
Phase 4 はプロジェクトのアーキテクチャの記憶を保護します。

#### 主な作業項目

- `batonel verify`
- 必須の contract チェック
- 配置の整合チェック
- ステータスチェック
- スキャフォルドの整合チェック
- 将来の CI 例

#### 最初のバージョンの verify スコープ

まず次のようなチェックから始めます。

- 必要なファイルが存在する
- 必要なフィールドが存在する
- ロール名がファイル全体で整合している
- artifact パスが配置ルールと一致している
- 期待される artifact に contract と prompt ファイルが存在する

#### 期待される成果物

- ローカル verify コマンドが機能する
- verify 出力が理解可能
- 同じルールを使用して examples をチェックできる
- CI での使用が可能になる

#### 完了の定義

Phase 4 は以下の時に完了します。

- ユーザーが構造のずれを検出できる
- ユーザーが不足している contract または prompt ファイルを検出できる
- ロール / パスの不一致が明確に表示される
- ローカルおよび基本的な CI 例で verify をデモンストレーション可能

#### 作りすぎてはいけないもの

作りすぎてはいけないもの：

- 完全な静的コード分析
- コンパイラ統合
- 深い依存グラフ検査
- 高度なポリシー DSL
- 1 つの言語への重いランタイム結合

最初の verify フェーズは構造と contract の整合に集中すべきです。

---

### Phase 5: Preset とエコシステムへの適合

#### 目標

Batonel を実際のプロジェクトへの採用を容易にし、スタイル間でより再利用可能にします。

#### このフェーズが重要な理由

コアフローが存在すれば、ユーザーはより速い出発点を望みます。
Preset とエコシステム対応のデフォルトが摩擦を削減します。

#### 主な作業項目

- preset パッケージングアプローチを定義する
- 現在の example-to-preset の進化を形式化する
- Rust preset を作成する
- 汎用 preset を作成する
- example リポジトリパターンを追加する
- GitHub workflow の例を追加する
- 新しいプロジェクトのオンボーディングを改善する

#### 期待される成果物

- 少なくとも 1 つの再利用可能な preset が存在する
- example 構造が preset の概念に明確にマッピングされている
- プロジェクトのブートストラップが速くなる
- エコシステム固有の慣習が適用しやすくなる

#### 完了の定義

Phase 5 は以下の時に完了します。

- ユーザーがすべてを手動で定義する代わりに preset から始められる
- examples と preset が明確な関係を持っている
- 少なくとも 1 つの言語固有と 1 つの言語非依存のパスが存在する
- Batonel が現実的なプロジェクトへの採用が容易に感じられる

#### 作りすぎてはいけないもの

作りすぎてはいけないもの：

- 早期に多すぎる preset
- 深いフレームワーク固有の特化
- 過度に硬直した preset のロック
- すべての既存リポジトリの自動移行
- 完全なプラグインエコシステム

焦点は有用な出発点であり、最大のカバレッジではありません。

---

### フェーズ横断的な原則

いくつかの原則はすべてのフェーズに適用されます。

#### 実装の前に概念を

実装は安定した概念に従うべきであり、その場で発明すべきではありません。

#### マジックよりも明示性を

Batonel はコードを実行するだけでなく、ファイルを読むことで理解できるべきです。

#### Artifact を主要な実行ユニットとして維持する

Artifact は計画、contract、prompt、将来の verify の中心です。

#### ソースコードの外にアーキテクチャを保全する

重要なアーキテクチャの意図は本番コードの中だけに存在すべきではありません。

#### 早期のエコシステムロックインを避ける

Batonel は Rust に適した examples から始めることができますが、Rust 専用になってはいけません。

---

### 推奨される実践的なマイルストーン順序

実践的な内部マイルストーンのシーケンスはこのようになるかもしれません。

1. リポジトリと examples をブートストラップする
2. 概念と用語集を安定させる
3. スキーマドラフトを安定させる
4. architecture flow と preset をドキュメント化する
5. 最小の設定読み込みを実装する
6. パス解決を実装する
7. スキャフォルド生成を実装する
8. prompt 生成を実装する
9. verify を実装する
10. examples を再利用可能な preset に進化させる

このシーケンスは作業を基盤的かつ漸進的に保ちます。

---

### このドキュメントの使い方

このファイルは次のことを決定する際に使用します。

- 次に何に取り組むか
- フェーズが実際に完了しているかどうか
- 機能が早すぎるかどうか
- コントリビューションのスコープの設定方法
- プロジェクトの成熟度をコントリビューターに説明する方法

`ROADMAP.md` がプロジェクトがどこへ向かっているかを述べるなら、
このファイルは各段階が実践的にどのように動作すべきかを説明します。

---

### まとめ

ロードマップは Batonel が焦点を維持するのを助けるべきです。

主なアイデアは次のとおりです。

- まずリポジトリを理解可能にする
- 次にモデルを安定させる
- 次に運用可能にする
- 次に AI に有用にする
- 次に verify 可能にする
- 次に preset を通じて再利用可能にする

1 つだけ覚えておくなら、これを覚えてください。

**Batonel は層ごとに成長すべきです：まず明確さ、次に構造、次に実行、そして再利用。**

---

## 追加更新（Phase 6-9 の実行詳細）

以下は、Phase 5 完了後に追加された実行タスクの要点です。
英語Issueを実行単位として運用し、計画文書は英日併記で管理します。

### Phase 6: OSS完成と継続利用基盤の土台

#### 目標

オンボーディングを決定論的に安定化し、契約情報を標準化する。

#### 主な作業

- `init` / `plan` の再現性を強化
- `project.baton.yaml` を契約の正本として標準化
- ドキュメントと examples の e2e 検証を拡充

#### 実行Issue

- [#127](https://github.com/Arcflect/batonel/issues/127)
- [#128](https://github.com/Arcflect/batonel/issues/128)
- [#129](https://github.com/Arcflect/batonel/issues/129)

### Phase 7: Audit / Policy / CI 統合

#### 目標

生成後も継続的に準拠性を保てる運用基盤を整える。

#### 主な作業

- `audit` ベースラインと PR ゲートの導入
- `fix` の安全境界定義（dry-run優先）
- 最小ポリシープロファイルの適用

#### 実行Issue

- [#130](https://github.com/Arcflect/batonel/issues/130)
- [#131](https://github.com/Arcflect/batonel/issues/131)
- [#132](https://github.com/Arcflect/batonel/issues/132)

### Phase 8: Preset Registry と Guard

#### 目標

Preset 流通を拡大しつつ、contract-first / sidecar-first を維持する。

#### 主な作業

- preset registry の publish/install 試作
- Guard サイドカーによる実行時・CI の検証
- preset が単なるフォルダ雛形へ崩れないことの整合確認

#### 実行Issue

- [#133](https://github.com/Arcflect/batonel/issues/133)
- [#134](https://github.com/Arcflect/batonel/issues/134)
- [#135](https://github.com/Arcflect/batonel/issues/135)

### Phase 9: Migration / Org-level Control / Reporting

#### 目標

組織導入に必要な更新運用と可視化を提供する。

#### 主な作業

- preset versioning と migration 支援
- org/team override 優先順位モデル
- 複数repo横断の compliance report

#### 実行Issue

- [#136](https://github.com/Arcflect/batonel/issues/136)
- [#137](https://github.com/Arcflect/batonel/issues/137)
- [#138](https://github.com/Arcflect/batonel/issues/138)

### Phase 10: Autonomous Governance

#### 目標

安全なレビュー境界を維持しながら、運用ガバナンスを自動化する。

#### 主な作業

- `batonel` CLI 配布・リリース手順の標準化
- 違反トリアージと是正優先度付けの自動化
- 承認付き `fix` 段階適用ワークフローの導入

#### 実行Issue

- [#152](https://github.com/Arcflect/batonel/issues/152)
- [#153](https://github.com/Arcflect/batonel/issues/153)
- [#154](https://github.com/Arcflect/batonel/issues/154)

### Phase 11: Enterprise Control Plane

#### 目標

組織運用に必要な統制責任と実行責任を明確化する。

#### 主な作業

- ガバナンス運用向けRBACロールモデルの提供
- 期限付きoverrideとポリシー例外のライフサイクル実装
- 監査証跡の保持・抽出フロー強化

#### 実行Issue

- [#155](https://github.com/Arcflect/batonel/issues/155)
- [#156](https://github.com/Arcflect/batonel/issues/156)
- [#157](https://github.com/Arcflect/batonel/issues/157)

### Phase 12: Ecosystem & Marketplace

#### 目標

信頼可能なPresetエコシステム運用を内外のコントリビューターへ拡張する。

#### 主な作業

- Preset署名と信頼検証パイプラインの定義
- パートナーPreset投稿・審査フローの構築
- エコシステム準拠成熟度ベンチマークモデルの公開

#### 完了の定義

- 署名済み Preset の検証が配布・インストールフローで利用可能である
- パートナー投稿・審査ワークフローがドキュメント化され運用中である
- ベンチマークモデルが参加リポジトリ間のコンプライアンス成熟度を比較できる；
  5 段階モデル（L0–L4）および自己評価ツールについては
  [docs/ecosystem-compliance-maturity.md](../docs/ecosystem-compliance-maturity.md) を参照

#### 関連ドキュメント

- [docs/ecosystem-compliance-maturity.md](../docs/ecosystem-compliance-maturity.md)
- [docs/partner-preset-review.md](../docs/partner-preset-review.md)
- [docs/governance-rbac.md](../docs/governance-rbac.md)
- [docs/decisions/0025-ecosystem-compliance-maturity-benchmark-model.md](../docs/decisions/0025-ecosystem-compliance-maturity-benchmark-model.md)

#### 実行Issue

- [#158](https://github.com/Arcflect/batonel/issues/158)
- [#159](https://github.com/Arcflect/batonel/issues/159)
- [#160](https://github.com/Arcflect/batonel/issues/160)

### 実行中Issue一覧（Phase6-Phase12）

進行管理は以下のIssue番号を基準とします。

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