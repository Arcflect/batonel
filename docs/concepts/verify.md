# Verify

## Overview

In Batonel, **Verify** is the process of checking whether architectural structure
and artifact definitions remain consistent over time.

Verification helps ensure that the project still matches the architecture
described by its project files, placement rules, artifact plans, contracts,
prompts, and scaffold expectations.

Verify is not only about detecting errors.
It is about preserving architectural intent as the project evolves.

---

## Purpose

The purpose of verify is to protect consistency between architectural definitions
and the project state.

Verify answers questions such as:

- Do the expected files exist?
- Do artifact roles remain aligned across files?
- Do resolved paths still match placement rules?
- Do contracts include the required fields?
- Do prompts exist where expected?
- Are artifact statuses internally consistent?

Without verification, architecture can drift into:

- missing files
- inconsistent naming
- contract gaps
- stale prompts
- structural mismatches
- unclear implementation state

---

## Responsibilities

Verify is responsible for checking whether the Batonel model remains coherent.

It is responsible for validating things such as:

- required file presence
- role consistency
- path consistency
- contract completeness
- prompt presence
- status consistency
- scaffold consistency

Verify is not responsible for inventing architecture.
It checks the architecture that has already been defined.

Verify is also not, at least initially, a full code-analysis engine.

---

## Core idea

The core idea of verify is simple:

**architecture should remain inspectable and enforceable after generation**

Batonel is not meant to stop at scaffold generation.
It should also help users detect when the repository drifts away
from its explicit architectural model.

This is especially important in AI-assisted workflows,
where implementation can happen quickly and drift can happen quietly.

---

## Relationship to other concepts

Verify is downstream from the rest of the Batonel model.

The relationship is:

- the project defines the architectural frame
- modules organize functional areas
- roles classify artifacts
- placement rules define structure
- artifact plans define what should exist
- contracts define boundaries
- prompts define AI handoff
- scaffolds materialize structure
- verify checks whether these remain consistent

This makes verify the main protective layer of the model.

---

## Why verify matters

Batonel is built around explicit architecture.

That explicit architecture should not live only at generation time.
It should continue to matter as the project changes.

Verify matters because it helps answer:

- Has the structure drifted?
- Are artifact contracts missing?
- Are prompts still aligned with artifacts?
- Does the project still reflect the intended architecture?

Without verify, the explicit model can slowly become decorative rather than operational.

---

## What verify should check first

In its earliest form, verify should focus on structure and contract consistency.

Examples of early checks include:

- required input files exist
- planned artifacts have corresponding contract files
- contract names match artifact names
- contract roles match artifact roles
- contract modules match artifact modules
- artifact paths match placement rules
- required contract fields are present
- expected prompt files exist
- status values are present and internally consistent

These checks are enough to make the architectural model operational
without requiring deep source code understanding.

---

## What verify should not start with

Verify should not begin with:

- compiler integration
- AST parsing
- import graph analysis
- framework-specific inspection
- deep language-specific static analysis
- full architecture linting from code alone

Those may become useful later,
but they should not replace the sidecar-first and contract-centered model too early.

Batonel begins from architecture,
so verify should begin from architecture too.

---

## Verify vs contract

Verify is not the same as a contract.

- a **contract** defines what an artifact is responsible for
- **verify** checks whether that contract exists, is complete, and remains aligned

Contracts define the intended boundary.
Verify protects the continued integrity of that boundary.

---

## Verify vs scaffold

Verify is not the same as scaffold.

- a **scaffold** creates structure
- **verify** checks whether that structure still matches the model

Scaffold is generative.
Verify is protective.

The two are closely related, but they serve different purposes.

---

## Verify vs code-aware analysis

Verify is broader than code-aware analysis, but also starts earlier.

- **early verify** focuses on structure and sidecar consistency
- **later verify** may optionally include code-aware checks

This distinction matters because Batonel is designed to be useful
before full implementation exists.

That means verify must remain meaningful even when:

- code is incomplete
- placeholders are still present
- implementation is only partially started

This is why verify starts from architectural files,
not from source code parsing.

---

## Design principles

Verify should be:

- explicit
- understandable
- aligned with the documented model
- useful before deep code analysis exists
- strict enough to catch drift
- simple enough to explain and trust

A good verify system should make architectural consistency visible
without becoming mysterious or overly heavy.

---

## What verify should not do

Verify should not:

- become the primary place where architecture is defined
- invent new rules not present in the documented model
- depend too early on one language ecosystem
- assume production code is already complete
- become so complex that contributors cannot understand what is being checked

Verification should strengthen clarity, not hide it.

---

## Examples of verification questions

Examples of useful verification questions include:

- Does every artifact in `artifacts.plan.yaml` have a corresponding contract?
- Does each contract path align with the role-to-path mapping?
- Does each prompt correspond to a real artifact contract?
- Are required contract fields present?
- Are status values valid?
- Are examples internally consistent with the documented schemas?

