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

## Future directions

In the future, prompts may also support:

- role-specific prompt presets
- model-specific output modes
- compact and detailed prompt variants
- prompt validation against contracts
- editor and agent integration

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

## 将来の方向性

将来的に、prompt は次のものをサポートするかもしれません。

- ロール固有の prompt プリセット
- モデル固有の出力モード
- コンパクトと詳細な prompt バリアント
- contract に対する prompt バリデーション
- エディタとエージェントの統合