# Architecture Flow

This document explains how Archflow works as a flow from architectural intent
to implementation scaffolding and AI handoff.

The goal of this document is to show:

- what Archflow takes as input
- how Archflow interprets that input
- what Archflow produces as output
- how its core concepts connect together

This is a conceptual flow description, not an implementation spec.

---

## Overview

Archflow is an architecture-to-execution bridge.

It starts with structured architectural definitions and turns them into:

- planned artifact placement
- artifact-level contracts
- AI handoff prompts
- scaffolded implementation units
- future verification targets

The central idea is simple:

**design first, resolve structure, define boundaries, then hand off implementation**

---

## High-level flow

At a high level, Archflow follows this sequence:

1. define project context
2. define placement rules
3. define planned artifacts
4. apply contract templates
5. resolve artifact paths
6. generate artifact contracts
7. generate AI handoff prompts
8. scaffold implementation structure
9. verify consistency over time

This flow is meant to preserve architectural intent before and during implementation.

---

## Flow inputs

Archflow begins from structured input files.

The main input files are:

- `project.arch.yaml`
- `placement.rules.yaml`
- `artifacts.plan.yaml`
- `contracts.template.yaml` (optional but recommended)

These files define the architecture before code is written.

### 1. Project context

The project definition establishes the architectural frame.

It tells Archflow:

- what kind of project this is
- which architectural style it follows
- which modules exist
- what language orientation is assumed

Without project context, the rest of the model has no stable frame.

### 2. Placement rules

Placement rules define where artifacts of each role should live.

They tell Archflow:

- where `entity` artifacts belong
- where `usecase` artifacts belong
- where `controller` artifacts belong
- what file extension or naming pattern may apply

Without placement rules, artifacts cannot be resolved into structure.

### 3. Artifact plan

The artifact plan defines what should exist.

It tells Archflow:

- which artifacts to prepare
- which module each artifact belongs to
- which role each artifact has
- optional inputs and outputs

Without an artifact plan, Archflow has no execution units to work with.

### 4. Contract templates

Contract templates provide reusable defaults by role.

They tell Archflow:

- what responsibilities are typical for a role
- what behaviors are forbidden for a role
- what dependency boundaries usually apply

Without templates, contracts can still exist, but consistency becomes weaker.

---

## Core interpretation flow

Once the input files are available, Archflow interprets them in layers.

### Step 1. Read project context

Archflow first reads the project definition.

This establishes the global context in which all later interpretation happens.

At this stage, Archflow identifies:

- architecture style
- module space
- language orientation
- optional workspace or structural settings

### Step 2. Load placement rules

Next, Archflow loads placement rules and creates a role-to-path map.

At this stage, Archflow can answer:

- if an artifact is a `usecase`, where should it go?
- if an artifact is a `repository_port`, where should it go?
- if an artifact is a `controller`, where should it go?

This step resolves structure intent, but not behavioral intent.

### Step 3. Load artifact plan

Then Archflow loads the artifact list.

At this stage, Archflow knows the concrete implementation units that should exist.

Each artifact becomes an execution target with:

- name
- module
- role
- optional inputs
- optional outputs
- optional status

This is the point where architecture becomes actionable.

### Step 4. Resolve artifact paths

Using placement rules and artifact roles, Archflow resolves where each artifact should live.

For example:

- `entity` -> `src/domain/entities/`
- `usecase` -> `src/application/usecases/`
- `controller` -> `src/interfaces/controllers/`

If an artifact defines an explicit path override, that may replace the default mapping.

At this stage, Archflow can determine the expected scaffold location of every artifact.

### Step 5. Apply contract templates

If contract templates exist, Archflow applies role-based defaults to each artifact.

This fills in common defaults such as:

- responsibilities
- forbidden behaviors
- allowed dependencies
- forbidden dependencies
- implementation size

This step creates the first draft of artifact boundaries.

### Step 6. Produce artifact contracts

Archflow then turns artifact identity + resolved path + template defaults
into artifact-specific contracts.

