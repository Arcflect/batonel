# 0007 Batonel remains sidecar-first before code-aware analysis

- Status: accepted
- Date: 2026-03-28

## Context

Batonel is being designed as an architecture-to-execution bridge.

At this stage, its main assets are:

- project definitions
- placement rules
- artifact plans
- contracts
- prompts
- scaffold structure
- examples
- schema drafts

A natural future direction is code-aware analysis, such as:

- source file inspection
- import analysis
- dependency graph analysis
- AST-aware validation
- language-specific structural checks

These directions may become useful later, especially for stronger verification.

However, introducing code-aware analysis too early would change the center of gravity of the project.

It would create pressure toward:

- language-specific behavior
- parser-first design
- implementation-derived architecture interpretation
- heavier runtime complexity
- less useful pre-code workflows

This would be risky because Batonel’s current value comes from making architecture explicit before implementation is complete.

The project therefore needs a clear sequencing decision.

## Decision

Batonel remains **sidecar-first before code-aware analysis**.

This means the primary architectural records remain:

- project definition files
- placement rules
- artifact plans
- contract files
- prompt files

Source code may become an additional signal later,
but it is not the primary source of truth in early Batonel.

Code-aware analysis may be added in the future,
but only after the sidecar-centered model is stable enough.

In practical terms, the ordering is:

1. define architecture through structured files
2. generate scaffold, contracts, and prompts
3. verify structure and contract consistency
4. only later expand into optional code-aware analysis where useful

## Consequences

What becomes easier:
- preserving language-agnostic design
- supporting pre-implementation workflows
- keeping contracts and prompts central
- making examples meaningful even without full code
- keeping early verification focused and clear
- avoiding premature parser complexity

What becomes harder:
- validating implementation details directly from source code in early phases
- catching drift that only appears inside code
- providing advanced language-specific architectural enforcement immediately

This is an intentional tradeoff.

Batonel is designed to start from explicit architecture,
not reverse-engineer architecture from code.

## Why this fits Batonel

This decision aligns with earlier project decisions.

It supports the idea that:

- Batonel is an architecture-to-execution bridge
- contracts are the source of truth for artifact boundaries
- prompts are derived from contracts
- sidecar files are first-class
- verification starts with structure and contract consistency
- examples should stabilize before more operational expansion

This decision keeps the project coherent.

If Batonel became code-aware too early,
it could drift toward becoming a language-specific architecture analysis tool,
which is not its primary early purpose.

## What “sidecar-first” means in practice

Sidecar-first means that important architectural intent should live in files such as:

- `project.baton.yaml`
- `placement.rules.yaml`
- `artifacts.plan.yaml`
- `*.contract.yaml`
- `*.prompt.md`

It also means:

- code is not required for Batonel to be useful
- placeholders can still be meaningful
- the model remains usable in design-first workflows
- AI handoff can happen before full implementation exists

This is important because Batonel is meant to work in the phase between design and completed code.

## What this decision does not mean

This decision does **not** mean:

- Batonel will never inspect code
- code-aware validation is a bad idea
- language-specific integrations are forbidden
- source code should be ignored forever

It only means that code-aware analysis should remain **downstream** from the sidecar model.

Code-aware features should extend Batonel later.
They should not redefine its foundation too early.

## Alternatives considered

### Make code-aware analysis a primary early feature

Not chosen because it would push the project toward language-specific complexity before the core model is stable.

### Treat source code and contracts as equal sources of truth

Not chosen because this creates ambiguity in early phases and weakens the clarity of architectural ownership.

### Avoid code-aware analysis entirely

Not chosen because future code-aware checks may provide real value,
especially for stronger verification and ecosystem fit.

## Future direction

In the future, Batonel may support optional code-aware analysis such as:

- import-pattern checks
- lightweight dependency checks
- role-aware file inspections
- optional AST-backed verification
- ecosystem-specific structural validation

But these should remain additive layers.

The core flow remains:

**project -> placement rules -> artifact plan -> contract -> prompt -> scaffold -> verify**

Code-aware analysis, if added, should appear after this core flow is stable.

## Notes

This decision should guide future discussions about:

- verification scope
- language-specific adapters
- parser integration
- AST-backed validation
- ecosystem-specific enforcement

If future code-aware features are added,
they should be evaluated against this question:

Does this feature extend the sidecar-first model,
or does it try to replace it too early?

---

## 日本語

# 0007 Batonel はコード認識分析の前に sidecar ファーストのままである

- ステータス：採択済み
- 日付：2026-03-28

## コンテキスト

Batonel は設計から実行への橋渡しとして設計されています。

この段階でのメインの asset は次のとおりです。

- プロジェクト定義
- 配置ルール
- artifact プラン
- contract
- prompt
- スキャフォルド構造
- examples
- スキーマドラフト

自然な将来の方向性はコード認識分析であり、例えば：

- ソースファイルの検査
- インポート解析
- 依存グラフ解析
- AST 認識のバリデーション
- 言語固有の構造チェック

これらの方向性は後で、特により強力な verify のために有用になるかもしれません。

しかし、コード認識分析を早期に導入することでプロジェクトの重心が変わります。

