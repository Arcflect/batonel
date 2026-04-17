# 0022 Guard Examples-First Behavior During Preset Implementation

- Status: accepted
- Date: 2026-04-03

## Context

Batonel now has active preset implementation work in parallel with established
examples and onboarding documentation.

This creates a sequencing risk:

- preset packaging and CLI behavior may evolve faster than example stabilization
- contributors may treat examples as runtime fixtures instead of teaching assets
- preset scope may expand before the example-to-preset transition conditions are met

Earlier decisions already define direction:

- [ADR-0005](./0005-examples-precede-presets.md): examples precede presets
- [ADR-0017](./0017-formalize-example-to-preset-mapping.md): mapping and transition rules
- [ADR-0020](./0020-align-presets-with-examples-docs-and-schemas.md): alignment expectations

What is still needed is an explicit implementation-time guardrail policy so
contributors can decide whether a preset change is in-sequence or premature.

## Decision

Preset implementation remains downstream from examples-first progression.

### 1. Examples remain documentation-first assets

Examples are authoritative for teaching and model communication.

Contributors should treat `examples/*` primarily as:

- explanatory architecture input/output references
- narrative assets for onboarding and contributor understanding
- source context for deriving reusable preset defaults

Examples should not be re-scoped into preset runtime internals.

### 2. Preset implementation eligibility gate

A preset implementation or expansion is in-scope only when all are true:

- mapped example defaults are stable and internally consistent
- reusable-vs-illustrative boundaries are explicit for the affected content
- documentation already explains the example behavior before preset operationalization
- no divergence from mapping/alignment rules exists unless documented by decision

If any condition fails, the change should be handled as example/documentation
stabilization work first.

### 3. What counts as outpacing the current model

The following are treated as out-of-sequence and should be deferred:

- adding new supported preset ids without a stabilized mapped example direction
- introducing preset-only semantics not explained in examples/docs
- widening preset CLI surface before defaults and boundaries are documented
- letting preset defaults drift from mapped examples without explicit decision

### 4. Contributor review checklist

For preset-related pull requests, reviewers should explicitly check:

- does this change keep examples educational and readable?
- is preset behavior derived from documented example defaults?
- is this operationalization step justified by current mapping rules?
- does this PR avoid introducing architecture doctrine through presets?

## Consequences

- Preset work remains incremental and tied to proven examples.
- Contributors get a simple sequence gate for deciding where work belongs.
- Batonel preserves examples-first intent while still enabling gradual preset adoption.

---

## 日本語

# 0022 preset 実装時に examples-first 振る舞いを守る

- ステータス: 承認済み
- 日付: 2026-04-03

## コンテキスト

Batonel では、確立した examples と onboarding docs と並行して、
preset 実装作業が進んでいます。

この状況にはシーケンス上のリスクがあります。

- preset のパッケージングや CLI 挙動が example の安定化より先行する可能性
- contributors が examples を teaching 資産ではなく runtime fixture として扱う可能性
- example-to-preset 移行条件を満たす前に preset スコープが拡大する可能性

既存 decision は方向を定義済みです。

- [ADR-0005](./0005-examples-precede-presets.md): examples は presets より先
- [ADR-0017](./0017-formalize-example-to-preset-mapping.md): マッピングと移行ルール
- [ADR-0020](./0020-align-presets-with-examples-docs-and-schemas.md): 整合期待

不足しているのは、実装時点で「この preset 変更は順序内か、早すぎるか」を
判断できる明示的ガードレール方針です。

## 決定事項

preset 実装は examples-first 進行の下流に維持します。

### 1. examples は documentation-first 資産として維持する

examples は teaching とモデル伝達のための一次資産です。

contributors は `examples/*` を主に次として扱います。

- 説明用アーキテクチャ入出力リファレンス
- onboarding と contributor 理解のためのナラティブ資産
- 再利用可能 preset defaults を導出するための出発点

examples を preset runtime 内部に再定義してはいけません。

### 2. preset 実装の適用可否ゲート

次をすべて満たす場合にのみ、preset 実装/拡張を in-scope とします。

- 対応 example defaults が安定し、内部整合している
- 影響範囲の reusable と illustrative の境界が明示されている
- preset 運用化の前に example 挙動が docs で説明されている
- マッピング/整合ルールからの差分がある場合は decision で明示されている

いずれかを満たさない場合、その変更は先に
example/documentation の安定化作業として扱います。

### 3. 現行モデルを追い越すとみなすケース

次は順序外として扱い、延期します。

- 安定した対応 example 方向なしで新しい supported preset id を追加する
- examples/docs で説明されていない preset 専用意味論を導入する
- defaults と境界の文書化前に preset CLI 面を拡張する
- decision なしに mapped examples から preset defaults をドリフトさせる

### 4. contributor review チェックリスト

preset 関連 PR では、レビュー時に次を明示確認します。

- この変更は examples の教育的な読みやすさを保っているか
- preset 挙動は文書化された example defaults から導出されているか
- この運用化ステップは current mapping rules で正当化されるか
- preset を通じて architecture doctrine を持ち込んでいないか

## 結果

- preset 作業は、実証済み example に結びついた段階的進化を維持できる。
- contributors は、作業の置き場所を判断するための単純なシーケンスゲートを得る。
- Batonel は examples-first 意図を保ちながら、preset 採用を段階的に進められる。