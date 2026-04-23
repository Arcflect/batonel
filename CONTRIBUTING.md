# Contributing to Batonel

Thank you for your interest in Batonel.
This project is still early, so clear communication and small, well-scoped contributions are especially appreciated.

Batonel is an architecture-to-execution bridge for AI-assisted development.
It aims to turn architectural intent into artifact-level scaffolding, contracts, and AI handoff prompts.

---

## English

### Ways to contribute

You can contribute in several ways across the project's core areas:

**Core**:
- report core bugs
- propose CLI features
- refine schemas
- implement scoped CLI issues

**Presets & Governance**:
- suggest architecture rules
- propose new partner presets
- improve ecosystem compliance

**Docs & Examples**:
- improve core documentation
- improve examples
- clarify concept definitions

If you want to understand where contributions are especially helpful right now, see:

- `docs/contributing-areas.md`

### Before opening an issue

Please check whether the same topic already exists.
If not, choose the most appropriate issue type:

- **Bug report**: something is broken or inconsistent
- **Feature request**: a new capability or improvement
- **Architecture rule request**: a new rule, contract pattern, placement rule, or artifact policy

For changes that affect core concepts or project direction, it is usually better to open an issue or discussion before starting implementation.

### Before opening a pull request

Please keep pull requests focused.
For large changes, open an issue first so we can align on scope and direction.

This is especially important for changes that affect:

- core concepts
- schema design
- role naming
- contract philosophy
- prompt philosophy
- verification scope
- preset direction

Before opening a PR for those areas, please also review:

- `docs/decisions/README.md`

### Contribution principles

When contributing, please keep these principles in mind:

1. **Artifact-first**  
   Prefer explicit artifact contracts over implicit assumptions.

2. **Sidecar-first**  
   Prefer language-agnostic metadata such as YAML or Markdown when possible.

3. **Design before automation**  
   Avoid adding smart automation before the contract model is clear.

4. **Small and explainable**  
   Favor simple behavior that is easy to understand and verify.

5. **AI-friendly but human-readable**  
   Outputs should work well for AI tools, but must remain understandable to humans.

6. **Contracts are primary**  
   Prompts should be derived from contracts, not treated as the primary source of truth.

7. **Examples before presets**  
   Prefer strengthening examples and concepts before expanding preset machinery.

### What to read before contributing

Recommended reading:

- `README.md`
- `ROADMAP.md`
- `docs/roadmap-detail.md`
- `docs/schema-guide.md`
- `docs/architecture-flow.md`
- `docs/presets.md`

For concept-related contributions, also read:

- `docs/concepts/project.md`
- `docs/concepts/artifact.md`
- `docs/concepts/contract.md`
- `docs/concepts/prompt.md`

For architectural decisions, read:

- `docs/decisions/README.md`

### Pull request checklist

Before submitting a PR, please confirm:

- the change is scoped and explained
- documentation is updated if needed
- the change satisfies or updates the relevant [Acceptance Criteria](./docs/acceptance-criteria.md)
- naming is consistent
- the intent is clear for both humans and AI users
- examples are updated when behavior changes
- schemas are updated if structure changes
- decision records are reviewed if the change affects project direction

### Commit and PR style

Recommended style:

- `docs: improve README wording`
- `feat: add scaffold command skeleton`
- `fix: correct contract file path resolution`
- `design: revise artifact contract terminology`

Please prefer small PRs with clear intent over large mixed changes.

### Good first contributions

Good early contributions include:

- README refinements
- terminology clarification
- contract schema suggestions
- example project improvements
- issue template improvements
- schema wording cleanup
- documentation cross-link improvements

A good first contribution is one that improves clarity without outrunning the current model.

### Examples and docs should stay aligned

If your change affects behavior or structure, please check whether related files should also be updated.

Typical places to keep aligned:

- `examples/`
- `docs/concepts/`
- `schemas/`
- `docs/schema-guide.md`
- `docs/architecture-flow.md`

Not every PR needs to update all of them,
but major conceptual drift should be avoided.

### Communication

Please be respectful, specific, and constructive.

If something is unclear, asking a focused question is better than making a large assumption.

---

## 日本語

### 貢献のしかた

プロジェクトの主要領域を通じて、さまざまな形で貢献いただけます:

