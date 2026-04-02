# 0023 Guard Sidecar-First and Contract-First Behavior in Presets

- Status: accepted
- Date: 2026-04-03

## Context

Preset packaging is now part of the onboarding path.
As presets become more visible, there is a risk they are interpreted as
directory-only starter templates.

If that happens, Archflow would lose core model intent:

- contract-centered artifact boundaries
- sidecar files as first-class operational artifacts
- prompt generation and verification derived from contracts

Existing decisions already define these principles in other areas:

- [ADR-0002](./0002-contract-is-the-source-of-truth-for-artifact-boundaries.md)
- [ADR-0004](./0004-sidecar-files-are-first-class.md)
- [ADR-0014](./0014-guard-contract-first-and-sidecar-first-behavior-in-phase-4-verification.md)

What is needed now is a preset-specific guardrail so setup acceleration does not
flatten Archflow into folder scaffolding without architectural meaning.

## Decision

Preset design and implementation must preserve sidecar-first and contract-first
behavior as non-negotiable model invariants.

### 1. Contract-first invariant for presets

Presets must include or derive reusable contract defaults as part of identity.

Minimum requirement:

- `contracts.template.yaml` is required for a supported preset

Interpretation:

- role boundaries are carried by contract templates, not only by paths
- placement defaults are supportive, not the source of boundary truth
- preset usefulness is measured by architectural intent, not directory shape alone

### 2. Sidecar-first invariant for preset workflows

Preset onboarding must keep sidecar outputs first-class in normal flow.

This means preset-driven projects are expected to continue with sidecar-aware
commands (`plan`, `scaffold`, `prompt`, `verify`) where sidecars are generated,
referenced, and validated as operational artifacts.

Presets are entry points into the sidecar model, not an alternative to it.

### 3. What preset design must not become

The following are out of model and should be rejected unless replaced by an
explicit future decision:

- preset definitions that only provide folder layout without contract defaults
- onboarding guidance that treats sidecar files as optional byproducts
- preset success criteria based only on generated source path structure
- documentation that implies contracts can be skipped when presets are used

### 4. Review checklist for preset-related changes

For preset additions/changes, reviewers should check:

- does the preset preserve contract templates as first-class defaults?
- does the onboarding path keep sidecar outputs visible and expected?
- are boundaries described in contract terms, not path-only terms?
- does this change avoid reducing presets to directory templates?

## Consequences

- Presets remain aligned with Archflow core philosophy.
- Setup speed improves without sacrificing architectural intent.
- Contributors get clear criteria for rejecting model-weakening preset changes.

---

## 日本語

# 0023 preset における sidecar-first と contract-first 挙動を保護する

- ステータス: 承認済み
- 日付: 2026-04-03

## コンテキスト

preset packaging は onboarding 経路の一部になりました。
preset の可視性が上がるにつれて、preset が
「ディレクトリ雛形だけのスターター」と解釈されるリスクがあります。

そうなると Archflow の中核意図が失われます。

- contract 中心の artifact 境界
- sidecar ファイルを first-class な運用 artifact として扱うこと
- contract から導出される prompt 生成と verification

既存 decision は、他領域でこれら原則を定義済みです。

- [ADR-0002](./0002-contract-is-the-source-of-truth-for-artifact-boundaries.md)
- [ADR-0004](./0004-sidecar-files-are-first-class.md)
- [ADR-0014](./0014-guard-contract-first-and-sidecar-first-behavior-in-phase-4-verification.md)

今必要なのは preset 特有のガードレールです。
セットアップ高速化が、アーキテクチャ意味を持たない
フォルダスキャフォルディングへと Archflow を平坦化しないようにします。

## 決定事項

preset の設計と実装は、sidecar-first と contract-first 挙動を
交渉不可のモデル不変条件として保持します。

### 1. preset における contract-first 不変条件

preset は identity の一部として、再利用可能 contract defaults を
含むか導出できなければなりません。

最小要件:

- supported preset では `contracts.template.yaml` を必須とする

解釈:

- role 境界は path だけでなく contract template で運ばれる
- placement defaults は補助であり、境界真実の源ではない
- preset の有用性は directory 形状だけでなく architecture intent で測る

### 2. preset ワークフローにおける sidecar-first 不変条件

preset onboarding は通常フローで sidecar outputs を first-class に保ちます。

つまり、preset から始めたプロジェクトでも sidecar を生成・参照・検証する
コマンド (`plan`, `scaffold`, `prompt`, `verify`) を継続利用することが前提です。

preset は sidecar モデルへの入口であり、代替ではありません。

### 3. preset 設計が陥ってはならない状態

次はモデル外であり、将来の明示 decision がない限り採用しません。

- contract defaults を持たず folder layout だけを提供する preset 定義
- sidecar ファイルを任意の副産物として扱う onboarding ガイダンス
- 生成ソース path 構造のみで preset 成功を判定する基準
- preset 利用時に contract を省略可能と示唆するドキュメント

### 4. preset 変更レビュー用チェックリスト

preset 追加/変更時、レビューで次を確認します。

- preset は contract templates を first-class defaults として保持しているか
- onboarding 経路は sidecar outputs を可視で期待値として扱っているか
- 境界は path-only ではなく contract 用語で記述されているか
- この変更は preset を directory template へ還元していないか

## 結果

- preset は Archflow のコア哲学と整合したまま維持される。
- セットアップ速度を上げつつ、アーキテクチャ意図を損なわない。
- モデルを弱める preset 変更を却下するための明確基準が得られる。