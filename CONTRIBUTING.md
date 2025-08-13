# Contributing to Selenet

This document defines contribution rules, commit conventions, code style, internationalization policy, and review process. Keep commits small, logically scoped, and written in clear English.

概要（日本語）: 本書はコントリビューション規約、コミット規約、コード規約、多言語方針、レビュー手順を定めます。コミットは小さく論理的に分割し、英語で記述してください。

## 1. Principles / 原則
- Security by design; privacy-first.
- Standards-driven implementation.
- No C/C++ or C-based binary dependencies.
- Reproducibility, clarity, and traceability.

## 2. Branching / ブランチ運用
- Use short-lived feature branches from `master`.
- Keep PRs focused and reviewable.

## 3. Commit Messages (English only) / コミットメッセージ（英語限定）
Conventional Commits を採用します。形式:

<type>(optional scope): <summary>

<body>

<footer>

Types: feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert

Rules:
- Summary: imperative mood; <= 72 chars preferred.
- Body: explain What/Why/How; include breaking changes clearly.
- Footer: link issues (e.g., Closes #123).

## 4. Code Style / コード規約
- Naming: descriptive, full words; avoid 1–2 char identifiers.
- Control flow: guard clauses; handle errors early; never swallow errors.
- Comments: concise English only; explain "why", not trivial "how".
- Formatting: honor existing style; prefer multi-line over dense one-liners.
- Do not leave TODOs in code; implement fully or open an issue.

## 5. Internationalization / 多言語対応
- Source comments: English only.
- User-facing docs: English + Japanese when feasible.
- UI strings: externalize into locale resources during implementation.

## 6. Tests / テスト
- Add unit/integration tests with features when applicable.
- Cover positive/negative/edge cases and perf-sensitive paths.

## 7. Security / セキュリティ
- Follow least-privilege and safe-by-default.
- Avoid unsafe patterns; prefer memory-safe languages and constructs.

## 8. Review Process / レビュー
- Self-review before opening a PR.
- Provide reproduction steps or benchmarks if relevant.
- Address feedback promptly; keep commits granular.

## 9. Files intentionally untracked / 追跡しないファイル
The following MUST remain untracked:
- `spec/` (local design docs)
- `TODO.md` (local planning board)

If public, stable excerpts are needed, extract them into tracked docs (e.g., `docs/`) without violating the exclusion policy above.