A contract represents the behavioral boundary of one artifact.

It typically includes:

- artifact identity
- role
- module
- resolved path
- responsibilities
- must-not rules
- dependency boundaries
- inputs and outputs
- implementation scope
- status

At this stage, architecture is no longer only structural.
It becomes executable at the artifact level.

### Step 7. Generate prompts

From each artifact contract, Archflow generates AI handoff prompts.

A prompt packages the contract into an implementation-oriented format.

It usually contains:

- the target artifact
- its role
- its module
- its responsibilities
- its constraints
- its inputs and outputs
- completion criteria

This step makes the architecture directly usable by AI coding tools.

### Step 8. Generate scaffold output

Archflow can then scaffold the project structure.

This may include:

- directories
- placeholder implementation files
- contract files
- prompt files
- metadata files

At this point, implementation can begin with clear boundaries.

---

## Output model

Archflow produces outputs in several layers.

### Structural outputs

These describe where things should live.

Examples:
- resolved file paths
- generated directories
- scaffolded placeholder files

### Contract outputs

These describe what each artifact is supposed to do.

Examples:
- `create_user.contract.yaml`
- `user.contract.yaml`

### Prompt outputs

These describe how an artifact should be handed to an AI system.

Examples:
- `create_user.prompt.md`
- `user.prompt.md`

### Verification targets

These describe what should remain true over time.

Examples:
- required contract presence
- role-to-path consistency
- status consistency
- future dependency or code-aware checks

---

## Conceptual dependency chain

The internal dependency chain of Archflow looks like this:

```text
project
  -> placement rules
  -> artifact plan
     -> resolved artifact path
     -> contract template application
        -> artifact contract
           -> prompt
           -> scaffold
           -> verify target
```

This means:

- the project defines the frame
- placement rules define location
- artifacts define units of work
- contracts define boundaries
- prompts define AI handoff
- scaffold and verify operationalize the result

---

## Why this flow matters

Many tools help with one part of this process.

Some tools help define specs.
Some tools help instruct AI systems.
Some tools help lint code after implementation.

Archflow is focused on the flow in between.

Its main value is preserving architectural intent across the transition from:

- design
- to structure
- to artifact definition
- to implementation handoff

This is especially important when using lightweight models,
because smaller models need tighter boundaries and clearer context.

---

## Example flow

A simple example may look like this:

1. define a project with one `user` module
2. define `entity` and `usecase` placement rules
3. define two artifacts:
  - `user`
  - `create_user`
4. apply role-based defaults from contract templates
5. resolve paths:
  - `user` -> `src/domain/entities/user.rs`
  - `create_user` -> `src/application/usecases/create_user.rs`
6. generate:
  - `user.contract.yaml`
  - `create_user.contract.yaml`
7. generate:
  - `user.prompt.md`
  - `create_user.prompt.md`
8. scaffold placeholder implementation files

This turns architecture into concrete implementation context.

---

## Design principles behind the flow

Archflow’s flow is built on several principles.

### Architecture before implementation

Structure and responsibility should be defined before code generation begins.

### Artifact-level execution

The useful unit of implementation is not the whole repository.
It is the artifact.

### Contracts as architectural memory

Architecture should not live only in diagrams or team memory.
Contracts preserve it at the artifact level.

### Prompts are derived, not primary

The prompt is not the source of truth.
The contract is.

### Sidecar-first design

Important architectural data should remain usable even before full code exists.

---

## What this flow does not assume

Archflow does not require:

- full code parsing
- framework-specific assumptions
- one fixed architecture style
- one programming language
- one AI vendor or tool

The flow is meant to stay useful even in early design phases.

---

## Future evolution of the flow

In the future, this flow may extend to include:

- schema validation
- preset expansion
- project import from existing repositories
- optional lightweight code-aware checks
- editor integration
- CI verification pipelines
- role-specific prompt variants

Even as those grow, the core flow remains the same:

**project** -> **structure** -> **artifact** -> **contract** -> **prompt** -> **scaffold** -> **verify**

