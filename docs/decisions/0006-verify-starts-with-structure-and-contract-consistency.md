# 0006 Verify starts with structure and contract consistency

- Status: accepted
- Date: 2026-03-28

## Context

Archflow is expected to include a `verify` capability in the future.

That verification layer could grow in many directions, for example:

- required file presence checks
- role-to-path consistency checks
- contract completeness checks
- prompt presence and derivation checks
- dependency boundary checks
- code-aware import checks
- static analysis integration
- language-specific validation

Because verification can expand quickly, the project needs an early decision about
where verification should begin.

Without this decision, there is a risk that `verify` becomes too broad too early,
or that it drifts into code analysis before the core structural model is stable.

Archflow’s current maturity is centered on:

- project definitions
- placement rules
- artifact plans
- contracts
- prompts
- scaffold structure

That means the first verification layer should protect those assets first.

## Decision

Archflow verification begins with **structure and contract consistency**.

The first scope of `verify` should focus on questions such as:

- do required files exist?
- do planned artifacts have corresponding contract files?
- do roles align across input and generated files?
- do artifact paths match placement rules?
- do required contract fields exist?
- do prompt files exist where expected?
- do artifact statuses remain internally consistent?

In other words, `verify` begins by checking whether the architecture-defined structure
and artifact boundaries remain coherent over time.

It does **not** begin with deep code-aware checks.

## Consequences

What becomes easier:
- defining a clear first version of `verify`
- keeping verification aligned with the current Archflow model
- protecting architectural intent before code parsing exists
- making examples and future CLI behavior easier to validate
- supporting language-agnostic usage in early phases

What becomes harder:
- using `verify` as a full architecture linter from the start
- immediately supporting language-specific code inspection
- catching implementation drift that only appears inside source code

This is an intentional tradeoff.

The first job of `verify` is to protect the explicit architectural model,
not to fully understand all implementation details.

## Initial verification targets

The first version of `verify` should prioritize checks such as:

### 1. Required input file presence

Examples:
- `project.arch.yaml` exists
- `placement.rules.yaml` exists
- `artifacts.plan.yaml` exists

### 2. Artifact-to-contract consistency

Examples:
- every planned artifact has a contract
- contract names match artifact names
- contract roles match planned artifact roles
- contract modules match planned artifact modules

### 3. Role-to-path consistency

Examples:
- artifact roles exist in placement rules
- resolved contract paths match placement rules
- explicit path overrides are recognized consistently

### 4. Required contract field presence

Examples:
- `name`
- `module`
- `role`
- `path`
- `responsibilities`
- `must_not`
- `status`

### 5. Prompt presence and derivation consistency

Examples:
- expected prompt files exist
- prompt artifact names align with contracts
- prompt structure reflects contract-level intent

### 6. Status consistency

Examples:
- artifact and contract status do not conflict
- invalid status values are surfaced
- lifecycle state is present when required

## What verify should not start with

The first version of `verify` should not start with:

- compiler integration
- language-specific AST parsing
- import graph analysis
- full dependency graph enforcement
- framework-specific rule engines
- model-vendor-specific prompt validation

These may become useful later,
but they should remain downstream from structural and contract verification.

## Why this fits Archflow

This decision aligns with several earlier project decisions.

It supports the idea that:

- Archflow is an architecture-to-execution bridge
- contracts are the source of truth for artifact boundaries
- prompts are derived from contracts
- sidecar files are first-class
- examples come before more operational preset machinery

Because Archflow starts from architecture rather than code,
its first verification layer should also begin from architecture rather than code.

## Alternatives considered

### Start verification with code-aware checks

Not chosen because code-aware validation would introduce language coupling too early
and would outrun the current model maturity.

### Start verification with prompts only

Not chosen because prompts are derived artifacts, not the primary architectural source of truth.

### Delay verification until after CLI maturity

Not chosen because even early examples and generated structures benefit from consistency checking.

### Treat verification as only a future CI concern

Not chosen because local verification is useful even before formal CI integration exists.

## Notes

This decision does not reject code-aware verification in the future.

It only establishes ordering:

**verify should begin with structure and contract consistency, then expand later if needed.**

---

## 日本語

# 0006 Verify は構造と contract の整合から始まる

- ステータス：採択済み
- 日付：2026-03-28

## コンテキスト

Archflow は将来的に `verify` 機能を含むことが期待されています。

その verify レイヤーは多くの方向に成長する可能性があります。例えば：

- 必須ファイルの存在チェック
- ロール-パスの整合性チェック
- contract の完全性チェック
- prompt の存在と導出チェック
- 依存関係境界チェック
- コード認識のインポートチェック
- 静的解析の統合
- 言語固有のバリデーション

verify は急速に拡張できるため、プロジェクトは verify がどこから始まるべきかについて早期の決定が必要です。

