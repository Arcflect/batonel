# 0005 Examples precede presets

- Status: accepted
- Date: 2026-03-28

## Context

Archflow already has examples such as:

- minimal
- generic-layered
- rust-clean-hexagonal

A future preset system is likely,
but implementing presets too early would risk locking unstable concepts into reusable packages before the model is mature enough.

The project needs a sequencing decision.

## Decision

Examples come before presets.

This means:

- examples are the first step for teaching and exploring the model
- examples should stabilize naming, structure, and expectations
- presets should be introduced only after examples prove reusable patterns

Examples are descriptive first.
Presets are operational later.

## Consequences

What becomes easier:
- learning from concrete cases
- refining the model before operational packaging
- avoiding premature preset rigidity
- evolving role naming and contract defaults safely

What becomes harder:
- offering instant reusable preset bootstrap early
- optimizing for adoption speed before conceptual stability

This sequencing keeps the project aligned with its layered roadmap.

## Alternatives considered

### Build presets immediately

Not chosen because the concept model is still stabilizing.

### Ignore presets entirely

Not chosen because reusable starting points are likely to become important later.

### Treat examples and presets as the same thing

Not chosen because they serve different purposes.
Examples teach.
Presets operationalize.

## Notes

This decision supports the current documentation direction in `docs/presets.md`.

---

## 日本語

# 0005 Examples は preset より先に来る

- ステータス：採択済み
- 日付：2026-03-28

## コンテキスト

Archflow にはすでに次のような examples があります。

- minimal
- generic-layered
- rust-clean-hexagonal

将来の preset システムが見込まれますが、
モデルが十分に成熟する前に不安定な概念を再利用可能なパッケージにロックするリスクがあるため、
preset を早期に実装することは避けた方が良いでしょう。

プロジェクトにはシーケンスの決定が必要です。

## 決定

Examples は preset より先に来ます。

これが意味することは：

- examples はモデルを教え、探索するための最初のステップである
- examples は命名、構造、期待値を安定させるべきである
- preset は examples が再利用可能なパターンを証明した後にのみ導入されるべきである

Examples は最初に記述的です。
Preset は後で運用的になります。

## 結果

容易になること：
- 具体的なケースから学ぶこと
- 運用的なパッケージングの前にモデルを洗練させること
- 早期の preset の硬直性を避けること
- ロール命名と contract のデフォルトを安全に進化させること

難しくなること：
- 早期に即座に再利用可能な preset のブートストラップを提供すること
- 概念的安定性の前に採用速度を最適化すること

このシーケンスは層状のロードマップとプロジェクトを整合させます。

## 検討された代替案

### すぐに preset を構築する

選択されなかった理由：概念モデルがまだ安定していないため。

### preset を完全に無視する

選択されなかった理由：再利用可能な出発点は後で重要になる可能性が高いため。

### examples と preset を同じものとして扱う

選択されなかった理由：それらは異なる目的を果たすため。
Examples は教えます。
Preset は運用化します。

## 注記

この決定は `docs/presets.md` の現在のドキュメントの方向性をサポートします。