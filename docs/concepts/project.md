# Project

## Overview

In Batonel, a **Project** is the top-level definition of architectural intent.

A project describes the overall structure that Batonel should use when generating
placement rules, artifacts, contracts, and AI handoff prompts.

It is the highest-level unit in the Batonel model.

---

## Purpose

The purpose of a project is to define the architectural frame in which all lower-level
elements are interpreted.

A project answers questions such as:

- What kind of system is this?
- What architectural style does it follow?
- What modules exist?
- What conventions should be applied?
- What language or environment is assumed?

Without a project definition, Batonel cannot consistently interpret artifact placement
or responsibility boundaries.

---

## Responsibilities

A project is responsible for defining:

- the project name
- the architectural style
- the language orientation
- the module list
- optional workspace or structural settings
- the context in which artifacts and contracts are generated

A project is not responsible for defining detailed behavior of individual artifacts.
That belongs to artifacts and contracts.

---

## Core fields

Typical fields include:

- `name`
- `architecture_style`
- `language`
- `modules`
- `workspace` (optional)
- `metadata` (optional)

---

## Example

```yaml
project:
  name: sample-app
  architecture_style: clean-hexagonal
  language: rust

workspace:
  enabled: true
  members:
    - crates/domain
    - crates/application
    - crates/adapters/http
    - crates/adapters/db

modules:
  - name: user
    features:
      - create_user
      - user_entity
```

---

## Relationship to other concepts

A project contains or frames:

- modules
- placement rules
- artifact plans
- contract templates
- prompts

The project does not replace those concepts.
Instead, it provides the context in which they are interpreted.

---

## Design principles

A project definition should be:

- explicit
- stable
- human-readable
- minimal but sufficient
- independent from implementation details where possible

A project should describe architectural intent, not business logic.

---

## What a project should not do

A project should not:

- define the internal code of one artifact
- replace contracts
- encode framework-specific behavior unless necessary
- mix architectural intent with low-level implementation detail

---

## What a project should not do

A project should not:

- define the internal code of one artifact
- replace contracts
- encode framework-specific behavior unless necessary
- mix architectural intent with low-level implementation detail

---

## Why it matters

In AI-assisted development, the project definition gives a shared architectural frame
to both humans and tools.

It helps Batonel answer:

- where an artifact belongs
- how a role should be interpreted
- what structure should be scaffolded
- which examples or presets best fit the repository

Without a clear project concept, the rest of the model becomes inconsistent.

---

## 日本語

# Project

## 概要

Batonel における **Project** は、アーキテクチャの意図のトップレベルの定義です。

プロジェクトは、Batonel がモジュール、配置ルール、artifact、contract、prompt を解釈する全体的な枠組みを記述します。

---

## 目的

プロジェクト定義の目的は、グローバルなアーキテクチャコンテキストを確立することです。

プロジェクトは次のような質問に答えます。

- このリポジトリはどのような種類のシステムか？
- どのアーキテクチャスタイルに従っているか？
- どのモジュールが存在するか？
- どの言語またはエコシステムを想定しているか？

プロジェクト定義がなければ、残りの Batonel モデルには安定したコンテキストがありません。

---

## 責務

プロジェクトは次のことを定義する責務を持ちます。

- プロジェクトの名前とアイデンティティ
- アーキテクチャスタイルまたはアプローチ
- 言語の方向性またはエコシステム
- モジュール空間（プロジェクト内でどのモジュールが存在するか）
- 任意のグローバル設定

プロジェクトは次の責務を持ちません。

- 個々の artifact を定義すること（それは artifact プランに属します）
- ロールを配置にマッピングすること（それは配置ルールに属します）
- artifact の責務を定義すること（それは contract に属します）

---

## コアフィールド

プロジェクト定義は通常、次のフィールドを含みます。

- `name`: プロジェクトの識別子
- `architecture_style`: アーキテクチャアプローチ（例: `clean`, `hexagonal`, `layered`, `simple`）
- `language`: 主要な言語またはエコシステム（例: `rust`, `typescript`, `generic`）
- `modules`: このプロジェクトに属するモジュールのリスト

---

## 例

```yaml
project:
  name: user-service
  architecture_style: clean-hexagonal
  language: rust

modules:
  - name: user
    features:
      - user registration
      - user profile management

  - name: auth
    features:
      - authentication
      - session management
```

---

## 他の概念との関係

プロジェクトは Batonel モデルの最上位です。

- **project**: グローバルコンテキストを設定する
- **module**: プロジェクト内の機能領域を定義する
- **artifact**: モジュール内の具体的な実装ユニット
- **placement rule**: artifact のロールをパスにマッピングする
- **contract**: artifact の振る舞いの境界を定義する

プロジェクトは Batonel が artifact、モジュール、ロール、パスを解釈する際に使用します。

Batonel は次のことを決定するのに役立てます：

- artifact がどこに属するか
- ロールをどのように解釈するか
- どの構造をスキャフォルドすべきか
- どの examples やプリセットがリポジトリに最も合うか

明確なプロジェクト概念がなければ、モデルの残りの部分が不整合になります。