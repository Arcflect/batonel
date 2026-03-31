# 0009-minimal-cli-error-handling-policy-for-phase-3

- Status: accepted
- Date: 2026-03-31

## Context

As part of Phase 3 (AI Handoff Layer), the Archflow CLI must handle specific failures related to contract loading, parsing, and prompt generation. This policy extends [ADR-0008](./0008-minimal-cli-error-handling-policy-for-phase-2.md) to the prompt generation workflow.

The objective is to maintain a predictable, "Fail Fast" behavior for configuration errors while ensuring the tool remains flexible enough to handle custom extension points (Roles) without unnecessary crashes.

## Decision

We will adopt the following error handling behaviors for Phase 3:

### 1. Missing Contract Behavior
If a user requests a prompt for an artifact but the corresponding `.contract.yaml` is not found at the resolved path:
- **Policy:** **Fail Fast**. Output `Error: Configuration file not found at [PATH]` and exit with code `1`.
- **Reasoning:** Since prompts are derived from contracts, a missing contract means we have no authoritative truth to hand off. Silent guesses are forbidden.

### 2. Invalid Contract Schema Behavior
If a `.contract.yaml` exists but is malformed or missing required fields (`name`, `role`, `module`):
- **Policy:** **Fail Fast**. Output the specific parsing error from `serde-yaml` and exit with code `1`.

### 3. Unsupported Role Behavior
If an artifact has a `role` value that is not recognized by the internal role-aware prompt mapper (e.g., a custom role like `service_layer`):
- **Policy:** **Graceful Fallback**. Generate a "generic" prompt containing default architectural constraints instead of an error.
- **Reasoning:** Archflow is designed to be extensible. Users should be able to define custom roles and still get a functional (if un-optimized) prompt without the tool crashing.

### 4. Output Mode Failure
If an invalid `--mode` is provided via CLI:
- **Policy:** **System-Level Failure**. Handled automatically by `clap`'s `ValueEnum` verification prior to command execution.

## Consequences

- **Consistency**: Users receive the same "Fail Fast" behavior across both Phase 2 and Phase 3 for missing files or bad configuration syntax.
- **Extensibility**: The "Graceful Fallback" for roles allows the system to remain useful even for projects with bespoke architectural terminology.

---

## 日本語

# 0009-minimal-cli-error-handling-policy-for-phase-3

- ステータス: 承認済み
- 日付: 2026-03-31

## コンテキスト

Phase 3 (AI Handoff Layer) において、Archflow CLI は Contract の読み込み、パース、およびプロンプト生成に関連する特定のエラーを処理する必要があります。このポリシーは、[ADR-0008](./0008-minimal-cli-error-handling-policy-for-phase-2.md) をプロンプト生成ワークフローに拡張したものです。

目的は、設定エラーに対して予測可能な「フェイルファスト（Fail Fast）」な挙動を維持しつつ、カスタム拡張ポイント（Role）に対して不必要なクラッシュを起こさない柔軟性を確保することです。

## 決定事項

Phase 3 では、以下のエラーハンドリング動作を採用します。

### 1. Contract 不足時の動作
ユーザーがアーティファクトのプロンプトを要求したが、解決されたパスに `.contract.yaml` が見つからない場合:
- **ポリシー:** **フェイルファスト（Fail Fast）**。 `Error: Configuration file not found at [PATH]` を出力し、直ちに終了コード `1` で終了します。
- **理由:** プロンプトは Contract から導出されるため、Contract がない場合は真実味のあるハンドオフが不可能です。暗黙の推測による生成は禁止します。

### 2. 不正な Contract スキーマへの動作
`.contract.yaml` は存在するが、構文が不正であったり必須フィールド（`name`, `role`, `module`）が欠落している場合:
- **ポリシー:** **フェイルファスト（Fail Fast）**。 `serde-yaml` による詳細なパースエラーを出力し、直ちに終了コード `1` で終了します。

### 3. 未知の Role に対する動作
アーティファクトの `role` の値が、内部のロール対応プロンプトマッパーで認識されない場合（例: `service_layer` などのカスタムロール）:
- **ポリシー:** **緩やかなフォールバック（Graceful Fallback）**。エラーにはせず、デフォルトのアーキテクチャ制約を含む「汎用的なプロンプト」を生成します。
- **理由:** Archflow は拡張性を重視しています。ユーザーが独自の Role を定義しても、プロンプト生成が中断されることなく機能し続ける（最適化はされなくても動作する）必要があります。

### 4. 出力モードの失敗
CLI 経由で無効な `--mode` が指定された場合:
- **ポリシー:** **システムレベルの失敗**。コマンド実行前に、`clap` の `ValueEnum` 検証機能によって自動的に処理されます。

## 結果 (Consequences)

- **一貫性**: ファイル不足や構文エラーに対して、Phase 2 と同様のフェイルファストな挙動がユーザーに提供されます。
- **拡張性**: Role に対するフォールバックにより、独自のアーキテクチャ用語を使用しているプロジェクトでもシステムが有用であり続けます。