**Core**:
- コアのバグ報告
- CLI の機能提案
- schema の改善
- 小さく分割された CLI issue の実装

**Presets & Governance**:
- アーキテクチャルールの提案
- パートナー Preset の提案
- エコシステムのコンプライアンス改善

**Docs & Examples**:
- コアドキュメントの改善
- examples の改善
- 概念定義の明確化

いまどこに貢献しやすいかを知りたい場合は、次も見てください。

- `docs/contributing-areas.md`

### Issue を作る前に

同じ内容がすでに登録されていないか確認してください。
そのうえで、適切な issue 種別を選んでください。

- **Bug report**: 動作不良や不整合
- **Feature request**: 新機能や改善提案
- **Architecture rule request**: 配置ルール、contract パターン、artifact 方針の提案

コア概念やプロジェクトの方向性に関わる変更は、実装前に issue や discussion でそろえることをおすすめします。

### Pull Request を作る前に

PR はできるだけ小さく、目的が明確なものにしてください。
大きな変更は、先に issue で方向性をそろえてから進めてください。

特に、次に関わる変更は事前確認を推奨します。

- コア概念
- schema 設計
- role 名
- contract の考え方
- prompt の考え方
- verify の範囲
- preset の方向性

そのような変更を行う前には、次も確認してください。

- `docs/decisions/README.md`

### 貢献時の原則

1. **Artifact-first**  
   暗黙知ではなく、artifact 単位の契約を重視します。

2. **Sidecar-first**  
   可能な限り、言語非依存な YAML / Markdown などのメタ情報を優先します。

3. **Design before automation**  
   contract モデルが固まる前に、過度な自動化を入れないでください。

4. **Small and explainable**  
   複雑すぎる仕組みより、説明しやすく検証しやすい仕組みを優先します。

5. **AI-friendly but human-readable**  
   AI が使いやすいだけでなく、人間にも読みやすいことを大切にします。

6. **Contracts are primary**  
   prompt は contract から導出されるものであり、source of truth そのものではありません。

7. **Examples before presets**  
   preset を広げる前に、examples と concepts の整合を強くします。

### 貢献前に読んでほしいもの

おすすめの読み順:

- `README.md`
- `ROADMAP.md`
- `docs/roadmap-detail.md`
- `docs/schema-guide.md`
- `docs/architecture-flow.md`
- `docs/presets.md`

概念に関わる変更をする場合は、次も確認してください。

- `docs/concepts/project.md`
- `docs/concepts/artifact.md`
- `docs/concepts/contract.md`
- `docs/concepts/prompt.md`

設計判断に関わる変更をする場合は、次も確認してください。

- `docs/decisions/README.md`

### PR チェックリスト

PR を送る前に、次を確認してください。

- 変更範囲が明確か
- 必要なドキュメント更新があるか
- 関連する [受け入れ基準 (Acceptance Criteria)](./docs/acceptance-criteria.md) を満たしているか（または更新したか）
- 命名が一貫しているか
- 人間にも AI にも意図が伝わるか
- 振る舞いが変わるなら examples も更新されているか
- 構造が変わるなら schemas も見直されているか
- 方向性に関わるなら decision records を確認しているか

### コミット / PR のスタイル例

- `docs: improve README wording`
- `feat: add scaffold command skeleton`
- `fix: correct contract file path resolution`
- `design: revise artifact contract terminology`

大きな変更を一度に入れるより、小さく意図が明確な PR を歓迎します。

### 最初の貢献としておすすめ

- README の改善
- 用語整理
- contract schema の提案
- example project の改善
- issue template の改善
- schema 文言の改善
- docs 間リンクの整理

最初の貢献として強いのは、**現在のモデルを壊さずに明確さを上げる変更**です。

### examples と docs の整合について

振る舞いや構造に影響する変更では、関連ファイルの整合も意識してください。

主に見直す候補:

- `examples/`
- `docs/concepts/`
- `schemas/`
- `docs/schema-guide.md`
- `docs/architecture-flow.md`

すべての PR で全部を更新する必要はありませんが、
大きなズレが発生しないようにしてください。

### コミュニケーション

敬意を持ち、具体的で、建設的なコミュニケーションをお願いします。

不明点がある場合は、大きく仮定して進めるより、
先に小さく確認するほうが歓迎されます。