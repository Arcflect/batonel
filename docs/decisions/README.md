# Decisions

This directory contains architectural and product decisions for Archflow.

The purpose of these records is to preserve important reasoning over time.

Archflow includes many design choices that may otherwise become unclear later, such as:

- what the core concepts mean
- what is considered source of truth
- how prompts relate to contracts
- how examples relate to presets
- how much Archflow should depend on code-aware analysis
- how verification should begin

These decision records help contributors understand not only **what** the project does,
but also **why** it was designed this way.

---

## Why this directory exists

Archflow is concept-heavy.

That means important project decisions are not only implementation details.
They also include:

- model boundaries
- terminology choices
- workflow priorities
- sequencing decisions
- tradeoffs between flexibility and strictness

Without decision records, these choices can drift into:

- scattered discussions
- implicit maintainer memory
- inconsistent documentation
- implementation guesses

This directory exists to reduce that drift.

---

## How to read these files

Each decision file should explain:

- the context
- the decision
- the consequences

Start with the earliest decision if you want to understand the project direction from the beginning.

Decision files are numbered in roughly chronological order.

---

## File naming convention

Use this pattern:

`NNNN-short-kebab-case-title.md`

Examples:

- `0001-archflow-is-an-architecture-to-execution-bridge.md`
- `0002-contract-is-the-source-of-truth-for-artifact-boundaries.md`

The number keeps ordering stable.
The title keeps the decision readable in repository views.

---

## Suggested status values

A decision file may include one of these status values:

- `proposed`
- `accepted`
- `superseded`
- `deprecated`

If a decision is replaced later, do not delete the old file.
Instead:

- keep the old file
- mark it as superseded
- add a note pointing to the newer decision

This preserves project history.

---

## When to create a new decision

Create a decision record when a change affects:

- project direction
- conceptual model
- source of truth
- workflow sequencing
- schema philosophy
- preset philosophy
- prompt behavior
- verification scope
- major implementation tradeoffs

Do not create a decision file for every minor edit.

A good rule is:

Create a decision record when future contributors are likely to ask,
“Why was it done this way?”

---

## Relationship to other docs

Decision files are different from:

- concept docs  
  These explain what a concept means.

- schema docs  
  These explain structure and fields.

- roadmap docs  
  These explain what should happen next.

Decision files explain why important choices were made.

---

## Current initial decisions

The initial set of decisions focuses on core Archflow direction:

- `0001` Archflow is an architecture-to-execution bridge
- `0002` Contract is the source of truth for artifact boundaries
- `0003` Prompts are derived from contracts
- `0004` Sidecar files are first-class
- `0005` Examples precede presets
- `0006` Verify starts with structure and contract consistency
- `0007` Archflow remains sidecar-first before code-aware analysis
- `0008` Minimal CLI error handling policy for Phase 2
- `0009` Minimal CLI error handling policy for Phase 3
- `0010` Guard contract-first behavior in prompt generation
- `0011` Minimal verification model
- `0012` Minimal error handling policy for Phase 4 verification
- `0013` First-version verification rule boundaries
- `0014` Guard contract-first and sidecar-first behavior in Phase 4 verification
- `0015` Define minimal preset model for Phase 5
- `0016` Define preset packaging approach
- `0017` Formalize example-to-preset mapping
- `0018` Design minimal project bootstrap flow from presets
- `0019` Define preset customization boundaries
- `0020` Align presets with examples, docs, and schemas
- `0021` Minimal error handling policy for preset-based onboarding
- `0022` Guard examples-first behavior during preset implementation

These decisions establish the early shape of the project.

---

## Summary

This directory helps Archflow preserve intent over time.

If you remember only one thing, remember this:

**documentation explains the model, but decision records explain why the model exists in its current form**

---

## 日本語

このディレクトリには、Archflow のアーキテクチャおよびプロダクトに関する決定事項が含まれています。

これらの記録の目的は、重要な reasoning を時間をかけて保全することです。

Archflow には、後になって不明確になる可能性のある多くの設計上の選択が含まれています。例えば：

- コアコンセプトが何を意味するか
- 何が真実の源とみなされるか
- prompt が contract とどのように関係するか
- examples が preset とどのように関係するか
- Archflow がコード認識分析にどの程度依存すべきか
- verify がどのように始まるべきか

これらの決定記録は、コントリビューターがプロジェクトが**何**をするかだけでなく、**なぜ**このように設計されたかを理解するのに役立ちます。