この決定がなければ、`verify` が早すぎて広くなりすぎるか、コア構造モデルが安定する前にコード解析に流れ込むリスクがあります。

Archflow の現在の成熟度は次に中心を置いています。

- プロジェクト定義
- 配置ルール
- artifact プラン
- contract
- prompt
- スキャフォルド構造

つまり、最初の verify レイヤーはまずこれらの asset を保護すべきです。

## 決定

Archflow の verify は**構造と contract の整合**から始まります。

`verify` の最初のスコープは次のような質問に焦点を当てるべきです。

- 必要なファイルが存在するか？
- 計画された artifact に対応する contract ファイルがあるか？
- ロールは入力と生成されたファイル全体で整合しているか？
- artifact パスは配置ルールと一致しているか？
- 必要な contract フィールドが存在するか？
- 期待される場所に prompt ファイルが存在するか？
- artifact のステータスは内部的に一貫しているか？

言い換えれば、`verify` はアーキテクチャで定義された構造と artifact の境界が時間とともに一貫したままかどうかを確認することから始まります。

深いコード認識チェックから始まりません。

## 結果

容易になること：
- `verify` の明確な最初のバージョンを定義すること
- 現在の Archflow モデルと整合した verify を保つこと
- コード解析が存在する前にアーキテクチャの意図を保護すること
- examples と将来の CLI の動作を検証しやすくすること
- 初期フェーズでの言語非依存の使用をサポートすること

難しくなること：
- 最初から完全なアーキテクチャ linter として `verify` を使用すること
- 言語固有のコード検査をすぐにサポートすること
- ソースコード内にのみ現れる実装のずれを捉えること

これは意図的なトレードオフです。

`verify` の最初の仕事は明示的なアーキテクチャモデルを保護することであり、すべての実装の詳細を完全に理解することではありません。

## 最初の verify 対象

`verify` の最初のバージョンは次のようなチェックを優先するべきです。

### 1. 必須入力ファイルの存在

例：
- `project.arch.yaml` が存在する
- `placement.rules.yaml` が存在する
- `artifacts.plan.yaml` が存在する

### 2. Artifact-contract の整合性

例：
- すべての計画された artifact に contract がある
- contract の名前が artifact の名前と一致する
- contract のロールが計画された artifact のロールと一致する
- contract のモジュールが計画された artifact のモジュールと一致する

### 3. ロール-パスの整合性

例：
- artifact のロールが配置ルールに存在する
- 解決された contract のパスが配置ルールと一致する
- 明示的なパスオーバーライドが一貫して認識される

### 4. 必須 contract フィールドの存在

例：
- `name`
- `module`
- `role`
- `path`
- `responsibilities`
- `must_not`
- `status`

### 5. Prompt の存在と導出の整合性

例：
- 期待される prompt ファイルが存在する
- prompt の artifact 名が contract と整合する
- prompt の構造が contract レベルの意図を反映する

### 6. ステータスの整合性

例：
- artifact と contract のステータスが矛盾しない
- 無効なステータス値が表示される
- 必要な場合にライフサイクルの状態が存在する

## Verify が最初に始めるべきでないこと

`verify` の最初のバージョンは以下から始めるべきではありません。

- コンパイラ統合
- 言語固有の AST 解析
- インポートグラフ解析
- 完全な依存グラフの強制
- フレームワーク固有のルールエンジン
- モデルベンダー固有の prompt バリデーション

これらは後で有用になるかもしれませんが、構造と contract の verify の下流にとどまるべきです。

## これが Archflow に合う理由

この決定はいくつかの以前のプロジェクトの決定と整合します。

これは次のアイデアをサポートします。

- Archflow は設計から実行への橋渡しである
- contract は artifact の境界にとっての真実の源である
- prompt は contract から導出される
- sidecar ファイルはファーストクラスである
- examples は より運用的な preset 機構より先に来る

Archflow はコードではなくアーキテクチャから始まるため、最初の verify レイヤーもコードではなくアーキテクチャから始まるべきです。

## 検討された代替案

### コード認識チェックから verify を開始する

選択されなかった理由：コード認識のバリデーションは言語の結合を早期に導入し、現在のモデルの成熟度を超えてしまうため。

### Prompt のみで verify を開始する

選択されなかった理由：prompt は導出された artifact であり、アーキテクチャの主要な真実の源ではないため。

### CLI の成熟後まで verify を遅らせる

選択されなかった理由：早期の examples と生成された構造でも整合性チェックから利益を得るため。

### Verify を将来の CI の懸念のみとして扱う

選択されなかった理由：フォーマルな CI 統合が存在する前でもローカル verify は有用であるため。

## 注記

この決定は将来のコード認識 verify を拒否しません。

順序を確立するだけです：

**verify は構造と contract の整合から始まり、後で必要に応じて拡張すべきです。**