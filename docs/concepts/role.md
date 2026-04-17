# Role

## Overview

In Batonel, a **Role** is the architectural classification assigned to an artifact.

A role describes what kind of implementation unit an artifact is,
such as an `entity`, `usecase`, `controller`, `repository_port`, or `gateway`.

Roles are one of the central concepts in Batonel because they connect
architectural intent to structure, contracts, prompts, and future verification.

---

## Purpose

The purpose of a role is to provide a stable architectural label
that Batonel can use across multiple layers of the system.

A role answers questions such as:

- What kind of artifact is this?
- Where should this artifact live?
- Which default contract rules should apply?
- What kind of prompt should be generated for it?
- How should this artifact be interpreted in the architecture?

Without roles, Batonel would have artifact names,
but no consistent way to attach structural or behavioral meaning to them.

---

## Responsibilities

A role is responsible for defining:

- the architectural type of an artifact
- the link between artifacts and placement rules
- the link between artifacts and contract templates
- the link between artifacts and prompt defaults
- a reusable vocabulary for architectural interpretation

A role is not responsible for defining one specific artifact.
That belongs to the artifact definition.

A role is also not responsible for defining exact behavior for every instance.
That belongs to contracts and contract templates.

---

## Core fields

A role is usually represented as a string value inside other files.

Typical usage appears in:

- `artifacts.plan.yaml`
- `placement.rules.yaml`
- `contracts.template.yaml`
- generated contract files
- generated prompt files

Examples of role values include:

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

## Example

```yaml
artifacts:
  - name: create_user
    module: user
    role: usecase

  - name: user
    module: user
    role: entity
```

In this example:

- `create_user` is interpreted as a usecase
- `user` is interpreted as an entity

That single role value affects structure, contracts, and prompts.

---

## Relationship to other concepts

A role connects several parts of the Batonel model.

The relationship is:

- the project defines the overall architectural frame
- the module defines a meaningful area within the project
- the artifact defines a concrete implementation unit
- the role defines what kind of unit that artifact is
- the placement rule defines where that role should live
- the contract template defines default behavior for that role
- the prompt generation can use the role to shape implementation handoff

This makes role one of the main connecting concepts in Batonel.

---

## Why role is central in Batonel

Role is central because it allows Batonel to reuse architectural meaning.

Instead of defining everything from scratch for every artifact,
Batonel can use role as a shared layer of interpretation.

For example, if an artifact has role `entity`,
Batonel can infer things such as:

- it probably belongs in a domain-oriented location
- it should not depend on infrastructure concerns
- its contract should emphasize invariants
- its prompt should stay focused on domain behavior

This reduces duplication and keeps the model more coherent.

---

## Design principles

A role should be:

- stable
- understandable
- reusable across artifacts
- meaningful in the architecture
- aligned with placement and contract behavior

A good role name should make architectural interpretation easier,
not harder.

---

## What a role should not do

A role should not:

- replace the artifact name
- replace the module name
- encode a full business feature
- become too specific to one single artifact
- become so vague that it loses structural meaning

For example, a role such as `very_special_user_creation_logic_handler`
would be too specific.
That is closer to an artifact name than a reusable role.

---

## Role vs artifact

A role is not the same as an artifact.

- a **role** is a reusable architectural category
- an **artifact** is a concrete implementation unit

For example:

- role: `usecase`
- artifact: `create_user`

This distinction is important because many artifacts may share the same role.

---

## Role vs module

A role is not the same as a module.

- a **module** groups artifacts by functional area
- a **role** classifies artifacts by architectural type

For example:

- module: `user`
- role: `usecase`
- artifact: `create_user`

This means the same role may appear in many modules.

---

## Role vs contract template

A role is not the same as a contract template.

- a **role** identifies an architectural type
- a **contract template** provides reusable defaults for that type

For example:

- role: `entity`
- contract template for `entity`:
  - protect domain invariants
  - avoid infrastructure dependencies

The role identifies.
The template elaborates.

---

## Examples of roles

Common example roles include:

- `entity`
- `usecase`
- `service`
- `repository_port`
- `repository_interface`
- `controller`
- `handler`
- `gateway`
- `repository_impl`

Not every project needs all of these.

Different examples or presets may use different subsets of roles.

For example:

- `minimal` may use only `entity` and `usecase`
- `generic-layered` may use `entity`, `service`, `controller`, `gateway`
- `rust-clean-hexagonal` may use `entity`, `usecase`, `repository_port`, `repository_impl`, `http_handler`

---

## Role naming guidance

