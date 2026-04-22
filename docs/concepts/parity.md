# Parity

In Batonel, **Parity** refers to the strict alignment between examples, presets, schemas, and generated outputs.

It is a first-class quality rule that ensures Batonel's components are consistently synchronized. 
When parity is maintained, users can trust that learning an example will directly translate into using its matching preset.

## The Three Dimensions of Parity

### 1. Example-to-Preset Parity
Presets are operational packages built from the reusable defaults of teaching examples.
If a configuration file (like `contracts.template.yaml`) is shared between an example and its derived preset, the contents of that file **must be identical** in both places.

If you update a rule in `examples/rust-clean-hexagonal/batonel/placement.rules.yaml`, you must copy that exact change to `presets/rust-clean-hexagonal/placement.rules.yaml`.

### 2. Expected Output Parity
Examples include an `expected/` directory showing what Batonel generates.
Batonel uses scripts (like `sync_example_prompts.py`) to generate AI prompts from the example contracts.
The files checked into the repository must perfectly match the output of these generation scripts.

If you update a contract in an example, you must re-run the prompt sync script before committing.

### 3. Schema Parity
The versioning declarations within `project.baton.yaml` and `preset.yaml` metadata must align with the official schemas defined in the `schemas/` directory.
Batonel relies on these headers to ensure project safety during initialization.

## Why Parity Matters

Batonel is an architecture-to-execution bridge.
If the components within the bridge drift, the bridge becomes unreliable.
Enforcing parity mechanically guarantees that the documentation, the teaching examples, and the operational tools will remain synchronized over time.

---

## 日本語

# Parity（パリティ・等価性）

Batonel における **Parity（パリティ）** とは、example、preset、スキーマ、および生成される出力の間の厳密な整合性を指します。

これは、Batonel のコンポーネントが常に同期されていることを保証する、第一級の品質ルールです。
Parity が維持されている場合、ユーザーは「ある example を学ぶことが、そのペアとなる preset を使用することに直接つながる」と信頼することができます。

## Parity の 3 つの側面

### 1. Example と Preset の Parity
preset は、学習用 example の再利用可能なデフォルトから構築される運用パッケージです。
設定ファイル（`contracts.template.yaml` など）が example とそこから派生した preset の間で共有されている場合、そのファイルの内容は両方の場所で **完全に一致** していなければなりません。

`examples/rust-clean-hexagonal/batonel/placement.rules.yaml` のルールを更新した場合、そのまったく同じ変更を `presets/rust-clean-hexagonal/placement.rules.yaml` にもコピーする必要があります。

### 2. Expected Output（期待される出力）の Parity
example には、Batonel が生成するものを示す `expected/` ディレクトリが含まれています。
Batonel はスクリプト（`sync_example_prompts.py` など）を使用して、example の contract から AI プロンプトを生成します。
リポジトリにチェックインされているファイルは、これらの生成スクリプトの出力と完全に一致していなければなりません。

example 内の contract を更新した場合、コミットする前にプロンプト同期スクリプトを再実行する必要があります。

### 3. Schema（スキーマ）の Parity
`project.baton.yaml` や `preset.yaml` のメタデータ内のバージョン宣言は、`schemas/` ディレクトリで定義されている公式スキーマと整合していなければなりません。
Batonel は、初期化時のプロジェクトの安全性を確保するためにこれらのヘッダーに依存しています。

## なぜ Parity が重要なのか

Batonel は、設計から実行への橋渡し（architecture-to-execution bridge）です。
橋の中にあるコンポーネントがずれてしまうと、橋は信頼できないものになってしまいます。
Parity を機械的に強制することで、ドキュメント、学習用 example、および運用ツールが時間の経過とともに同期され続けることが保証されます。
