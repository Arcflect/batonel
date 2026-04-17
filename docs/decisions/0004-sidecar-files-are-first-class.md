# 0004 Sidecar files are first-class

- Status: accepted
- Date: 2026-03-28

## Context

Batonel needs a way to preserve architectural intent even before full implementation exists.

Possible storage strategies included:

- comments inside implementation files
- repository-wide instruction files only
- separate contract and prompt files
- code-aware extraction only

A code-only strategy would make early design and language-agnostic workflows harder.
A comment-only strategy would depend too much on implementation language and file state.

## Decision

Sidecar files are treated as **first-class artifacts** in Batonel.

Important examples include:

- `*.contract.yaml`
- `*.prompt.md`

These files are not secondary documentation.
They are operational parts of the Batonel model.

This supports workflows where:

- code does not exist yet
- placeholder files exist but implementation is incomplete
- architecture needs to remain explicit outside source code

## Consequences

What becomes easier:
- language-agnostic modeling
- pre-implementation design workflows
- prompt generation
- contract-centered verification
- artifact-level architectural memory

What becomes harder:
- relying only on inline code comments
- assuming source files are the only meaningful artifacts

This decision also supports future multi-language use.

## Alternatives considered

### Comments inside implementation files as the main storage layer

Not chosen because this is too dependent on language and implementation timing.

### Repository-wide instruction files only

Not chosen because they are too broad for artifact-level boundaries.

### Code-aware extraction as the main approach

Not chosen because it would make early-phase usage and non-code-first workflows weaker.

## Notes

This decision does not forbid inline comments.
It only says that sidecar files are first-class and must be treated seriously.

---

## 日本語

# 0004 Sidecar ファイルはファーストクラスである

- ステータス：採択済み
- 日付：2026-03-28

## コンテキスト

Batonel は、完全な実装が存在する前でもアーキテクチャの意図を保全する方法が必要です。

考えられたストレージ戦略には次のものが含まれていました。

- 実装ファイル内のコメント
- リポジトリ全体の指示ファイルのみ
- 別の contract と prompt ファイル
- コード認識による抽出のみ

コードのみの戦略では、初期設計と言語非依存のワークフローが難しくなります。
コメントのみの戦略では、実装言語とファイルの状態に依存しすぎます。

## 決定

Sidecar ファイルは Batonel において**ファーストクラスの artifact**として扱われます。

重要な例には次のものが含まれます。

- `*.contract.yaml`
- `*.prompt.md`

これらのファイルは二次的なドキュメントではありません。
Batonel モデルの運用上の部分です。

これは次のワークフローをサポートします。

- コードがまだ存在しない場合
- 仮ファイルが存在するが実装が不完全な場合
- アーキテクチャがソースコードの外部で明示的に保持される必要がある場合

## 結果

容易になること：
- 言語非依存のモデリング
- 実装前の設計ワークフロー
- prompt 生成
- contract 中心の verify
- artifact レベルのアーキテクチャメモリ

難しくなること：
- インラインコードコメントのみに依存すること
- ソースファイルのみが意味のある artifact であると仮定すること

この決定は将来のマルチ言語使用もサポートします。

## 検討された代替案

### 実装ファイル内のコメントをメインストレージレイヤーとして

選択されなかった理由：言語と実装のタイミングに依存しすぎるため。

### リポジトリ全体の指示ファイルのみ

選択されなかった理由：artifact レベルの境界には広すぎるため。

### コード認識の抽出をメインアプローチとして

選択されなかった理由：初期フェーズの使用と非コードファースト型のワークフローが弱くなるため。

## 注記

この決定はインラインコメントを禁止しません。
Sidecar ファイルがファーストクラスであり、真剣に扱わなければならないと述べているだけです。