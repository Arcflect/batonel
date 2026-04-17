# 0014 Guard Contract-First and Sidecar-First Behavior in Phase 4 Verification

- Status: accepted
- Date: 2026-04-01

## Context

Phase 4 established the first operational `batonel verify` command.
As its check set grows, there is a structural risk that future checks begin to
bypass the contract model or treat source code as the primary evidence.

This mirrors the risk addressed in prompt generation by
[ADR-0010](./0010-guard-contract-first-behavior-in-prompt-generation.md),
but now applies to the verify command itself.

Without an explicit guard:

- verify could drift toward source code as the primary truth signal
- sidecar files could be treated as secondary or optional
- contributors might add checks that work backward from implementation rather than forward from contracts
- the architectural model could diverge from what on-disk state verifies

This decision complements:

- [ADR-0002](./0002-contract-is-the-source-of-truth-for-artifact-boundaries.md): contract is the source of truth
- [ADR-0004](./0004-sidecar-files-are-first-class.md): sidecar files are first-class
- [ADR-0006](./0006-verify-starts-with-structure-and-contract-consistency.md): verify starts with structure and contract consistency
- [ADR-0007](./0007-batonel-remains-sidecar-first-before-code-aware-analysis.md): sidecar-first before code-aware analysis
- [ADR-0013](./0013-first-version-verification-rule-boundaries.md): first-version verification rule boundaries

## Decision

We formally define two invariants for `batonel verify`.

### 1. Contract-First Invariant

Verification derives its expectations from contract sidecar files, not from source code.

The check order enforces this:

1. Artifact plan defines what should exist (`artifacts.plan.yaml`)
2. Contract sidecar confirms the architectural boundary (`.contract.yaml`)
3. Source scaffold is checked only for presence, not content

This means:

- `contract-exists` must precede any content comparison involving the artifact
- `contract-identity` and `contract-fields` are first-class checks, not optional
- source file content is never parsed or inspected in Phase 4

If a contract is missing, verification reports a `Fail` and stops further contract-derived checks
for that artifact.
It does not fall back to reading source code in place of the contract.

### 2. Sidecar-First Invariant

Sidecar files (`.contract.yaml`, `.prompt.md`) are primary verification targets.

This means:

- sidecar existence is a `Fail` condition for contracts and a `Warn` condition for prompts
- sidecar content is read and validated independently of source code presence
- orphaned contracts are detected through sidecar scanning, not source traversal
- `scaffold-source-exists` confirms that a placeholder file exists on disk;
  it does not read or analyze its content

Future contributors must not invert this:
sidecar checks must not depend on source code content being present or correct.

### 3. What these invariants prohibit

The following additions to `batonel verify` would violate this decision and require
a new ADR before being accepted:

- parsing source file imports to validate dependency boundaries
- comparing source code structure against contract fields
- using source file content as a fallback when a contract is missing
- treating the absence of a sidecar file as acceptable if the source file is "complete"
- adding checks that only apply after source code exists

## Consequences

- Verification remains useful before implementation is started.
- The contract model stays authoritative for defining what is correct.
- Sidecar files remain the primary signals that verify must protect.
- Future code-aware analysis is explicitly deferred to a later phase, consistent with ADR-0007.
- Contributors adding new checks to `verify` have a clear test: does this check derive from sidecar files, or from source code? If the latter, it belongs in a future phase.

---

## 日本語

# 0014 Phase 4 verification における Contract-First および Sidecar-First 挙動の保護

- ステータス: 承認済み
- 日付: 2026-04-01

## コンテキスト

Phase 4 では、最初の実用的な `batonel verify` コマンドを確立しました。
チェックセットが成長するにつれ、将来のチェックが contract モデルをバイパスしたり、
ソースコードを主要な根拠として扱い始めるという構造的なリスクがあります。

これは、[ADR-0010](./0010-guard-contract-first-behavior-in-prompt-generation.md) でプロンプト生成に対して対処したリスクと同様ですが、
今回は verify コマンド自体に適用されます。