これは次に向けたプレッシャーを生み出します。

- 言語固有の動作
- パーサーファーストの設計
- 実装から導出されたアーキテクチャの解釈
- 重いランタイムの複雑さ
- より有用でない pre-code ワークフロー

Batonel の現在の価値は、実装が完了する前にアーキテクチャを明示的にすることから来ているため、これは危険です。

プロジェクトにはそのため、明確なシーケンス決定が必要です。

## 決定

Batonel はコード認識分析の前に**sidecar ファースト**のままです。

これは主要なアーキテクチャの記録が次のものであり続けることを意味します。

- プロジェクト定義ファイル
- 配置ルール
- artifact プラン
- contract ファイル
- prompt ファイル

ソースコードは後で追加のシグナルになるかもしれませんが、初期の Batonel での主要な真実の源ではありません。

コード認識分析は将来追加されるかもしれませんが、sidecar 中心のモデルが十分に安定した後にのみです。

実践的には、順序は次のとおりです。

1. 構造化されたファイルを通じてアーキテクチャを定義する
2. スキャフォルド、contract、prompt を生成する
3. 構造と contract の整合を verify する
4. 後でのみ、有用な場合に任意のコード認識分析に拡張する

## 結果

容易になること：
- 言語非依存の設計を保全すること
- 実装前ワークフローをサポートすること
- contract と prompt を中心に保つこと
- コードが完全でなくても examples を意味のあるものにすること
- 早期の verify を焦点が当たってクリアにし続けること
- 早期のパーサーの複雑さを避けること

難しくなること：
- 初期フェーズでソースコードから実装の詳細を直接バリデートすること
- コード内にのみ現れるずれを捉えること
- すぐに高度な言語固有のアーキテクチャの強制を提供すること

これは意図的なトレードオフです。

Batonel は明示的なアーキテクチャから始まるように設計されており、コードからアーキテクチャをリバースエンジニアリングするためではありません。

## これが Batonel に合う理由

この決定は以前のプロジェクトの決定と整合します。

これは次のアイデアをサポートします。

- Batonel は設計から実行への橋渡しである
- contract は artifact の境界にとっての真実の源である
- prompt は contract から導出される
- sidecar ファイルはファーストクラスである
- verify は構造と contract の整合から始まる
- examples はより運用的な展開の前に安定すべきである

この決定はプロジェクトの一貫性を保ちます。

Batonel が早すぎてコード認識になれば、言語固有のアーキテクチャ解析ツールになる方向にずれる可能性があり、これは初期のプライマリ目的ではありません。

## 実践での「sidecar ファースト」の意味

Sidecar ファーストとは、重要なアーキテクチャの意図が次のようなファイルに存在すべきであることを意味します。

- `project.baton.yaml`
- `placement.rules.yaml`
- `artifacts.plan.yaml`
- `*.contract.yaml`
- `*.prompt.md`

これはまた次のことも意味します。

- Batonel が有用であるためにコードは必要ない
- 仮ファイルはまだ意味を持てる
- モデルは設計ファーストのワークフローで使用可能なままである
- 完全な実装が存在する前に AI ハンドオフができる

これは Batonel が設計と完成したコードの間のフェーズで機能することを意図しているため重要です。

## この決定が意味しないこと

この決定は次を意味**しません**。

- Batonel がコードを検査しない
- コード認識バリデーションが悪いアイデアである
- 言語固有の統合が禁止されている
- ソースコードを永遠に無視すべきである

これはコード認識分析が sidecar モデルの**下流**にとどまるべきであることを意味するだけです。

コード認識機能は後で Batonel を拡張すべきです。
早すぎてその基盤を再定義すべきではありません。

## 検討された代替案

### コード認識分析を主要な早期機能にする

選択されなかった理由：コアモデルが安定する前にプロジェクトを言語固有の複雑さに向かわせるため。

### ソースコードと contract を同等の真実の源として扱う

選択されなかった理由：初期フェーズで曖昧さが生じ、アーキテクチャの所有権の明確さが弱まるため。

### コード認識分析を完全に避ける

選択されなかった理由：将来のコード認識チェックは実際の価値を提供するかもしれず、特により強力な verify とエコシステムへの適合のためにそうかもしれないため。

## 将来の方向性

将来的に、Batonel は次のような任意のコード認識分析をサポートするかもしれません。

- インポートパターンチェック
- 軽量な依存関係チェック
- ロール認識のファイル検査
- 任意の AST サポートの verify
- エコシステム固有の構造バリデーション

しかし、これらは追加的なレイヤーにとどまるべきです。

コアフローは次のまま残ります。

**project -> placement rules -> artifact plan -> contract -> prompt -> scaffold -> verify**

コード認識分析は、このコアフローが安定した後に追加されるべきです。

## 注記

この決定は将来の議論を導くべきです。

- verify のスコープ
- 言語固有のアダプター
- パーサー統合
- AST サポートのバリデーション
- エコシステム固有の強制

将来のコード認識機能が追加される場合、次の質問に対して評価されるべきです。

この機能は sidecar ファーストのモデルを拡張するか、
または早すぎてそれを置き換えようとしているか？