# 0016 Define Preset Packaging Approach

- Status: accepted
- Date: 2026-04-02

## Context

ADR-0015 defined the minimal internal model for presets.
The next question is how a preset should be represented in the repository
and consumed by Batonel.

Without a packaging decision:

- contributors may package presets in inconsistent shapes
- project defaults, placement rules, and contract templates may be split across unrelated locations
- preset naming may drift between directory names, user-facing names, and future CLI identifiers
- future implementation may need to support unnecessary indirection too early

This decision must stay aligned with [docs/presets.md](./../presets.md)
and must not overbuild distribution, registry, or remote-install behavior.

This decision builds on:

- [ADR-0005](./0005-examples-precede-presets.md): examples precede presets
- [ADR-0015](./0015-define-minimal-preset-model-for-phase-5.md): minimal preset model
- [ADR-0002](./0002-contract-is-the-source-of-truth-for-artifact-boundaries.md): contract is the source of truth

## Decision

We define presets as **self-contained directories under `presets/`**.

Each preset has one directory:

- `presets/<preset-name>/`

The directory name is the canonical preset identifier used by contributors
and future CLI selection.

### 1. Required preset files

Each preset directory contains these required files:

- `preset.yaml`: preset metadata manifest
- `project.baton.yaml`: project defaults for the preset
- `placement.rules.yaml`: role-to-path defaults for the preset
- `contracts.template.yaml`: role-based contract defaults for the preset
- `README.md`: human-readable explanation of intent and usage

These files make the preset self-describing and operational.

### 2. Optional preset files

A preset directory may also contain:

- `artifacts.plan.yaml`: starter artifact inventory
- `prompts/`: prompt-default assets, if prompt defaults become first-class later
- `examples.md` or metadata references back to source examples

Optional files must remain additive.
They do not change what the core preset package is.

### 3. How preset contents are referenced

In the first packaging model, presets are **directory-contained**, not fragmented.

This means:

- `preset.yaml` identifies which standard files belong to the preset by convention
- `project.baton.yaml`, `placement.rules.yaml`, and `contracts.template.yaml` live inside the same preset directory
- a preset does not point at files under `examples/` as its runtime source
- a preset does not rely on cross-directory composition for its required defaults

This keeps loading simple and keeps presets inspectable as standalone packages.

### 4. Naming conventions

Preset naming follows these rules:

- use lowercase kebab-case for the preset directory name
- keep names architecture- or ecosystem-descriptive, not organization-specific
- keep the manifest `name` aligned with the directory name

Examples:

- `minimal`
- `generic-layered`
- `rust-clean-hexagonal`

Names should be stable enough to appear in documentation and future CLI UX
without alias mapping.

### 5. Explicit non-goals

This decision does not introduce:

- a preset registry
- remote preset installation
- version negotiation between preset packages
- nested preset inheritance
- arbitrary file graph composition across multiple directories

Those concerns may be added later if needed, but they are not part of the first packaging model.

## Consequences

- Contributors have one stable repository shape for preset implementation.
- Batonel can consume presets through simple path-based loading in a future phase.
- Presets remain easy to inspect and review because required files live together.
- Examples can still inspire presets without becoming runtime dependencies of presets.

---

## 日本語

# 0016 preset のパッケージング方針を定義する

- ステータス: 承認済み
- 日付: 2026-04-02

## コンテキスト

ADR-0015 では preset の最小内部モデルを定義しました。
次の問いは、preset をリポジトリ内でどのように表現し、
Batonel がどのように利用するかです。

パッケージング方針がなければ：

- コントリビューターが preset を不整合な形で配置する可能性がある
- project defaults、placement rules、contract templates が無関係な場所に分散する可能性がある
- preset 名がディレクトリ名、ユーザー向け名称、将来の CLI 識別子の間でドリフトする可能性がある
- 将来の実装が不要な間接参照を早期にサポートしなければならなくなる可能性がある

この決定は [docs/presets.md](./../presets.md) と整合していなければならず、
distribution、registry、remote install の振る舞いを過剰に構築してはなりません。

この決定は以下に基づきます：

- [ADR-0005](./0005-examples-precede-presets.md): examples は preset より先に来る
- [ADR-0015](./0015-define-minimal-preset-model-for-phase-5.md): 最小 preset モデル
- [ADR-0002](./0002-contract-is-the-source-of-truth-for-artifact-boundaries.md): contract は真実の源である

## 決定事項

私たちは preset を **`presets/` 配下の自己完結したディレクトリ** として定義します。

各 preset は 1 つのディレクトリを持ちます。

- `presets/<preset-name>/`

このディレクトリ名は、コントリビューターと将来の CLI 選択で使われる
正規の preset 識別子です。

### 1. 必須の preset ファイル

各 preset ディレクトリは次の必須ファイルを含みます。

- `preset.yaml`: preset metadata manifest
- `project.baton.yaml`: preset の project defaults
- `placement.rules.yaml`: preset の role-to-path defaults
- `contracts.template.yaml`: preset のロールベース contract defaults
- `README.md`: 意図と使い方を説明する人間向けドキュメント

これらのファイルにより、preset は自己記述的かつ運用可能になります。

### 2. 任意の preset ファイル

preset ディレクトリは次を含んでもよいです。

- `artifacts.plan.yaml`: starter artifact inventory
- `prompts/`: 将来 prompt defaults がファーストクラスになった場合の prompt-default assets
- `examples.md` または source example へのメタデータ参照

任意ファイルは追加的なものに留めなければなりません。
core な preset package の定義自体は変えません。

### 3. preset 内容の参照方法

最初の packaging モデルでは、preset は **ディレクトリ内包型** であり、分割型ではありません。

これは次を意味します。

- `preset.yaml` はどの標準ファイルが preset に属するかを慣習で示す
- `project.baton.yaml`、`placement.rules.yaml`、`contracts.template.yaml` は同じ preset ディレクトリ内に置かれる
- preset は `examples/` 配下のファイルを実行時の source として参照しない
- preset は必須デフォルトのためにディレクトリ横断の構成に依存しない

これにより、読み込みは単純になり、preset は独立したパッケージとして検査しやすくなります。

### 4. 命名規則

preset の命名は次のルールに従います。

- preset ディレクトリ名は lowercase kebab-case を使う
- 名前は organization 固有ではなく、architecture または ecosystem を説明するものにする
- manifest の `name` はディレクトリ名と一致させる

例：

- `minimal`
- `generic-layered`
- `rust-clean-hexagonal`

名前は、別名マッピングなしでドキュメントや将来の CLI UX に現れてもよい程度に安定しているべきです。

### 5. 明示的な非目標

この決定では次を導入しません。

- preset registry
- remote preset installation
- preset package 間の version negotiation
- nested preset inheritance
- 複数ディレクトリにまたがる任意のファイルグラフ構成

これらは必要であれば将来追加できますが、最初の packaging モデルには含まれません。

## 結果

- コントリビューターは preset 実装のための安定したリポジトリ形状を持てる。
- 将来のフェーズで Batonel は単純な path-based loading によって preset を利用できる。
- 必須ファイルが同じ場所にあるため、preset は検査・レビューしやすいままでいられる。
- examples は preset の着想源であり続けられるが、preset の runtime dependency にはならない。
