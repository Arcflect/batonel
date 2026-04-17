# 0017 Formalize Example-to-Preset Mapping

- Status: accepted
- Date: 2026-04-02

## Context

Batonel now has:

- a minimal preset model (ADR-0015)
- a preset packaging approach (ADR-0016)
- three active examples (`minimal`, `generic-layered`, `rust-clean-hexagonal`)

What is still ambiguous is how each example transitions into a supported preset,
and which parts of an example remain illustrative versus reusable.

Without an explicit mapping policy:

- example work and preset work can drift apart
- contributors may copy example-only artifacts into presets without stable intent
- preset evolution timing can become inconsistent across architectures

This issue is about structure and direction,
not immediate implementation of all presets.

## Decision

We define a formal mapping from each current example to one preset direction,
plus transition rules for what becomes reusable defaults.

### 1. Canonical mapping

The current example-to-preset map is:

| Example | Preset direction | Preset id |
|---|---|---|
| `examples/minimal` | minimal starter preset | `minimal` |
| `examples/generic-layered` | language-agnostic layered preset | `generic-layered` |
| `examples/rust-clean-hexagonal` | Rust clean/hexagonal preset | `rust-clean-hexagonal` |

The preset id follows ADR-0016 naming conventions and is the stable identifier
for packaging and future CLI selection.

### 2. Illustrative vs reusable boundaries

For each example, files and content are classified as follows.

Reusable defaults (eligible for preset packaging):

- project defaults (`project.baton.yaml`)
- role-to-path defaults (`placement.rules.yaml`)
- role-based contract defaults (`contracts.template.yaml`)
- optional starter artifact plan when it is broadly reusable

Illustrative content (stays example-first):

- tutorial-oriented explanatory text
- one-off artifact names chosen only for teaching flow
- expected output snapshots tied to documentation demonstration
- narrowly scoped scenario details not intended as reusable defaults

### 3. Transition rules

An example can move to a supported preset only when all of the following are true:

- naming is stable and aligned with preset id conventions
- role set and placement rules are internally consistent
- contract template defaults are reusable beyond one tutorial scenario
- the package can be represented in the ADR-0016 preset directory shape
- maintainers judge it broadly useful enough to recommend as a starting point

If these conditions are not met, the example remains illustrative and should not be
treated as a supported preset package yet.

### 4. Contributor workflow implications

When updating examples:

- contributors should mark whether a change is illustrative-only or preset-reusable
- reusable default changes should preserve mapping stability for the target preset id
- example-only improvements should avoid accidental expansion of preset scope

This keeps examples and presets connected without collapsing them into the same artifact type.

## Consequences

- The relationship between current examples and future presets is explicit.
- Contributors can evolve examples with clear transition expectations.
- Preset implementation can proceed incrementally without ambiguity about source examples.
- Batonel keeps examples descriptive and presets operational, consistent with ADR-0005.

---

## 日本語

# 0017 example から preset へのマッピングを正式化する

- ステータス: 承認済み
- 日付: 2026-04-02

## コンテキスト

Batonel には現在、次があります。

- 最小 preset モデル（ADR-0015）
- preset パッケージング方針（ADR-0016）
- 3 つの active examples（`minimal`、`generic-layered`、`rust-clean-hexagonal`）

まだ曖昧なのは、各 example がどのように supported preset に移行するか、
また example のどの部分が説明用で、どの部分が再利用可能かです。

明示的なマッピング方針がなければ：

- example 作業と preset 作業が乖離しうる
- コントリビューターが安定した意図なしに example 専用要素を preset にコピーしうる
- アーキテクチャ間で preset 進化のタイミングが不整合になりうる

この issue は構造と方向性に関するものであり、
すべての preset を即時実装するものではありません。

## 決定事項

現在の各 example から 1 つの preset 方向への正式なマッピングと、
何が再利用可能デフォルトになるかの移行ルールを定義します。

### 1. 正式なマッピング

現在の example-to-preset マップは次のとおりです。

| Example | Preset direction | Preset id |
|---|---|---|
| `examples/minimal` | 最小スターター preset | `minimal` |
| `examples/generic-layered` | 言語非依存レイヤード preset | `generic-layered` |
| `examples/rust-clean-hexagonal` | Rust clean/hexagonal preset | `rust-clean-hexagonal` |

Preset id は ADR-0016 の命名規則に従い、
パッケージングと将来の CLI 選択で使う安定識別子です。

### 2. 説明用と再利用可能の境界

各 example のファイルと内容を次のように分類します。

再利用可能デフォルト（preset packaging 対象）:

- project defaults（`project.baton.yaml`）
- role-to-path defaults（`placement.rules.yaml`）
- role-based contract defaults（`contracts.template.yaml`）
- 広く再利用できる場合に限る optional starter artifact plan

説明用コンテンツ（example 優先で維持）:

- チュートリアル指向の説明テキスト
- 教示フローのためだけに選ばれた単発の artifact 名
- ドキュメントデモに紐づく expected output スナップショット
- 再利用デフォルトを意図しない狭いシナリオ詳細

### 3. 移行ルール

example が supported preset に移行できるのは、次をすべて満たす場合のみです。

- 命名が安定し、preset id 規約に整合している
- role セットと配置ルールが内部的に一貫している
- contract template defaults が 1 つのチュートリアルを超えて再利用可能である
- ADR-0016 の preset directory shape で表現可能である
- メンテナーが推奨スタート地点として十分に汎用的と判断する

これらを満たさない場合、example は説明用途のままとし、
supported preset package として扱いません。

### 4. コントリビューターのワークフローへの影響

example 更新時は次を行います。

- 変更が illustrative-only か preset-reusable かを明示する
- 再利用可能デフォルトの変更は target preset id のマッピング安定性を保つ
- example-only 改善で preset スコープが意図せず拡張しないようにする

これにより、examples と presets を接続しつつ、同一 artifact type へと混同させないことができます。

## 結果

- current examples と future presets の関係が明示化される。
- コントリビューターは明確な移行期待を持って example を進化できる。
- preset 実装は source examples に対する曖昧さなく段階的に進められる。
- Batonel は、ADR-0005 と整合して examples を記述的、presets を運用的に維持できる。