Role names should be:

- concise
- reusable
- architecture-oriented
- consistent across files

Good role names tend to describe a kind of artifact,
not one specific artifact instance.

Examples of good role names:

- `entity`
- `usecase`
- `controller`
- `gateway`

Examples of weak role names:

- `user_create_file`
- `special_logic_part`
- `main_thing`
- `handler2`

Weak role names make placement and contract behavior harder to reason about.

Note:
`service` can be valid when it is an explicit architectural role with a clear contract.
If the name is used as a catch-all bucket, prefer a more specific role that states
responsibility and boundary ownership.

---

## Why it matters

Many parts of Batonel depend on role consistency.

If role naming drifts, then all of the following become weaker:

- path resolution
- contract template application
- prompt consistency
- example clarity
- future verification

A stable role system makes Batonel much easier to scale.

---

## Future directions

In the future, roles may also support:

- role aliases
- preset-specific role maps
- role inheritance or specialization
- role validation rules
- role-specific verification checks
- ecosystem-specific role conventions

Even with those extensions, the basic purpose stays the same:

a role gives an artifact reusable architectural meaning.

---

## 日本語

# Role（ロール）

## 概要

Batonel における **Role** は、artifact のアーキテクチャ上の分類です。

ロールは artifact が何を表すかを記述します。例えば：

- `usecase`: ビジネスロジックのオーケストレーター
- `entity`: 振る舞いを持つドメインオブジェクト
- `controller`: 外部インターフェースのハンドラー
- `repository_port`: データアクセスの抽象化境界
- `adapter`: 外部システムのコネクター

ロールにより Batonel は artifact の名前を知らなくても、artifact がプロジェクト構造のどこに属するかを知ることができます。

---

## 目的

ロールの目的は、アーキテクチャ上の意味を artifact に付与することです。

ロールは次のような質問に答えます。

- この artifact は何を表しているか？
- どのディレクトリ層に属するか？
- どのような責務のパターンを持っているか？
- どのような依存関係が適切か？

ロールがなければ、Batonel はアーキテクチャ上の意味なしに名前のリストだけを持つことになります。

---

## 責務

ロールは次の責務を持ちます。

- artifact のアーキテクチャ上の分類を提供する
- 配置ルール解決のための基礎を形成する
- contract テンプレートのデフォルトを有効にする
- prompt 生成のコンテキストを提供する

ロールは次の責務を持ちません。

- artifact がどこに配置されるかを直接指定すること（それは配置ルールに属します）
- artifact の完全な責務を定義すること（それは contract に属します）
- アーキテクチャルールを強制すること（それは contract と将来の verify に属します）

---

## ロールの例

さまざまなアーキテクチャスタイルにまたがるロールの例：

### クリーン / ヘキサゴナルアーキテクチャ

- `entity`
- `value_object`
- `aggregate`
- `domain_service`
- `usecase` / `interactor`
- `repository_port`
- `adapter`
- `controller`
- `presenter`

### レイヤードアーキテクチャ

- `domain_model`
- `service`
- `repository`
- `controller`

### シンプル

- `handler`
- `model`
- `store`

ロールの命名はプロジェクトに依存します。
Batonel 自体は固定されたロールセットを強制しません。

---

## 他の概念との関係

ロールは複数の概念に接続します。

- **artifact**: ロールを持つもの
- **placement rule**: ロールをファイルパスにマッピングする
- **contract template**: ロールのデフォルト振る舞いを定義する
- **contract**: artifact 固有の責務と制約を定義する（ロールのデフォルトをオーバーライドまたは拡張する場合がある）

ロールは一貫した命名と分類の基礎を形成します。

---

## ロールを一貫させることが重要な理由

ロールの命名をプロジェクトまたはプリセット内で一貫させることにより：

- 配置ルールが予測通りに機能する
- contract テンプレートが信頼性よく適用される
- コントリビューターが artifact がアーキテクチャのどこに属するかを素早く理解できる
- verify がロールの整合性をチェックできる

命名の不整合は（例: あるファイルで `usecase`、別のファイルで `application_service`）静かな崩壊を引き起こす可能性があります。

---

## 将来の方向性

将来的に、ロールは次のものをサポートするかもしれません。

- プリセット固有のロールマップ
- ロールの継承または専門化
- ロールバリデーションルール
- ロール固有の verify チェック
- エコシステム固有のロール慣習

それらの拡張があっても、基本的な目的は変わりません。

ロールは artifact に再利用可能なアーキテクチャ上の意味を与えます。