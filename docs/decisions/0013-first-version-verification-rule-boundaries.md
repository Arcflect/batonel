# 0013 First-Version Verification Rule Boundaries

- Status: accepted
- Date: 2026-04-01

## Context

Phase 4 (Verification) introduced the first operational `batonel verify` command.
As the check set grew, it became necessary to explicitly state what is inside scope
and what is outside scope for this first version.

Without explicit boundaries, the risk is:

- verify expands prematurely into code-aware analysis
- contributors add checks that conflict with the sidecar-first model
- the scope of Phase 4 becomes ambiguous across issues and pull requests

This decision complements:

- [ADR-0006](./0006-verify-starts-with-structure-and-contract-consistency.md): verify starts with structure and contract consistency
- [ADR-0007](./0007-batonel-remains-sidecar-first-before-code-aware-analysis.md): sidecar-first before code-aware analysis
- [ADR-0011](./0011-minimal-verification-model.md): minimal verification model
- [ADR-0012](./0012-minimal-error-handling-policy-for-phase-4-verification.md): Phase 4 error handling policy

## Decision

### Included: checks present in Phase 4 verify

The following checks are explicitly included in the first version of `batonel verify`.
Each check is identified by its stable `check_id` used in the verification report.

#### Root file presence

| check_id | Description |
|---|---|
| `root-file-exists` | Required batonel config files exist (`project.baton.yaml`, `placement.rules.yaml`, `artifacts.plan.yaml`, `contracts.template.yaml`) |

#### Role consistency

| check_id | Description |
|---|---|
| `role-defined` | Artifact role exists in placement rules |
| `role-path-match` | Explicit artifact path override matches role-based expectation (Warn on mismatch) |

#### Scaffold structure

| check_id | Description |
|---|---|
| `scaffold-directory-exists` | Expected directory for artifact exists on disk |
| `scaffold-source-exists` | Expected source file placeholder exists on disk |
| `scaffold-sidecars-complete` | Both contract and prompt sidecars exist alongside artifact |

#### Contract consistency

| check_id | Description |
|---|---|
| `contract-exists` | Contract sidecar file exists for planned artifact |
| `contract-identity` | Contract `name`, `module`, `role`, and `path` fields match artifact plan values |
| `contract-parse` | Contract file is valid YAML and loadable |
| `contract-fields` | Required contract fields (`name`, `module`, `role`, `path`, `responsibilities`, `must_not`, `status`) are present and non-empty |
| `orphaned-contract` | Contract file exists on disk but has no corresponding artifact in the plan |

#### Prompt consistency

| check_id | Description |
|---|---|
| `prompt-exists` | Prompt sidecar exists for planned artifact (Warn on missing) |
| `prompt-artifact-naming` | Prompt heading matches expected artifact name |
| `prompt-contract-identity` | Prompt `## Role` and `## Module` sections match artifact plan values |

#### Status consistency

| check_id | Description |
|---|---|
| `artifact-status-valid` | Artifact status value is one of the allowed lifecycle values |
| `contract-status-valid` | Contract status value is one of the allowed lifecycle values and is present |
| `artifact-contract-status-consistent` | Artifact and contract status values match |

---

### Excluded: checks explicitly outside Phase 4 scope

The following checks are **not** part of Phase 4 verify and should not be added
without a new scoping decision.

#### Source code inspection

- reading or parsing source file contents
- checking function signatures, struct definitions, or types
- verifying that the implementation reflects contract responsibilities
- detecting unused imports or dependency boundary violations in code

#### AST and compiler integration

- compiler invocation
- AST traversal
- language-specific static analysis
- linting that requires language toolchain setup

#### Import and dependency graph analysis

- tracking which modules import which
- enforcing dependency boundaries through import analysis
- detecting circular imports

#### Framework or ecosystem-specific checks

- framework-level conventions (e.g., controller naming in a web framework)
- package manager integration
- build output inspection

#### Deep content comparison

- comparing prompt content semantics against contract responsibilities
- checking implementation correctness
- measuring implementation completeness beyond placeholder detection

---

### Boundary rationale

The included checks stay within the sidecar and structure layer as established by
[ADR-0007](./0007-batonel-remains-sidecar-first-before-code-aware-analysis.md).
All checks operate on batonel files, config files, and file system structure,
not on source code content.

The excluded checks require either:

- language toolchain availability
- production-grade source code
- deeper runtime integration

These can become useful in a later phase, but they should not replace the
sidecar-first model before that model is stable.

## Consequences

- **What becomes easier:** Contributors know which checks belong in Phase 4 and
  which require a new scoping decision.
