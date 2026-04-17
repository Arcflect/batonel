# Artifact

## Overview

In Batonel, an **Artifact** is the smallest planned implementation unit.

An artifact represents a concrete thing that should exist in the project structure,
such as a file, module boundary, handler, use case, entity, repository port, or gateway.

Artifacts are the primary execution units that Batonel scaffolds, documents, and prepares
for human or AI implementation.

---

## Purpose

The purpose of an artifact is to bridge architecture and implementation.

An artifact answers questions such as:

- What should be created?
- Where should it live?
- What role does it play?
- What module does it belong to?
- What should be handed to an implementer?

Artifacts make architecture operational.

---

## Responsibilities

An artifact is responsible for defining:

- its identity
- its role
- its module membership
- its planned location
- its expected inputs and outputs
- its implementation status

An artifact is not responsible for describing all behavioral rules in detail.
That belongs to the contract.

---

## Core fields

Typical fields include:

- `name`
- `module`
- `role`
- `path` (resolved or generated)
- `inputs` (optional)
- `outputs` (optional)
- `status` (optional)

---

## Example

```yaml
artifacts:
  - name: create_user
    module: user
    role: usecase
    inputs:
      - CreateUserCommand
    outputs:
      - CreateUserResult
```

---

## Relationship to other concepts

An artifact is shaped by:

- the project context
- placement rules
- role templates
- contracts
- prompts

An artifact is the unit that connects those concepts together.

Typical flow:

1. the project defines the architectural context
2. placement rules define where a role should live
3. the artifact identifies one concrete implementation unit
4. the contract defines its responsibilities and constraints
5. the prompt turns that contract into implementation handoff context

---

## Design principles

An artifact should be:

- concrete
- small enough to implement in isolation
- meaningful in the architecture
- stable enough to track through planning and execution
- easy to hand off to a human or lightweight model

Artifacts should avoid being too large or too vague.

---

## What an artifact should not do

An artifact should not:

- encode full business policy by itself
- replace the project definition
- replace the contract
- mix multiple unrelated responsibilities
- become so broad that it stops being a useful implementation unit

---

## Why it matters

Batonel is centered on artifact-level execution.

This is important because AI coding tools often perform best when the task is:

- narrow
- explicit
- bounded
- context-rich

Artifacts give Batonel a practical unit for:

- placement
- scaffolding
- contract generation
- prompt generation
- verification

---

## Examples of artifacts

Examples of artifacts include:

- user
- create_user
- user_repository
- create_user_handler
- postgres_user_repository

These are not only file names.
They are architectural execution units.

---

## Future directions

In the future, artifacts may also support:

- grouping or dependency references
- ownership metadata
- lifecycle state transitions
- artifact splitting recommendations
- mapping to generated code or existing repository files

---

## 日本語

# Artifact

## 概要

Batonel における **Artifact** は、実装のための最小の計画済み単位です。

artifact は、プロジェクト構造に存在すべき具体的なもの、つまりファイル、ハンドラー、エンティティ、サービス、リポジトリ境界、アダプターユニットなどを表します。

Artifact は Batonel がスキャフォルドし、制約を与え、人間または AI システムにハンドオフする主要なユニットです。

---

## 目的

artifact の目的は、実装可能な最小単位をアーキテクチャの意図で定義することです。

artifact は次のような質問に答えます。

- 何を実装すべきか？
- どこに配置すべきか？
- どのような責務を持つべきか？
- どのような依存関係が許可または禁止されているか？
- どのようにして AI ツールにハンドオフされるか？

Artifact がなければ、Batonel はプロジェクト全体の粗い意図のみを持ち、具体的な実行ユニットを持ちません。

---

## 責務

Artifact は次のことを定義する責務を持ちます。

- 名前とアイデンティティ
- ロールと分類
- モジュールへの帰属
- スキャフォルド後の期待されるパス
- それに関連した contract
- それに関連した prompt

Artifact は次の責務を持ちません。

- モジュール全体の目的を定義すること（それはモジュールに属します）
- 完全なビジネスフローを定義すること（それは複数の artifact にまたがります）
- それ自体の配置を解決すること（それは配置ルールに属します）

---

## コアフィールド

Artifact は通常、次のフィールドを持ちます。

- `name`: artifact の識別子（例: `create_user`、`user`、`user_repository`）
- `module`: artifact が属するモジュール（例: `user`、`auth`）
- `role`: artifact のアーキテクチャ上のロール（例: `usecase`、`entity`、`controller`）
- `inputs`: artifact が受け取ると期待されるデータ型（任意）
- `outputs`: artifact が生成すると期待されるデータ型（任意）
- `status`: artifact のライフサイクル状態（任意）

これらのフィールドを使用して、Batonel は次のことができます。

- artifact のファイルパスを解決する
- artifact の contract を生成する
- artifact の prompt を生成する
- 将来の verify でスキャフォルドの一貫性をチェックする

---

## 例

```yaml
artifacts:
  - name: create_user
    module: user
    role: usecase
    inputs:
      - CreateUserCommand
    outputs:
      - CreateUserResult

  - name: user
    module: user
    role: entity
    outputs:
      - User
```

この例では：

- `create_user` は `user` モジュール内の usecase artifact です
- `user` は `user` モジュール内の entity artifact です

それぞれが独自の contract、prompt、解決されたパスを持ちます。

---

## 他の概念との関係

Artifact は Batonel モデルの中心です。

それは次のものによってサポートされます。

- **project**: artifact が解釈されるグローバルコンテキストを定義する
- **module**: artifact がグループ化される機能領域を定義する
- **role**: artifact のアーキテクチャ上の分類を定義する
- **placement rule**: artifact のファイルパスを解決する
- **contract**: artifact の責務と制約を定義する
- **prompt**: artifact の実装指示を AI 向けの形式で定義する
- **scaffold**: artifact の構造を具体的なリポジトリ出力に変換する

Artifact はこれらすべての概念を 1 か所に結びつけます。

---

## Artifact が Batonel で中心的な理由

Batonel における実装の有用な最小単位は、リポジトリ全体や大きなサービスではありません。
Artifact です。

Artifact が中心的な理由は次のとおりです。

- 計画と実装の最小の扱いやすい単位を表す
- 再利用可能な contract テンプレートとロールに接続できる
- AI モデルにハンドオフ可能な独立したスコープを提供する
- 将来的に verify の追跡可能な単位を提供する

これは特に AI 支援ワークフローで重要です。なぜなら、有用なコンテキストは artifact のスコープの精度に依存するからです。

---

## 将来の方向性

将来的に、artifact は次のものをサポートするかもしれません。

- より豊富なステータスの追跡
- artifact 間の依存関係の宣言
- プリセット固有のデフォルト
- より詳細なライフサイクル管理
- verify に対してチェックされる artifact の整合性

それらが成長しても、基本的な目的は変わりません。

artifact は Batonel における計画、スキャフォルディング、実装ハンドオフの中心的な単位です。