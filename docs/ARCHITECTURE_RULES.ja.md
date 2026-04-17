# Batonel Architecture Rules

## 目的

本ドキュメントは、Batonel におけるアーキテクチャおよびコーディング上の必須ルールを定義するものです。

Batonel は CLI ツールですが、明確な境界を持つアプリケーションとして扱わなければなりません。
本プロジェクトは、**Modular Monolith** を前提にした **Hexagonal Architecture（Ports and Adapters）** を採用します。

この文書は、人間と生成 AI の両方を対象に書かれています。
コードの追加・編集・リファクタリングを行う際は、必ず本ルールに従ってください。

---

## 1. アーキテクチャ方針

Batonel は次のアーキテクチャを採用します。

- **スタイル**: Hexagonal Architecture（Ports and Adapters）
- **デプロイ形態**: Modular Monolith
- **相互作用モデル**: CLI の各コマンドを Application UseCase に対応させる
- **主目的**: アーキテクチャ判断、計画ロジック、検証ロジック、生成ロジックを I/O やフレームワーク詳細から独立させる

Batonel は単なる薄い CLI ラッパーではありません。
Batonel は、CLI を通して提供される「アーキテクチャ判断エンジン」です。

---

## 2. 基本原則

### 2.1 必須原則

コードベースは必ず次の原則に従うこと。

1. **Domain-first**
   - コアの業務ロジックは domain 層に置かなければならない。
   - domain ロジックは CLI、filesystem、network、外部サービスに依存してはならない。

2. **依存は内向き**
   - 依存関係は domain に向かって内側へ向かなければならない。
   - 外側の層は内側の層に依存してよい。
   - 内側の層は外側の層に依存してはならない。

3. **境界を明示する**
   - ファイル I/O、Git 実行、テンプレート描画、LLM 呼び出し、コンソール出力は、必ず ports / interfaces の背後に隔離しなければならない。

4. **UseCase 指向のアプリケーションフロー**
   - 各 CLI コマンドは application 層の UseCase を呼び出さなければならない。
   - CLI に業務ロジックを書いてはならない。

5. **モジュールとして成長させる**
   - 新機能は、責務と所有範囲が明確なモジュールとして追加しなければならない。
   - 共有コードは最小限に抑えなければならない。

---

## 3. レイヤ定義

Batonel のコードは、以下のレイヤに分離しなければなりません。

### 3.1 `cli/`

責務:
- 引数を解析する
- コマンドライン入力を application 入力へ変換する
- UseCase を呼び出す
- 結果を表示またはシリアライズする

ルール:
- 業務ルールを持ってはならない
- 明確に bootstrapping の範囲に限られない限り、直接ファイル読み書きをしてはならない
- UseCase が存在する場合、infra を直接呼び出してはならない
- 薄い層に保つべきである

### 3.2 `app/`

責務:
- アプリケーションの振る舞いを組み立てる
- domain サービスと ports を調停する
- UseCase を定義する
- 入出力 DTO を変換する

ルール:
- 複数の domain object や port を協調させてもよい
- 低レベルな I/O 実装詳細を持ってはならない
- domain に属すべき業務ルールの捨て場所になってはならない
- ワークフローを明快に表現すべきである

### 3.3 `domain/`

責務:
- コア概念
- 判断ロジック
- ルール
- 検証
- 計画ロジック
- 不変条件

ルール:
- `clap`、`tokio`、`std::fs`、`reqwest`、git コマンド実行、端末描画に依存してはならない
- データの取得元や、結果の書き込み先を知ってはならない
- Batonel において最も重要な業務ロジックを含まなければならない

### 3.4 `ports/`

責務:
- 外界とのインターフェースを定義する

例:
- filesystem access
- git access
- llm client
- template rendering
- output rendering
- 必要に応じた repository 的抽象

ルール:
- traits または同等の抽象でなければならない
- 実装ではなく能力を表現しなければならない
- 安定かつ最小でなければならない

### 3.5 `infra/`

責務:
- ports の具体実装
- 実際の filesystem access
- 実際の git 実行
- 実際の API client
- 実際の template engine
- 実際の console formatter

