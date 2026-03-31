# 0010 Guard Contract-First Prompt Generation

- Status: accepted
- Date: 2026-03-31

## Context

Phase 3 introduces [Prompts](./docs/concepts/prompt.md)—derived instructions for AI implementers. However, as AI constraints grow, there is a risk that "Prompt-specific" architectural rules begin to appear in the tool's codebase or configuration that bypass the core [Artifact Contract](./docs/concepts/contract.md) model.

If the generator starts adding semantic logic that is not present in the Contract, we violate the "Contract as Source of Truth" principle ([ADR-0002](./0002-contract-is-the-source-of-truth-for-artifact-boundaries.md)), leading to "Drift" where the prompted code contains undocumented invariants.

## Decision

We formally define **Prompt Generation as a Strict Projection** of the Contract model.

### 1. The Strict Projection Rule
- A `Prompt` object must only contain data that is either:
    - Directly copied from the `Contract`.
    - Contextually derived from the `Project` or `Module` metadata.
    - Contextually inferred from the `Role` using an authoritative mapping of **Role-Based Completion Criteria**.

### 2. Guarding Semantic Injection
- The generator (Phase 3 logic) is forbidden from "hallucinating" or adding ad-hoc instructions that are not traceable to the Contract or the Role definition.
- Any "smart" defaults (like role-based completion checklists) must be treated as **System Defaults for Roles**, not as arbitrary "Prompt Logic".

### 3. Traceability
- Every line of a generated prompt should be identifiable as belonging to one of the Contract's semantic fields (`Responsibilities`, `Must Not`, `Inputs`, `Outputs`) or as a standard injection of the chosen `Role`.

## Consequences

- **Stability**: Generated prompts are guaranteed to never drift from the architectural intent documented in the `.contract.yaml`.
- **Extensibility**: Future role-specific optimizations will remain centralized and predictable.
- **Testing**: We can automate verification that prompt generation remains a pure function of the architectural state.

---

## 日本語

# 0010 プロンプト生成における設計（Contract）中心主義の保護

- ステータス: 承認済み
- 日付: 2026-03-31

## コンテキスト

Phase 3 では、AI 実装者向けの指示書として [Prompts](./docs/concepts/prompt.md) を導入しています。しかし、AI への制約が複雑になるにつれ、ツールや設定ファイルの中に「プロンプト専用」のアーキテクチャルールが現れ、コアとなる [Artifact Contract](./docs/concepts/contract.md) モデルをバイパスしてしまうリスクがあります。

ジェネレーターが Contract に存在しない意味的なロジックを独自に付加し始めると、「Contract が真実の源である」という原則 ([ADR-0002](./0002-contract-is-the-source-of-truth-for-artifact-boundaries.md)) が崩れ、生成されたコードにドキュメント化されていない不変条件が含まれる「ドリフト」が発生します。

## 決定事項

私たちは、**「プロンプト生成は Contract モデルの厳格な投影（Strict Projection）である」** と正式に定義します。

### 1. 厳密な投影ルール
- `Prompt` オブジェクトには、以下のいずれかに該当するデータのみを含めることができます。
    - `Contract` から直接コピーされたもの。
    - `Project` または `Module` のメタデータから文脈的に導出されたもの。
    - **ロールベースの完了基準**という権威あるマッピングを使用して、`Role` から文脈的に推論されたもの。

### 2. 意味的なインジェクションの防止
- ジェネレーター（Phase 3 のロジック）は、Contract や Role の定義に遡ることができない、アドホックな指示を追加（いわゆるハルシネーション）することを禁止します。
- ロールに基づく完了基準（チェックリスト）などの「賢い」デフォルト設定は、任意の「プロンプト用ロジック」ではなく、**「ロールに対するシステムデフォルト」** として扱う必要があります。

### 3. 追跡可能性 (Traceability)
- 生成されたプロンプトのすべての行は、Contract の意味的なフィールド (`Responsibilities`, `Must Not`, `Inputs`, `Outputs`) に属するものか、選択された `Role` の標準的なインジェクションであることを特定できる必要があります。

## 結果 (Consequences)

- **安定性**: 生成されたプロンプトは、`.contract.yaml` に記録されたアーキテクチャ上の意図から決して逸脱しないことが保証されます。
- **拡張性**: 将来のロール固有の最適化は、中央集約的かつ予測可能な状態を保てます。
- **テスト可能性**: プロンプト生成がアーキテクチャ状態の純粋な関数（Pure Function）であることを、自動テストで検証することが容易になります。
