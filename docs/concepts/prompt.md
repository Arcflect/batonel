# Prompt

## Overview

In Archflow, a **Prompt** is the AI handoff representation of an artifact contract.

A prompt translates project context, artifact identity, and contract constraints
into a form that can be directly given to an AI coding tool or lightweight model.

The prompt is not the source of truth.
The contract is the source of truth.
The prompt is a delivery format derived from that contract.

---

## Purpose

The purpose of a prompt is to make an artifact directly implementable by an AI system.

A prompt answers questions such as:

- What should the model implement?
- What is the role of this artifact?
- What must it do?
- What must it avoid?
- What dependencies are allowed?
- What should count as complete?

Prompts reduce the need for a human to restate the same architectural context every time.

---

## Responsibilities

A prompt is responsible for:

- packaging artifact context for implementation
- restating responsibilities clearly
- restating constraints clearly
- presenting expected inputs and outputs
- defining completion expectations
- making artifact contracts usable by AI systems

A prompt is not responsible for inventing architecture.
It should reflect the contract, not replace it.

---

## Core fields or sections

Typical prompt sections include:

- artifact name
- role
- module
- responsibilities
- must not
- allowed dependencies
- forbidden dependencies
- inputs
- outputs
- completion criteria

---

## Canonical Section Order

To ensure consistency across multiple artifacts and different output modes, Archflow follows a strict canonical section order:

1.  **Identity**: Artifact Name, Role, and Module.
2.  **Scope**: Responsibilities and "Must Not" constraints.
3.  **Connectivity**: Allowed and Forbidden dependencies.
4.  **Interface**: Expected Inputs and Outputs.
5.  **Fulfillment**: Completion Criteria (Definition of Done).

This stable order makes it easier for both humans and AI models to scan and compare prompts.

Prompts may be represented as Markdown, plain text, or structured output.

---

## Example

```md
# Artifact Prompt: create_user

Implement the `create_user` artifact.

## Role
usecase

## Module
user

## Responsibilities
- Execute one application use case
- Coordinate domain behavior
- Persist through an abstract repository boundary

## Must not
- Write SQL directly
- Return transport-specific responses

## Allowed dependencies
- domain
- application

## Forbidden dependencies
- interfaces
- infrastructure

## Inputs
- CreateUserCommand

## Outputs
- CreateUserResult

## Completion criteria
- The artifact has one clear responsibility
- The implementation respects architectural boundaries
- No infrastructure-specific logic appears in this artifact

---

## Relationship to other concepts

A prompt is derived from:

- the project context
- the artifact identity
- the artifact contract

The relationship is:

- project defines the architectural frame
- artifact defines the execution unit
- contract defines the boundary
- prompt delivers that boundary to an AI implementer

Prompts should be treated as generated interfaces, not primary architectural records.

---

## Design principles

A prompt should be:

- concise
- explicit
- implementation-oriented
- faithful to the contract
- easy for an AI model to follow
- easy for a human to inspect before use

A good prompt should reduce ambiguity without adding noise.

---

## What a prompt should do well

A strong prompt should:

- clearly identify the target artifact
- clearly identify allowed scope
- clearly identify forbidden behavior
- clearly identify expected result
- avoid unnecessary prose
- remain usable by smaller or cheaper models

This is especially important for artifact-level implementation workflows.

---

## What a prompt should not do

A prompt should not:

- introduce new rules not present in the contract
- drift away from the artifact definition
- contain large unrelated architectural explanations
- try to solve multiple artifacts at once
- become so long that lightweight models lose focus

---

## Why it matters

In many AI-assisted workflows, the bottleneck is not code generation ability.
It is context precision.

Repository-wide instructions help, but they are often too broad.

Prompts allow Archflow to hand over implementation in smaller, clearer units.

This makes them useful for:

- lightweight model workflows
- editor-based AI assistance
- human review before implementation
- reproducible artifact generation

---

## Prompt quality guidelines

A strong prompt usually has:

- one artifact target
- one clear role
- explicit responsibilities
- explicit constraints
- explicit completion criteria

A weak prompt usually has:

- vague intent
- hidden assumptions
- mixed responsibilities
- too much repository-wide context
- too much freeform prose

---

---

## Output Modes

Different AI models have different token constraints and verbosity preferences.

### Standard Mode (`--mode standard`)
Best for high-reasoning models (GPT-4, Gemini Pro, Claude 3). Includes full headers and detailed sections. This is the default mode.

### Compact Mode (`--mode compact`)
Best for lightweight/fast models or in-editor inline completion. Strips metadata headers and uses comma-separated formatting for dependencies to save tokens.

---

## Role-Aware Defaults

Phase 3 introduces **Role-Aware Prompts**. Even if you leave your contract relatively simple, Archflow automatically injects "Completion Criteria" based on the architectural role.

| Role | Default AI Guidance |
| :--- | :--- |
| **`entity`** | Strictly protects domain invariants. No infrastructure leak. |
| **`usecase`** | Coordinates domain but implements zero DB or HTTP logic. |
| **`handler`** | Only translates transport models; embeds zero business rules. |
| **`repository`** | Translates between persistence data and pure domain models. |

---

## Future directions

In the future, prompts may also support:

- prompt validation against contracts
- editor and agent integration
- dynamic prompt tailoring based on model feedback
- automated handoff for background agents

---

## 日本語

# Prompt

## 概要

Archflow における **Prompt** は、artifact contract の AI ハンドオフ表現です。

prompt は、プロジェクトコンテキスト、artifact のアイデンティティ、contract の境界を、AI コーディングツールに直接渡せる形式に変換します。

Prompt は contract から導出されます。
Prompt は主要な真実の源ではありません。

---

## 目的

prompt の目的は、artifact の責務と制約を実装のハンドオフ形式に変換することです。

prompt は次のような質問に答えます。

- この artifact の対象は何か？
- そのロールとモジュールは何か？
- その責務は何か？
- 何をしてはいけないか？
- どのような入力と出力を期待するか？
- 完了基準は何か？

---

## 標準的なセクション順序

複数のアーティファクト間や異なる出力モード間での一貫性を保つため、Archflow は厳格なセクション順序に従います。

1.  **Identity（アイデンティティ）**: Artifact 名、Role、Module。
2.  **Scope（スコープ）**: 責務（Responsibilities）と禁止事項（Must Not）。
3.  **Connectivity（接続性）**: 許可された依存関係と禁止された依存関係。
4.  **Interface（インターフェース）**: 期待される入力（Inputs）と出力（Outputs）。
5.  **Fulfillment（履行）**: 完了基準（Completion Criteria / Definition of Done）。

この安定した順序により、人間も AI モデルもプロンプトをスキャンし、比較することが容易になります。

Prompt がなければ、Archflow から AI ツールへの接続は手動で行われる必要があります。

---

## 責務

Prompt は次の責務を持ちます。

- contract データを AI 向けの形式に変換する
- 対象の artifact を明確に特定する
- アーキテクチャの制約を実装可能な指示に変換する
- 実装のスコープを境界付ける

Prompt は次の責務を持ちません。

- 新しいアーキテクチャルールを定義すること（それは contract に属します）
- contract を置き換えること（contract が権威を持ちます）
- 手書きのアドホックな指示になること
- contract に記録されていない方法でビジネスロジックを発明すること

---

## Prompt と Contract の関係

これは Archflow における最も重要な関係の 1 つです。

- **contract** は artifact の境界の権威ある定義です
- **prompt** は contract データを AI 向けの実装指示に変換したものです

これが意味することは：

- prompt は contract から生成されるべきです
- prompt は contract からのデータを反映すべきです
- prompt は新しい制約や責務を導入すべきではありません
- 誰かがアーキテクチャの意図を理解したい場合、prompt ではなく contract を参照すべきです

---

## 例（Prompt の構造）

典型的な artifact prompt は次のような構造をとります。

```markdown
# Artifact: create_user

