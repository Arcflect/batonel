# 0001 Batonel is an architecture-to-execution bridge

- Status: accepted
- Date: 2026-03-28

## Context

Batonel could have been positioned in several different ways.

Possible interpretations included:

- a scaffold generator
- a prompt generation tool
- a repository template system
- an architecture linting tool
- a spec-driven planning tool

Each of these captures part of the project, but none captures the whole intended flow.

The core problem Batonel addresses is the gap between architecture and implementation.
That gap becomes even more important in AI-assisted development, where implementation may be handed to lightweight models that need explicit structure and constraints.

## Decision

Batonel is defined as an **architecture-to-execution bridge**.

This means Batonel is intended to connect:

- architectural intent
- structural planning
- artifact definition
- responsibility boundaries
- AI handoff
- future verification

Batonel is not defined primarily as a template repository, prompt toolkit, or linter,
even though it may include aspects of those.

## Consequences

This decision makes several things clearer.

What becomes easier:
- explaining the product direction
- deciding what belongs in scope
- evaluating whether a feature supports the core flow
- keeping concepts and implementation aligned

What becomes harder:
- narrowly optimizing for only one sub-problem
- positioning Batonel as a generic utility tool
- expanding into unrelated tooling too early

This decision supports the current documentation direction and the roadmap layering.

## Alternatives considered

### Batonel as a scaffold generator

Not chosen because it is too narrow.
Scaffolding is important, but Batonel also defines contracts and prompts.

### Batonel as a prompt generation tool

Not chosen because prompts are derived outputs, not the primary purpose.

### Batonel as an architecture linter

Not chosen because linting is a later operational layer, not the starting point.

### Batonel as a spec-driven planning system

Not chosen because Batonel focuses more specifically on artifact-level implementation handoff.

## Notes

This decision is foundational and should be treated as a framing decision for future scope discussions.

---

## 日本語

# 0001 Batonel は設計から実行への橋渡しである

- ステータス：採択済み
- 日付：2026-03-28

## コンテキスト

Batonel はいくつかの異なる方法で位置づけることができました。

考えられた解釈には次のものが含まれていました。

- スキャフォルドジェネレーター
- prompt 生成ツール
- リポジトリテンプレートシステム
- アーキテクチャ linting ツール
- 仕様駆動の計画ツール

これらはそれぞれプロジェクトの一部を捉えていますが、意図されたフロー全体を捉えるものはありませんでした。

Batonel が取り組むコアな問題は、アーキテクチャと実装の間のギャップです。
このギャップは AI 支援開発においてより重要になります。なぜなら、実装が明示的な構造と制約を必要とする軽量モデルに委ねられる場合があるからです。

## 決定

Batonel は**設計から実行への橋渡し**として定義されます。

これは Batonel が次を結ぶことを意図していることを意味します。

- アーキテクチャの意図
- 構造的な計画
- artifact の定義
- 責務の境界
- AI へのハンドオフ
- 将来の verify

Batonel は主にテンプレートリポジトリ、prompt ツールキット、または linter として定義されません。
たとえそれらの側面を含む可能性があるとしても。

## 結果

この決定はいくつかのことを明確にします。

容易になること：
- 製品の方向性を説明すること
- スコープに何が属するかを決定すること
- 機能がコアフローをサポートするかどうかを評価すること
- 概念と実装を整合し続けること

難しくなること：
- 1 つのサブ問題のみに絞って最適化すること
- Batonel を汎用ユーティリティツールとして位置づけること
- 早期に無関係なツーリングに拡張すること

この決定は現在のドキュメントの方向性とロードマップのレイヤリングをサポートします。

## 検討された代替案

### スキャフォルドジェネレーターとしての Batonel

選択されなかった理由：範囲が狭すぎるため。
スキャフォルディングは重要ですが、Batonel は contract と prompt も定義します。

### Prompt 生成ツールとしての Batonel

選択されなかった理由：prompt は導出された出力であり、主要な目的ではないため。

### アーキテクチャ linter としての Batonel

選択されなかった理由：linting は後段の運用レイヤーであり、出発点ではないため。

### 仕様駆動の計画システムとしての Batonel

選択されなかった理由：Batonel はより具体的に artifact レベルの実装ハンドオフに焦点を当てているため。

## 注記

この決定は基盤的なものであり、将来のスコープ議論の枠組みの決定として扱われるべきです。