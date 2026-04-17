# 0011 Minimal Verification Model

- Status: accepted
- Date: 2026-03-31

## Context

As Batonel enters Phase 4 (Verification), it requires a structured internal model to represent the state of architectural consistency checks. This model must be robust enough to handle the initial scope of [ADR-0006](./0006-verify-starts-with-structure-and-contract-consistency.md) while remaining extensible for future code-aware analysis.

Without a consistent result structure, the verification logic would likely become a collection of ad-hoc `eprintln!` calls, making it difficult to generate aggregated reports or integrate with external CI systems.

## Decision

We will implement a centralized verification model in `src/model/verify.rs`.

### 1. `VerifyTarget`
Check targets are represented as an enum to support diverse architectural assets:
- **RootConfig**: Core project YAML files.
- **Artifact**: Logical entries in the plan.
- **Contract / Prompt**: Sidecar files.
- **SourceFile**: The actual implementation code.

### 2. `VerifyStatus`
We categorize results into four distinct states:
- **Pass**: The check succeeded.
- **Fail**: A hard violation (Architecture drift).
- **Warn**: A soft recommendation or potential issue.
- **Skip**: Pre-requisites not met.

### 3. `CheckResult` and `VerifyReport`
Results are packaged into a unified `VerifyReport`, which provides:
- A flat list of all `CheckResult` objects.
- Aggregate summary statistics (Pass/Fail/Warn/Skip counts).
- A definitive `is_success()` predicate (Success defined as zero failures).

## Consequences

- **Visibility**: Error reporting becomes structured and machine-readable (via Serde).
- **Extensibility**: New checks (e.g., dependency boundary linter) can simply emit a `CheckResult` and be automatically included in the report.
- **Testing**: We can verify the verification engine itself by asserting against the `VerifyReport` model.

---

## 日本語

# 0011 最小限の検証モデル (Minimal Verification Model)

- ステータス: 承認済み
- 日付: 2026-03-31

## コンテキスト

Batonel が Phase 4 (Verification) に移行するにあたり、アーキテクチャの一貫性チェックの状態を表現するための構造化された内部モデルが必要となります。このモデルは、[ADR-0006](./0006-verify-starts-with-structure-and-contract-consistency.md) で定義された初期スコープをカバーしつつ、将来のコード認識解析に向けた拡張性を備えている必要があります。

一貫した結果構造がないと、検証ロジックはアドホックな `eprintln!` の集まりになり、集計レポートの生成や外部 CI システムとの統合が困難になります。

## 決定事項

`src/model/verify.rs` に一元化された検証モデルを実装します。

### 1. `VerifyTarget`
チェック対象を Enum として表現し、多様な資産をサポートします。
- **RootConfig**: コアプロジェクト YAML ファイル。
- **Artifact**: 計画内の論理エントリ。
- **Contract / Prompt**: サイドカーファイル。
- **SourceFile**: 実際の実装コード。

### 2. `VerifyStatus`
結果を 4 つの状態に分類します。
- **Pass**: チェック成功。
- **Fail**: 致命的な違反（アーキテクチャの乖離）。
- **Warn**: 推奨事項または潜在的な問題。
- **Skip**: 前提条件が満たされていない。

### 3. `CheckResult` と `VerifyReport`
結果は統一された `VerifyReport` にパッケージ化されます。
- すべての `CheckResult` オブジェクトのリスト。
- 集計統計（Pass/Fail/Warn/Skip の数）。
- 明確な `is_success()` 判定。

## 結果 (Consequences)

- **可視性**: エラー報告が構造化され、Serde 経由で機械読み取り可能になります。
- **拡張性**: 依存関係境界の検証など、新しいチェックロジックを容易に追加できます。
- **テスト可能性**: `VerifyReport` モデルに対してアサーションを行うことで、検証エンジン自体のテストが可能になります。