---

## Summary

Archflow works by turning architecture into progressively more executable forms.

It starts with project-level intent,
resolves that into structure,
turns structure into artifact boundaries,
turns those boundaries into contracts,
and turns contracts into implementation handoff.

If you remember only one thing, remember this:

**Archflow does not start from code.
It starts from architecture and turns it into executable implementation context.**

---

## 日本語

このドキュメントは、Archflow がアーキテクチャの意図から実装の骨組みおよび AI へのハンドオフへと至るフローとして、どのように機能するかを説明します。

このドキュメントの目的は、次の点を示すことです。

- Archflow が入力として何を受け取るか
- Archflow がその入力をどのように解釈するか
- Archflow が出力として何を生成するか
- コアとなる概念がどのようにつながっているか

これは概念的なフローの説明であり、実装仕様ではありません。

---

### 概要

Archflow は、設計から実装への橋渡しです。

構造化されたアーキテクチャ定義から始まり、次のものへと変換します。

- 計画された artifact の配置
- artifact レベルの contract
- AI へのハンドオフ prompt
- スキャフォルドされた実装ユニット
- 将来の verify 対象

中心的なアイデアはシンプルです。

**設計を先に行い、構造を解決し、境界を定義し、実装をハンドオフする**

---

### 高レベルのフロー

Archflow は以下のシーケンスで動作します。

1. プロジェクトコンテキストを定義する
2. 配置ルールを定義する
3. 計画された artifact を定義する
4. contract テンプレートを適用する
5. artifact のパスを解決する
6. artifact contract を生成する
7. AI ハンドオフ prompt を生成する
8. 実装構造をスキャフォルドする
9. 時間をかけて整合性を verify する

このフローは、実装前・実装中のアーキテクチャの意図を保全するためのものです。

---

### フローの入力

Archflow は構造化された入力ファイルから始まります。

主な入力ファイルは次のとおりです。

- `project.arch.yaml`
- `placement.rules.yaml`
- `artifacts.plan.yaml`
- `contracts.template.yaml`（任意だが推奨）

これらのファイルは、コードが書かれる前にアーキテクチャを定義します。

#### 1. プロジェクトコンテキスト

プロジェクト定義は、アーキテクチャの枠組みを確立します。

Archflow に伝えることは次のとおりです。

- このプロジェクトがどのような種類のものか
- どのアーキテクチャスタイルに従うか
- どのモジュールが存在するか
- どの言語を想定しているか

プロジェクトコンテキストがなければ、残りのモデルには安定した枠組みがありません。

#### 2. 配置ルール

配置ルールは、各ロールの artifact がどこに配置されるべきかを定義します。

Archflow に伝えることは次のとおりです。

- `entity` artifact はどこに属するか
- `usecase` artifact はどこに属するか
- `controller` artifact はどこに属するか
- どのファイル拡張子や命名パターンが適用されるか

配置ルールがなければ、artifact を構造に解決することができません。

#### 3. Artifact プラン

Artifact プランは、何が存在すべきかを定義します。

Archflow に伝えることは次のとおりです。

- どの artifact を準備するか
- 各 artifact がどのモジュールに属するか
- 各 artifact がどのロールを持つか
- 任意の入力と出力

Artifact プランがなければ、Archflow には対応する実行ユニットがありません。

#### 4. Contract テンプレート

Contract テンプレートは、ロール別の再利用可能なデフォルトを提供します。

Archflow に伝えることは次のとおりです。

- あるロールにとって典型的な責務は何か
- あるロールで禁止されている振る舞いは何か
- 通常どのような依存境界が適用されるか

テンプレートがなくても contract は存在できますが、一貫性は弱くなります。

---

### コアの解釈フロー

入力ファイルが揃ったら、Archflow はそれらを層ごとに解釈します。

#### Step 1. プロジェクトコンテキストを読み込む

Archflow はまずプロジェクト定義を読み込みます。

