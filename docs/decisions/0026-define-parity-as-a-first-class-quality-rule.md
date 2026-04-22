# 0026 Define Parity as a First-Class Quality Rule

- Status: accepted
- Date: 2026-04-22

## Context

Batonel's repository complexity is growing. It now maintains:

- internal schema definitions (`schemas/`)
- multiple teaching examples (`examples/`)
- supported reusable presets (`presets/`)
- expected scaffolding output for examples (`examples/**/expected/...`)
- generated AI prompts (`sync_example_prompts.py`)

If these assets drift apart, user trust is broken. Examples might teach configurations that the schemas reject, or presets might diverge from the examples they were supposedly derived from.

## Decision

We define "Parity" as a first-class quality rule in Batonel. 
Parity is the strict, machine-enforceable alignment between related assets.

We establish three core parity domains:

1. **Example-to-Preset Parity**: Common configuration files (`project.baton.yaml`, `placement.rules.yaml`, `contracts.template.yaml`, `artifacts.plan.yaml`, `guard.sidecar.yaml`, `policy.profile.yaml`) inside a supported preset package must strictly match their upstream example defaults byte-for-byte.
2. **Output Parity**: Checked-in expected outputs (such as AI prompts in `examples/**/expected/.batonel/prompts/`) must perfectly match the output of their generator scripts (e.g., `sync_example_prompts.py`).
3. **Schema Parity**: Schema metadata versions declared inside yaml configuration files must align with versions published in `schemas/`.

These rules will be enforced in CI. If a PR breaks parity, it must be fixed by updating all sides of the relationship (e.g. running generator scripts, or copying the example file to the preset).

## Consequences

- The concept of Parity becomes a standard architectural requirement for contributors.
- Maintenance overhead increases slightly because every change to an example default requires a sync to its derived preset.
- The project gains resilience against configuration drift, ensuring that Batonel works properly for new users straight out of the box.

---

## 日本語

# 0026 Parity(等価性)を第一級の品質ルールとして定義する

- ステータス: 承認済み
- 日付: 2026-04-22

## コンテキスト

Batonel リポジトリの複雑さは増しています。現在、以下の要素を維持しています。

- 内部スキーマ定義 (`schemas/`)
- 複数の学習用 example (`examples/`)
- サポートされる再利用可能な preset (`presets/`)
- example 用の期待されるスキャフォルディング出力 (`examples/**/expected/...`)
- 生成された AI プロンプト (`sync_example_prompts.py`)

これらの資産に差異が生じると、ユーザーの信頼が損なわれます。example がスキーマで拒否される構成を教えてしまったり、preset がそれが派生したはずの example から乖離してしまう可能性があります。

## 決定事項

私たちは、「Parity（パリティ・等価性）」を Batonel における第一級の品質ルールとして定義します。
Parity とは、関連する資産間の厳格で、機械的に適用可能な一致のことです。

3 つのコアとなる Parity ドメインを確立します。

1. **Example と Preset の Parity**: サポートされる preset パッケージ内の共通設定ファイル（`project.baton.yaml`、`placement.rules.yaml`、`contracts.template.yaml`、`artifacts.plan.yaml`、`guard.sidecar.yaml`、`policy.profile.yaml`）は、上流の example のデフォルトと完全にバイト単位で一致していなければなりません。
2. **Output の Parity**: チェックインされている期待される出力（`examples/**/expected/.batonel/prompts/`内の AI プロンプトなど）は、生成スクリプト（例：`sync_example_prompts.py`）の出力と完全に一致していなければなりません。
3. **Schema の Parity**: YAML 設定ファイル内で宣言されているスキーマのメタデータバージョンは、`schemas/` で公開されているバージョンと整合していなければなりません。

これらのルールは CI で強制されます。PR が Parity を壊す場合、関係する全面を更新すること（生成スクリプトの再実行や、example ファイルの preset へのコピーなど）で修正しなければなりません。

## 結果

- Parity の概念は、コントリビューターにとっての標準的なアーキテクチャ要件になります。
- example のデフォルトに対するすべての変更で派生した preset への同期が必要になるため、メンテナンスのオーバーヘッドはわずかに増加します。
- プロジェクトは設定の乖離に対する回復力を得て、新しいユーザーが Batonel をすぐに正しく使用できることを保証します。