## Role
usecase

## Module
user

## Purpose
Orchestrate user creation by validating input and persisting a new user entity.

## Responsibilities
- Validate the incoming CreateUserCommand
- Create a new User aggregate
- Persist the User through the repository port
- Return a CreateUserResult

## Must Not
- Access the database directly
- Handle HTTP or transport concerns

## Inputs
- CreateUserCommand

## Outputs
- CreateUserResult

## Completion Criteria
- The use case accepts the command
- Validates input
- Creates and persists the user
- Returns a typed result
```

この形式は、一般的な AI コーディングアシスタントが実装を続けられるようにするのに十分な情報を提供します。

---

## 他の概念との関係

Prompt は次のものと接続します。

- **artifact**: prompt を受け取るもの
- **contract**: prompt が導出されるところ
- **module**: prompt が属するコンテキスト
- **role**: prompt のメタデータの一部

Prompt は Archflow のアーキテクチャモデルと AI ツールの間のブリッジです。

---

---

## 出力モード

AI モデルによってトークン制限や好みの冗長さが異なるため、2種類の出力モードを用意しています。

### Standard モード (`--mode standard`)
デフォルト設定です。GPT-4 や Gemini Pro、Claude 3 などの高性能モデルに適した、見出しや区切り線の多い詳細な Markdown 形式です。

### Compact モード (`--mode compact`)
軽量モデルやエディタ内補完に適しています。見出しを削り、依存関係などをカンマ区切りで 1 行にまとめることで、トークン消費を抑えます。

---

## ロールベースの自動最適化 (Phase 3)

アーティファクトの **Role** に応じて、AI が守るべき「完了基準（Completion Criteria）」を自動的に注入します。

| Role | デフォルトの AI 指示内容 |
| :--- | :--- |
| **`entity`** | ドメインの不変条件を厳格に守り、インフラ層の詳細は含めない。 |
| **`usecase`** | 単一のフローを統制するが、DB や HTTP の具体的なロジックは書かない。 |
| **`handler`** | トランスポートモデルの翻訳に専念し、ビジネスルールを混ぜない。 |
| **`repository`** | 永続化データとドメインモデル間の変換を安全に行う。 |

---

## 将来の方向性

将来的に、prompt は次のものをサポートするかもしれません。

- ロール固有の prompt プリセット
- モデル固有の出力モード
- コンパクトと詳細な prompt バリアント
- contract に対する prompt バリデーション
- エディタとエージェントの統合