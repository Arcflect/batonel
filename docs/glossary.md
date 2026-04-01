# Glossary

This glossary defines the core terms used in Archflow.

The goal is to keep terminology stable across examples, documentation,
schemas, and future CLI behavior.

---

## Project

The top-level definition of architectural intent.

A project describes the overall frame in which Archflow interprets modules,
placement rules, artifacts, contracts, and prompts.

Typical examples:
- a Rust clean / hexagonal application
- a generic layered service
- a modular monolith with multiple bounded modules

---

## Module

A named architectural area within a project.

A module groups related artifacts and features.
It is usually aligned with a business capability or cohesive technical area.

Examples:
- `user`
- `auth`
- `billing`

A module is not necessarily a package or directory by itself,
though it may map to those in a specific project.

---

## Role

The architectural role assigned to an artifact.

A role helps Archflow decide:

- where an artifact should live
- what template should apply
- what responsibilities are typical
- what constraints should be generated

Examples:
- `entity`
- `usecase`
- `service`
- `repository_port`
- `repository_interface`
- `controller`
- `handler`
- `gateway`
- `repository_impl`

---

## Artifact

The smallest planned implementation unit in Archflow.

An artifact is a concrete thing that should exist in the project structure,
such as a file, handler, entity, service, repository boundary, or adapter unit.

Artifacts are the main units Archflow scaffolds, constrains, and hands off
to humans or AI systems.

Examples:
- `user`
- `create_user`
- `user_repository`
- `create_user_handler`

---

## Placement Rule

A rule that maps a role to a location in the project structure.

Placement rules help Archflow determine where an artifact should live.

Examples:
- `entity` -> `src/domain/entities/`
- `usecase` -> `src/application/usecases/`
- `controller` -> `src/interfaces/controllers/`

Placement rules define location, not behavior.

---

## Artifact Plan

A structured list of artifacts that should be created for a project.

An artifact plan usually includes artifact names, roles, modules,
and optionally inputs and outputs.

It is one of the main inputs Archflow uses for scaffold generation.

---

## Contract

The definition of an artifact’s responsibilities and constraints.

A contract describes:

- what an artifact should do
- what it must not do
- what dependencies are allowed
- what dependencies are forbidden
- what inputs and outputs are expected
- how focused the implementation should be

Contracts are the main way Archflow preserves architectural intent
during implementation.

---

## Contract Template

A reusable rule set for generating contracts by role.

A contract template provides default responsibilities, constraints,
and implementation guidance for a given role.

Examples:
- a default `entity` template
- a default `usecase` template
- a default `controller` template

Artifact-specific contracts may extend or refine these templates.

---

## Prompt

The AI handoff representation of an artifact contract.

A prompt turns project context, artifact identity, and contract boundaries
into a format that can be directly given to an AI coding tool.

Prompts are derived from contracts.
They are not the primary source of truth.

---

## Scaffold

The generated structural output produced by Archflow.

Scaffolding may include:

- directories
- placeholder files
- contract files
- prompt files
- metadata files

Scaffolding is meant to make implementation easier and more consistent.

---

## Verify

The process of checking whether project structure and artifact definitions
remain consistent with Archflow rules.

Verification may include checking:

- required contract fields
- placement consistency
- artifact status consistency
- contract/prompt presence
- future optional code-aware checks

---

## Preset

A reusable starting package for a common architectural style or ecosystem.

A minimal preset contains:

- preset metadata (`name`, `summary`, `architecture_style`, `ecosystem`)
- project defaults
- placement rules
- contract templates

A preset may optionally include a starter artifact plan or prompt defaults.

A preset is operational.
An example is descriptive.

Examples:
- Rust clean / hexagonal preset
- generic layered preset

---

## AI Handoff

The act of passing an artifact to an AI system for implementation.

In Archflow, AI handoff is based on:

- project context
- artifact definition
- contract
- generated prompt

The goal is to make implementation clearer, smaller in scope,
and less likely to drift from architecture.

---

## Sidecar File

A file that accompanies an implementation artifact but is not itself
the implementation.

In Archflow, sidecar files commonly include:

- `*.contract.yaml`
- `*.prompt.md`

Sidecar files are important because they allow Archflow to work
even before production code exists.

---

## Status

The lifecycle state of an artifact or contract.

Status helps track where an artifact is in the workflow.

Examples:
- `planned`
- `scaffolded`
- `implementing`
- `reviewing`
- `done`

Status is especially useful for AI-assisted workflows and future verification.

---

## Architecture-to-Execution Bridge

A short way to describe what Archflow is.

It means Archflow sits between:

- architecture design
- structural planning
- AI handoff
- implementation scaffolding
- future verification

Archflow does not stop at documentation.
It turns design intent into executable implementation context.

---

## 日本語

この用語集は Archflow で使用されるコアな用語を定義します。

terminology を example、ドキュメント、スキーマ、将来の CLI 動作にわたって安定させることが目的です。

---

### Project（プロジェクト）

アーキテクチャの意図のトップレベルの定義。

プロジェクトは、Archflow がモジュール、配置ルール、artifact、contract、prompt を解釈する全体的な枠組みを記述します。

典型的な例：
- Rust のクリーン / ヘキサゴナルアプリケーション
- 汎用レイヤードサービス
- 複数の境界を持つモジュラーモノリス

---

### Module（モジュール）

プロジェクト内の名前付きアーキテクチャ領域。

モジュールは関連する artifact と機能をグループ化します。
通常はビジネス機能または凝集した技術的領域に合わせて配置されます。

例：
- `user`
- `auth`
- `billing`

