# 0025-ecosystem-compliance-maturity-benchmark-model

- Status: accepted
- Date: 2026-04-16

## Context

As Batonel's Phase 12 expands the trusted preset ecosystem, there is a need to go
beyond individual governance controls (signing, review, RBAC) and provide a unified
model for evaluating and comparing the overall governance posture of participating
repositories.

Without a structured benchmark, operators have no shared vocabulary for describing
how mature a repository's governance adoption is, and reviewers have no consistent
basis for evaluating whether a new participant meets minimum expectations.

Questions to answer:

- How do we express the degree of governance compliance across a repository?
- What dimensions matter, and in what order should they be satisfied?
- How do we keep the model auditable without requiring a complex scoring engine?
- How does the model relate to the controls already defined in Phase 12 Task 1 and Task 2?

## Decision

We will use a **five-level ordinal maturity model** (L0–L4) where each level:

1. Defines a named, human-readable compliance posture
2. Lists concrete, pass/fail criteria that a reviewer or automated script can verify
3. Is cumulative — satisfying a higher level requires all prior levels to be satisfied

The levels are:

- **L0 — Unregistered**: No governance controls in place
- **L1 — Trust Anchored**: Valid `allowed_signers` with Ed25519 keys
- **L2 — Signing Operational**: Signed preset bundles with CI-integrated verification
- **L3 — Review Controlled**: Human review process, RBAC role mapping, revocation documented
- **L4 — Audit Continuous**: Compliance check integrated into CI as a merge gate

The model is documented in `docs/ecosystem-compliance-maturity.md` and supported by
a shell script (`scripts/check_compliance_level.sh`) that operators can run locally
or in CI to self-assess.

The benchmark intentionally does not introduce a continuous numerical score. Ordinal
levels are easier to communicate, easier to audit, and less prone to gaming through
partial credit accumulation.

## Consequences

- **What becomes easier**: Governance reviewers and repository operators share a
  common vocabulary. A repository can state "we are at L3" and a reviewer knows
  exactly what that means without re-inspecting configuration files manually.
- **What becomes harder**: Advancing a level requires satisfying all criteria at
  that level — partial progress does not move the level up. Teams must complete
  each level in order.
- **What future work is enabled**: The model can be extended (e.g., an L5 level
  covering multi-repository federation or third-party attestation) without invalidating
  existing assessments. The shell script model can be extended to emit structured JSON
  for programmatic consumption in future reporting pipelines.
- **What tradeoffs were accepted**:
  - No automated scoring server or external reporting service is introduced in this
    phase. The model is entirely self-assessed via the script or manual checklist.
  - The model covers governance posture only, not architectural quality or preset content.
    These are separate concerns handled by `batonel verify` and the review workflow.

## Alternatives considered

- **Continuous percentage score**: Rejected. A percentage score (e.g., "72% compliant")
  creates ambiguity about which controls are missing and invites partial completion gaming.
  Ordinal levels with explicit pass/fail criteria are more auditable.
- **Binary compliant/non-compliant**: Rejected. Binary status loses meaningful
  information about adoption progress. A repository that has trust anchors but not CI
  verification is meaningfully more advanced than one with nothing — the model should
  reflect this.
- **External compliance attestation service**: Rejected for this phase. Introducing a
  networked attestation server adds significant operational complexity and a dependency
  footprint that is inconsistent with Batonel's offline-first governance philosophy.
  Self-assessment tooling is sufficient for the current phase.
- **Single checklist without levels**: Rejected. A flat checklist does not communicate
  the minimum viable posture or the recommended adoption order, making it harder for
  new participants to understand where to start.

## Notes

- This ADR formalizes the design decisions behind the document introduced in
  `docs/ecosystem-compliance-maturity.md`.
- The maturity model builds directly on:
  - ADR `0024`: `preset-signature-and-trust-verification` (L1 and L2 foundations)
  - `docs/partner-preset-review.md` (L3 review process)
  - `docs/governance-rbac.md` (L3 role mapping)
- L4 is the expected minimum posture for any repository that participates in the
  Batonel trusted ecosystem as a publisher of signed presets.

---

## 日本語

### Context

Batonel の Phase 12 が信頼された Preset エコシステムを拡張するにつれ、
個々のガバナンスコントロール（署名、レビュー、RBAC）を超えて、
参加リポジトリの全体的なガバナンスポスチャを評価・比較するための
統一されたモデルが必要になりました。

構造化されたベンチマークがなければ、オペレーターはリポジトリのガバナンス採用成熟度を
説明するための共通語彙を持てず、レビュアーは新たな参加者が最低限の期待を満たすかどうかを
評価するための一貫した基準を持てません。

回答すべき問い：

- リポジトリ全体のガバナンスコンプライアンスの度合いをどう表現するか？
- どのディメンションが重要で、どの順序で満たすべきか？
- 複雑なスコアリングエンジンを必要とせずにモデルを監査可能に保つにはどうするか？
- このモデルは Phase 12 Task 1 および Task 2 で定義されたコントロールとどう関係するか？

### Decision

**5 段階の序数成熟度モデル**（L0–L4）を使用します。各レベルは：

1. 名前付きの、人間が読める準拠ポスチャを定義する
2. レビュアーまたは自動スクリプトが検証できる具体的なパス・フェイル基準を列挙する
3. 累積的である — 上位レベルを満たすには、前のすべてのレベルを満たす必要がある

モデルは `docs/ecosystem-compliance-maturity.md` にドキュメント化され、
`scripts/check_compliance_level.sh` シェルスクリプトによってサポートされます。

本ベンチマークは意図的に数値スコアを導入していません。序数レベルは
伝達しやすく、監査しやすく、部分点の積み上げによる操作を受けにくいです。

### Consequences

- **容易になること**: ガバナンスレビュアーとリポジトリオペレーターが共通語彙を共有します。
- **困難になること**: レベルの維持には、そのレベルのすべての基準を満たす必要があります。
- **可能になる今後の取り組み**: モデルは既存の評価を無効化せずに拡張できます。
- **受け入れたトレードオフ**: このフェーズでは自動スコアリングサーバーや外部レポートサービスは導入しません。

### Alternatives considered

- **連続パーセントスコア**: 採用せず。パーセントスコアは曖昧さを生み、部分達成の操作を招きます。
- **バイナリ準拠・非準拠**: 採用せず。バイナリステータスは採用進捗に関する有意義な情報を失います。
- **外部コンプライアンス証明サービス**: このフェーズでは採用せず。
- **レベルのない単一チェックリスト**: 採用せず。フラットなチェックリストでは最低限のポスチャや
  推奨採用順序が伝わらず、新規参加者が何から始めればよいかを理解しにくくなります。
