# Presets

This document explains what a **preset** means in Archflow,
how presets relate to examples,
and how the current examples may evolve into reusable presets over time.

The goal is to clarify that presets are not rigid architecture templates.
They are reusable starting points for project structure, role conventions,
contract defaults, and implementation scaffolding.

---

## Overview

In Archflow, a **preset** is a reusable configuration package
for a common architectural style or ecosystem.

A minimal preset contains:

- preset metadata (`name`, `summary`, `architecture_style`, `ecosystem`)
- project defaults
- placement rules
- contract templates

A preset may optionally include:

- a starter artifact plan
- prompt defaults
- references to example structures

A preset helps users start faster without having to define every rule from scratch.

---

## Why presets matter

Archflow is designed to be flexible.

That flexibility is important, but it also means new users may ask:

- where should I start?
- which role names should I use?
- what directory structure should I adopt?
- how should contracts be shaped for this architecture?

Presets help answer those questions.

They provide a practical starting point while still allowing customization.

A preset is not meant to replace architectural thinking.
It is meant to reduce setup cost and improve consistency.

---

## Presets are not fixed architecture truth

Archflow should not assume that there is one correct architecture.

Different teams may prefer:

- simple layouts
- layered architecture
- clean architecture
- hexagonal architecture
- modular monolith structures
- ecosystem-specific conventions

Presets exist to support these styles,
not to declare one of them universally correct.

This is a core Archflow principle:

**presets support architectural intent, they do not dictate it**

---

## Relationship between examples and presets

The current `examples/` directory is the natural starting point
for future preset design.

Examples show:

- how input files may look
- how output structure may look
- how roles and contracts may be organized
- how Archflow can express different architectural styles

Presets build on the same ideas,
but move from “illustration” to “reusable configuration”.

You can think of the relationship like this:

- **example** = a teaching artifact
- **preset** = a reusable starting package

Examples are descriptive.
Presets are operational.

---

## Minimal preset model

To keep Phase 5 minimal, a preset should be understood as having two parts.

### 1. Metadata

The minimum metadata needed for a preset is:

- `name`: stable preset identifier
- `summary`: short explanation of what the preset is for
- `architecture_style`: the architectural style the preset represents
- `ecosystem`: the intended language or ecosystem target, or `generic`

### 2. Reusable defaults

The minimum reusable content of a preset is:

- project defaults
- placement rules
- contract templates by role

This is enough to make a preset operational without turning it into a full project framework.

### Optional additions

A preset may later include:

- a starter artifact plan
- prompt defaults
- links or provenance back to an example

### Explicit non-goals

A preset is not:

- a complete application template
- a replacement for examples
- a replacement for contract sidecars
- a packaging layer for runtime dependencies or framework internals

---

## Current examples and future preset direction

Archflow currently includes these examples:

- `minimal`
- `generic-layered`
- `rust-clean-hexagonal`

These examples are useful as documentation today,
and they may become the foundation for future preset definitions.

### `minimal`

Current role:
- the smallest example for understanding the Archflow model

Future preset direction:
- a minimal preset for the smallest useful setup
- useful for experiments, demos, and onboarding
- useful when users want the least amount of structure

Likely preset characteristics:
- small role set
- minimal directory structure
- minimal contract defaults
- minimal prompt shape

---

### `generic-layered`

Current role:
- a language-agnostic layered architecture example

Future preset direction:
- a general-purpose layered preset
- useful for teams that want clear boundaries without ecosystem lock-in
- useful as a neutral default before choosing language-specific conventions

Likely preset characteristics:
- roles for domain, application, interfaces, infrastructure
- broad compatibility across languages
- generic file extensions or configurable output
- moderate default contract structure

---

### `rust-clean-hexagonal`

Current role:
- a Rust-oriented example using clean / hexagonal structure

Future preset direction:
- a Rust clean / hexagonal preset
- useful for Rust projects that care about strong architectural boundaries
- useful for workspace-oriented repository layouts

Likely preset characteristics:
- Rust-friendly role naming
- workspace-aware path conventions
- strong separation of domain, application, and adapters
- more explicit dependency boundary defaults

---

## What a preset may contain

A preset may eventually package some or all of the following:

### 1. Project defaults

Examples:
- architecture style
- language orientation
- workspace defaults
- default modules or starter module structure

### 2. Placement rules

Examples:
- `entity` -> `src/domain/entities/`
- `usecase` -> `src/application/usecases/`
- `controller` -> `src/interfaces/controllers/`

### 3. Contract templates

Examples:
- default responsibilities by role
- default forbidden behaviors by role
- default dependency boundaries by role
- default implementation size guidance

