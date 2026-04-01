# 0015 Define Minimal Preset Model for Phase 5

- Status: accepted
- Date: 2026-04-02

## Context

Phase 5 introduces presets as reusable starting packages.
The repository already has examples and a preset direction in
[docs/presets.md](./../presets.md), but the internal preset shape is still too loose.

Without a minimal model:

- examples and presets may drift toward meaning the same thing
- future preset implementation may package too much unstable behavior
- contributors may treat a preset as a full project framework instead of a reusable starting point
- preset work may become inconsistent across architectures and ecosystems

This decision builds on:

- [ADR-0005](./0005-examples-precede-presets.md): examples precede presets
- [ADR-0002](./0002-contract-is-the-source-of-truth-for-artifact-boundaries.md): contract is the source of truth
- [ADR-0004](./0004-sidecar-files-are-first-class.md): sidecar files are first-class

## Decision

We define a preset as a **reusable starting package of architectural defaults**.

A preset is operational, but minimal.
It is not a full application framework, and it is not a replacement for examples.

### 1. What a preset contains

A preset has two layers.

#### A. Preset metadata

The minimal metadata for a preset is:

- `name`: stable preset identifier
- `summary`: short human-readable description
- `architecture_style`: the architectural style the preset represents
- `ecosystem`: intended language or ecosystem target, or `generic` when not tied to one ecosystem

#### B. Reusable defaults

The minimal reusable content for a preset is:

- project defaults
- placement rules
- contract templates by role

These are the smallest reusable assets that let a preset provide a meaningful starting point
without becoming a full project framework.

### 2. What a preset may optionally contain

A preset may later include:

- a starter artifact plan
- prompt defaults
- example references or provenance

These are optional additions.
They are not required to recognize something as a preset.

### 3. What a preset is not

A preset is not:

- a complete generated project
- a frozen architecture truth
- a replacement for contract files
- a replacement for examples
- a packaging format for runtime dependencies, build pipelines, or framework internals

Preset implementation must stay focused on reusable architectural starting defaults.

### 4. How a preset differs from an example

- an **example** teaches the model through a concrete, inspectable case
- a **preset** starts a new project through reusable defaults

Examples are descriptive.
Presets are operational.

An example may become the basis for a preset only after its naming, role conventions,
and structure prove stable enough to reuse.

## Consequences

- Preset implementation has a clear model boundary before file formats or CLI flows are introduced.
- Contributors have a stable minimum shape for preset work across architectures.
- The repository can introduce presets gradually without collapsing them into examples.
- Presets remain aligned with contract-first and sidecar-first principles rather than becoming project generators.

---

## 日本語

# 0015 Phase 5 に向けた最小 preset モデルの定義

- ステータス: 承認済み
- 日付: 2026-04-02

## コンテキスト

Phase 5 では、preset を再利用可能な出発点パッケージとして導入します。
リポジトリにはすでに examples と [docs/presets.md](./../presets.md) に preset の方向性がありますが、
内部的な preset の形はまだ緩すぎます。

最小モデルがなければ：

- example と preset が同じ意味にドリフトする可能性がある
- 将来の preset 実装が不安定な振る舞いを過剰にパッケージ化する可能性がある
- コントリビューターが preset を再利用可能な出発点ではなく完全なプロジェクトフレームワークとして扱う可能性がある
- アーキテクチャやエコシステムごとの preset 作業が不整合になる可能性がある

この決定は以下に基づきます：

- [ADR-0005](./0005-examples-precede-presets.md): examples は preset より先に来る
- [ADR-0002](./0002-contract-is-the-source-of-truth-for-artifact-boundaries.md): contract は真実の源である
- [ADR-0004](./0004-sidecar-files-are-first-class.md): sidecar ファイルはファーストクラスである

## 決定事項

私たちは preset を **アーキテクチャ上のデフォルトを再利用可能にまとめた出発点パッケージ** と定義します。

preset は運用的ですが、最小限であるべきです。
完全なアプリケーションフレームワークではなく、example の置き換えでもありません。

### 1. preset に含まれるもの

preset は 2 層で構成されます。

#### A. Preset メタデータ

preset に必要な最小メタデータは次のとおりです。

- `name`: 安定した preset 識別子
- `summary`: 人間が読める短い説明
- `architecture_style`: その preset が表すアーキテクチャスタイル
- `ecosystem`: 想定する言語またはエコシステム。特定しない場合は `generic`

#### B. 再利用可能なデフォルト

preset に必要な最小の再利用コンテンツは次のとおりです。

- project defaults
- placement rules
- ロールごとの contract templates

これらは、preset が完全なプロジェクトフレームワークにならずに
意味のある出発点を提供するための最小資産です。

### 2. preset が任意で含めてもよいもの

preset は将来的に次を含んでもよいです。

- starter artifact plan
- prompt defaults
- example への参照や由来情報

これらは任意の追加です。
preset として認識するための必須条件ではありません。

### 3. preset ではないもの

preset は次のものではありません。

- 完全に生成されたプロジェクト
- 固定されたアーキテクチャの真実
- contract ファイルの置き換え
- example の置き換え
- ランタイム依存、ビルドパイプライン、フレームワーク内部をパッケージ化する形式

preset 実装は、再利用可能なアーキテクチャ上の出発点デフォルトに集中しなければなりません。

### 4. preset と example の違い

- **example** は具体的で観察可能なケースを通じてモデルを教える
- **preset** は再利用可能なデフォルトを通じて新しいプロジェクトを始める

Examples は記述的です。
Presets は運用的です。

example が preset の基盤になるのは、命名、ロール慣習、構造が
再利用できるほど安定した後に限られます。

## 結果

- preset 実装は、ファイル形式や CLI フローを導入する前に明確なモデル境界を持てる。
- コントリビューターは、アーキテクチャをまたいで一貫した最小 shape を共有できる。
- リポジトリは、examples と混同せずに preset を段階的に導入できる。
- preset は project generator になるのではなく、contract-first と sidecar-first の原則に整合したままでいられる。