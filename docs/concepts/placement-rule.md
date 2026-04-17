# Placement Rule

## Overview

In Batonel, a **Placement Rule** defines where artifacts of a given role should live.

A placement rule maps a role such as `entity`, `usecase`, or `controller`
to a path in the project structure.

Placement rules are one of the core mechanisms Batonel uses to turn
architectural intent into scaffoldable structure.

---

## Purpose

The purpose of a placement rule is to make structural placement explicit.

A placement rule answers questions such as:

- Where should artifacts of this role be created?
- How should this role appear in the project structure?
- Should this role use a particular file extension or naming pattern?

Without placement rules, artifacts can exist conceptually,
but Batonel cannot consistently resolve them into concrete locations.

---

## Responsibilities

A placement rule is responsible for defining:

- the path associated with a role
- optional file extension conventions
- optional naming conventions
- optional sidecar directory conventions

A placement rule is not responsible for defining behavioral constraints.
That belongs to contracts and contract templates.

---

## Core fields

Typical fields include:

- `role`
- `path`
- `file_extension` (optional)
- `naming` (optional)
- `sidecar` (optional)

In most user-facing configuration, placement rules are represented
as a mapping from role name to configuration.

---

## Example

```yaml
roles:
  entity:
    path: src/domain/entities/
    file_extension: rs

  usecase:
    path: src/application/usecases/
    file_extension: rs

  controller:
    path: src/interfaces/controllers/
    file_extension: ts
```

---

## Relationship to other concepts

A placement rule connects the conceptual model to actual structure.

The relationship is:

- the project defines the architectural frame
- the artifact defines a concrete execution unit
- the role identifies what kind of unit it is
- the placement rule resolves where that unit should live

Placement rules work closely with:

- `project`
- `artifact`
- `role`
- `scaffold`

They do not replace contracts.

---

## Design principles

A placement rule should be:

- explicit
- easy to read
- stable across a project
- aligned with role naming
- independent from unnecessary implementation detail

A good placement rule should reduce ambiguity in project structure.

---

## What a placement rule should not do

A placement rule should not:

- define business behavior
- define artifact responsibilities
- replace contracts
- encode too much architecture discussion inside path configuration
- become so custom that role-to-structure mapping is no longer understandable

---

## Why it matters

Many architecture discussions become implementation friction
at the moment someone asks:

- where should this file go?
- does this belong in domain or application?
- should this be under adapters or interfaces?

Placement rules make those decisions explicit ahead of time.

They are essential for:

- scaffold generation
- path resolution
- structural consistency
- future verification

---

## Examples of placement rules

Examples include:

- `entity` -> `src/domain/entities/`
- `usecase` -> `src/application/usecases/`
- `repository_port` -> `src/application/ports/outbound/`
- `controller` -> `src/interfaces/controllers/`
- `gateway` -> `src/infrastructure/gateways/`

The exact paths may vary by preset or project style,
but the role-to-location pattern remains central.

---

## Future directions

In the future, placement rules may also support:

- preset-provided defaults
- path override precedence rules
- role-specific file naming policies
- ecosystem-specific conventions
- stronger validation against scaffold output

---

## 日本語

# Placement Rule（配置ルール）

## 概要

Batonel における **Placement Rule** は、ロールをプロジェクト構造の場所にマッピングするルールです。

配置ルールは Batonel が各 artifact のロールに基づいてファイルパスを解決するのに役立ちます。

---

## 目的

配置ルールの目的は、アーキテクチャ上の分類（ロール）を具体的な場所（ファイルパス）に変換することです。

配置ルールは次のような質問に答えます。

- `entity` ロールを持つ artifact はどこに配置すべきか？
- `usecase` ロールを持つ artifact はどこに配置すべきか？
- `controller` ロールを持つ artifact はどこに配置すべきか？

配置ルールがなければ、Batonel はアーキテクチャの意図を具体的な構造に変換できません。

---

## 責務

配置ルールは次の責務を持ちます。

- ロールを具体的なディレクトリパスにマッピングする
- ファイル拡張子や命名パターンを定義する（任意）
- 一貫したスキャフォルド出力の基盤を提供する

配置ルールは次の責務を持ちません。

- artifact の振る舞いを定義すること（それは contract に属します）
- artifact の責務を定義すること（それも contract に属します）
- ロール自体を定義すること（それはロール概念に属します）

---

## 例

```yaml
roles:
  entity:
    path: src/domain/entities/
    extension: .rs

  usecase:
    path: src/application/usecases/
    extension: .rs

  controller:
    path: src/interfaces/controllers/
    extension: .rs

  repository_port:
    path: src/domain/ports/
    extension: .rs
```

この例では：

- `entity` ロールのすべての artifact は `src/domain/entities/` に配置される
- `usecase` ロールのすべての artifact は `src/application/usecases/` に配置される

これにより Batonel はアーキテクチャスタイルの宣言から直接パスを解決できます。

---

## 他の概念との関係

配置ルールは次のものと連携します。

- **role**: 配置ルールを適用するアーキテクチャ分類
- **artifact**: 解決されたパスを与えられるユニット
- **project**: パスが解釈されるグローバルコンテキスト
- **scaffold**: 解決されたパスに基づいて構造を生成する

配置ルールは場所を定義しますが、振る舞いは定義しません。

---

## 将来の方向性

将来的に、配置ルールは次のものをサポートするかもしれません。

- プリセットが提供するデフォルト
- パスオーバーライドの優先順位ルール
- ロール固有のファイル命名ポリシー
- エコシステム固有の慣習
- スキャフォルド出力に対するより強いバリデーション