# Preset Trust Verification

Archflow ensures the integrity and origin authenticity of distribution presets using cryptographically signed release assets. This document defines the structural governance operations for maintaining this pipeline.

The target audience for this document includes platform operators, security auditors, and release engineers who need to understand how trust is established, verified, and safely rotated over time.

## Root of Trust

The primary mechanism for providing origin authenticity is **Ed25519 SSH Signatures**. 
All official releases and trusted partner preset bundles are signed out-of-band during the release process.

The "Root of Trust" for verifying these signatures is directly committed to the repository at `.github/trust/allowed_signers`. 

Any consumer can verify the integrity of a preset manually via:
```bash
ssh-keygen -Y verify -f .github/trust/allowed_signers -I governance@arcflect.io -n file -s preset.tar.gz.sig < preset.tar.gz
```

Because public keys are distributed within the repository itself, they are naturally tied to the version control history, allowing any system to determine exactly which keys were considered authoritative at any given commit.

## Key Lifecycle Handling

Consistent operational hygiene requires a predictable key lifecycle.

### 1. Generation

To ensure non-repudiation, private keys are generated strictly in offline environments or highly restricted CI secure enclaves. 
- Generated keys MUST use the Ed25519 algorithm.
- No private key material should ever touch an unencrypted disk on an internet-connected boundary outside of a short-lived memory context during the actual signing operation.

### 2. Distribution

Public identities (Trust Anchors) are formatted as single-line OpenSSH formats and placed in the `.github/trust/allowed_signers` file. 
- A given public key is bound to a specific semantic identity, usually identifying the system emitting the signature (e.g., `governance@arcflect.io`).
- Adding a new identity to this file constitutes an explicit designation of trust.

### 3. Rotation

To minimize the impact of long-term key exhaustion, keys undergo planned rotation:
- **Overlap Period**: When a new key is generated, it is added to the `allowed_signers` file while the previous key remains. Both keys remain valid during a transition period (e.g., 30 days).
- New releases are immediately signed with the *new* active key.
- Consumers of verification tooling will continue to trust older presets signed by the previous key until that key is formally removed from the explicit trust list.

### 4. Revocation

If a private key is believed to be compromised, or when its overlap period securely concludes, it must be explicitly revoked.
Revocation is performed by retaining the public key in the `allowed_signers` file but prepending the `revoked` marker to its declaration:
```text
revoked governance@arcflect.io ssh-ed25519 AAAAC3...
```
Verification tooling explicitly rejects signatures matching a revoked public key, forcing an integrity failure for all artifacts signed by that identity.

---

## 日本語

# Preset のトラスト検証

Archflow は、暗号学的に署名されたリリースアセットを用いることで、配布される Preset の完全性とオリジンの真正性を保証します。本ドキュメントでは、このパイプラインを維持するための構造的ガバナンス運用について定義します。

本ドキュメントは、トラストがどのように確立され、検証され、安全にローテーションされるかを理解する必要があるプラットフォーム・オペレーター、セキュリティ監査員、およびリリース・エンジニアを対象としています。

## トラストの起点 (Root of Trust)

オリジンの真正性を提供する主なメカニズムは **Ed25519 SSH Signatures** です。
公式のリリースおよび信頼できるパートナーの Preset バンドルはすべて、リリースプロセスにおいてアウトオブバンドで署名されます。

これらの署名を検証するための「トラストの起点（Root of Trust）」は、リポジトリの `.github/trust/allowed_signers` に直接コミットされています。

利用者は以下のコマンドを通じて、手動で Preset の完全性を検証することができます：
```bash
ssh-keygen -Y verify -f .github/trust/allowed_signers -I governance@arcflect.io -n file -s preset.tar.gz.sig < preset.tar.gz
```

公開鍵がリポジトリ自体に配布されているため、それらはバージョン管理の履歴と自然に結びつき、任意のコミットにおいてどの鍵が権威づけられていたかをどのシステムからでも正確に判定できます。

## 鍵のライフサイクル処理

一貫した運用衛生を保つには、予測可能な鍵のライフサイクルが必要です。

### 1. 生成 (Generation)

否認防止を確実にするため、秘密鍵は厳密にオフライン環境、または高度に制限された CI のセキュアエンクレーブでのみ生成されます。
- 生成される鍵は Ed25519 アルゴリズムを使用しなければなりません（MUST）。
- 秘密鍵のデータは、実際の署名操作時の短命なメモリコンテキストを除き、インターネットに接続された境界上の暗号化されていないディスクに決して触れさせてはなりません。

### 2. 配布 (Distribution)

公開 ID（トラスト・アンカー）は1行の OpenSSH 形式でフォーマットされ、`.github/trust/allowed_signers` ファイルに配置されます。
- ある公開鍵は特定のセマンティック・アイデンティティに紐付き、通常は署名を発行するシステム（例: `governance@arcflect.io`）を識別します。
- このファイルに新しい ID を追加することは、明示的な信頼の指定を意味します。

### 3. ローテーション (Rotation)

長期的な鍵の利用によるリスクを最小化するため、鍵は計画的にローテーションされます：
- **オーバーラップ期間**: 新しい鍵が生成されると、古い鍵が残されたまま `allowed_signers` ファイルに追加されます。移行期間中（例: 30日間）は、両方の鍵が有効です。
- 新しいリリースは、ただちに *新しく* アクティブになった鍵で署名されます。
- 検証ツールの利用者は、古い鍵が明示的なトラストリストから正式に削除されるまで、以前の鍵で署名された古い Preset を引き続き信頼します。

### 4. 失効 (Revocation)

秘密鍵の漏洩が疑われる場合、もしくはオーバーラップ期間が安全に終了した場合、鍵は明示的に失効させなければなりません。
失効は、公開鍵を `allowed_signers` ファイルに保持したまま、その宣言の先頭に `revoked` マーカーを付与することで実行されます：
```text
revoked governance@arcflect.io ssh-ed25519 AAAAC3...
```
検証ツールは、失効した公開鍵に一致する署名を明示的に拒否し、その ID によって署名されたすべてのアーティファクトに対して完全性エラーを強制します。