### 4. Prompt defaults

Examples:
- standard artifact prompt sections
- role-specific completion criteria
- prompt formatting variants

### 5. Example artifacts

Examples:
- starter artifact plans
- common artifact names by architecture style
- small sample module definitions

### 6. Optional verification defaults

Future examples:
- required contract fields
- role consistency checks
- structure validation defaults

---

## Preset lifecycle

A useful way to think about preset maturity is in stages.

### Stage 1: example

The structure exists as a documented example.

Purpose:
- explain the concept
- teach the architecture style
- show expected input and output

### Stage 2: draft preset

The structure becomes reusable with fewer manual edits.

Purpose:
- provide a copyable starting point
- establish stable naming and role conventions
- reduce setup cost

### Stage 3: supported preset

The preset becomes an officially supported Archflow starting package.

Purpose:
- give users a stable preset path
- reduce ambiguity in project bootstrap
- support consistent scaffold generation and future verification

This means examples can evolve into presets gradually.
They do not need to become presets all at once.

---

## How examples should evolve into presets

Not every example should automatically become a preset.

An example is a good preset candidate when it is:

- understandable
- repeatable
- broadly useful
- internally consistent
- stable enough to teach as a recommended starting point

A weak preset candidate is:

- too experimental
- too narrow
- too tied to one internal project
- too inconsistent in role naming or contract behavior

The right approach is to let examples prove themselves first.

---

## Suggested preset model

A future preset may look conceptually like this:

- preset name
- intended architecture style
- intended language or ecosystem
- included project defaults
- included placement rules
- included contract templates
- optional starter artifact plan
- optional prompt defaults

For example:

- `minimal`
- `generic-layered`
- `rust-clean-hexagonal`

This means the current examples already hint at future preset names.

---

## Presets and customization

A preset should always be customizable.

Users should be able to:

- start from a preset
- change placement rules
- change role names
- refine contract templates
- add or remove modules
- override prompt defaults

This is important because Archflow is meant to preserve intent,
not force one fixed structure.

A good preset accelerates setup without removing flexibility.

---

## Presets and examples should coexist

Even after presets exist, examples should still remain.

Why?

Because they serve different purposes.

### Examples help users learn
Examples explain the model and show concrete input/output pairs.

### Presets help users start
Presets reduce bootstrap effort and provide reusable defaults.

A project may begin from a preset,
while the examples continue to serve as documentation and comparison material.

---

## How this connects to current repository structure

Today, the current examples live under:

- `examples/minimal/`
- `examples/generic-layered/`
- `examples/rust-clean-hexagonal/`

In the future, a preset system might reference those same structures
or extract reusable definitions from them.

For example, a future preset directory might look like:

- `presets/minimal/`
- `presets/generic-layered/`
- `presets/rust-clean-hexagonal/`

This does not need to exist yet.

For now, the important thing is to treat examples as the conceptual foundation
for future presets.

---

## Practical interpretation for the current phase

At the current stage of Archflow, the repository does not need a full preset system yet.

What it does need is:

- clear examples
- stable role naming
- stable concept definitions
- stable schema drafts
- clear documentation of how examples may become presets

That is exactly why this document exists.

It explains the direction without requiring preset implementation too early.

---

## Summary

A preset in Archflow is a reusable starting point for architecture-aware scaffolding.

It is not a rigid template.
It is not architecture dogma.
It is not a replacement for design.

It is a practical package of defaults that helps users start faster and more consistently.

The current examples:

- `minimal`
- `generic-layered`
- `rust-clean-hexagonal`

are the natural foundation for future presets.

If you remember only one thing, remember this:

**examples teach the model, presets operationalize it**

---

## 日本語

このドキュメントは Archflow における **preset** の意味、preset と examples の関係、そして現在の examples が将来的に再利用可能な preset へと進化する方法を説明します。

Preset は硬直したアーキテクチャテンプレートではなく、プロジェクト構造、ロール慣習、contract のデフォルト、実装スキャフォルディングのための再利用可能な出発点であることを明確にすることが目的です。

---

### 概要

Archflow における **preset** は、一般的なアーキテクチャスタイルまたはエコシステムのための再利用可能な設定パッケージです。

preset には次のものが含まれる場合があります。

- プロジェクトのデフォルト
- 配置ルール
- contract テンプレート
- artifact の慣習
- prompt のデフォルト
- example 構造

Preset はユーザーがゼロからすべてのルールを定義しなくても素早く始められるようにします。

---

### Preset が重要な理由

Archflow は柔軟に設計されています。

その柔軟性は重要ですが、新しいユーザーが次のように質問することもあります。

