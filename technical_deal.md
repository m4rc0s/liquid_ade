# Technical Deal: Liquid ADE

Technical agreements every Developer and agent follows. Code in `apps/` is generated
against this file.

## Approved Stack

- **Backend / Core:**
  - Language: Rust 2021 edition
  - HTTP / WebSocket Framework: Axum 0.8
  - Async Runtime: Tokio
  - Asset Embedding: `rust-embed`
  - Serialization: Serde, `serde_json`
  - Filesystem Monitoring: `notify`
- **Frontend / Client:**
  - Language: TypeScript 5.x
  - Framework: React 19 (Pure Static SPA, Zero SSR)
  - Build Tool: Vite
  - Design System: Astryx Design System (`@astryxdesign/core`) + Tailwind CSS
  - State Management: Zustand
  - Markdown / AST: Unified, Remark, Rehype
- **Inference & Execution:**
  - Protocol: JSON-RPC 2.0 ACP (Agent Client Protocol)
  - Model Gateway: Configurable Multi-LLM Gateway with Google Gemini REST API (`gemini-2.5-flash`, `gemini-2.5-pro`) as primary initial provider, pluggable for OpenAI GPT and Copilot.
  - HTTP Streaming Client: `reqwest` with Server-Sent Events / chunk streaming.
  - Sandboxing: Podman (rootless, daemonless)
  - Task Runner: `just` (`Justfile`)

## Engineering Standards

- **Upstream & Spec Primacy:** Focus on visual product definition, SCPE canonical file generation, Jira-style board tracking, and interactive task execution; downstream code compilation is secondary.
- **Package-by-Feature / Vertical Slicing:** Code is organized by domain feature rather than technical layer.
- **Fail-Fast & Strict Contracts:** Input parsing is criterious and defensive. Malformed or out-of-boundary payloads are rejected immediately with typed errors.
- **Single Source of Truth (SSOT):** Plain-text files versioned in Git are canonical. The UI is a real-time projection of disk state.
- **Zero Production Node.js Dependency:** Production binary embeds compiled UI assets and runs without external Node.js runtime.
- **Compiler Warnings Policy:** Non-critical compiler/linter warnings are permitted in production when explicitly justified, but zero errors are tolerated.

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
