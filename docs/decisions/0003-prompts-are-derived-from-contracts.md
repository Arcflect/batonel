# 0003 Prompts are derived from contracts

- Status: accepted
- Date: 2026-03-28

## Context

Batonel includes prompts because AI-assisted implementation is a major use case.

However, prompts can easily drift.
They may become:

- manually edited
- model-specific
- overly verbose
- inconsistent across roles
- disconnected from architectural intent

The project needs a clear rule for how prompts relate to contracts.

## Decision

Prompts are treated as **derived outputs** from contracts.

This means:

- prompts should reflect contract data
- prompts should not introduce new architectural rules
- prompts should not replace contract files
- prompt generation should remain traceable to artifact boundaries

A prompt is a delivery format for implementation handoff.
It is not the authoritative architectural record.

## Consequences

What becomes easier:
- stable AI handoff behavior
- future prompt regeneration
- consistency across examples
- contract-centered workflow design

What becomes harder:
- freeform manual prompt drift
- prompt-specific rules that bypass the contract model

This also supports multiple future prompt formats,
such as compact, detailed, or role-specific outputs.

## Alternatives considered

### Prompt as a co-equal source of truth

Not chosen because it creates ambiguity about where architectural rules truly live.

### Prompt as manually authored primary artifact

Not chosen because it weakens repeatability and consistency.

### No prompt generation at all

Not chosen because artifact-level AI handoff is one of the key reasons Batonel exists.

## Notes

Future model-specific formatting may still exist,
but it should remain downstream from the contract.

---

## 日本語

# 0003 Prompt は contract から導出される

- ステータス：採択済み
- 日付：2026-03-28

## コンテキスト

Batonel は AI 支援実装が主要なユースケースであるため、prompt を含んでいます。

しかし、prompt は容易にずれる可能性があります。
次のようになるかもしれません。

- 手動で編集される
- モデル固有になる
- 過度に冗長になる
- ロール間で不整合になる
- アーキテクチャの意図から切り離される

プロジェクトには、prompt が contract とどのように関係するかについての明確なルールが必要です。

## 決定

Prompt は contract からの**導出された出力**として扱われます。

これが意味することは：

- prompt は contract データを反映すべきである
- prompt は新しいアーキテクチャルールを導入すべきでない
- prompt は contract ファイルを置き換えるべきでない
- prompt 生成は artifact の境界まで追跡可能であるべきである

Prompt は実装ハンドオフの配信形式です。
権威あるアーキテクチャの記録ではありません。

## 結果

容易になること：
- 安定した AI ハンドオフの動作
- 将来の prompt の再生成
- examples 全体での一貫性
- contract 中心のワークフロー設計

難しくなること：
- 自由形式の手動 prompt のずれ
- contract モデルをバイパスする prompt 固有のルール

これはコンパクト、詳細、またはロール固有の出力などの複数の将来の prompt 形式もサポートします。

## 検討された代替案

### Prompt を同等の真実の源として

選択されなかった理由：アーキテクチャルールが実際にどこにあるかについて曖昧さが生じるため。

### Prompt を手動作成の主要な artifact として

選択されなかった理由：繰り返し可能性と一貫性が弱くなるため。

### Prompt 生成をまったく行わない

選択されなかった理由：artifact レベルの AI ハンドオフは Batonel が存在する主な理由の 1 つであるため。

## 注記

将来のモデル固有のフォーマットは存在するかもしれませんが、
contract の下流にとどまるべきです。