明示的なガードがなければ：

- verify がソースコードを主要な真実のシグナルとして扱う方向にドリフトする可能性がある
- sidecar ファイルが二次的またはオプションとして扱われる可能性がある
- コントリビューターが実装から逆方向に、contract から前向きにではなく、チェックを追加する可能性がある
- アーキテクチャモデルがディスク上の状態が verify する内容から乖離する可能性がある

この決定は以下を補完します：

- [ADR-0002](./0002-contract-is-the-source-of-truth-for-artifact-boundaries.md): contract は真実の源である
- [ADR-0004](./0004-sidecar-files-are-first-class.md): sidecar ファイルはファーストクラスである
- [ADR-0006](./0006-verify-starts-with-structure-and-contract-consistency.md): verify は構造と contract の整合から始まる
- [ADR-0007](./0007-batonel-remains-sidecar-first-before-code-aware-analysis.md): コード認識分析の前に sidecar ファースト
- [ADR-0013](./0013-first-version-verification-rule-boundaries.md): 初版 verification ルール境界

## 決定事項

`batonel verify` に対して 2 つの不変条件を正式に定義します。

### 1. Contract-First 不変条件

verification はソースコードからではなく、contract sidecar ファイルから期待値を導出します。

チェックの順序がこれを強制します：

1. artifact plan が何が存在すべきかを定義する（`artifacts.plan.yaml`）
2. contract sidecar がアーキテクチャの境界を確認する（`.contract.yaml`）
3. source scaffold は存在のみがチェックされ、内容はチェックされない

これが意味すること：

- `contract-exists` は、artifact に関わるコンテンツ比較より先に実行されなければならない
- `contract-identity` と `contract-fields` はファーストクラスのチェックであり、オプションではない
- ソースファイルの内容は Phase 4 では決してパースまたは検査されない

contract が欠落している場合、verification は `Fail` を報告し、
その artifact に対するそれ以降の contract 由来のチェックを停止します。
contract の代替としてソースコードを読み込む形にはなりません。

### 2. Sidecar-First 不変条件

Sidecar ファイル（`.contract.yaml`、`.prompt.md`）はプライマリの verification ターゲットです。

これが意味すること：

- sidecar の存在は、contract に対しては `Fail` 条件、prompt に対しては `Warn` 条件
- sidecar の内容は、ソースコードの存在とは独立して読み込まれ検証される
- 孤立した contract は sidecar のスキャンによって検出され、ソーストラバーサルによってではない
- `scaffold-source-exists` はプレースホルダーファイルがディスク上に存在することを確認する；
  その内容を読み込んだり分析したりはしない

将来のコントリビューターはこれを逆転させてはなりません：
sidecar のチェックはソースコードの内容が存在するまたは正しいことに依存してはなりません。

### 3. これらの不変条件が禁止すること

`batonel verify` への以下の追加はこの決定に違反し、
受け入れられる前に新しい ADR を必要とします：

- 依存関係の境界を検証するためのソースファイルのインポートのパース
- contract フィールドに対するソースコード構造の比較
- contract が欠落している場合のソースコードをフォールバックとして使用
- sidecar ファイルの欠如をソースコードが「完成」している場合に許容可能として扱うこと
- ソースコードが存在した後にのみ適用されるチェックの追加

## 結果

- verification は実装が開始される前から有用であり続ける。
- contract モデルは何が正しいかを定義する上で権威的であり続ける。
- sidecar ファイルは verify が保護しなければならないプライマリシグナルであり続ける。
- 将来のコード認識分析は、ADR-0007 と一致して、後のフェーズに明示的に延期される。
- `verify` に新しいチェックを追加するコントリビューターは明確なテストを持つ：
  このチェックは sidecar ファイルから導出されるか、それともソースコードから？後者の場合、将来のフェーズに属する。