ルール:
- ports を実装しなければならない
- コアのアーキテクチャ判断ロジックを持ってはならない
- テストで置き換え可能であるべきである

### 3.6 `shared/`

責務:
- 本当に横断的で安定したプリミティブのみを置く

ルール:
- 小さく保たなければならない
- 隠れた依存バケツになってはならない
- 無関係な utility 関数の置き場にしてはならない

---

## 4. 必須ディレクトリ構成

推奨構成は次のとおりです。

```text
src/
├─ main.rs
├─ cli/
│  ├─ mod.rs
│  ├─ args.rs
│  └─ commands/
├─ app/
│  ├─ mod.rs
│  ├─ dto/
│  └─ usecase/
├─ domain/
│  ├─ mod.rs
│  ├─ project/
│  ├─ preset/
│  ├─ planning/
│  ├─ validation/
│  └─ generation/
├─ ports/
│  ├─ mod.rs
│  ├─ filesystem.rs
│  ├─ git.rs
│  ├─ llm.rs
│  ├─ template.rs
│  └─ output.rs
├─ infra/
│  ├─ mod.rs
│  ├─ filesystem/
│  ├─ git/
│  ├─ llm/
│  ├─ template/
│  └─ output/
└─ shared/
   ├─ error.rs
   └─ result.rs
```

この構成は、強い理由が文書化されていない限り、提案ではなくルールとする。

---

## 5. 依存ルール

以下の依存ルールは必須です。

### 許可される方向

- `cli -> app`
- `app -> domain`
- `app -> ports`
- `infra -> ports`
- `infra -> domain`（データ変換や adapter 統合で必要な場合のみ）
- `main -> cli`, `main -> app`, `main -> infra`

### 禁止される方向

- `domain -> app`
- `domain -> cli`
- `domain -> infra`
- `ports -> infra`
- `ports -> cli`
- `app -> cli`

### 重要な注意

domain object が外部からのデータを必要とする場合、application 層が port を通じてそのデータを取得し、それを domain に渡さなければならない。
domain が自力でデータ取得を行ってはならない。

---

## 6. 機能モデリングのルール

Batonel の機能は、技術的仕組みではなく、概念を中心にモデリングしなければならない。

### 良いモジュール名の例

- `project`
- `preset`
- `planning`
- `validation`
- `generation`

### 悪いモジュール名の例

- `helpers`
- `common_utils`
- `services`
- `misc`
- `manager`
- `processor`

汎用名は責務を隠す。
責務はモジュール名から見えるようにしなければならない。

---

## 7. コマンド設計ルール

各トップレベル CLI コマンドは、必ず 1 つの UseCase に対応しなければならない。

例:

- `batonel init` -> `InitProjectUseCase`
- `batonel plan` -> `PlanArchitectureUseCase`
- `batonel validate` -> `ValidateProjectUseCase`
- `batonel generate` -> `GenerateArtifactsUseCase`

### 必須ルール

- コマンドハンドラは薄く保たなければならない
- コマンドハンドラは以下のみを行うこと:
  - CLI 引数を受け取る
  - 入力 DTO に変換する
  - UseCase を呼ぶ
  - 出力を描画する
- コマンドハンドラは以下を行ってはならない:
  - アーキテクチャ判断ロジック
  - 複雑な分岐ルール
  - 明示的な bootstrapping を除く外部システムの直接操作

---

## 8. Domain ルール

domain 層は Batonel の中心です。

### Domain に含めるべきもの

- project model
- preset model
- plan model
- validation rules
- architecture decisions
- recommendation rules
- invariants
- rule evaluation logic

### Domain に含めてはならないもの

- terminal formatting
- JSON/YAML file reading logic
- HTTP calls
- shell command execution
- git process execution
- 可能な限り避けられる direct logging dependency
- 現実的に代替不能でない限り、framework 固有型

### Domain のコーディングルール

- Domain object は小さく明示的であるべきである
- Domain service は可能な限り決定的であるべきである
- Domain logic は filesystem や network なしでテスト可能でなければならない
- 隠れた副作用は禁止

---

## 9. Port と Adapter のルール