これにより、その後のすべての解釈が行われるグローバルコンテキストが確立されます。

この段階で Archflow は以下を特定します。

- アーキテクチャスタイル
- モジュール空間
- 言語の方向性
- 任意のワークスペースまたは構造設定

#### Step 2. 配置ルールを読み込む

次に、Archflow は配置ルールを読み込み、ロール-パスのマップを作成します。

この段階で Archflow が答えられることは次のとおりです。

- artifact が `usecase` の場合、どこに配置すべきか
- artifact が `repository_port` の場合、どこに配置すべきか
- artifact が `controller` の場合、どこに配置すべきか

このステップは構造の意図を解決しますが、振る舞いの意図は解決しません。

#### Step 3. artifact プランを読み込む

次に Archflow は artifact リストを読み込みます。

この段階で Archflow は、存在すべき具体的な実装ユニットを把握します。

各 artifact は以下を持つ実行ターゲットになります。

- 名前
- モジュール
- ロール
- 任意の入力
- 任意の出力
- 任意のステータス

ここでアーキテクチャが実行可能になります。

#### Step 4. artifact のパスを解決する

配置ルールと artifact のロールを使用して、Archflow は各 artifact がどこに配置されるべきかを解決します。

例：

- `entity` → `src/domain/entities/`
- `usecase` → `src/application/usecases/`
- `controller` → `src/interfaces/controllers/`

artifact が明示的なパスオーバーライドを定義している場合、デフォルトのマッピングを置き換えることができます。

この段階で、すべての artifact の期待されるスキャフォルド場所を決定できます。

#### Step 5. contract テンプレートを適用する

contract テンプレートが存在する場合、Archflow は各 artifact にロールベースのデフォルトを適用します。

これにより、次のような一般的なデフォルトが埋め込まれます。

- 責務
- 禁止された振る舞い
- 許可された依存関係
- 禁止された依存関係
- 実装サイズ

このステップで、artifact の境界の最初のドラフトが作成されます。

#### Step 6. artifact contract を生成する

Archflow は artifact のアイデンティティ + 解決されたパス + テンプレートのデフォルトを、artifact 固有の contract に変換します。

contract は 1 つの artifact の振る舞いの境界を表します。

通常含まれるものは次のとおりです。

- artifact のアイデンティティ
- ロール
- モジュール
- 解決されたパス
- 責務
- must-not ルール
- 依存境界
- 入力と出力
- 実装スコープ
- ステータス

この段階で、アーキテクチャはもはや構造だけではありません。
artifact レベルで実行可能になります。

#### Step 7. prompt を生成する

各 artifact contract から、Archflow は AI ハンドオフ prompt を生成します。

prompt は contract を実装指向の形式にパッケージします。

通常含まれるものは次のとおりです。

- 対象となる artifact
- そのロール
- そのモジュール
- その責務
- その制約
- その入力と出力
- 完了基準

このステップにより、アーキテクチャが AI コーディングツールによって直接利用可能になります。

#### Step 8. スキャフォルドを生成する

Archflow はプロジェクト構造をスキャフォルドできます。

含まれる可能性があるものは次のとおりです。

- ディレクトリ
- 仮の実装ファイル
- contract ファイル
- prompt ファイル
- メタデータファイル

この時点で、明確な境界を持って実装を開始できます。

---

### 出力モデル

Archflow はいくつかの層で出力を生成します。

#### 構造的な出力

物事がどこに配置されるべきかを記述します。

例：
- 解決されたファイルパス
- 生成されたディレクトリ
- スキャフォルドされた仮ファイル

#### Contract の出力

各 artifact が何をすべきかを記述します。

例：
- `create_user.contract.yaml`
- `user.contract.yaml`

#### Prompt の出力

artifact を AI システムにどのようにハンドオフするかを記述します。

例：
- `create_user.prompt.md`
- `user.prompt.md`

#### Verify 対象

時間をかけて真であり続けるべきことを記述します。

例：
- 必要な contract の存在
- ロール-パスの整合性
- ステータスの整合性
- 将来の依存関係またはコード認識チェック

