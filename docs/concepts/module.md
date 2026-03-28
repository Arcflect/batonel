# Module

## Overview

In Archflow, a **Module** is a named architectural area within a project.

A module groups related artifacts and features into a coherent unit.
It usually represents a business capability, a bounded functional area,
or a stable architectural grouping.

Modules help Archflow organize artifacts in a way that is meaningful
beyond raw directory structure.

---

## Purpose

The purpose of a module is to provide a stable grouping layer between
the whole project and individual artifacts.

A module answers questions such as:

- Which artifacts belong together?
- Which functional area does this artifact belong to?
- How should a project be divided into meaningful areas?
- How should related features be grouped?

Without modules, artifacts may still exist,
but the project loses an important organizing concept.

---

## Responsibilities

A module is responsible for defining:

- a named area within the project
- a grouping boundary for related artifacts
- an optional list of features
- a stable unit of architectural organization

A module is not responsible for defining exact file placement rules.
That belongs to placement rules.

A module is also not responsible for defining the full behavioral boundary
of each artifact.
That belongs to contracts.

---

## Core fields

Typical fields include:

- `name`
- `features` (optional)

Modules are usually defined inside the project definition.

---

## Example

```yaml
modules:
  - name: user
    features:
      - create_user
      - user_entity

  - name: auth
    features:
      - login
      - refresh_token
```

---

## Relationship to other concepts

A module exists inside the project context.

The relationship is:

- the project defines the overall architectural frame
- the module defines a meaningful area within that frame
- the artifact belongs to one module
- the role defines what kind of artifact it is
- the placement rule defines where it should live
- the contract defines what it is responsible for

This means a module helps connect architectural meaning
to the artifact model.

---

## Design principles

A module should be:

- meaningful
- stable
- understandable by contributors
- broad enough to group related artifacts
- narrow enough to avoid becoming meaningless

A good module reflects a coherent area of the system.

---

## What a module should not do

A module should not:

- replace the whole project structure
- duplicate placement rules
- act as a full contract definition
- become just a technical folder label with no architectural meaning
- group unrelated concerns together without a clear reason

---

## Why it matters

Modules help Archflow keep structure understandable.

They are useful because they allow the project to express:

- which artifacts belong to the same functional area
- how features are grouped
- how architectural boundaries are organized at a level above files
- how a project can scale beyond only a handful of artifacts

Without modules, the project risks becoming a flat list of artifacts.

---

## Examples of modules

Examples include:

- `user`
- `auth`
- `billing`
- `inventory`
- `catalog`

A module does not need to map directly to a package, crate, or folder,
although it may do so in some projects.

For example:

- in a small project, a module may simply be a conceptual grouping
- in a larger project, a module may align with a bounded directory area
- in a workspace-based project, a module may span multiple technical packages

---

## Module vs feature

A module is broader than a feature.

- a **module** defines a stable area of the system
- a **feature** defines a more specific capability within that area

For example:

- module: `user`
- features:
  - `create_user`
  - `get_user`
  - `update_user`

This distinction helps Archflow avoid treating every feature
as if it were a top-level architectural unit.

---

## Module vs artifact

A module is broader than an artifact.

- a module groups related work
- an artifact is a concrete implementation unit

For example:

- module: `user`
- artifacts:
  - `user`
  - `create_user`
  - `user_repository`
  - `create_user_handler`

This makes the module a useful middle layer between project and artifact.

---

## Future directions

In the future, modules may also support:

- module-level metadata
- ownership information
- module-specific presets
- module-level verification scope
- module dependency hints
- richer feature grouping

---

## 日本語

# Module

## 概要

Archflow における **Module** は、プロジェクト内の名前付きアーキテクチャ領域です。

モジュールは関連する artifact と機能をグループ化します。
通常はビジネス機能または凝集した技術的領域に合わせて配置されます。

---

## 目的

モジュールの目的は、artifact をより大きな意味のある領域にまとめることです。

モジュールは次のような質問に答えます。

- この artifact が存在するビジネスまたは技術的な領域は何か？
- どの artifact が一緒に属するか？
- このグループはどのような機能に関連しているか？

モジュールがなければ、artifact は孤立した名前のリストとなり、より大きな構造の意味を失います。

---

## 責務

モジュールは次の責務を持ちます。

- artifact をまとめて整理する
- プロジェクト構造に機能的な意味を与える
- 配置と contract 解釈のための名前空間を提供する

モジュールは次の責務を持ちません。

- artifact のロールを決定すること（それはロールに属します）
- artifact がどこに配置されるかを決定すること（それは配置ルールに属します）
- artifact の責務を定義すること（それは contract に属します）

---

## コアフィールド

モジュールは通常、次のものを持ちます。

- `name`: モジュールの識別子（例: `user`、`auth`、`billing`）
- `features`: モジュールに属する機能または artifact の説明（任意）

---

## 例

```yaml
modules:
  - name: user
    features:
      - user registration
      - user profile management
      - user authentication

  - name: billing
    features:
      - subscription management
      - invoice generation
      - payment processing
```

この例では：

- `user` はユーザー関連の artifact をグループ化する
- `billing` は請求関連の artifact をグループ化する

それぞれのモジュールは独立したアーキテクチャ領域を表します。

---

## 他の概念との関係

モジュールはアーキテクチャ階層の中間レイヤーです。

- **project**: モジュールが定義されるグローバルコンテキスト
- **module**: プロジェクト内の名前付き領域
- **artifact**: モジュールに属する具体的な実装ユニット
- **role**: artifact のアーキテクチャ上の分類（モジュールを超えて再利用可能）

同じロールが複数のモジュールに現れる可能性があります。
例えば、`entity` ロールは `user` モジュールと `billing` モジュールの両方にある場合があります。

---

## モジュールが Archflow で重要な理由

モジュールは artifact に文脈を与えます。

モジュールがなければ、`create_user` と `create_invoice` は同等に分離された artifact に見えます。
モジュールにより、`create_user` は `user` ドメインに属し、`create_invoice` は `billing` ドメインに属することがわかります。

これは AI ハンドオフにおいて特に有用です。なぜなら、モジュールのコンテキストがより集中した実装を助けるからです。

---

## 将来の方向性

将来的に、モジュールは次のものをサポートするかもしれません。

- モジュールレベルのメタデータ
- 所有権情報
- モジュール固有のプリセット
- モジュールレベルの verify スコープ
- モジュール依存関係のヒント
- より豊富な機能グループ化