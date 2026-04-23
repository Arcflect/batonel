# Contributing Areas

This document explains where contributors can help Batonel today.

Its purpose is to make contribution opportunities easier to understand by clarifying:

- which areas already exist
- which areas still need definition
- which areas are good for design contributions
- which areas are good for implementation contributions
- which areas are intentionally not ready yet

Batonel is still evolving in layers.
That means not every contribution area has the same level of maturity.

This document helps contributors choose the right place to start.

---

## Overview

Batonel currently has several major contribution areas:

- repository and documentation
- examples
- concept model
- schema design
- CLI implementation
- AI handoff design
- verification design
- presets and ecosystem fit
- governance and trust

Some of these areas are ready for immediate contribution.
Others are still better suited for discussion and design refinement.

---

## 1. Repository and documentation

### Why this area matters

Batonel is still defining its public shape.
Clear documentation makes the project understandable before the implementation grows.

### Good contribution types

- improve wording in existing docs
- clarify README sections
- improve document navigation
- refine contributor guidance
- fix inconsistencies in terminology
- improve cross-links between docs

### Good first issues in this area

- broken or missing links
- inconsistent naming across docs
- missing references between examples and concepts
- small readability improvements
- missing explanation of a concept already used elsewhere

### Best for

- first-time contributors
- writers
- editors
- contributors who want to learn the project before writing code

---

## 2. Examples

### Why this area matters

Examples are one of the clearest ways to explain Batonel.

They show how architectural intent becomes:

- placement rules
- artifact plans
- contracts
- prompts
- expected outputs

### Good contribution types

- improve existing examples
- add missing contract fields where appropriate
- improve expected output consistency
- add explanation to example README files
- propose new example structures
- align examples with evolving concept definitions

### Good first issues in this area

- make example naming more consistent
- align one example with current schema terminology
- improve one example README
- add missing prompt or contract file in an existing example
- simplify an example that is too noisy

### Best for

- contributors who like concrete artifacts
- contributors who want to work without touching runtime code first
- people interested in architecture examples

---

## 3. Concept model

### Why this area matters

Batonel depends on a stable conceptual model.

If concepts such as `artifact`, `contract`, or `preset` drift in meaning,
implementation will become inconsistent.

### Good contribution types

- clarify concept boundaries
- refine concept definitions
- identify overlap between concepts
- suggest missing concepts
- improve explanatory docs for how concepts connect

### Good discussion topics in this area

- what should count as an artifact
- how strict contracts should be
- whether prompts should be generated or partially editable
- how presets differ from examples
- how role naming should evolve

### Best for

- contributors interested in design and architecture thinking
- contributors who want to shape the model before implementation hardens
- maintainers and frequent contributors

---

## 4. Schema design

### Why this area matters

Schemas are the bridge between documentation and implementation.

They make the model more precise and help future CLI behavior stay consistent.

### Good contribution types

- improve schema readability
- tighten field definitions
- propose optional or required field changes
- align schemas with examples
- suggest validation-oriented improvements
- reduce ambiguity across schema drafts

### Good contribution topics

- field naming consistency
- optional vs required boundaries
- future compatibility with JSON Schema
- path override behavior
- contract and prompt schema alignment

### Best for

- contributors who like structured formats
- contributors who want to shape future validation behavior
- contributors comfortable with system modeling

---

## 5. CLI implementation

### Why this area matters

The CLI is where Batonel becomes operational.

This area turns concepts and schemas into a working tool.

### Current focus areas

- config loading
- file parsing
- path resolution
- scaffold generation
- prompt generation
- verification entry points
- **acceptance criteria verification** (see [Acceptance Criteria](./acceptance-criteria.md))

### Good contribution types

- implement config parsing
- implement role-to-path resolution
- implement scaffold directory creation
- implement sidecar file generation
- add basic CLI tests
- improve error handling in early commands

### Best for

- Rust contributors
- contributors interested in practical tooling
- contributors comfortable turning specs into code

### Important note

CLI work should follow the concept and schema model,
not invent new concepts during implementation unless discussed first.

---

## 6. AI handoff design

### Why this area matters

AI handoff is one of the main reasons Batonel exists.

The goal is to make artifacts implementable by humans or lightweight models
with clear constraints.

### Good contribution types