- どこから始めればよいか？
- どのロール名を使うべきか？
- どのディレクトリ構造を採用すべきか？
- このアーキテクチャに対して contract をどのような形にすべきか？

Preset はこれらの質問に答えるのに役立ちます。

カスタマイズを許可しながら実用的な出発点を提供します。

Preset はアーキテクチャ的思考を置き換えることを意図していません。
セットアップコストを削減し、一貫性を改善することを意図しています。

---

### Preset は固定されたアーキテクチャの真実ではない

Archflow は 1 つの正しいアーキテクチャがあると仮定すべきではありません。

異なるチームが好む可能性があります。

- シンプルなレイアウト
- レイヤードアーキテクチャ
- クリーンアーキテクチャ
- ヘキサゴナルアーキテクチャ
- モジュラーモノリス構造
- エコシステム固有の慣習

Preset はこれらのスタイルをサポートするために存在します。
そのうちの 1 つが普遍的に正しいと宣言するためではありません。

これは Archflow のコア原則です。

**preset はアーキテクチャの意図をサポートし、それを指示するものではない**

---

### Examples と preset の関係

現在の `examples/` ディレクトリは将来の preset 設計の自然な出発点です。

Examples が示すものは次のとおりです。

- 入力ファイルがどのように見えるか
- 出力構造がどのように見えるか
- ロールと contract がどのように整理されるか
- Archflow が異なるアーキテクチャスタイルをどのように表現できるか

Preset は同じアイデアを基盤にしていますが、「説明」から「再利用可能な設定」へと移行します。

関係性はこのように考えられます。

- **example** = 学習用のアーティファクト
- **preset** = 再利用可能な出発点パッケージ

Examples は記述的です。
Preset は運用的です。

---

### 現在の examples と将来の preset の方向性

Archflow には現在これらの examples が含まれています。

- `minimal`
- `generic-layered`
- `rust-clean-hexagonal`

これらの examples は今日のドキュメントとして有用であり、将来の preset 定義の基盤になるかもしれません。

#### `minimal`

現在の役割：
- Archflow モデルを理解するための最小の example

将来の preset の方向性：
- 最小限の有用なセットアップのための最小 preset
- 実験、デモ、オンボーディングに有用
- 最小限の構造が欲しいユーザーに有用

想定される preset の特性：
- 小さなロールセット
- 最小限のディレクトリ構造
- 最小限の contract デフォルト
- 最小限の prompt 形状

---

#### `generic-layered`

現在の役割：
- 言語非依存のレイヤードアーキテクチャの example

将来の preset の方向性：
- 汎用レイヤード preset
- エコシステムロックインなしに明確な境界を望むチームに有用
- 言語固有の慣習を選ぶ前のニュートラルなデフォルトとして有用

想定される preset の特性：
- domain、application、interfaces、infrastructure のロール
- 言語間での広い互換性
- 汎用のファイル拡張子または設定可能な出力
- 中程度のデフォルト contract 構造

---

#### `rust-clean-hexagonal`

現在の役割：
- クリーン / ヘキサゴナルアーキテクチャを使用した Rust 向け example

将来の preset の方向性：
- Rust クリーン / ヘキサゴナル preset
- 強いアーキテクチャ境界を重視する Rust プロジェクトに有用
- ワークスペース指向のリポジトリレイアウトに有用

想定される preset の特性：
- Rust に適したロール命名
- ワークスペース対応のパス慣習
- domain、application、adapters の強い分離
- より明示的な依存境界のデフォルト

---

### Preset に含まれる可能性があるもの

将来の preset は以下の一部またはすべてをパッケージかもしれません。

#### 1. プロジェクトのデフォルト

例：
- アーキテクチャスタイル
- 言語の方向性
- ワークスペースのデフォルト
- デフォルトモジュールまたはスターターモジュール構造

#### 2. 配置ルール

例：
- `entity` → `src/domain/entities/`
- `usecase` → `src/application/usecases/`
- `controller` → `src/interfaces/controllers/`

#### 3. Contract テンプレート

例：
- ロール別のデフォルト責務
- ロール別のデフォルト禁止振る舞い
- ロール別のデフォルト依存境界
- デフォルトの実装サイズガイダンス

#### 4. Prompt のデフォルト

例：
- 標準的な artifact prompt セクション
- ロール固有の完了基準
- prompt フォーマットバリアント

#### 5. Artifact の例

例：
- スターター artifact プラン
- アーキテクチャスタイル別の一般的な artifact 名
- 小さなサンプルモジュール定義

#### 6. 任意の verify デフォルト

将来の例：
- 必須の contract フィールド
- ロールの整合チェック
- 構造バリデーションのデフォルト

---

