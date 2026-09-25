# Changelog

All notable changes to the Liquid ADE project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Root workspace automation in `Justfile` (`just check`, `just lint`, `just fmt`, `just test`, `just dev`, `just build`).
- Code formatting enforcement: `rustfmt.toml` for Rust backend and `.prettierrc` for React/TypeScript frontend.
- Branch-based development policy: feature branches (`feat/<epic-slug>`) merged via `--no-ff` back into `main`.
- Dual-Ledger Architecture (`ADR-008`): Separation of volatile streaming transcript from schema-bounded durable SQLite state ledger.
- Living Spec Canvas & 1-Click Interactive Task Runner specification (`epic_08_liquid_board_and_task_runner`):
  - Action Pill above task title (`▶ Start task` → `🔄 Task in progress` → `✅ Task completed [View changes] [View execution]`).
  - Right execution drawer with Steering Documents pills and streaming action cards (Read, Edit with diff, Diagnostics, Command with terminal logs).
- Embedded SQLite Engine specification (`ADR-007`) for Project Registry, LLM provider settings, and Execution Checkpoints.
- Multi-LLM Gateway specification (`ADR-005`) with Google Gemini REST streaming API.
- C4 Level 1 and Level 2 architectural diagrams and specifications in `architecture.md`.
- Canonical SCPE v0.3.0 Upstream documents (`product_vision.md`, `architecture.md`, `technical_deal.md`, `roadmap.md`, `glossary.md`).
