# Technical Deal: Liquid ADE

Technical agreements every Developer and agent follows. Code in `apps/` is generated
against this file.

## Approved Stack

- **Backend / Core:**
  - Language: Rust 2021 edition
  - HTTP / WebSocket Framework: Axum 0.8
  - Async Runtime: Tokio
  - Embedded Database: SQLite via `rusqlite` (`features = ["bundled"]`) for Project Registry, IDE settings, and Agent Checkpoints
  - Asset Embedding: `rust-embed`
  - Serialization: Serde, `serde_json`
  - Filesystem Monitoring: `notify`
- **Frontend / Client:**
  - Language: TypeScript 5.x
  - Framework: React 19 (Pure Static SPA, Zero SSR)
  - Build Tool: Vite
  - Design System: Astryx Design System (`@astryxdesign/core`) + Tailwind CSS
  - State Management: Zustand
  - Icons: `lucide-react` (minimalist 1.5px stroke width per UI/UX guidelines)
  - Markdown / AST: Unified, Remark, Rehype
- **Inference & Execution:**
  - Protocol: JSON-RPC 2.0 ACP (Agent Client Protocol)
  - Model Gateway: Configurable Multi-LLM Gateway with Google Gemini REST API (`gemini-2.5-flash`, `gemini-2.5-pro`) as primary initial provider, pluggable for OpenAI GPT and GitHub Copilot.
  - HTTP Streaming Client: `reqwest` with Server-Sent Events / chunk streaming.
  - Sandboxing: Podman (rootless, daemonless)
  - Task Runner: `just` (`Justfile`)

## Engineering Standards

- **Storage Separation of Concerns:**
  - *Living Specifications & Code (Git SSOT):* Plain markdown files (`product_vision.md`, `architecture.md`, `features/`) versioned in Git are the sovereign product contract.
  - *IDE Metadata & Checkpoints (SQLite):* Local SQLite database (`~/.liquid/liquid.db`) stores known project paths, user preferences/API keys, and agent execution restore points (`last_task_id`, session resumption tokens).
- **Upstream & Spec Primacy:** Focus on visual product definition, SCPE canonical file generation, Jira-style board tracking, and interactive task execution; downstream code compilation is secondary.
- **Package-by-Feature / Vertical Slicing:** Code is organized by domain feature rather than technical layer.
- **Fail-Fast & Strict Contracts:** Input parsing is criterious and defensive. Malformed or out-of-boundary payloads are rejected immediately with typed errors.
- **Single Source of Truth (SSOT):** Plain-text files versioned in Git are canonical. The UI is a real-time projection of disk state.
- **Zero Production Node.js Dependency:** Production binary embeds compiled UI assets and runs without external Node.js runtime.
- **Compiler Warnings Policy:** Non-critical compiler/linter warnings are permitted in production when explicitly justified, but zero errors are tolerated.

## Code Quality, Linters & Formatters

- **Rust (Backend):**
  - **Linter:** `cargo clippy --workspace --all-targets -- -D warnings` (strict zero-tolerance for compiler warnings; exceptions must be explicitly annotated with `#[allow(...)]` and justified).
  - **Formatter:** `cargo fmt --all -- --check` (enforces Rust standard 2021 style).
- **TypeScript / React (Frontend):**
  - **Linter:** `oxlint` (configured via `.oxlintrc.json`, high-performance Rust-based linter).
  - **Type-Checker:** `tsc --noEmit` (strict mode enabled in `tsconfig.json`).
  - **Test Runner:** `vitest` + `@testing-library/react` (declared test harness for frontend UI components and user interaction flows; adoption sequenced in dedicated testing epic).
  - **Rules:** Hook dependencies must be valid, zero undeclared anys, explicit return types on API contracts.
- **SCPE Specifications:**
  - **Validator:** `python3 .agents/skills/scpe/scripts/scpe.py validate --strict` (must pass with 0 errors and 0 warnings before any epic handoff).
  - **Readiness Gate:** `python3 .agents/skills/scpe/scripts/scpe.py gate-check <feature>/<epic>` (must pass with 0 blocking issues before moving to Ready).

## Versioning & Git Conventions

- **Semantic Versioning (SemVer 2.0.0):** `MAJOR.MINOR.PATCH`
  - `MAJOR`: Incompatible API breaking changes, protocol breaks, or paradigm shifts.
  - `MINOR`: Backward-compatible new features (e.g. adding Gemini provider, Liquid Board, Task Runner).
  - `PATCH`: Backward-compatible bug fixes, UI adjustments, and performance corrections.
  - **Manifest Alignment:** `version` in `apps/ade/Cargo.toml` and `apps/ade/ui/package.json` must stay synchronized with active release milestones.
- **Git Commit Messages (Conventional Commits):**
  - **Language:** Commit messages **MUST ALWAYS be written in English**, regardless of user prompt language.
  - **Format:** `<type>(<optional scope>): <description in english>`
  - **Types:** `feat` (new feature), `fix` (bug fix), `docs` (spec/documentation change), `refactor` (code refactoring), `test` (adding/updating tests), `perf` (performance), `chore` (maintenance/tooling), `build` (Cargo/Vite dependencies).

## Workspace Automation (Justfile)

Unified task automation via `just` provides single-command determinism for both humans and agents:
- `just check`: Runs full suite validation (`scpe.py validate --strict`, `cargo clippy`, `cargo fmt --check`, `oxlint`, and `tsc`).
- `just lint`: Runs code linters (`cargo clippy` and `oxlint`).
- `just fmt`: Formats all Rust and TypeScript source files.
- `just test`: Runs automated test suite (`cargo test`).
- `just dev`: Starts local development environment.
- `just build`: Compiles production release binary with embedded static SPA.

## AI Guardrails

- Humans decide, agents execute. No agent approves its own work or settles an open business question.
- Agents build only epics in state `Ready`, and only the scope of their `plan.md`.
- Every task in `tasks.md` cites the rules (`R#`) or examples (`S#`) it implements.
- Every example has at least one test named `<epic_name>#S#`.
- Code lives only in `apps/<app>/`, inside the boundaries of its `app.md`.
- Any edit to the `plan.md` of a `Done` epic makes it `Stale`.
- Always update `tasks.md` immediately upon completing and verifying each task, marking it as done (`- [x]`).

## Constraints

- Memory Footprint: Idle RSS must remain strictly under 50MB.
- UI Responsiveness: Local file synchronization latency must be sub-50ms.
- Portability: Single standalone binary executable on Linux/macOS/Windows.
- Sandboxing: Downstream execution must never run with root privileges or direct host access.

## Definition of Done

- Tasks in `tasks.md` all checked.
- Tests for every example pass (`cargo test` / `vitest`).
- Code reviewed by a Senior Architect or Developer.
- `quick_status.md` log updated with learnings.
- Working vertical slice verified end-to-end.
