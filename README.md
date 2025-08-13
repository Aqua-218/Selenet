Selenet Browser
================

Overview (English)
------------------
Selenet is an experimental, privacy-first, performance-oriented web browser project aiming for high compatibility with open standards. The long-term goal is a fully self-hosted stack (HTML/CSS/JS engine, networking, storage, UI/DevTools) without C/C++ or C-based binary dependencies.

- Standards-driven: HTML, DOM, CSS, URL, Fetch, HTTP/1.1–3, TLS 1.3, ECMAScript.
- Security by design: site isolation, sandboxing, strict policies (CSP/COOP/COEP/CORP, etc.).
- Internationalization and accessibility as first-class citizens.
- Transparent implementation with reproducible builds.

Note: Detailed specifications and plans live locally under `spec/` and `TODO.md`. They are intentionally excluded from version control to keep the public history focused. See "Local Documentation" below.

概要（日本語）
--------------
Selenet は、プライバシーを最優先しつつ高性能と互換性を追求する新世代ブラウザの実験的プロジェクトです。長期的には、C/C++ および C 系バイナリ依存に頼らない完全自前実装のスタック（HTML/CSS/JS エンジン、ネットワーク、ストレージ、UI/DevTools）を目指します。

- 仕様準拠: HTML, DOM, CSS, URL, Fetch, HTTP/1.1–3, TLS 1.3, ECMAScript。
- セキュリティ重視: サイト分離、サンドボックス、厳格なポリシー（CSP/COOP/COEP/CORP 等）。
- 国際化/アクセシビリティを第一級に扱う設計。
- 再現ビルドによる透明性。

Project Structure
-----------------
This repository intentionally starts minimal. Core components and code will be added step by step as specification-driven milestones graduate from design to implementation.

- `.editorconfig`: Shared editor conventions.
- `.gitattributes`: Line-ending normalization and diff/syntax hints.
- `.gitignore`: Keeps local specs (`spec/`) and task board (`TODO.md`) untracked.
- `spec/` (ignored): Architecture and detailed designs.
- `TODO.md` (ignored): Phase checklists and milestones.

Local Documentation
-------------------
The following files are intentionally untracked but present locally:

- `spec/design.md`: High-level and component-level design.
- `spec/requirements.md`: Requirements and acceptance criteria.
- `TODO.md`: Phased task checklists and targets.

To view them, open the files directly on your machine. If you need to include parts of these documents in version control, extract stable, public-friendly sections into tracked docs under `docs/` (to be added later) to avoid exposing volatile or internal-only material.

Internationalization
--------------------
Repository documentation will be written in both English and Japanese where appropriate. UI strings and locale resources will be externalized during the implementation phases.

Contributing
------------
Small, focused commits with clear English messages are required. Keep changes logically scoped and traceable back to specifications. Avoid introducing C/C++ or C-based binary dependencies.


