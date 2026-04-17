# 0021 Minimal Error Handling Policy for Preset-Based Onboarding

- Status: accepted
- Date: 2026-04-03

## Context

Preset-based onboarding is now available through `batonel init --preset`.
As adoption grows, contributors need shared expectations for preset-related
failures without introducing heavy UX or complex recovery behavior.

This policy extends the minimal CLI error philosophy from:

- [ADR-0008](./0008-minimal-cli-error-handling-policy-for-phase-2.md)
- [ADR-0018](./0018-design-minimal-project-bootstrap-flow-from-presets.md)

## Decision

Preset onboarding adopts a **"Clear and Explicit Failure"** policy:

- fail fast for invalid startup context
- show one clear `[!]` error line with actionable detail
- exit code `1` on onboarding failure
- no silent fallback to another preset

### 1. Unknown preset behavior

If the selected preset id is not found in available preset roots:

- **Behavior:** Fail immediately
- **Message:** includes unknown preset id and available preset ids when present
- **Exit code:** `1`

### 2. Incomplete preset definition behavior

If a preset exists but required files are missing or unreadable
(`project.baton.yaml`, `placement.rules.yaml`, `contracts.template.yaml`):

- **Behavior:** Fail immediately
- **Message:** includes required file path and IO/read error detail
- **Exit code:** `1`

No partial generation is attempted from incomplete required preset inputs.

### 3. Unsupported preset selection behavior

For this phase, support is defined minimally:

- a preset is considered supported when it is discoverable by id and contains
  required files compatible with existing root-config assumptions

If a selected preset is not supportable under this rule,
onboarding treats it as explicit failure (not fallback).

### 4. Invalid override behavior

If override input is invalid (for now, `--project-name` is empty/whitespace):

- **Behavior:** Fail immediately
- **Message:** identifies invalid override input
- **Exit code:** `1`

## Consequences

- Preset-related failures are predictable and understandable.
- Contributors share one minimal expectation model for onboarding errors.
- UX stays intentionally simple in this phase.

---

## 日本語

# 0021 preset ベースオンボーディング向け最小エラーハンドリングポリシー

- ステータス: 承認済み
- 日付: 2026-04-03

## コンテキスト

`batonel init --preset` により preset ベースオンボーディングが利用可能になりました。
採用が進むにつれて、preset 関連失敗について、過剰な UX を作り込まずに
共通期待値を持つ必要があります。

このポリシーは次を土台にします。

- [ADR-0008](./0008-minimal-cli-error-handling-policy-for-phase-2.md)
- [ADR-0018](./0018-design-minimal-project-bootstrap-flow-from-presets.md)

## 決定事項

preset オンボーディングは **「明確で明示的な失敗 (Clear and Explicit Failure)」** を採用します。

- 起動文脈が不正ならフェイルファスト
- 行動可能な詳細付きで `[!]` エラーを明示
- オンボーディング失敗時は終了コード `1`
- 別 preset への暗黙フォールバックは禁止

### 1. unknown preset の挙動

選択した preset id が利用可能 preset roots で見つからない場合:

- **挙動:** 即時失敗
- **メッセージ:** 不明 preset id と、存在する場合は利用可能 preset id 一覧を表示
- **終了コード:** `1`

### 2. 不完全 preset 定義の挙動

preset は存在するが必須ファイル
(`project.baton.yaml`, `placement.rules.yaml`, `contracts.template.yaml`)
が欠落または読み取り不能な場合:

- **挙動:** 即時失敗
- **メッセージ:** 必須ファイルパスと IO/読み取りエラー詳細を表示
- **終了コード:** `1`

必須入力が不完全な状態での部分生成は行いません。

### 3. unsupported preset selection の挙動

このフェーズでは support を最小定義します。

- preset は、id で発見可能であり、既存 root-config 前提と互換な
  必須ファイルを持つ場合に supported とみなす

このルールで support できない preset を選んだ場合、
オンボーディングは明示的失敗として扱います（フォールバックなし）。

### 4. invalid override の挙動

override 入力が不正な場合（現時点では `--project-name` が空/空白）:

- **挙動:** 即時失敗
- **メッセージ:** 不正な override 入力を特定して表示
- **終了コード:** `1`

## 結果

- preset 関連失敗が予測可能かつ理解しやすくなる。
- コントリビューターがオンボーディングエラーの最小共通モデルを共有できる。
- このフェーズでは意図的にシンプルな UX を維持できる。
