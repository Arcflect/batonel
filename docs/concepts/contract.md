# Contract

## Overview

In Archflow, a **Contract** defines the responsibilities and constraints of an artifact.

A contract explains what an artifact is supposed to do, what it must not do,
what dependencies it may rely on, and what outputs it is expected to provide.

The contract is the main mechanism Archflow uses to preserve architectural intent
during implementation.

---

## Purpose

The purpose of a contract is to make artifact behavior explicit before implementation begins.

A contract answers questions such as:

- What is this artifact for?
- What are its responsibilities?
- What must it never do?
- What dependencies are allowed?
- What boundaries must be preserved?
- What does “done” mean for this artifact?

This makes the architecture usable in day-to-day implementation.

---

## Responsibilities

A contract is responsible for defining:

- responsibilities
- prohibited behavior
- allowed dependencies
- forbidden dependencies
- expected inputs and outputs
- implementation size or scope
- completion intent
- status

A contract is not responsible for deciding where the artifact lives.
That belongs to placement rules and the artifact definition.

---

## Core fields

Typical fields include:

- `name`
- `module`
- `role`
- `path`
- `responsibilities`
- `must_not`
- `allowed_dependencies`
- `forbidden_dependencies`
- `inputs` (optional)
- `outputs` (optional)
- `implementation_size` (optional)
- `status`

---

## Example

```yaml
name: create_user
module: user
role: usecase
path: src/application/usecases/create_user.rs

responsibilities:
  - "Execute one application use case"
  - "Coordinate domain behavior"
  - "Persist through an abstract repository boundary"

must_not:
  - "Write SQL directly"
  - "Return transport-specific responses"

allowed_dependencies:
  - "domain"
  - "application"

forbidden_dependencies:
  - "interfaces"
  - "infrastructure"

inputs:
  - "CreateUserCommand"

outputs:
  - "CreateUserResult"

implementation_size: "small"
status: "planned"
```

---

## Relationship to other concepts

A contract belongs to an artifact.

- the project provides context
- the artifact identifies the execution unit
- the contract defines its behavioral boundary
- the prompt turns the contract into AI-ready implementation context

The contract is the most important architectural safeguard in Archflow.

---

## Design principles

A contract should be:

- explicit
- specific
- small enough to be actionable
- understandable by both humans and AI tools
- stable enough to guide implementation
- independent from unnecessary framework details

A good contract should reduce ambiguity, not add it.

---

## What a contract should contain

A good contract should describe:

- what the artifact is responsible for
- what it must not do
- which layer or dependencies it may interact with
- what kind of inputs and outputs are expected
- how large or focused the implementation should be

---

## What a contract should not do

A contract should not:

- turn into a full design document
- include unrelated architectural discussion
- contain detailed source code
- encode every possible implementation decision
- become so vague that it stops constraining behavior

---

## Why it matters

Without contracts, architecture often exists only in:

- diagrams
- docs
- team memory
- review comments

That makes implementation drift likely, especially when using lightweight AI models.

Contracts give Archflow a way to preserve architecture at the artifact level.

They are useful for:

- scaffold generation
- human onboarding
- AI handoff
- review alignment
- future verification

---

## Contract quality guidelines

A strong contract usually has:

- 1 to 5 clear responsibilities
- explicit forbidden behavior
- clear dependency boundaries
- a realistic implementation scope
- language that can be turned into a prompt without major rewriting

A weak contract usually has:

- vague statements
- overlapping responsibilities
- no real constraints
- hidden assumptions
- too much implementation detail

---

## Future directions

In the future, contracts may also support:

- machine-readable validation rules
- required acceptance checks
- optional verification hints
- contract inheritance from templates
- repository-specific policy extensions

---

## 日本語

# Contract

## 概要

Archflow における **Contract** は、artifact の責務と制約の定義です。

contract は次のことを記述します。

- artifact が何をすべきか
- artifact が何をしてはいけないか
- どの依存関係が許可されているか
- どの依存関係が禁止されているか
- どのような入力と出力が期待されるか
- 実装がどの程度集中しているべきか

Contract は Archflow が実装中にアーキテクチャの意図を保全する主な方法です。

---

## 目的

contract の目的は、artifact の振る舞いの境界を記録することです。

contract は次のような質問に答えます。

- この artifact は何をすることを意図しているか？
- この artifact は何をしては絶対にいけないか？
- この artifact はどのような入力と出力を持つか？
- どのような依存関係がアーキテクチャ的に正しいか？
- この artifact の実装はどの程度集中しているべきか？

Contract がなければ、アーキテクチャの意図は計画段階よりも先に保全されません。

---

## 責務

Contract は次のことを定義する責務を持ちます。

- artifact の responsibilities（何をするか）
- must-not ルール（何をしてはいけないか）
- 許可された依存関係
- 禁止された依存関係
- 期待される入力と出力
- 意図された実装サイズとフォーカス
- ライフサイクルステータス

Contract は次の責務を持ちません。

- artifact がどこに配置されるかを定義すること（それは配置ルールに属します）
- artifact のロール分類を定義すること（それはロールと artifact プランに属します）
- prompt コンテンツを直接定義すること（prompt は contract から導出されます）

---

## 例

```yaml
name: create_user
module: user
role: usecase
path: src/application/usecases/create_user.rs

responsibilities:
  - validate the incoming CreateUserCommand
  - create a new User aggregate
  - persist the User through the repository port
  - return a CreateUserResult

must_not:
  - directly access the database
  - handle HTTP or transport concerns
  - contain business logic unrelated to user creation

allowed_dependencies:
  - UserRepository (port)
  - User (entity)
  - CreateUserCommand
  - CreateUserResult

status: planned
```

この例は次のことを示します。

- `create_user` が何をすべきか
- 何をしてはいけないか
- どの依存関係が許可されているか
- そのライフサイクルステータス

---

## Prompt との関係

Contract と prompt は密接に関連していますが、同じではありません。

- **contract** は artifact の境界の権威ある定義です
- **prompt** は contract データを AI 向けの実装指示に変換します

Prompt は contract から導出されます。
Prompt は contract を置き換えません。

---

## 他の概念との関係

contract は複数の他の概念に接続します。

- **project**: contract が解釈されるグローバルコンテキスト
- **module**: contract が属する領域
- **role**: contract のデフォルトを形成するアーキテクチャ分類
- **artifact**: contract が定義するもの
- **placement rule**: contract パスを解決するために使用される
- **contract template**: contract に適用されるロールベースのデフォルト
- **prompt**: contract から導出される AI ハンドオフドキュメント

---

## Contract が Archflow で中心的な理由

Contract は Archflow の操作上の中心です。

なぜなら：

- アーキテクチャの意図を計画段階を超えて保全する
- すべての artifact に対して一貫した境界定義を提供する
- plan と implementation handoff の間のブリッジを確立する
- 将来の verify の基礎を形成する

Contract がなければ、Archflow はスキャフォルドツールに過ぎなくなります。
Contract により、Archflow は artifact レベルで実行可能になります。

---

## 将来の方向性

将来的に、contract は次のものをサポートするかもしれません。

- 機械読み取り可能なバリデーションルール
- 必須の承認チェック
- 任意の verify ヒント
- テンプレートからの contract 継承
- リポジトリ固有のポリシー拡張