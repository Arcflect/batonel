# 0018 Design Minimal Project Bootstrap Flow From Presets

- Status: accepted
- Date: 2026-04-03

## Context

Phase 5 introduced preset model and packaging:

- ADR-0015: minimal preset model
- ADR-0016: preset packaging approach
- ADR-0017: example-to-preset mapping

The remaining gap is startup behavior.
Users need a clear, minimal way to initialize a new project from a preset,
without manual copying of multiple files.

This must align with existing `archflow init` behavior and stay minimal.

## Decision

`archflow init` becomes the single bootstrap entrypoint for both:

- default initialization (no preset)
- preset-based initialization (`--preset`)

### 1. Minimal bootstrap flow

The minimal preset bootstrap flow is:

1. user runs `archflow init --preset <preset-id>`
2. CLI loads preset files from `presets/<preset-id>/`
3. CLI generates root config files in the current directory
4. existing files are not overwritten; they are skipped with a message

Generated root files (from preset) are:

- `project.arch.yaml`
- `placement.rules.yaml`
- `contracts.template.yaml`
- optional `artifacts.plan.yaml` when present in preset package

### 2. Effect of preset selection

Preset selection defines the initial defaults for:

- project architecture context
- role-to-path placement rules
- role-based contract template defaults
- optional starter artifact inventory

In other words, preset selection changes generated config content,
not the command sequence after initialization.

### 3. Immediate override scope

The first-version override scope remains minimal.

Supported immediate override:

- `--project-name <name>`: overrides `project.name` in `project.arch.yaml`

Not included yet:

- direct per-role override flags
- partial file-merge editing semantics
- interactive preset wizard

Users can edit generated files directly after init for deeper customization.

### 4. Error handling and discoverability

If a preset id is not found, init fails with an error and shows available preset ids.

This keeps behavior predictable and avoids silent fallback to a different preset.

## Consequences

- Preset-based onboarding is faster than manual file setup.
- Future CLI enhancements have a clear minimal target behavior.
- `archflow init` remains the canonical start command.
- Complexity stays intentionally low for the first bootstrap model.

---

## 日本語

# 0018 preset からの最小プロジェクトブートストラップフローを設計する

- ステータス: 承認済み
- 日付: 2026-04-03

## コンテキスト

Phase 5 では preset モデルと packaging が導入されました。

- ADR-0015: 最小 preset モデル
- ADR-0016: preset パッケージング方針
- ADR-0017: example-to-preset マッピング

残っているギャップは起動フローです。
ユーザーは複数ファイルを手動コピーせず、preset から新規プロジェクトを始める
明確で最小な方法を必要としています。

この設計は既存の `archflow init` と整合しつつ、最小である必要があります。

## 決定事項

`archflow init` を次の単一ブートストラップ入口にします。

- デフォルト初期化（preset なし）
- preset ベース初期化（`--preset`）

### 1. 最小ブートストラップフロー

preset ベースの最小フローは次です。

1. ユーザーが `archflow init --preset <preset-id>` を実行
2. CLI が `presets/<preset-id>/` から preset ファイルを読み込む
3. CLI がカレントディレクトリに root config ファイルを生成
4. 既存ファイルは上書きせず、skip メッセージを出す

preset 由来で生成される root ファイルは次です。

- `project.arch.yaml`
- `placement.rules.yaml`
- `contracts.template.yaml`
- preset package に存在する場合の optional `artifacts.plan.yaml`

### 2. preset 選択の影響

preset 選択は次の初期デフォルトを定義します。

- project のアーキテクチャ文脈
- role-to-path の配置ルール
- role ベースの contract template defaults
- optional な starter artifact inventory

つまり、preset 選択が変えるのは生成される config 内容であり、
初期化後のコマンドシーケンスではありません。

### 3. 即時 override 範囲

初版の override 範囲は最小を維持します。

サポートされる即時 override:

- `--project-name <name>`: `project.arch.yaml` の `project.name` を上書き

現時点で含めないもの:

- role 単位の直接 override フラグ
- 部分ファイルマージ編集セマンティクス
- 対話型 preset ウィザード

より深いカスタマイズは init 後に生成ファイルを直接編集します。

### 4. エラーハンドリングと discoverability

preset id が見つからない場合、init はエラー終了し、利用可能な preset id を表示します。

これにより、別 preset への暗黙フォールバックを避け、挙動を予測可能にします。

## 結果

- preset ベース onboarding は手動セットアップより高速になる。
- 将来 CLI 拡張のための明確な最小ターゲット挙動が定まる。
- `archflow init` は正規の開始コマンドとして維持される。
- 初期ブートストラップモデルの複雑性を意図的に低く保てる。
