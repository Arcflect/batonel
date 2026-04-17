# 0012 Minimal Error Handling Policy for Phase 4 Verification

- Status: accepted
- Date: 2026-04-01

## Context

Phase 4 introduced structural verification across project files, artifacts, contracts,
prompts, and scaffold outputs. As verification checks expanded, contributors needed a
single policy that explains how verify-related failures are classified and reported.

Without a shared policy, verify behavior can feel inconsistent:

- some conditions might look fatal while others only warn
- contributors may disagree on whether a case should fail CI
- users may not know which checks block exit code `1`

This policy extends [ADR-0008](./0008-minimal-cli-error-handling-policy-for-phase-2.md),
[ADR-0009](./0009-minimal-cli-error-handling-policy-for-phase-3.md), and
[ADR-0011](./0011-minimal-verification-model.md) specifically for verification outcomes.

## Decision

Phase 4 verify adopts a **"Aggregate and Classify"** policy:

- verify always runs all possible checks for the current context
- each check emits `Pass`, `Fail`, `Warn`, or `Skip`
- the command exits with code `1` only when at least one `Fail` exists
- warnings are visible but non-blocking

The minimum behavior per failure class is defined below.

### 1. Missing File Behavior (Root Context Files)

If `project.baton.yaml`, `placement.rules.yaml`, `artifacts.plan.yaml`, or
`contracts.template.yaml` is missing:

- **Classification:** `Fail` (`root-file-exists`)
- **Execution behavior:** Aggregate (do not crash), then skip deep checks that
  depend on successfully loaded configs.
- **Exit behavior:** exit code `1` because report contains failures.

### 2. Missing Contract Behavior

If a planned artifact does not have the expected `.contract.yaml` sidecar:

- **Classification:** `Fail` (`contract-exists`)
- **Execution behavior:** Continue with remaining artifacts/checks.
- **Exit behavior:** exit code `1`.

### 3. Invalid Field Behavior (Contract Required Fields)

If required contract fields are missing/empty (e.g., `name`, `module`, `role`,
`path`, `responsibilities`, `must_not`, `status`) or contract parsing fails:

- **Classification:** `Fail` (`contract-fields`, `contract-parse`)
- **Execution behavior:** Report per-field problems where available, continue
  evaluating other checks.
- **Exit behavior:** exit code `1`.

### 4. Path Mismatch Behavior

Path mismatch handling is split intentionally:

- **Contract path mismatch vs resolved path:** `Fail` (`contract-identity`)
- **Role-based explicit artifact path override mismatch:** `Warn`
  (`role-path-match`) to allow intentional overrides while keeping drift visible.

Exit behavior follows normal rules: only `Fail` causes exit code `1`.

### 5. Prompt Missing Behavior

If expected prompt sidecar is missing:

- **Classification:** `Warn` (`prompt-exists`)
- **Execution behavior:** Continue verification.
- **Exit behavior:** does not fail verify by itself.

Rationale: prompt presence is strongly recommended for AI handoff, but not yet a
hard architecture blocker in Phase 4.

### 6. Status Mismatch Behavior

Status outcomes follow minimum lifecycle consistency rules:

- invalid artifact status value: `Fail` (`artifact-status-valid`)
- invalid or missing contract status value: `Fail` (`contract-status-valid`)
- artifact/contract status conflict: `Fail`
  (`artifact-contract-status-consistent`)
- artifact status missing while contract status exists: `Warn`
  (`artifact-contract-status-consistent`)

## Consequences

- **What becomes easier:** Contributors can predict which verification outcomes
  fail CI and which are advisory.
- **What becomes harder:** Some soft-drift cases remain non-blocking until a
  stricter phase is explicitly decided.
- **What future work is enabled:** A future ADR can tighten selected warnings to
  failures without redesigning the reporting model.

## Notes

This policy keeps the first verification UX intentionally simple and consistent:

- clear grouped CLI output
- deterministic exit rule (`Fail` => exit code `1`)
- no rich diagnostic UI in this phase

---

## 日本語

# 0012 Phase 4 Verification 向け最小エラーハンドリングポリシー

- ステータス: 承認済み
- 日付: 2026-04-01

## コンテキスト

Phase 4 では、プロジェクト設定・artifact・contract・prompt・scaffold 構造に
またがる検証が導入されました。チェックが増えるにつれて、verify の失敗が
どのように分類され、どう報告されるかを明文化する必要があります。

共通ポリシーがない場合、次のようなズレが発生します。

- どのケースが致命的かが曖昧になる
- CI を失敗させる条件について認識が分かれる
- 利用者が終了コード `1` の条件を判断しづらくなる

このポリシーは
[ADR-0008](./0008-minimal-cli-error-handling-policy-for-phase-2.md)、
[ADR-0009](./0009-minimal-cli-error-handling-policy-for-phase-3.md)、
[ADR-0011](./0011-minimal-verification-model.md) を Verify 用に拡張します。

## 決定事項

Phase 4 の verify は **「集約して分類する (Aggregate and Classify)」** 方針を採用します。

- verify は可能なチェックを最後まで実行する
- 各チェックは `Pass` / `Fail` / `Warn` / `Skip` を返す
- `Fail` が 1 件でもあれば終了コード `1`
- `Warn` は可視化するが単独では失敗にしない

以下を最小動作として規定します。

### 1. 設定ファイル不足時の動作

`project.baton.yaml` / `placement.rules.yaml` / `artifacts.plan.yaml` /
`contracts.template.yaml` が不足している場合:

- **分類:** `Fail` (`root-file-exists`)
- **実行:** クラッシュはせず集約。依存する深いチェックはスキップ。
- **終了:** 失敗が含まれるため終了コード `1`。

### 2. Contract 不足時の動作

artifact に対応する `.contract.yaml` が見つからない場合:

- **分類:** `Fail` (`contract-exists`)
- **実行:** 他 artifact の検証は継続。
- **終了:** 終了コード `1`。

### 3. 必須フィールド不正時の動作

contract の必須フィールド欠落・空値、またはパース失敗の場合:

- **分類:** `Fail` (`contract-fields`, `contract-parse`)
- **実行:** 可能な限り項目ごとの失敗を出しつつ継続。
- **終了:** 終了コード `1`。

### 4. パス不一致時の動作

パス不一致は意図的に 2 種類へ分けます。

- 解決済みパスと contract.path の不一致: `Fail` (`contract-identity`)
- role ベース期待値と explicit artifact path の不一致: `Warn`
  (`role-path-match`)

前者は契約不整合、後者はドリフト可視化として扱います。

### 5. Prompt 不足時の動作

期待される prompt sidecar が不足している場合:

- **分類:** `Warn` (`prompt-exists`)
- **実行:** 検証継続。
- **終了:** 単独では失敗にしない。

理由: Phase 4 では prompt は強く推奨だが、まだハードブロッカーにはしない。

### 6. Status 不整合時の動作

status は最小ライフサイクル整合ルールで扱います。

- artifact status が不正値: `Fail` (`artifact-status-valid`)
- contract status が不正または欠落: `Fail` (`contract-status-valid`)
- artifact/contract status 不一致: `Fail`
  (`artifact-contract-status-consistent`)
- artifact status 欠落 + contract status あり: `Warn`
  (`artifact-contract-status-consistent`)

## 結果 (Consequences)

- **容易になること:** どの結果が CI を失敗させるかが明確になる。
- **難しくなること:** 一部のソフトドリフトは警告止まりで残る。
- **将来につながること:** 後続 ADR で warning を fail に格上げしやすい。

## ノート

Phase 4 では、UX を作り込みすぎず、次を重視します。

- 見やすい CLI 集約出力
- 決定的な終了条件（`Fail` があれば `1`）
- リッチ UI は持ち込まない