### Ports

Ports は、必要な能力を定義する。

例:
- `FileSystem`
- `GitClient`
- `TemplateRenderer`
- `LlmClient`
- `OutputWriter`

### Port 設計ルール

- Port は 1 つの責務に集中していなければならない
- Port は実装詳細ではなく意図を表現しなければならない
- やむを得ない場合を除き、adapter 固有の詳細を露出してはならない
- 巨大な “god port” は禁止

### Adapters

Adapters は ports を実装する。

例:
- local filesystem adapter
- process-based git adapter
- OpenAI adapter
- local model adapter
- markdown renderer adapter
- console writer adapter

### Adapter ルール

- Adapter は必ず port を実装しなければならない
- Adapter は業務方針を定義してはならない
- Adapter は third-party error を internal error に変換してよい
- Adapter はテストで差し替え可能であるべきである

---

## 10. 設定ファイルのルール

Batonel は `project.baton.yaml` のようなプロジェクト設定ファイルを読む可能性が高い。

### 必須ルール

- 生の config file の parsing は domain の外で行うこと
- 構造上の妥当性チェックは config loading の近くで行ってよい
- 意味解釈は責務に応じて domain または app に置くこと
- 生の config format を全体へ漏らしてはならない

推奨パターン:

1. raw file を読む
2. raw config struct に parse する
3. domain に関係する config model に変換する
4. planning / validation logic を実行する

Parsing と業務判断を 1 つの関数に混在させてはならない。

---

## 11. 出力ルール

Batonel は text, markdown, json など複数の出力形式を持ちうる。

### 必須ルール

- Domain object は CLI 表示用の整形を自分で行ってはならない
- 出力整形は output adapter または presentation mapping code に属する
- UseCase は、可能な限り整形済み terminal string ではなく構造化結果を返すべきである
- `--json` 互換性は合理的な範囲で維持すべきである

悪い例:
- UseCase が装飾済み terminal string を返す

良い例:
- UseCase が構造化結果を返す
- output layer が text/json/markdown に整形する

---

## 12. エラーハンドリングのルール

### 必須ルール

- エラーは明示的でなければならない
- エラー境界は明確でなければならない
- infra のエラーは application にとって扱いやすいエラーへ変換すべきである
- 無言失敗は禁止

### 推奨アプローチ

可読性が向上するなら、domain 固有・application 固有の error type を用いること。

例:
- `ConfigLoadError`
- `PresetResolutionError`
- `PlanBuildError`
- `ValidationError`
- `GenerationError`

### 禁止パターン

- どこでも曖昧な文字列エラーだけを返す
- エラーを握りつぶす
- 想定内失敗で panic する
- user-facing message と internal diagnostic を構造なく混在させる

---

## 13. テストルール

Batonel は判断ツールであるため、テストは必須である。

### Domain テスト

必ず検証すべきもの:
- rule behavior
- planner decisions
- validator results
- preset resolution
- invariants

これらのテストは、filesystem・network・git 依存なしで動かなければならない。

### Application テスト

必ず検証すべきもの:
- usecase orchestration
- port interaction
- result mapping

これらのテストでは fake や mock を用いるべきである。

### Integration テスト

検証してよいもの:
- config loading
- real filesystem interaction
- real command behavior
- generated artifacts

### 必須原則

Batonel の判断の中心に近いロジックほど、integration test のみに依存してはならない。

---

## 14. 命名ルール

### モジュール

domain 指向の名前を使うこと。

良い例:
- `planning`
- `validation`
- `preset`
- `project`

悪い例:
- `utils`
- `stuff`
- `helper`
- `service_layer`

### 型名

型名は責務を明らかにしなければならない。

良い例:
- `ArchitecturePlanner`
- `PresetResolver`
- `ProjectConfig`
- `ValidationResult`

悪い例:
- `Manager`
- `Processor`
- 何を扱うか不明な `Handler`
- 意味が曖昧な `Service`

### 関数名

関数名は業務上の意図を表すこと。

良い例:
- `build_plan`
- `resolve_preset`
- `validate_project`
- `generate_artifacts`