- refine prompt structure
- improve completion criteria design
- define prompt modes such as compact vs detailed
- improve prompt consistency across examples
- identify which contract fields should always flow into prompts

### Good discussion topics

- what makes a prompt usable by a smaller model
- how much context is too much
- whether prompts should be fully generated or partly templated
- how role-specific prompt defaults should work

### Best for

- contributors interested in AI-assisted development
- contributors who use coding assistants in practice
- contributors interested in prompt quality and constraints

---

## 7. Verification design

### Why this area matters

Without verification, contracts and scaffold structure may drift over time.

Verification protects architectural intent after initial generation.

### Good contribution types

- define verify scope
- suggest first verification rules
- expand **Product Guarantees** (see [Acceptance Criteria](./acceptance-criteria.md))
- identify required consistency checks
- propose example verification output
- design status and file presence checks

### Good first verification targets

- required file presence
- required contract fields
- role consistency across files
- path consistency with placement rules
- contract/prompt presence for planned artifacts

### Best for

- contributors who like rule systems
- contributors interested in consistency and tooling quality
- contributors who want to help before deep code parsing exists

---

## 8. Presets and ecosystem fit

### Why this area matters

Presets will make Batonel easier to adopt in real projects.

They help turn examples and conventions into reusable starting points.

### Good contribution types

- refine existing example-to-preset direction
- suggest role sets for future presets
- identify ecosystem-specific needs
- propose default project bootstrap structures
- compare preset boundaries across architectures

### Good discussion topics

- when an example should become a preset
- how opinionated presets should be
- how to keep presets flexible
- how many presets should be supported early on

### Best for

- contributors interested in architecture patterns
- contributors with ecosystem experience
- contributors who want to improve adoption paths

---

## 9. Governance and Trust

### Why this area matters

As Batonel expands into an ecosystem of reusable presets, governance and trust become critical.
This area ensures that users can verify offline signatures for partner presets and that RBAC policies correctly authorize overrides.

### Good contribution types

- add or update allowed public keys for ecosystem verification
- refine compliance maturity benchmarks
- propose improvements to the RBAC schema
- refine audit evidence retention flows

### Good discussion topics

- how strict preset signature verification should be
- exceptions handling for internal corporate presets
- balancing offline-first requirements with central registry ideas

### Best for

- contributors interested in security and compliance
- DevSecOps practitioners
- partners publishing shared ecosystem presets

---

## Areas that are especially good right now

At the current stage of Batonel, the most contribution-friendly areas are:

### High readiness
- documentation improvements
- example refinement
- glossary and terminology cleanup
- schema clarification
- concept alignment

### Medium readiness
- early CLI parsing and scaffold work
- prompt generation structure
- verification rule drafting

### Lower readiness for direct implementation
- full preset engine
- editor integration
- deep code-aware checks
- advanced plugin ecosystem
- vendor-specific AI integrations

These lower-readiness areas are still useful discussion topics,
but they are not the best place to begin implementation.

---

## Good first contribution paths

If you want a practical way to start, choose one of these paths.

### Path 1: documentation-first
- read the core docs
- improve one concept page
- improve cross-linking
- fix inconsistent naming

### Path 2: examples-first
- choose one example
- check it against schemas
- improve consistency
- improve its README

### Path 3: schema-first
- review one schema draft
- identify ambiguous fields
- propose clarification
- align examples with the improved schema

### Path 4: implementation-first
- implement one parsing step
- implement one resolution step
- implement one generation step
- keep behavior aligned with docs and examples

---

## What contributors should avoid for now

To keep the project coherent, contributors should avoid pushing too early into these areas without alignment:

- adding many new concepts at once
- creating architecture-specific behavior not grounded in the current model
- coupling Batonel too tightly to one language or framework
- making prompts the primary source of truth
- implementing advanced features before the minimal flow is stable

This does not mean these ideas are bad.
It means timing matters.

---

## How to choose the right area

A simple way to choose is:

- if you want low-risk contribution, start with docs or examples
- if you want to shape the model, work on concepts or schemas
- if you want to build the tool, work on CLI internals
- if you care about AI workflows, work on prompts and handoff design
- if you care about long-term consistency, work on verification design

All of these are valid ways to contribute.

---

## Summary

Batonel has multiple contribution areas, but not all are equally mature yet.

The best areas to contribute right now are:

- documentation
- examples
- concept clarification
- schema refinement

The next layer after that is:

- minimal CLI
- prompt generation
- verification drafting

If you remember only one thing, remember this:

**the best contribution is the one that strengthens clarity without outrunning the current model**

---

## 日本語

このドキュメントは、今日コントリビューターが Batonel のどこを手伝えるかを説明します。

目的は、次の点を明確にすることでコントリビューションの機会を把握しやすくすることです。

- どの領域がすでに存在するか
- どの領域はまだ定義が必要か
- どの領域が設計への貢献に適しているか
- どの領域が実装への貢献に適しているか
- どの領域が意図的にまだ準備できていないか

Batonel はまだ層ごとに進化しています。
つまり、すべてのコントリビューション領域が同じ成熟度にあるわけではありません。

このドキュメントは、コントリビューターが適切な開始点を選ぶのに役立ちます。

---

### 概要

Batonel には現在いくつかの主要なコントリビューション領域があります。

- リポジトリとドキュメント
- examples
- 概念モデル
- スキーマ設計
- CLI 実装
- AI ハンドオフ設計
- verify 設計
- preset とエコシステムへの適合
- ガバナンスと信頼 (Governance and Trust)

これらの領域の一部は即時のコントリビューションの準備ができています。
その他はまだ議論と設計の精緻化に適しています。

---

### 1. リポジトリとドキュメント

#### この領域が重要な理由

Batonel はまだその公開形状を定義しています。
明確なドキュメントは、実装が成長する前にプロジェクトを理解可能にします。

#### 良いコントリビューションの種類

- 既存のドキュメントの文言を改善する
- README のセクションを明確にする
- ドキュメントのナビゲーションを改善する
- コントリビューターガイダンスを洗練させる
- 用語の不整合を修正する
- docs 間のクロスリンクを改善する

#### この領域の最初の issue として良いもの

- 壊れた、または不足しているリンク
- ドキュメント間の命名の不一致
- examples と concepts の間の参照の欠如
- 小さな読みやすさの改善
- 他の場所ですでに使用されている概念の説明の欠如

#### 最適な対象者

- 初めてのコントリビューター
- ライター
- エディター
- コードを書く前にプロジェクトを学びたいコントリビューター

---

### 2. Examples

#### この領域が重要な理由

Examples は Batonel を説明する最も明確な方法の 1 つです。

アーキテクチャの意図がどのようになるかを示します。

- 配置ルール
- artifact プラン
- contract
- prompt
- 期待される出力

#### 良いコントリビューションの種類

- 既存の examples を改善する
- 適切な場合に不足している contract フィールドを追加する
- 期待される出力の一貫性を改善する
- example の README ファイルに説明を追加する
- 新しい example 構造を提案する
- 進化する概念定義に合わせて examples を整合させる

#### この領域の最初の issue として良いもの

- example の命名をより一貫させる
- 1 つの example を現在のスキーマ用語に合わせる
- 1 つの example の README を改善する
- 既存の example に不足している prompt または contract ファイルを追加する
- ノイズが多すぎる example を簡素化する

#### 最適な対象者

- 具体的な artifact で作業したいコントリビューター
- 最初のうちはランタイムコードに触れずに作業したいコントリビューター
- アーキテクチャの例に興味がある人

---

### 3. 概念モデル

#### この領域が重要な理由

Batonel は安定した概念モデルに依存しています。

`artifact`、`contract`、`preset` などの概念が意味でずれると、実装が不整合になります。

#### 良いコントリビューションの種類

- 概念の境界を明確にする
- 概念定義を洗練させる
- 概念間の重複を特定する
- 不足している概念を提案する
- 概念がどのように接続するかの説明ドキュメントを改善する

#### この領域の良い議論トピック

- artifact として何を数えるべきか
- contract がいかに厳密であるべきか
- prompt が生成されるべきか部分的に編集可能であるべきか
- preset が examples とどう異なるか
- ロール命名がどのように進化すべきか

#### 最適な対象者

- 設計とアーキテクチャ思考に興味があるコントリビューター
- 実装が固まる前にモデルを形作りたいコントリビューター
- メンテナーと頻繁なコントリビューター

---

### 4. スキーマ設計

#### この領域が重要な理由

スキーマはドキュメントと実装の間の橋渡しです。

