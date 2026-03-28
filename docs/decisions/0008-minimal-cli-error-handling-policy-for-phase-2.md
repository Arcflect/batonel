# 0008-minimal-cli-error-handling-policy-for-phase-2

- Status: accepted
- Date: 2026-03-29

## Context

As Archflow enters Phase 2 (Minimal CLI), it requires a consistent strategy for how and when the CLI communicates failures to the user.

A common pitfall inside code-generation tools is overbuilding UX (such as beautiful colored diagnostic spans, auto-recovery mechanisms, or complex fallback defaults) before the concepts themselves are stable. Alternatively, under-building error handling results in silent crashes or opaque stack traces that hurt early contributor adoption.

We need a policy that defines:
- What happens when configuration files are missing.
- What happens when YAML is malformed.
- What happens when artifact roles cannot be resolved.
- What happens when path resolution or IO fails.

## Decision

We will adopt a **"Fail Fast on Context, Aggregate on Execution"** error handling strategy for Phase 2.

### 1. Missing File Behavior
If a root context configuration file (`project.arch.yaml`, `placement.rules.yaml`, `artifacts.plan.yaml`) is missing, the CLI will output a clear standard error message containing the missing filepath and immediately exit with code `1`.
- **Policy:** Explicit failure. No auto-recovery, no implicit discovery.

### 2. Invalid YAML Behavior
If a root context configuration file contains invalid YAML syntax or breaks schema expectations, the CLI will output the parsing error natively via `serde-yaml` and immediately exit with code `1`.
- **Policy:** Explicit failure. Silent fallbacks are forbidden.

### 3. Missing Role Mapping Behavior
If an artifact requests a `role` that does not exist in `placement.rules.yaml`:
- **Policy:** Aggregated failure. The CLI will emit a console error prefix `[!]` describing the missing role resolution, increment an error counter, and continue evaluating the remaining artifacts. 
- At the end of the execution block, the CLI will exit with code `1` if the error counter is greater than `0`.

### 4. Path Resolution and Write Failures
If the generator cannot create a directory or cannot write a placeholder file (due to IO permissions):
- **Policy:** Aggregated failure. The execution will catch the specific `thiserror`, prefix the log with `[!]`, skip that specific artifact action, evaluate the remaining queue, and exit with code `1` at the end.

## Consequences

- **What becomes easier:** Early contributors and users understand precisely where they made a typo in their `yaml` setup. 
- **What becomes harder:** We are explicitly delaying "rich UX" diagnostic helpers. Users must read basic terminal output to debug configurations.
- **Future work enabled:** This ensures our error payloads remain structurally simple enum variants (`thiserror`), keeping the minimal CLI lightweight enough to rapidly pivot into the Verifier Engine (Phase 4) without getting bogged down maintaining standard-error graphic libraries.

## Notes

This explicitly satisfies the Phase 2 roadmap goal: *"Do not overbuild. Clear and consistent is enough."*

---

## 日本語

# 0008-minimal-cli-error-handling-policy-for-phase-2

- Status: accepted
- Date: 2026-03-29

## コンテキスト

Archflow が Phase 2 (Minimal CLI) に入るにあたり、CLI が エラー をユーザーにどのように伝えるかについての、一貫した戦略が必要になります。

コード生成ツールにおけるよくある失敗は、概念そのものが安定する前に UX (美しいダイアグノスティックの表示、自動復旧メカニズム、複雑なフォールバック設定など) を過剰に作り込んでしまうことです。一方で、エラー処理を手抜きしすぎると、無言のクラッシュや不可解なスタックトレースが発生し、初期のコントリビューターの参加を阻害します。

以下の動作を定義するポリシーが必要です。
- 設定ファイルが不足している場合の動作
- YAML が不正な場合の動作
- ロールマッピングが見つからない場合の動作
- パス解決やファイル出力が失敗した場合の動作

## 決定事項

Phase 2 では、**「コンテキストはフェイルファスト、実行は集約（Fail Fast on Context, Aggregate on Execution）」** のエラーハンドリング戦略を採用します。

### 1. 設定ファイル不足時の動作
ベースとなる設定ファイル (`project.arch.yaml`, `placement.rules.yaml`, `artifacts.plan.yaml`) が見つからない場合、CLI は不足しているパスを含む明確な標準エラーメッセージを出力し、直ちに終了コード `1` で終了します。
- **ポリシー:** 明示的な失敗。自動復旧や暗黙的なファイル探索は行いません。

### 2. 不正な YAML に関する動作
設定ファイルに不正な YAML 構文が含まれているか、スキーマの期待値に反している場合、CLI は `serde-yaml` 経由でパースエラーをそのまま出力し、直ちに終了コード `1` で終了します。
- **ポリシー:** 明示的な失敗。サイレントなフォールバックは禁止です。

### 3. ロールマッピング不足時の動作
Artifact が `placement.rules.yaml` に存在しない `role` を要求した場合:
- **ポリシー:** 集約エラー（Aggregated failure）。CLI はコンソールに `[!]` から始まるエラープレフィックスを出力して解決できなかったロールを警告し、エラーカウンターをインクリメントした後、残りの Artifact の評価を継続します。
- 実行ブロックの最後で、エラーカウンターが `0` より大きい場合は終了コード `1` で終了します。

### 4. パス解決および書き込み時の失敗
IO権限などの理由によって、ジェネレーターがディレクトリを作成できない、またはファイルを書き込めない場合:
- **ポリシー:** 集約エラー。実行ルーチンは `thiserror` のバリアントをキャッチし、ログに `[!]` を付けて該当 Artifact のみをスキップします。残りのキューを評価し終わった後、最後に終了コード `1` で終了します。

## 結果 (Consequences)

- **何が容易になるか:** 初期のユーザーは、自分の設定した YAML のどこでタイポ等のミスをしたのかを、極めてシンプルかつ正確に把握できます。
- **何が困難になるか:** 「リッチな診断 UX」は意図的に後回しにします。ユーザーはターミナルの基本的なエラーメッセージを読んでデバッグする必要があります。
- **可能になる今後の作業:** エラー構造体を単純な enum (`thiserror`) に保てるため、重いUX用ライブラリ群に縛られることなく、Phase 4 の Verifier 検証エンジンへと素早く派生させることが可能になります。

## ノート

これは Phase 2 ロードマップの *"作りすぎない。明確で一貫しているだけで十分（Do not overbuild. Clear and consistent is enough.）"* という目標を明確に満たすものです。
