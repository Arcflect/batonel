# 0019 Define Preset Customization Boundaries

- Status: accepted
- Date: 2026-04-03

## Context

Preset onboarding has become concrete through:

- ADR-0015: minimal preset model
- ADR-0016: preset packaging approach
- ADR-0018: minimal bootstrap flow from presets

As preset adoption grows, a common risk appears:
users may misunderstand presets as fixed architecture truth.

Archflow's model requires the opposite:
presets are starting points that users evolve intentionally.

## Decision

We define explicit customization boundaries for preset-based projects.

### 1. What users are expected to customize

Users are expected to customize after bootstrap:

- `project.arch.yaml`: project name, module set, feature naming
- `placement.rules.yaml`: path prefixes, file extensions, role-path mapping details
- `contracts.template.yaml`: responsibilities, must_not constraints, implementation size guidance
- `artifacts.plan.yaml`: starter artifact inventory (add/remove/rename artifacts)

These are normal and encouraged adaptations.

### 2. How placement rules, roles, and contract templates can evolve

Placement rules can evolve by repository context:

- changing root prefixes (`src/` -> `apps/api/src/`, `crates/`, etc.)
- changing file extensions per ecosystem
- refining role-to-path structure as the project grows

Roles can evolve when architecture intent is clarified:

- adding new roles for new boundary types
- splitting overloaded roles into clearer role definitions
- renaming roles when domain language improves

Contract templates can evolve by team policy:

- tightening or relaxing responsibilities
- refining must_not constraints
- adjusting implementation size expectations by role

### 3. What should remain stable as guardrails

Customization should preserve these guardrails:

- presets remain starting packages, not dogma
- contract-first and sidecar-first principles remain intact
- role meaning should stay coherent enough to map to placement and contracts
- changes should remain verifiable through existing `archflow verify` checks

### 4. Non-goal

This decision does not define a central policy engine that restricts customization.

The intent is clarity, not lock-in.

## Consequences

- Users understand presets as flexible accelerators.
- Teams can adapt presets without feeling they are violating the model.
- Preset philosophy remains aligned with Archflow's non-dogmatic architecture stance.

---

## 日本語

# 0019 preset カスタマイズ境界を定義する

- ステータス: 承認済み
- 日付: 2026-04-03

## コンテキスト

preset オンボーディングは次の決定で具体化されました。

- ADR-0015: 最小 preset モデル
- ADR-0016: preset パッケージング方針
- ADR-0018: preset からの最小ブートストラップフロー

preset 採用が進むと、共通のリスクが発生します。
ユーザーが preset を固定的なアーキテクチャ真理と誤解してしまうことです。

Archflow のモデルは逆を要求します。
preset は、ユーザーが意図的に進化させる出発点です。

## 決定事項

preset ベースプロジェクトのための明示的なカスタマイズ境界を定義します。

### 1. ユーザーがカスタマイズすべきもの

ブートストラップ後、ユーザーは次をカスタマイズすることが期待されます。

- `project.arch.yaml`: project 名、module セット、feature 命名
- `placement.rules.yaml`: path prefix、file extension、role-path mapping 詳細
- `contracts.template.yaml`: responsibilities、must_not 制約、実装サイズ指針
- `artifacts.plan.yaml`: starter artifact inventory（artifact の追加/削除/改名）

これらは通常かつ推奨される適応です。

### 2. placement rules・roles・contract templates の進化方法

placement rules は repository 文脈で進化できます。

- ルート prefix の変更（`src/` -> `apps/api/src/`、`crates/` など）
- ecosystem に合わせた file extension の変更
- project 成長に合わせた role-to-path 構造の洗練

roles はアーキテクチャ意図の明確化に応じて進化できます。

- 新しい境界タイプに対する role 追加
- 過負荷な role をより明確な role 定義へ分割
- ドメイン言語改善に合わせた role 名変更

contract templates はチーム方針に応じて進化できます。

- responsibilities の強化または緩和
- must_not 制約の洗練
- role ごとの実装サイズ期待値の調整

### 3. ガードレールとして安定させるべきこと

カスタマイズは次のガードレールを維持すべきです。

- preset はドグマではなく出発点パッケージであり続ける
- contract-first / sidecar-first 原則を維持する
- role の意味は placement と contract にマップできる程度に一貫させる
- 変更は既存の `archflow verify` チェックで検証可能な状態を保つ

### 4. 非目標

この決定は、カスタマイズを制限する中央ポリシーエンジンを定義しません。

意図は lock-in ではなく、明確化です。

## 結果

- ユーザーは preset を柔軟な加速装置として理解できる。
- チームはモデル違反の不安なく preset を適応できる。
- preset 哲学は Archflow の非ドグマ的な設計姿勢と整合したまま維持される。
