# 0020 Align Presets with Examples, Docs, and Schemas

- Status: accepted
- Date: 2026-04-03

## Context

The first preset implementations now exist:

- `presets/generic-layered/`
- `presets/rust-clean-hexagonal/`

At this stage, presets must remain an extension of existing Batonel assets,
not a disconnected parallel track.

Alignment is required across:

- mapped examples (`examples/*`)
- preset documentation (`docs/presets.md`, onboarding docs)
- schema assumptions for generated root config files

## Decision

### 1. Naming and mapping alignment

Preset ids remain aligned with the established example-to-preset mapping:

- `generic-layered` <-> `examples/generic-layered`
- `rust-clean-hexagonal` <-> `examples/rust-clean-hexagonal`

Naming uses lowercase kebab-case and stays stable for CLI selection.

### 2. Content alignment with example defaults

Preset defaults should mirror their mapped example defaults unless an explicit
decision records a divergence.

For current presets:

- role sets and role semantics align with mapped examples
- default path structures align with mapped examples
- default file extensions align with mapped examples

### 3. Schema alignment expectations

Preset-packaged files consumed by `batonel init --preset` must remain compatible
with current schema assumptions for root config files:

- `project.baton.yaml` -> project schema expectations
- `placement.rules.yaml` -> placement rules schema expectations
- `contracts.template.yaml` -> contracts template schema expectations
- optional `artifacts.plan.yaml` -> artifacts plan schema expectations

Presets should not introduce fields that bypass the existing architecture model.

### 4. Drift handling

If drift is found between preset defaults and mapped examples/docs/schema assumptions,
the fix should prefer restoring alignment in preset content and documentation.

Any intentional divergence should be explicitly documented in a new decision.

## Consequences

- Presets stay understandable as an extension of the current Batonel model.
- Contributors can reason about presets using existing examples and schemas.
- Preset adoption remains concrete without fragmenting project semantics.

---

## 日本語

# 0020 preset を examples・docs・schema と整合させる

- ステータス: 承認済み
- 日付: 2026-04-03

## コンテキスト

最初の preset 実装が存在するようになりました。

- `presets/generic-layered/`
- `presets/rust-clean-hexagonal/`

この段階では、preset は既存 Batonel 資産の拡張であるべきで、
分離した並行トラックになってはいけません。

整合が必要な対象:

- 対応 example（`examples/*`）
- preset ドキュメント（`docs/presets.md`、onboarding docs）
- 生成 root config ファイルに対する schema 前提

## 決定事項

### 1. 命名とマッピングの整合

preset id は既存の example-to-preset マッピングに整合させます。

- `generic-layered` <-> `examples/generic-layered`
- `rust-clean-hexagonal` <-> `examples/rust-clean-hexagonal`

命名は lowercase kebab-case を用い、CLI 選択のために安定性を保ちます。

### 2. example デフォルトとの内容整合

preset デフォルトは、明示的な決定による差分がない限り、
対応 example のデフォルトと一致させます。

現在の preset では次を一致させます。

- role セットと role 意味
- デフォルト path 構造
- デフォルト file extension

### 3. schema 前提との整合

`batonel init --preset` で利用される preset パッケージファイルは、
現在の root config 用 schema 前提と互換でなければなりません。

- `project.baton.yaml` -> project schema 前提
- `placement.rules.yaml` -> placement rules schema 前提
- `contracts.template.yaml` -> contracts template schema 前提
- optional `artifacts.plan.yaml` -> artifacts plan schema 前提

preset は既存のアーキテクチャモデルをバイパスするフィールドを導入しないこと。

### 4. ドリフトの扱い

preset デフォルトと対応 example/docs/schema 前提の間でドリフトが見つかった場合、
修正は preset 内容とドキュメントの再整合を優先します。

意図的な差分は、新しい decision として明示的に記録します。

## 結果

- preset は現在の Batonel モデルの拡張として理解しやすくなる。
- コントリビューターは既存 example と schema を使って preset を解釈できる。
- preset 採用の具体性を保ちつつ、意味論の分断を防げる。
