# Ecosystem Compliance Maturity Benchmark

This document defines the Archflow ecosystem compliance maturity model.

It provides a structured, auditable framework for evaluating how consistently
a repository applies Archflow governance controls.

The benchmark is intended for use by:

- Governance reviewers evaluating preset submissions
- Operators self-assessing their own repositories
- Maintainers tracking ecosystem-wide adoption posture

---

## Purpose

This benchmark answers the question:

> "To what degree does this repository follow Archflow governance expectations?"

The answer is expressed as a maturity level. Each level corresponds to a concrete
set of governance controls that can be verified by inspection or automated check.

A higher level does not imply a more senior organisation. It indicates that more
governance controls are in place and operational.

---

## Maturity Levels

The model defines five levels: **L0 through L4**.

Each level is cumulative. A repository at L3 must also satisfy L0, L1, and L2.

---

### L0 — Unregistered

**Definition**: No Archflow governance controls are in place.

The repository has not adopted any of the Archflow trust or governance mechanisms.

**Criteria** (all must be absent or unmet):

- No `allowed_signers` file is present or populated
- No preset bundle signatures (`.sig` files) are present
- No trust verification script is integrated
- No governance role mapping is documented

**Interpretation**: The repository is outside the governed ecosystem. No compliance
guarantees can be made about the origin or integrity of presets it produces or consumes.

---

### L1 — Trust Anchored

**Definition**: A valid trust anchor is established for the repository's signing identity.

The repository has declared at least one trusted public key, formatted and validated
in accordance with Archflow's signing requirements.

**Criteria** (all must hold):

- `.github/trust/allowed_signers` exists and is non-empty
- Every entry in `allowed_signers` uses `ssh-ed25519` key type exclusively
- No duplicate identities appear in the file
- The file passes `scripts/validate_allowed_signers.sh` without error
- At least one entry corresponds to a verified governance identity
  (e.g., `governance@arcflect.io` or a documented partner identity)

**Interpretation**: The repository has declared its signing identity. A consumer can
establish an initial chain of custody by referencing the stored public key anchor.

---

### L2 — Signing Operational

**Definition**: Preset bundles are signed and the verification pipeline is integrated.

The repository does not only declare a trust anchor — it actively signs artifacts
and verifies that signing in CI.

**Criteria** (all must hold):

- All requirements from L1 are satisfied
- Released preset bundles include corresponding `.sig` signature files
- `scripts/verify_trust.sh` exists and is executable
- `verify_trust.sh` is referenced and invoked in a CI workflow
- The CI workflow fails the build if signature verification fails

**Interpretation**: Signing is not just declared but operationally enforced. A consumer
can run `verify_trust.sh` against any distributed preset bundle and confirm its origin.

---

### L3 — Review Controlled

**Definition**: A governed human review process controls changes to the trust surface.

The signing pipeline is not sufficient alone. Governance controls must ensure that
the people who influence the trust surface are identified, their responsibilities are
documented, and key lifecycle changes are reviewed accordingly.

**Criteria** (all must hold):

- All requirements from L2 are satisfied
- A documented partner review workflow exists (e.g., `docs/partner-preset-review.md`)
- The review workflow mandates out-of-band identity verification before merging key changes
- Governance roles are documented that specify who may approve trust anchor updates
  (e.g., `governance-admin` or equivalent role in `docs/governance-rbac.md`)
- Key revocation procedure is documented and the revocation marker (`revoked`) is
  supported in validation tooling
- PRs touching `.github/trust/allowed_signers` require an explicit role-scoped approval

**Interpretation**: Changes to the trust surface are controlled. An auditor can trace
who approved each key addition or revocation, via what process, and under what authority.

---

### L4 — Audit Continuous

**Definition**: Governance posture is verified automatically on every relevant change.

The repository does not rely solely on operator discipline. A CI gate runs compliance
checks on every pull request that touches governance-sensitive paths, ensuring posture
cannot silently regress.

**Criteria** (all must hold):

- All requirements from L3 are satisfied
- `scripts/check_compliance_level.sh` exists and executes without error against the
  repository itself
- A CI workflow invokes `check_compliance_level.sh` on pull requests affecting
  governance-sensitive paths (trust files, signing scripts, policy documents)