モデルをより正確にし、将来の CLI の動作を一貫させるのに役立ちます。

#### 良いコントリビューションの種類

- スキーマの読みやすさを改善する
- フィールド定義を厳格にする
- 任意または必須フィールドの変更を提案する
- examples とスキーマを整合させる
- バリデーション指向の改善を提案する
- スキーマドラフト間の曖昧さを減らす

#### 良いコントリビューショントピック

- フィールド命名の一貫性
- 任意 vs 必須の境界
- JSON Schema との将来的な互換性
- パスオーバーライドの動作
- contract と prompt スキーマの整合

#### 最適な対象者

- 構造化フォーマットが好きなコントリビューター
- 将来のバリデーション動作を形作りたいコントリビューター
- システムモデリングに慣れたコントリビューター

---

### 5. CLI 実装

#### この領域が重要な理由

CLI は Batonel が実用的になる場所です。

この領域は概念とスキーマを実際に動作するツールに変換します。

#### 現在の focus 領域

- 設定の読み込み
- ファイルの解析
- パス解決
- スキャフォルド生成
- prompt 生成
- verify のエントリポイント
- **受け入れ基準（Acceptance Criteria）の検証** (詳細は [こちら](./acceptance-criteria.md))

#### 良いコントリビューションの種類

- 設定解析を実装する
- ロール-パス解決を実装する
- スキャフォルドディレクトリ作成を実装する
- sidecar ファイル生成を実装する
- 基本的な CLI テストを追加する
- 初期コマンドでのエラーハンドリングを改善する

#### 最適な対象者

- Rust コントリビューター
- 実用的なツーリングに興味があるコントリビューター
- 仕様をコードに変換することが得意なコントリビューター

#### 重要な注記

CLI の作業は概念とスキーマモデルに従うべきであり、最初に議論せずに実装中に新しい概念を発明してはいけません。

---

### 6. AI ハンドオフ設計

#### この領域が重要な理由

AI ハンドオフは Batonel が存在する主な理由の 1 つです。

目標は、artifact を人間または軽量モデルによって明確な制約で実装可能にすることです。

#### 良いコントリビューションの種類

- prompt 構造を洗練させる
- 完了基準の設計を改善する
- コンパクトと詳細などの prompt モードを定義する
- examples 全体で prompt の一貫性を改善する
- どの contract フィールドが常に prompt に流れるべきかを特定する

#### 良い議論トピック

- 小さなモデルが使えるようにするには
- どれほどのコンテキストが多すぎるか
- prompt が完全に生成されるべきか部分的にテンプレート化されるべきか
- ロール固有の prompt デフォルトがどのように機能すべきか

#### 最適な対象者

- AI 支援開発に興味があるコントリビューター
- コーディングアシスタントを実際に使用しているコントリビューター
- prompt の品質と制約に興味があるコントリビューター

---

### 7. Verify 設計

#### この領域が重要な理由

verify がなければ、contract とスキャフォルド構造は時間とともにずれることがあります。

Verify は初期生成後のアーキテクチャの意図を保護します。

#### 良いコントリビューションの種類

- verify のスコープを定義する
- 最初の verify ルールを提案する
- **製品保証（Product Guarantees）の拡充** (詳細は [こちら](./acceptance-criteria.md))
- 必要な整合性チェックを特定する
- verify 出力の例を設計する
- ステータスとファイル存在チェックを設計する

#### 最初の verify 対象として良いもの

- 必要なファイルの存在
- 必要な contract フィールド
- ファイル全体のロールの整合性
- 配置ルールとのパスの整合性
- 計画された artifact の contract / prompt の存在

#### 最適な対象者

- ルールシステムが好きなコントリビューター
- 整合性とツーリング品質に興味があるコントリビューター
- 深いコード解析が存在する前に貢献したいコントリビューター

---

### 8. Preset とエコシステムへの適合

#### この領域が重要な理由

Preset は Batonel を実際のプロジェクトへの採用を容易にします。

examples と慣習を再利用可能な出発点に変えるのに役立ちます。

#### 良いコントリビューションの種類

- 既存の example-to-preset の方向性を洗練させる
- 将来の preset のロールセットを提案する
- エコシステム固有のニーズを特定する
- デフォルトのプロジェクトブートストラップ構造を提案する
- アーキテクチャ間の preset 境界を比較する

#### 良い議論トピック