---

### このディレクトリが存在する理由

Archflow は概念が多いプロジェクトです。

つまり、重要なプロジェクトの決定は実装の詳細だけではありません。
次のものも含まれます。

- モデルの境界
- 用語の選択
- ワークフローの優先順位
- シーケンスの決定
- 柔軟性と厳格さのトレードオフ

決定記録がなければ、これらの選択は次のものに流れ込む可能性があります。

- 散在した議論
- 暗黙のメンテナーの記憶
- 不整合なドキュメント
- 実装上の推測

このディレクトリはその流れを減らすために存在します。

---

### これらのファイルの読み方

各決定ファイルは次のことを説明すべきです。

- コンテキスト
- 決定
- 結果

プロジェクトの方向性を最初から理解したい場合は、最古の決定から始めてください。

決定ファイルはおおよそ時系列順に番号が付けられています。

---

### ファイル命名規則

このパターンを使用してください。

`NNNN-short-kebab-case-title.md`

例：

- `0001-archflow-is-an-architecture-to-execution-bridge.md`
- `0002-contract-is-the-source-of-truth-for-artifact-boundaries.md`

数字により順序が安定します。
タイトルにより、リポジトリビューで決定が読みやすくなります。

---

### 推奨されるステータス値

決定ファイルには、次のステータス値のいずれかが含まれる場合があります。

- `proposed`（提案中）
- `accepted`（採択済み）
- `superseded`（上書きされた）
- `deprecated`（非推奨）

決定が後で置き換えられた場合、古いファイルを削除しないでください。
代わりに：

- 古いファイルを保持する
- superseded としてマークする
- 新しい決定を指すメモを追加する

これによりプロジェクトの歴史が保全されます。

---

### 新しい決定を作成するタイミング

変更が次に影響する場合に決定記録を作成してください。

- プロジェクトの方向性
- 概念モデル
- 真実の源
- ワークフローのシーケンス
- スキーマの哲学
- preset の哲学
- prompt の動作
- verify のスコープ
- 主要な実装上のトレードオフ

すべての小さな編集に対して決定ファイルを作成しないでください。

良いルールは次のとおりです。

将来のコントリビューターが「なぜこのようにされたのか？」と聞きそうな場合に決定記録を作成してください。

---

### 他のドキュメントとの関係

決定ファイルは次とは異なります。

- 概念ドキュメント  
  これらは概念が何を意味するかを説明します。

- スキーマドキュメント  
  これらは構造とフィールドを説明します。

- ロードマップドキュメント  
  これらは次に何が起こるべきかを説明します。

決定ファイルは、重要な選択がなぜ行われたかを説明します。

---

### 現在の初期決定事項

初期の決定事項は Archflow のコアな方向性に焦点を当てています。

- `0001` Archflow は設計から実行への橋渡しである
- `0002` Contract は artifact の境界にとっての真実の源である
- `0003` Prompt は contract から導出される
- `0004` Sidecar ファイルはファーストクラスである
- `0005` Examples は preset より先に来る
- `0006` Verify は構造と contract の整合から始まる
- `0007` Archflow はコード認識分析の前に sidecar ファースト のままである
- `0008` Phase 2 における最小 CLI エラーハンドリングポリシー
- `0009` Phase 3 における最小 CLI エラーハンドリングポリシー
- `0010` prompt 生成における contract-first 挙動の保護
- `0011` 最小限の verification モデル
- `0012` Phase 4 verification における最小エラーハンドリングポリシー
- `0013` 初版 verification ルール境界
- `0014` Phase 4 verification における contract-first および sidecar-first 挙動の保護
- `0015` Phase 5 に向けた最小 preset モデルの定義
- `0016` preset のパッケージング方針を定義する
- `0017` example から preset へのマッピングを正式化する
- `0018` preset からの最小プロジェクトブートストラップフローを設計する
- `0019` preset カスタマイズ境界を定義する
- `0020` preset を examples・docs・schema と整合させる
- `0021` preset ベースオンボーディング向け最小エラーハンドリングポリシー
- `0022` preset 実装時に examples-first 振る舞いを守る

これらの決定はプロジェクトの初期形状を確立します。

---

### まとめ

このディレクトリは Archflow が意図を時間をかけて保全するのを助けます。

1 つだけ覚えておくなら、これを覚えてください。

**ドキュメントはモデルを説明するが、決定記録はモデルがなぜ現在の形で存在するかを説明する**