---

### 概念的な依存チェーン

Archflow の内部依存チェーンは次のようになります。

```text
project
  -> placement rules
  -> artifact plan
     -> resolved artifact path
     -> contract template application
        -> artifact contract
           -> prompt
           -> scaffold
           -> verify target
```

これが意味することは次のとおりです。

- project は枠組みを定義する
- placement rules は場所を定義する
- artifacts は作業単位を定義する
- contracts は境界を定義する
- prompts は AI へのハンドオフを定義する
- scaffold と verify は結果を実用化する

---

### このフローが重要な理由

多くのツールはこのプロセスの一部を支援します。

仕様を定義するのに役立つツール、
AI システムへの指示に役立つツール、
実装後のコードを lint するツール、

などがあります。

Archflow はその中間のフローに焦点を当てています。

その主な価値は、次の遷移を通じてアーキテクチャの意図を保全することです。

- 設計
- 構造
- artifact の定義
- 実装のハンドオフ

これは軽量モデルを使用する場合に特に重要です。
なぜなら、小さなモデルはより厳密な境界と明確なコンテキストを必要とするからです。

---

### フロー例

シンプルな例として次のようなものが考えられます。

1. 1 つの `user` モジュールを持つプロジェクトを定義する
2. `entity` と `usecase` の配置ルールを定義する
3. 2 つの artifact を定義する：
   - `user`
   - `create_user`
4. contract テンプレートからロールベースのデフォルトを適用する
5. パスを解決する：
   - `user` → `src/domain/entities/user.rs`
   - `create_user` → `src/application/usecases/create_user.rs`
6. 生成する：
   - `user.contract.yaml`
   - `create_user.contract.yaml`
7. 生成する：
   - `user.prompt.md`
   - `create_user.prompt.md`
8. 仮の実装ファイルをスキャフォルドする

これにより、アーキテクチャが具体的な実装コンテキストに変換されます。

---

### フローの背後にある設計原則

Archflow のフローはいくつかの原則に基づいています。

#### 実装の前にアーキテクチャを

コード生成が始まる前に、構造と責務を定義すべきです。

#### Artifact レベルの実行

実装の有用な単位は、リポジトリ全体ではありません。
artifact です。

#### アーキテクチャの記憶としての contract

アーキテクチャは図やチームの記憶の中だけに存在すべきではありません。
Contract は artifact レベルでアーキテクチャを保全します。

#### Prompt は導出されるものであり、主体ではない

Prompt は真実の源ではありません。
Contract がそれです。

#### Sidecar ファイル優先の設計

重要なアーキテクチャデータは、完全なコードが存在する前でも使用可能であるべきです。

---

### このフローが前提としないこと

Archflow は以下を必要としません。

- 完全なコードの解析
- フレームワーク固有の仮定
- 1 つの固定されたアーキテクチャスタイル
- 1 つのプログラミング言語
- 1 つの AI ベンダーまたはツール

このフローは、初期設計フェーズでも有用であり続けることを目指しています。

---

### フローの将来の発展

将来的に、このフローは次のものを含むように拡張される可能性があります。

- スキーマバリデーション
- プリセットの展開
- 既存リポジトリからのプロジェクトインポート
- 任意の軽量コード認識チェック
- エディタ統合
- CI verify パイプライン
- ロール固有の prompt バリアント

それらが成長しても、コアフローは変わりません。

**project** → **structure** → **artifact** → **contract** → **prompt** → **scaffold** → **verify**

---

### まとめ

Archflow は、アーキテクチャを段階的により実行可能な形式に変換することで機能します。

プロジェクトレベルの意図から始まり、
それを構造に解決し、
構造を artifact の境界に変換し、
それらの境界を contract に変換し、
contract を実装のハンドオフに変換します。

1 つだけ覚えておくなら、これを覚えてください。

**Archflow はコードから始まらない。
アーキテクチャから始まり、それを実行可能な実装コンテキストに変換する。**