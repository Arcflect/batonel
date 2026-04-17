# 0002 Contract is the source of truth for artifact boundaries

- Status: accepted
- Date: 2026-03-28

## Context

Batonel includes multiple representations of architectural intent:

- examples
- project definitions
- artifact plans
- contracts
- prompts
- scaffolded files

Without a clear source of truth, contributors may treat prompts or placeholder files as the primary definition of an artifact.
That would make architectural boundaries less stable.

The project needs one central place where artifact responsibilities and constraints are defined.

## Decision

The **contract** is the source of truth for artifact-level responsibilities and constraints.

This means the contract is the canonical place to define:

- what an artifact is responsible for
- what it must not do
- what dependencies are allowed
- what dependencies are forbidden
- what inputs and outputs are expected
- what implementation size is intended

Prompts are derived from contracts.
Scaffolded files may reference contracts.
But contracts remain primary.

## Consequences

What becomes easier:
- consistent prompt generation
- future verification design
- artifact-level boundary preservation
- contributor understanding of where rules live

What becomes harder:
- treating prompts as informal ad hoc instructions
- keeping architecture only in example files or team memory

This decision also strengthens the sidecar-file model.

## Alternatives considered

### Prompt as source of truth

Not chosen because prompts are delivery-oriented and may vary by usage context.

### Placeholder implementation file as source of truth

Not chosen because code-adjacent files are too dependent on language and repository layout.

### Artifact plan as source of truth

Not chosen because the artifact plan identifies units, but does not fully define behavioral boundaries.

## Notes

This decision should guide future verify behavior and prompt design.

---

## 日本語

# 0002 Contract は artifact の境界にとっての真実の源である

- ステータス：採択済み
- 日付：2026-03-28

## コンテキスト

Batonel にはアーキテクチャの意図の複数の表現が含まれています。

- examples
- プロジェクト定義
- artifact プラン
- contract
- prompt
- スキャフォルドされたファイル

明確な真実の源がなければ、コントリビューターは prompt や仮ファイルを artifact の主要な定義として扱う可能性があります。
これによりアーキテクチャの境界はあまり安定しなくなります。

プロジェクトには、artifact の責務と constraint が定義される 1 つの中心的な場所が必要です。

## 決定

**contract** は artifact レベルの責務と constraint の真実の源です。

これは contract が次のことを定義するための正規の場所であることを意味します。

- artifact が何を担うべきか
- 何をしてはいけないか
- どの依存関係が許可されているか
- どの依存関係が禁止されているか
- どのような入力と出力が期待されるか
- どのような実装サイズが意図されているか

Prompt は contract から導出されます。
スキャフォルドされたファイルは contract を参照する場合があります。
しかし contract が主要です。

## 結果

容易になること：
- 一貫した prompt 生成
- 将来の verify 設計
- artifact レベルの境界保全
- ルールがどこに存在するかについてのコントリビューターの理解

難しくなること：
- prompt をインフォーマルなアドホックな指示として扱うこと
- アーキテクチャを example ファイルやチームの記憶の中だけに保持すること

この決定は sidecar ファイルモデルも強化します。

## 検討された代替案

### Prompt を真実の源として

選択されなかった理由：prompt は配信指向であり、使用コンテキストによって変わる可能性があるため。

### 仮の実装ファイルを真実の源として

選択されなかった理由：コード隣接のファイルは言語とリポジトリのレイアウトに依存しすぎているため。

### Artifact プランを真実の源として

選択されなかった理由：artifact プランはユニットを識別しますが、振る舞いの境界を完全に定義しないため。

## 注記

この決定は将来の verify の動作と prompt 設計を導くべきです。