- Test coverage exists for the compliance check script
  (e.g., `scripts/test_check_compliance_level.sh`)
- Any compliance regression blocks merge

**Interpretation**: The compliance posture is enforced continuously. Operators do not
need to remember to verify controls — the CI system does it on every relevant change.
This is the expected posture for repositories participating in the trusted ecosystem.

---

## Scoring Guidance

### How to assign a level

Evaluate the criteria for each level in order, from L0 upward. Assign the highest
level for which **all** criteria are met.

Partial satisfaction of a level's criteria does not qualify as that level. The
assessment is pass/fail per level, not a percentage score.

### Running the automated check

The `scripts/check_compliance_level.sh` script performs a file-system and
configuration inspection of the repository and emits a per-level result:

```bash
bash scripts/check_compliance_level.sh
```

Example output:

```
Checking Archflow ecosystem compliance maturity...

  L0 — Unregistered:        PASS (baseline)
  L1 — Trust Anchored:      PASS
  L2 — Signing Operational: PASS
  L3 — Review Controlled:   PASS
  L4 — Audit Continuous:    PASS

Result: L4 — Audit Continuous
```

If any level fails, the output lists the specific criterion that was not met.

### Manual review checklist

For each level, the following checklist can be used by a reviewer performing a
governance audit:

#### L1 Checklist

- [ ] `.github/trust/allowed_signers` is present
- [ ] All keys are `ssh-ed25519` type
- [ ] No duplicate identities
- [ ] `validate_allowed_signers.sh` passes
- [ ] At least one governance identity is present

#### L2 Checklist

- [ ] L1 satisfied
- [ ] `.sig` files accompany distributed preset bundles
- [ ] `verify_trust.sh` exists and is executable
- [ ] CI workflow invokes `verify_trust.sh`
- [ ] CI fails on verification error

#### L3 Checklist

- [ ] L2 satisfied
- [ ] Partner review workflow document exists
- [ ] Out-of-band verification is required by the review workflow
- [ ] Governance role mapping document exists
- [ ] Revocation procedure is documented
- [ ] `allowed_signers` changes require explicit role approval

#### L4 Checklist

- [ ] L3 satisfied
- [ ] `check_compliance_level.sh` exists and runs clean against this repository
- [ ] CI runs compliance check on governance-sensitive PRs
- [ ] Test script for the compliance checker exists
- [ ] Compliance regression blocks merge

---

## Advancement Criteria

To advance from one level to the next:

| Transition | Required action |
|------------|----------------|
| L0 → L1 | Add and validate `allowed_signers` with a trusted Ed25519 key |
| L1 → L2 | Sign preset bundles and integrate `verify_trust.sh` into CI |
| L2 → L3 | Document the review and revocation process; assign RBAC governance roles |
| L3 → L4 | Add `check_compliance_level.sh` and enforce it in CI as a gate |

---

## Benchmark Dimensions

The five levels collectively cover the following governance dimensions:

| Dimension | Covered from level |
|-----------|-------------------|
| Trust anchor declaration | L1 |
| Key type enforcement | L1 |
| Artifact signing | L2 |
| Verification pipeline (CI) | L2 |
| Human review process | L3 |
| Role-scoped approval | L3 |
| Key lifecycle (rotation, revocation) | L3 |
| Continuous posture enforcement | L4 |
| Test coverage for controls | L4 |

---

## Related Documents

- [docs/partner-preset-review.md](partner-preset-review.md)
- [docs/governance-rbac.md](governance-rbac.md)
- [docs/governance-exceptions.md](governance-exceptions.md)
- [docs/decisions/0024-preset-signature-and-trust-verification.md](decisions/0024-preset-signature-and-trust-verification.md)
- [docs/decisions/0025-ecosystem-compliance-maturity-benchmark-model.md](decisions/0025-ecosystem-compliance-maturity-benchmark-model.md)
- [scripts/check_compliance_level.sh](../scripts/check_compliance_level.sh)
- [scripts/validate_allowed_signers.sh](../scripts/validate_allowed_signers.sh)
- [scripts/verify_trust.sh](../scripts/verify_trust.sh)

---

## 日本語

このドキュメントは Archflow エコシステム準拠成熟度ベンチマークを定義します。