モジュールはそれ自体がパッケージやディレクトリである必要はありませんが、特定のプロジェクトではそれにマッピングされることがあります。

---

### Role（ロール）

artifact に割り当てられたアーキテクチャ上のロール。

ロールは Archflow が次のことを判断するのに役立ちます。

- artifact がどこに配置されるべきか
- どのテンプレートを適用するか
- どのような責務が典型的か
- どのような constraint を生成するか

例：
- `entity`
- `usecase`
- `service`
- `repository_port`
- `repository_interface`
- `controller`
- `handler`
- `gateway`
- `repository_impl`

---

### Artifact（アーティファクト）

Archflow における最小の計画済み実装ユニット。

artifact は、プロジェクト構造に存在すべき具体的なもの（ファイル、ハンドラー、エンティティ、サービス、リポジトリ境界、アダプターユニットなど）です。

Artifact は Archflow がスキャフォルドし、制約を与え、人間または AI システムにハンドオフする主要なユニットです。

例：
- `user`
- `create_user`
- `user_repository`
- `create_user_handler`

---

### Placement Rule（配置ルール）

ロールをプロジェクト構造の場所にマッピングするルール。

配置ルールは Archflow が artifact がどこに配置されるべきかを判断するのに役立ちます。

例：
- `entity` → `src/domain/entities/`
- `usecase` → `src/application/usecases/`
- `controller` → `src/interfaces/controllers/`

配置ルールは場所を定義しますが、振る舞いは定義しません。

---

### Artifact Plan（アーティファクトプラン）

プロジェクトのために作成すべき artifact の構造化されたリスト。

artifact プランには通常、artifact の名前、ロール、モジュール、および任意の入力と出力が含まれます。

これは Archflow がスキャフォルド生成に使用する主要な入力の 1 つです。

---

### Contract（コントラクト）

artifact の責務と constraint の定義。

contract は次のことを記述します。

- artifact が何をすべきか
- 何をしてはいけないか
- どの依存関係が許可されているか
- どの依存関係が禁止されているか
- どのような入力と出力が期待されるか
- 実装がどの程度集中しているべきか

Contract は Archflow が実装中にアーキテクチャの意図を保全する主な方法です。

---

### Contract Template（コントラクトテンプレート）

ロール別に contract を生成するための再利用可能なルールセット。

contract テンプレートは、特定のロールにデフォルトの責務、constraint、実装ガイダンスを提供します。

例：
- デフォルトの `entity` テンプレート
- デフォルトの `usecase` テンプレート
- デフォルトの `controller` テンプレート

artifact 固有の contract はこれらのテンプレートを拡張または精緻化することができます。

---

### Prompt（プロンプト）

artifact contract の AI ハンドオフ表現。

prompt は、プロジェクトコンテキスト、artifact のアイデンティティ、contract の境界を、AI コーディングツールに直接渡せる形式に変換します。

Prompt は contract から導出されます。
Prompt は主要な真実の源ではありません。

---

### Scaffold（スキャフォルド）

Archflow によって生成された構造的な出力。

スキャフォルディングには次のものが含まれる場合があります。

- ディレクトリ
- 仮ファイル
- contract ファイル
- prompt ファイル
- メタデータファイル

スキャフォルディングは、実装をより簡単で一貫したものにするためのものです。

---

### Verify（ベリファイ）

プロジェクト構造と artifact の定義が Archflow のルールと整合して一致したままかどうかをチェックするプロセス。

Verify には以下のチェックが含まれる場合があります。

- 必須の contract フィールド
- 配置の整合性
- artifact のステータスの整合性
- contract / prompt の存在
- 将来の任意のコード認識チェック

---

### Preset（プリセット）

一般的なアーキテクチャスタイルまたはエコシステム向けの再利用可能な出発点パッケージ。

最小 preset は次を含みます。

- preset metadata（`name`、`summary`、`architecture_style`、`ecosystem`）
- project defaults
- 配置ルール
- contract templates

preset は、starter artifact plan や prompt defaults を任意で含めてもよいです。

Preset は運用的です。
Example は記述的です。

例：
- Rust クリーン / ヘキサゴナルプリセット
- 汎用レイヤードプリセット

---

### AI Handoff（AI ハンドオフ）

実装のために artifact を AI システムに渡す行為。

Archflow での AI ハンドオフは次に基づいています。

- プロジェクトコンテキスト
- artifact の定義
- contract
- 生成された prompt

目的は、実装をより明確に、範囲を小さく、アーキテクチャからの逸脱を少なくすることです。

---

### Sidecar File（サイドカーファイル）

実装の artifact に付随するが、それ自体は実装ではないファイル。

Archflow では、sidecar ファイルには一般的に次のものが含まれます。

- `*.contract.yaml`
- `*.prompt.md`

Sidecar ファイルは、本番コードが存在する前でも Archflow が機能できるため重要です。

---

### Status（ステータス）

artifact または contract のライフサイクル状態。

ステータスは artifact がワークフローのどこにあるかを追跡するのに役立ちます。

例：
- `planned`
- `scaffolded`
- `implementing`
- `reviewing`
- `done`

ステータスは AI 支援ワークフローと将来の verify に特に有用です。

---

### Architecture-to-Execution Bridge（設計から実行への橋渡し）

Archflow が何であるかを簡潔に表現した言葉。

Archflow が次の間に位置することを意味します。

- アーキテクチャ設計
- 構造計画
- AI ハンドオフ
- 実装スキャフォルディング
- 将来の verify

Archflow はドキュメント作成で止まりません。
設計の意図を実行可能な実装コンテキストに変換します。