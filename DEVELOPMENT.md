# Development Guide / 開発ガイド

English

## Local Environment
- OS: Windows (initial), Linux/macOS later. Use UTF-8 and LF line endings.
- Git: Ensure `core.autocrlf=false` to avoid CRLF surprises (we normalize via `.gitattributes`).
- Editors: Respect `.editorconfig` (UTF-8, LF, 2-space indent). No trailing whitespace.

## Principles
- No C/C++ or C-based binary dependencies in the implementation.
- Specification-first development; changes must trace back to requirements/design.
- Small, focused commits in clear English using Conventional Commits.

## Workflow
- Branch from `master`: `feat/*`, `fix/*`, `docs/*`, etc.
- Open PR early; keep scope narrow; link to spec sections.
- CI must be green (policy checks, lint) before merge.

## Directories
- `spec/` and `TODO.md` are local-only (ignored). Do not commit them.
- Add any public stable docs under `docs/`.

日本語

## ローカル環境
- OS: まず Windows。UTF-8 と LF を使用。
- Git: `core.autocrlf=false` を推奨（`.gitattributes` で正規化）。
- エディタ: `.editorconfig` に従う（UTF-8, LF, 2 スペース）。

## 原則
- 実装に C/C++ および C 系依存を持ち込まない。
- 仕様ドリブンで開発し、要件/設計にトレース可能に。
- Conventional Commits による小さく明確な英語コミット。

## 手順
- `master` からブランチ作成（`feat/*`, `fix/*`, `docs/*` など）。
- PR は早めに、小さな範囲で、仕様へのリンクを添える。
- マージ前に CI（ポリシー/リント）をグリーンにする。

## ディレクトリ
- `spec/` と `TODO.md` はローカル専用（追跡禁止）。
- 公開に耐える安定した文書は `docs/` 配下に置く。