- **What becomes harder:** Requests for code-aware checks must reference this
  boundary and define a new phase scope explicitly.
- **What future work is enabled:** A future phase can add code-aware checks
  alongside existing structural checks without replacing them.

## Alternatives considered

### Define scope per-issue without a shared boundary record

Not chosen because it leads to inconsistent expectations across contributors
and makes it harder to explain what verify does and does not do.

### Tighten excluded checks immediately to prepare for code-aware analysis

Not chosen because the sidecar-first model is not yet fully stable.
Expanding too quickly would shift the center of gravity of the tool before
the structural layer is fully reliable.

## Notes

When a contributor proposes a new verify check, it should be evaluated against
this record:

- if the check touches only batonel files or file system structure, it may fit Phase 4
- if the check requires reading or parsing source code content, it requires a new scoping decision

---

## 日本語

# 0013 初版 verification ルール境界

- ステータス: 承認済み
- 日付: 2026-04-01

## コンテキスト

Phase 4 (Verification) において、最初の `batonel verify` コマンドが実装されました。
チェック内容が増えるに伴い、この初版のスコープに何が含まれ、何が除外されるかを
明確に記録する必要が生じました。

境界を明示しない場合のリスク：

- verify がコード認識型の分析に早期拡張される
- sidecar ファーストモデルに反するチェックが混入される
- Phase 4 の範囲が issue や PR をまたいで曖昧になる

このドキュメントは以下の ADR と組み合わせて使います。

- [ADR-0006](./0006-verify-starts-with-structure-and-contract-consistency.md)
- [ADR-0007](./0007-batonel-remains-sidecar-first-before-code-aware-analysis.md)
- [ADR-0011](./0011-minimal-verification-model.md)
- [ADR-0012](./0012-minimal-error-handling-policy-for-phase-4-verification.md)

## 決定事項

### 含まれるチェック（Phase 4 verify に実装済み）

以下のチェックが初版の `batonel verify` に明示的に含まれます。
各チェックは検証レポート内の安定した `check_id` で識別されます。

#### 設定ファイル存在確認

| check_id | 説明 |
|---|---|
| `root-file-exists` | 必須 batonel 設定ファイルが存在すること |

#### ロール整合性

| check_id | 説明 |
|---|---|
| `role-defined` | artifact のロールが placement rules に存在すること |
| `role-path-match` | explicit path override がロールベース期待値と一致すること（不一致は Warn） |

#### Scaffold 構造

| check_id | 説明 |
|---|---|
| `scaffold-directory-exists` | artifact の期待ディレクトリがディスク上に存在すること |
| `scaffold-source-exists` | 期待するソースファイルのプレースホルダーが存在すること |
| `scaffold-sidecars-complete` | contract と prompt の両 sidecar が artifact の隣に揃っていること |

#### Contract 整合性

| check_id | 説明 |
|---|---|
| `contract-exists` | 計画された artifact に対応する contract ファイルが存在すること |
| `contract-identity` | contract の `name`/`module`/`role`/`path` が artifact plan と一致すること |
| `contract-parse` | contract ファイルが有効な YAML で読み込めること |
| `contract-fields` | 必須フィールドが存在かつ非空であること |
| `orphaned-contract` | ディスク上に存在するが plan に対応 artifact がない contract |

#### Prompt 整合性

| check_id | 説明 |
|---|---|
| `prompt-exists` | artifact に対する prompt sidecar が存在すること（欠落は Warn） |
| `prompt-artifact-naming` | prompt ヘッダが期待 artifact 名と一致すること |
| `prompt-contract-identity` | prompt の `## Role` / `## Module` が artifact plan と一致すること |

#### Status 整合性

| check_id | 説明 |
|---|---|
| `artifact-status-valid` | artifact の status が許容ライフサイクル値のいずれかであること |
| `contract-status-valid` | contract の status が許容値かつ存在すること |
| `artifact-contract-status-consistent` | artifact と contract の status が一致すること |

---

### 除外されるチェック（Phase 4 スコープ外）

以下のチェックは Phase 4 に含まれず、新たなスコープ決定なしに追加すべきではありません。

- ソースコードの読み込みや解析
- コンパイラ統合・AST トラバーサル
- 言語固有の静的解析
- import/依存グラフ解析
- フレームワーク固有のルールチェック
- promtコンテンツと contract 責務のセマンティック比較

## 結果 (Consequences)

- **容易になること:** どのチェックが Phase 4 に属するかをコントリビューターが判断できる。
- **難しくなること:** コード認識型チェックの要望は境界と新フェーズスコープを明示する必要がある。
- **将来につながること:** 後続フェーズでコード認識型チェックを既存構造チェックと共存させやすい。