### Preset のライフサイクル

Preset の成熟度について考える有用な方法は、段階ごとです。

#### Stage 1: Example

構造がドキュメント化された example として存在します。

目的：
- 概念を説明する
- アーキテクチャスタイルを教える
- 期待される入力と出力を示す

#### Stage 2: ドラフト preset

構造が手動編集を減らして再利用可能になります。

目的：
- コピー可能な出発点を提供する
- 安定した命名とロール慣習を確立する
- セットアップコストを削減する

#### Stage 3: サポートされた preset

Preset が正式にサポートされた Archflow の出発点パッケージになります。

目的：
- ユーザーに安定した preset パスを提供する
- プロジェクトブートストラップの曖昧さを軽減する
- 一貫したスキャフォルド生成と将来の verify をサポートする

つまり、examples は徐々に preset に進化できます。
一度にすべてが preset になる必要はありません。

---

### Examples が preset に進化する方法

すべての example が自動的に preset になるべきではありません。

Example が良い preset 候補である場合は次のとおりです。

- 理解しやすい
- 繰り返し可能
- 広く有用
- 内部的に一貫している
- 推奨される出発点として教えるのに十分安定している

弱い preset 候補は次のとおりです。

- 実験的すぎる
- 範囲が狭すぎる
- 1 つの内部プロジェクトに縛られすぎている
- ロール命名または contract の振る舞いが不一致

正しいアプローチは、まず examples がその価値を証明するのを待つことです。

---

### 提案される preset モデル

将来の preset は概念的にこのようになるかもしれません。

- preset 名
- 意図するアーキテクチャスタイル
- 意図する言語またはエコシステム
- 含まれるプロジェクトのデフォルト
- 含まれる配置ルール
- 含まれる contract テンプレート
- 任意のスターター artifact プラン
- 任意の prompt デフォルト

例えば：

- `minimal`
- `generic-layered`
- `rust-clean-hexagonal`

つまり、現在の examples はすでに将来の preset 名を示唆しています。

---

### Preset とカスタマイズ

Preset は常にカスタマイズ可能であるべきです。

ユーザーは次のことができるべきです。

- preset から始める
- 配置ルールを変更する
- ロール名を変更する
- contract テンプレートを洗練させる
- モジュールを追加または削除する
- prompt のデフォルトをオーバーライドする

これは Archflow が意図を保全するためのものであり、1 つの固定された構造を強制するためではないため重要です。

良い preset はセットアップを加速させ、柔軟性を失わせません。

---

### Preset と examples は共存すべき

preset が存在した後も、examples はまだ残るべきです。

なぜなら、それらは異なる目的を果たすからです。

#### Examples はユーザーが学ぶのを助ける
Examples はモデルを説明し、具体的な入力/出力のペアを示します。

#### Preset はユーザーが始めるのを助ける
Preset はブートストラップの労力を削減し、再利用可能なデフォルトを提供します。

プロジェクトは preset から始まる場合があります。
一方で、examples はドキュメントと比較材料として引き続き機能します。

---

### 現在のリポジトリ構造との接続

今日、現在の examples は次の場所にあります。

- `examples/minimal/`
- `examples/generic-layered/`
- `examples/rust-clean-hexagonal/`

将来的に、preset システムはそれらと同じ構造を参照したり、そこから再利用可能な定義を抽出したりするかもしれません。

例えば、将来の preset ディレクトリはこのようになるかもしれません。

- `presets/minimal/`
- `presets/generic-layered/`
- `presets/rust-clean-hexagonal/`

これはまだ存在する必要はありません。

今のところ重要なのは、examples を将来の preset の概念的な基盤として扱うことです。

---

### 現段階での実践的な解釈

Archflow の現段階では、リポジトリにはまだ完全な preset システムは必要ありません。

必要なものは次のとおりです。

- 明確な examples
- 安定したロール命名
- 安定した概念定義
- 安定したスキーマドラフト
- examples が preset になる方法の明確なドキュメント

これがまさにこのドキュメントが存在する理由です。

早すぎる preset 実装を必要とせず、方向性を説明しています。

---

### まとめ

Archflow における preset は、アーキテクチャを意識したスキャフォルディングのための再利用可能な出発点です。

硬直したテンプレートではありません。
アーキテクチャの教義でもありません。
設計の代替でもありません。

より速く、より一貫してユーザーが始めるのを助ける実用的なデフォルトのパッケージです。

現在の examples：

- `minimal`
- `generic-layered`
- `rust-clean-hexagonal`

は将来の preset の自然な基盤です。

1 つだけ覚えておくなら、これを覚えてください。

**examples はモデルを教え、preset はそれを運用に移す**