リポジトリが Archflow のガバナンスコントロールをどの程度一貫して適用しているかを
評価するための、構造化された監査可能なフレームワークを提供します。

---

### 目的

このベンチマークは次の問いに答えます：

> 「このリポジトリは Archflow のガバナンス要件をどの程度満たしているか？」

答えは成熟度レベルとして表現されます。各レベルは、検査または自動チェックによって
検証可能な具体的なガバナンスコントロールの集合に対応します。

---

### 成熟度レベル

モデルは **L0 から L4** の 5 段階を定義します。

各レベルは累積的です。L3 のリポジトリは L0、L1、L2 の要件も満たさなければなりません。

---

#### L0 — 未登録（Unregistered）

**定義**: Archflow のガバナンスコントロールが存在しない。

リポジトリは Archflow の信頼またはガバナンスの仕組みをまだ採用していません。

**基準**（すべて未存在または未充足）:

- `allowed_signers` ファイルが存在しないか空である
- Preset バンドルの署名（`.sig` ファイル）が存在しない
- 信頼検証スクリプトが統合されていない
- ガバナンスロールのマッピングがドキュメント化されていない

---

#### L1 — 信頼アンカー確立（Trust Anchored）

**定義**: リポジトリの署名アイデンティティに対して有効な信頼アンカーが確立されている。

**基準**（すべて充足必須）:

- `.github/trust/allowed_signers` が存在し、空でない
- `allowed_signers` のすべてのエントリが `ssh-ed25519` 鍵タイプのみを使用
- ファイル内に重複するアイデンティティが存在しない
- `scripts/validate_allowed_signers.sh` がエラーなく通過する
- 少なくとも 1 つのエントリが検証済みガバナンスアイデンティティに対応している

---

#### L2 — 署名運用中（Signing Operational）

**定義**: Preset バンドルが署名され、検証パイプラインが統合されている。

**基準**（すべて充足必須）:

- L1 のすべての要件が満たされている
- リリースされた Preset バンドルに対応する `.sig` 署名ファイルが含まれている
- `scripts/verify_trust.sh` が存在し実行可能である
- CI ワークフローで `verify_trust.sh` が参照・実行されている
- 署名検証の失敗時に CI ビルドが失敗する

---

#### L3 — レビュー管理（Review Controlled）

**定義**: 信頼面への変更を制御する、ガバナンスに基づく人間によるレビュープロセスが存在する。

**基準**（すべて充足必須）:

- L2 のすべての要件が満たされている
- ドキュメント化されたパートナーレビューワークフローが存在する
- レビューワークフローが鍵変更のマージ前に帯域外のアイデンティティ確認を義務付けている
- 信頼アンカー更新を承認できる者を指定するガバナンスロールがドキュメント化されている
- 鍵の失効手順がドキュメント化され、失効マーカー（`revoked`）が検証ツールでサポートされている
- `.github/trust/allowed_signers` に触れる PR に明示的なロールスコープの承認が必要

---

#### L4 — 継続的監査（Audit Continuous）

**定義**: ガバナンスポスチャが関連するすべての変更で自動的に検証される。

**基準**（すべて充足必須）:

- L3 のすべての要件が満たされている
- `scripts/check_compliance_level.sh` が存在し、リポジトリ自身に対してエラーなく実行される
- CI ワークフローがガバナンス上の重要なパスに影響する PR に対して
  `check_compliance_level.sh` を実行する
- コンプライアンスチェックスクリプトのテストカバレッジが存在する
- コンプライアンスの後退によりマージが阻止される

---

### スコアリングガイダンス

各レベルの基準を L0 から順番に評価してください。**すべての**基準を満たす最高レベルを
割り当てます。

レベルの基準を部分的に満たすだけでは、そのレベルへの認定は行われません。
評価はレベルごとのパス・フェイルであり、パーセンテージスコアではありません。

---

### 関連ドキュメント

- [docs/partner-preset-review.md](partner-preset-review.md)
- [docs/governance-rbac.md](governance-rbac.md)
- [docs/decisions/0025-ecosystem-compliance-maturity-benchmark-model.md](decisions/0025-ecosystem-compliance-maturity-benchmark-model.md)
- [scripts/check_compliance_level.sh](../scripts/check_compliance_level.sh)
