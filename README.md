# Archflow

Turn architecture into executable scaffolding for AI-assisted development.

Archflow is an open source tool that converts design decisions into artifact-level implementation contracts.
Instead of only documenting architecture, Archflow helps define:

- where code should live
- what each artifact is responsible for
- what it must not do
- what context should be handed to an AI coding tool

Archflow is designed for teams that define the overall architecture first, then implement file by file with humans or lightweight AI models.

---

## English

### Why Archflow

AI coding tools are good at local implementation, but they often fail when structure is unclear.
Even when a team agrees on the architecture, development still gets stuck on questions like:

- Where should this file go?
- What is this file allowed to do?
- What must it never depend on?
- How can we hand this file to a smaller model without losing architectural intent?

Archflow fills that gap.

### What Archflow does

Archflow turns architecture into executable project scaffolding.

It can generate:

- directory and file structure
- artifact contracts
- responsibility definitions
- implementation constraints
- AI handoff prompts
- verification targets

This makes architecture usable during implementation, not just during planning.

### Core idea

Architecture should not stop at diagrams, folder trees, or markdown docs.
It should become a set of artifact contracts that both humans and AI tools can execute against.

Archflow is centered on artifact-level contracts such as:

- placement
- role
- responsibilities
- forbidden behavior
- allowed dependencies
- implementation size
- status

### Positioning

Archflow is not just a spec tool.
It is not just an agent instruction format.
It is not just an architecture linter.

It is an **architecture-to-execution bridge**.

---

## 日本語

### Archflow とは

Archflow は、設計で決めた内容を、AI 開発時代に使える**実装用の骨組み**へ変換するための OSS です。

アーキテクチャを文章や図で残すだけではなく、次のような情報を **artifact 単位の契約** として扱います。

- どこに配置するか
- 何を責務とするか
- 何をしてはいけないか
- どの依存を許可するか
- AI に何を渡して実装させるか

### なぜ必要か

生成 AI は局所的な実装は得意ですが、構造が曖昧だと誤った配置や責務逸脱を起こしやすくなります。

たとえば、次のような迷いが日常的に発生します。

- このファイルはどこに置くべきか
- このファイルは何をしてよいのか
- 何をしてはいけないのか
- 軽量モデルにどう渡せば設計意図を保てるのか

Archflow は、この曖昧さを減らすことを目的としています。

### 目指していること

Archflow は、設計を以下へ変換することを目指します。

- ディレクトリ構造
- 空ファイルや雛形
- `*.contract.yaml` のような責務契約
- `*.prompt.md` のような AI 実装指示
- verify 対象となる構造ルール

つまり、**設計を実装可能な単位まで下ろす**ための橋渡しです。

### ポジション

Archflow は、単なる仕様管理ツールでも、単なる AI 向け instruction ファイルでも、単なる lint ツールでもありません。

**設計から実装への橋渡しを行う OSS** です。

---

## Current status / 現在のステータス

Archflow is currently in early design and repository bootstrap stage.

現在の Archflow は、初期設計とリポジトリ整備の段階です。
最初の公開ゴールは次のとおりです。

- design file の読み込み
- placement rules の定義
- scaffold の生成
- artifact contract の生成
- AI handoff prompt の生成
- verify の最小実装

---

## Planned commands / 想定コマンド

```bash
archflow init
archflow plan
archflow scaffold
archflow prompt
archflow verify
```

---

## Community / コミュニティ

Please use GitHub Issues for bugs, feature requests, and architecture rule proposals.
For open-ended exploration, use GitHub Discussions when available.

バグ報告、機能提案、アーキテクチャルール提案は GitHub Issues を利用してください。
広めの議論は GitHub Discussions を想定しています。

---

## License / ライセンス

Apache License 2.0.

A short Japanese summary is available in `docs/LICENSE.ja.md`.

Apache License 2.0 を採用します。
日本語の参考サマリーは `docs/LICENSE.ja.md` にあります。