悪い例:
- `handle`
- `run_all`
- `process_data`
- `do_work`

---

## 15. AI Coding Rules

以下のルールは、このリポジトリを編集する生成 AI 向けの特別ルールである。

### AI MUST

- レイヤ境界を維持すること
- 業務ロジックを domain に置くこと
- orchestration を app に置くこと
- I/O を ports と infra adapters の背後に置くこと
- 小さく集中した module を作ること
- 汎用抽象より明示的な名前を優先すること
- 判断ロジックを変更したら tests を追加または更新すること
- command handler を薄く保つこと

### AI MUST NOT

- `main.rs` に業務ロジックを置いてはならない
- planning や validation のルールを CLI code に置いてはならない
- 強く具体的な理由がない限り `utils.rs` を追加してはならない
- domain から直接 filesystem/network/process logic を呼んではならない
- 実際の境界問題を解決しない抽象を追加してはならない
- 何でも知っている “god object” を作ってはならない
- domain logic から terminal 装飾済み出力を返してはならない

### AI SHOULD

- 可能な限り決定的な domain logic を優先する
- 構造化された戻り値を優先する
- parsing と interpretation を分離する
- trait は最小限に保つ
- adapter を差し替え可能に保つ
- 自明でない境界判断は文書化する

---

## 16. アンチパターン

以下はアーキテクチャ違反とみなす。

### 禁止アンチパターン

1. **Fat CLI**
   - command handler が実質的な業務判断をしている状態

2. **Anemic boundary collapse**
   - domain logic と file parsing / writing が混在している状態

3. **Utility dumping ground**
   - 所有者不明の `utils` や `common` が膨らみ続ける状態

4. **Adapter intelligence**
   - infra が業務ルールを決めている状態

5. **Stringly-typed flow**
   - 明示的な domain type を使わず raw string を至る所で受け渡す状態

6. **Premature framework lock-in**
   - 特定ライブラリや外部 API に合わせて domain を設計する状態

7. **Feature scattering**
   - 1 つの機能が所有関係なく無秩序に複数ディレクトリへ散らばる状態

---

## 17. 新機能追加時の手順

新機能を実装するときは、必ず次の順序で考えること。

1. 機能を domain の言葉で定義する
2. UseCase を特定する
3. 必要な ports を定義する
4. domain logic を実装する
5. application orchestration を実装する
6. adapters を実装する
7. tests を追加する
8. CLI から公開する

### 確認チェックリスト

- これはどんな domain concept か？
- 新しい UseCase か、既存の一部か？
- 外部 I/O を必要とするか？
- 必要なら既存 port はあるか？
- なければ新規 port を追加すべきか？
- コア判断ロジックは real I/O なしでテストできるか？

---

## 18. 軽量な判断ヒューリスティクス

迷ったときは次で判断すること。

### `domain/` に置くべきもの

- 「正しいアーキテクチャ判断は何か？」に答えるコード
- ルールを検証するコード
- preset を解決するコード
- plan を構築するコード
- 不変条件を守るコード

### `app/` に置くべきもの

- 「どの順番で処理を進めるか？」に答えるコード
- ports と domain services を協調させるコード
- 入出力をマッピングするコード

### `infra/` に置くべきもの

- 「外部のものを実際にどう読む・書く・呼ぶか？」に答えるコード

### `cli/` に置くべきもの

- 「ユーザーが terminal からどう呼び出すか？」に答えるコード

---

## 19. 最小品質ゲート

次の条件をすべて満たさない変更は merge してはならない。

- レイヤ境界が守られている
- 新しいアーキテクチャ違反を導入していない
- command handler が薄いままである
- domain logic が real I/O なしでテスト可能である
- 命名が責務駆動になっている
- 新しい外部アクセスが適切に抽象化されている
- 判断ロジックを変えた箇所に tests が追加または更新されている

---

## 20. 最終ルール

利便性とアーキテクチャ整合性が衝突した場合、文書化された強い理由がない限り、アーキテクチャ整合性を優先すること。

Batonel は、アーキテクチャ思考のためのツールである。
その内部構造自体が、プロダクトの信頼性の一部である。