- いつ example が preset になるべきか
- preset がどの程度opinionated であるべきか
- preset をどのように柔軟に保つか
- 初期に何個の preset をサポートすべきか

#### 最適な対象者

- アーキテクチャパターンに興味があるコントリビューター
- エコシステムの経験があるコントリビューター
- 採用パスを改善したいコントリビューター

---

### 9. ガバナンスと信頼 (Governance and Trust)

#### この領域が重要な理由

Batonel が再利用可能な preset エコシステムに広がるにつれ、ガバナンスと信頼がきわめて重要になります。
この領域は、ユーザーがパートナー preset のオフライン署名を検証でき、RBAC ポリシーがオーバーライドを正しく許可することを保証します。

#### 良いコントリビューションの種類

- エコシステム検証のための許可された公開鍵の追加や更新
- コンプライアンス成熟度ベンチマークの洗練
- RBAC スキーマの改善提案
- 監査証拠保存フローの改善

#### 良い議論トピック

- preset 署名検証はどの程度厳密にすべきか
- 内部コーポレート preset のための例外処理
- オフラインファーストの要件と中央レジストリのアイデアのバランス

#### 最適な対象者

- セキュリティとコンプライアンスに興味があるコントリビューター
- DevSecOps の実践者
- 共有エコシステム preset を公開するパートナー

---

### 今特に良い領域

Batonel の現段階では、最もコントリビューションしやすい領域は次のとおりです。

#### 準備度：高
- ドキュメントの改善
- example の洗練
- 用語集と用語の整理
- スキーマの明確化
- 概念の整合

#### 準備度：中
- 初期 CLI の解析とスキャフォルド作業
- prompt 生成構造
- verify ルールのドラフト

#### 直接実装の準備度：低
- 完全な preset エンジン
- エディタ統合
- 深いコード認識チェック
- 高度なプラグインエコシステム
- ベンダー固有の AI 統合

これらの低準備度の領域はまだ有益な議論トピックですが、実装を始めるには最適な場所ではありません。

---

### 最初のコントリビューションパス

実際的な始め方として、これらのパスのいずれかを選んでください。

#### パス 1：ドキュメント優先
- コアドキュメントを読む
- 1 つの概念ページを改善する
- クロスリンクを改善する
- 不一致な命名を修正する

#### パス 2：Example 優先
- 1 つの example を選ぶ
- スキーマと照合する
- 一貫性を改善する
- その README を改善する

#### パス 3：スキーマ優先
- 1 つのスキーマドラフトをレビューする
- 曖昧なフィールドを特定する
- 明確化を提案する
- 改善されたスキーマに examples を整合させる

#### パス 4：実装優先
- 1 つの解析ステップを実装する
- 1 つの解決ステップを実装する
- 1 つの生成ステップを実装する
- docs と examples に合わせた動作を維持する

---

### コントリビューターが今避けるべきこと

プロジェクトの一貫性を保つために、コントリビューターは整合なしにこれらの領域に早急に進むべきではありません。

- 一度に多くの新しい概念を追加する
- 現在のモデルに基づいていないアーキテクチャ固有の動作を作成する
- Batonel を 1 つの言語またはフレームワークに過度に結合する
- prompt を真実の主要な源にする
- 最小フローが安定する前に高度な機能を実装する

これらのアイデアが悪いという意味ではありません。
タイミングが重要です。

---

### 適切な領域の選び方

シンプルな選び方：

- リスクの低いコントリビューションが望みなら、docs または examples から始める
- モデルを形作りたいなら、概念またはスキーマに取り組む
- ツールを構築したいなら、CLI の内部に取り組む
- AI ワークフローに関心があるなら、prompt とハンドオフ設計に取り組む
- 長期的な整合性を大切にするなら、verify 設計に取り組む

これらはすべて有効なコントリビューションの方法です。

---

### まとめ

Batonel には複数のコントリビューション領域がありますが、すべてが同じ成熟度にあるわけではありません。

今コントリビューションするのに最適な領域：

- ドキュメント
- examples
- 概念の明確化
- スキーマの洗練

その次の層：

- 最小 CLI
- prompt 生成
- verify のドラフト作成

1 つだけ覚えておくなら、これを覚えてください。

**最良のコントリビューションとは、現在のモデルを追い越さずに明確さを強化するものです**