These are the kinds of checks that make Batonel operational over time.

---

## Why verify matters for AI-assisted development

In AI-assisted workflows, many changes may be made quickly across multiple artifacts.

That creates a risk that:

- files are created in the wrong place
- contracts are skipped
- prompts become stale
- naming drifts
- intended architectural boundaries weaken

Verify helps reduce that risk by checking the explicit model regularly.

This is especially useful when lightweight models are used,
because those workflows benefit from tighter guardrails.

---

## Future directions

In the future, verify may also support:

- optional code-aware checks
- import-pattern checks
- dependency boundary checks
- role-aware file inspections
- CI integration
- preset-aware verification defaults
- richer lifecycle validation
- project health summaries

Even if those features grow later, the basic purpose remains the same:

verify protects the consistency of the architecture-to-execution model.

---

## Summary

Verify is the consistency-checking layer of Batonel.

It exists to ensure that project structure, contracts, prompts,
and scaffold outputs remain aligned with architectural intent.

If you remember only one thing, remember this:

**scaffold creates the structure, verify protects it over time**

---

## 日本語

# Verify（検証）

## 概要

Batonel における **Verify** は、プロジェクト構造と artifact の定義が時間をかけて整合したままであるかどうかをチェックする操作です。

verify は次のような問いに答えます。

- 定義された artifact に対応するファイルは存在するか？
- ロール、パス、contract が一貫しているか？
- 実際の構造はアーキテクチャの意図と整合しているか？

Verify は Batonel のアーキテクチャメモリを保護する方法です。

---

## 目的

verify の目的は、アーキテクチャの整合性の崩壊を早期に検出することです。

verify は次のような問いに答えます。

- すべての計画された artifact に contract ファイルがあるか？
- ロール名はファイル全体で一貫しているか？
- artifact パスは配置ルールと一致しているか？
- 必要な contract フィールドは存在するか？
- 期待される場所に prompt ファイルは存在するか？

Verify がなければ、contract と prompt は実際の構造からずれる可能性があります。

---

## 責務

Verify は次の責務を持ちます。

- 必要な入力ファイルの存在をチェックする
- artifact と contract の整合性をチェックする
- ロールとパスの整合性をチェックする
- 必須の contract フィールドの存在をチェックする
- prompt の存在と導出の整合性をチェックする
- artifact ステータスの整合性をチェックする

Verify は次の責務を持ちません。

- 実際のビジネスロジックを実装すること
- コードのコンパイルや実行をチェックすること（最初のフェーズでは）
- ソースコード内の実装のずれを検出すること（最初のフェーズでは）

---

## Verify と Scaffold の関係

Scaffold と verify は異なる目的を果たします。

- **scaffold** は構造を作成します
- **verify** はその構造が時間とともに整合したままかどうかをチェックします

一般的なワークフローは次のとおりです。

1. 定義ファイルを書く（project、placement rules、artifacts）
2. スキャフォルドを実行する
3. Contract と prompt を生成または洗練させる
4. Verify を実行して整合性を確認する
5. 変更を加え、整合性を維持するために定期的に verify を再実行する

---

## Verify がチェックすること（最初のスコープ）

Batonel の最初の verify フェーズは次に焦点を当てます。

### 1. 必須入力ファイルの存在

- `project.baton.yaml` が存在する
- `placement.rules.yaml` が存在する
- `artifacts.plan.yaml` が存在する

### 2. Artifact-contract の整合性

- すべての計画された artifact に contract がある
- contract の名前が artifact と一致する
- contract のロールが artifact と一致する

### 3. ロール-パスの整合性

- artifact のロールが配置ルールに存在する
- 解決されたパスが配置ルールと一致する

### 4. 必須 contract フィールドの存在

- `name`、`module`、`role`、`path`、`responsibilities`、`must_not`、`status`

### 5. Prompt の整合性

- 期待される prompt ファイルが存在する
- prompt の artifact 名が contract と整合する

### 6. ステータスの整合性

- artifact と contract のステータスが矛盾しない

---

## Verify の将来の拡張

将来的に、verify は次のものをサポートするかもしれません。

- 任意のコード認識チェック
- 依存関係境界のバリデーション
- CI 統合
- 言語固有の構造チェック

しかし、最初の verify フェーズは sidecar ファーストモデルの構造的整合性をサポートします。

---

## 他の概念との関係

Verify は次のものに依存します。

- **project**: verify が行われるコンテキスト
- **artifact プラン**: 何が存在すべきかを定義する
- **placement rule**: artifact パスを解決するために使用される
- **contract**: verify の主要な真実の源
- **scaffold**: verify がチェックする構造を生成する

Verify は Batonel の一貫性チェックレイヤーです。

プロジェクト構造、contract、prompt、スキャフォルド出力がアーキテクチャの意図と整合したままであることを確保するために存在します。

1 つだけ覚えておくなら、これを覚えてください。

**スキャフォルドは構造を作成し、verify はそれを時間とともに保護する**