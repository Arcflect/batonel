# 0024-preset-signature-and-trust-verification

- Status: proposed
- Date: 2026-04-13

## Context

As Batonel scales to support an ecosystem of independent presets, it must establish a reliable mechanism to ensure the integrity and authenticity of preset bundles consumed by end-users. Consumers need confidence that a preset actually originated from the claimed source (Batonel maintainers or a trusted partner) and has not suffered from tampering during distribution.

To effectively deliver the phase objectives around ecosystem governance, we need to choose a signature mechanism and define the trust verification pipeline that includes key lifecycle handling.

Questions to answer:
- How do we digitally sign release assets (presets) without imposing heavy operational dependencies?
- Where do we store the public anchors (keys) that confirm authenticity?
- How is key lifecycle handling (rotation, revocation) managed transparently?

## Decision

We will use **Ed25519 SSH Signatures (`ssh-keygen -Y`)** to sign and verify presets.

- **Storage**: Public keys will be stored within the repository at `.github/trust/allowed_signers` using the standard OpenSSH `allowed_signers` format.
- **Verification**: The `scripts/verify_trust.sh` utility will execute `ssh-keygen -Y verify` against the preset bundle and its signature (`.sig`) using the repository's `allowed_signers` file as the root of trust.
- **Lifecycle Handling**:
  - Keys are generated offline in a secure enclave.
  - Active keys are appended to `allowed_signers` under a specific principle or identity namespace (e.g., `governance@arcflect.com`).
  - Key rotation is executed by adding a new key to the file while retaining the old one for a defined overlap period.
  - Key revocation is executed by adding the `revoked` marker to the public key entry, permanently invalidating it for future and past verifications.

Contributors should assume that any official release automation will mandate a signature, and downstream consumers can optionally verify the integrity natively using their existing SSH tooling.

## Consequences

- **What becomes easier**: Verifying presets requires zero external tooling installations for users since `ssh-keygen` is already distributed on any platform capable of running Git.
- **What becomes harder**: Key lifecycle operations are manual governance steps (e.g., maintainers must carefully manage the private key offline and submit PRs to rotate the public anchors).
- **What future work is enabled**: The `allowed_signers` format scales organically. We can easily segment trust boundaries (e.g., `partner@...`, `core@...`) or distribute the file through a centralized directory.
- **What tradeoffs were accepted**: Providing an offline-first, highly standard implementation takes precedence over features provided by more complex but automated setups like Sigstore's keyless identity federation. 

## Alternatives considered

- **Sigstore/Cosign**: Offers keyless OIDC-based signature and transparency logs. Rejected for the minimal phase because it complicates offline environments and hides explicit key lifecycle handling behind dynamic, short-lived ephemeral certificates, increasing operational opacity for a simple governance model.
- **GPG (GNU Privacy Guard)**: Highly established but suffers from difficult CLI ergonomics and complex web-of-trust setups. Storing and distributing GPG public keyrings is more friction-heavy than standard `.ssh` formatted strings.
- **Minisign**: Excellent cryptography and simplicity, but requires an additional tool installation for every user compared to the ubiquity of SSH tools.

---

## 日本語

### Context

Batonel が独立した Preset のエコシステムをサポートするようスケールするにつれ、エンドユーザーが利用する Preset バンドルの完全性と真正性を保証する信頼性の高いメカニズムを確立する必要があります。Preset の利用者は、その Preset が実際に主張するソース（Batonel メンテナや信頼できるパートナー）から提供されたものであり、配布中に改ざんされていないという確証を必要としています。

エコシステム・ガバナンスにおけるフェーズ目標を効果的に達成するため、署名メカニズムを選定し、鍵のライフサイクル管理を含むトラスト検証パイプラインを定義する必要があります。

回答すべき問い：
- 重い運用依存を強いることなく、リリースアセット（Preset）にどのようにデジタル署名を行うか？
- 真正性を確認するための公開アンカー（公開鍵）をどこに保存するか？
- 鍵のライフサイクル処理（ローテーション、失効処理）をいかに透明に管理するか？

### Decision

Preset の署名および検証には **Ed25519 SSH Signatures (`ssh-keygen -Y`)** を使用します。

- **保存先**: 公開鍵はリポジトリ内の `.github/trust/allowed_signers` に、標準的な OpenSSH の `allowed_signers` 形式で保存されます。
- **検証**: `scripts/verify_trust.sh` ユーティリティは、リポジトリの `allowed_signers` ファイルを信頼の起点として使用し、Preset バンドルとその署名 (`.sig`) に対して `ssh-keygen -Y verify` を実行します。
- **ライフサイクル処理**:
  - 鍵はオフラインのセキュアエンクレーブで生成されます。
  - 有効な鍵は、特定のプリンシパルまたは ID ネームスペース（例: `governance@arcflect.com`）のもと、`allowed_signers` に追記されます。
  - 鍵のローテーションは、定義されたオーバーラップ期間中は古い鍵を維持しつつ、新しい鍵をファイルに追加することで実行されます。
  - 鍵の失効処理は、公開鍵エントリに `revoked` マーカーを追記することで実行され、過去および将来の検証において永続的に無効化されます。

コントリビューターは、公式のリリース自動化において署名が必須となること、またダウンストリームでの利用者は既存の SSH ツールを用いてネイティブに完全性を検証できることを前提として進めるべきです。

### Consequences

- **容易になること**: Git を実行できるあらゆるプラットフォームに `ssh-keygen` がすでに配布されているため、ユーザーは外部ツールをインストールすることなくゼロ依存で Preset の検証が可能です。
- **困難になること**: 鍵のライフサイクル運用が手動のガバナンス・ステップとなります（たとえば、メンテナは秘密鍵を慎重にオフラインで管理し、公開アンカーをローテーションする PR を提出する必要があります）。
- **可能になる今後の取り組み**: `allowed_signers` 形式は自然にスケールします。トラストの境界（例: `partner@...`, `core@...`）を簡単に分割したり、ファイルを中央集権的なディレクトリ経由で配布することができます。
- **受け入れたトレードオフ**: フェデレーションや Sigstore のような OIDC ベースの動的で複雑な自動セットアップが提供する機能よりも、透明で標準的なオフラインファーストの実装を優先しました。

### Alternatives considered

- **Sigstore/Cosign**: キーレスな OIDC ベースの署名と透明性ログを提供します。オフライン環境を複雑にし、明示的な鍵のライフサイクル処理を動的で短命な一時的証明書の背後に隠してしまい、単純なガバナンスモデルにおいては運用の不透明さを増大させるため、今回の最小フェーズでは採用を見送りました。
- **GPG (GNU Privacy Guard)**: 非常に確立されていますが、CLI のエルゴノミクスが難解で、Web of Trust のセットアップが複雑です。GPG の公開鍵リングを保存し配布することは、標準的な `.ssh` 形式の文字列よりも摩擦が大きくなります。
- **Minisign**: 優れた暗号化機能とシンプルさを備えていますが、SSH ツールの普及度と比較すると、すべてのユーザーに追加のツールインストールを要求することになります。
