# Architecture: Liquid ADE

**System:** Liquid ADE (Agentic Development Environment)  
**Methodology:** SCPE v0.3.0 (Spec-Compiled Product Engineering)  
**Status:** Living Architectural SSOT  
**Last Updated:** 2026-09-23  

---

## System Context (C4 Level 1)

Liquid ADE is a local-first, spec-compiled product engineering environment. It bridges human product visionaries, senior architects, and autonomous AI agents through living specifications versioned in Git.

```
┌────────────────────────────────────────────────────────┐
│                   Human Stakeholders                   │
│   (Senior Architect / Tech Lead / Product Engineer)    │
└──────────────────────────┬─────────────────────────────┘
                           │
                           │ Uses Browser / Desktop UI (HTTP/WS)
                           ▼
┌────────────────────────────────────────────────────────┐
│                   Liquid ADE System                    │
│   Single-binary runtime (Rust Axum + React 19 SPA)     │
└──────────────┬──────────────────────────┬──────────────┘
               │                          │
               │ ACP (JSON-RPC 2.0)       │ OCI / CLI
               ▼                          ▼
┌──────────────────────────────┐  ┌──────────────────────┐
│  AI Agents (Claude/LiteLLM)  │  │   Podman Sandbox     │
│   Upstream / Downstream      │  │ (Isolated Execution) │
└──────────────────────────────┘  └──────────────────────┘
```

---

## Containers (C4 Level 2)

- **`liquid-ade` (`apps/ade`)**:
  - **Type:** Single-binary host application.
  - **Backend Core:** Rust with Tokio async runtime and Axum HTTP/WebSocket server. Manages filesystem operations, safe path traversal checks, file watching (notify-rs), ACP JSON-RPC 2.0 communication, and sandbox process spawning.
  - **Frontend SPA:** Pure Static Single Page Application built with React 19, TypeScript, Astryx Design System (`@astryxdesign/core`), and Zustand. Embedded directly into the Rust binary using `rust-embed` (Zero SSR, Zero Node.js runtime dependency in production).
- **Agent Mesh (ACP Protocol Server)**:
  - Dispatches tasks to local or cloud LLMs via LiteLLM abstraction.
- **Sandbox Runner (Podman)**:
  - Rootless, daemonless container runner executing agent tasks (build, test, lint) in ephemeral containment.

---

## Bounded Contexts

1. **Workspace Navigation & Inspection (`features/01-workspace-inspector`)**:
   - Responsibility: Validates SCPE workspace structure, monitors filesystem tree, scaffolds canonical projects, and hosts Inception Studio.
   - Owning App: `apps/ade`
2. **Spec Status & Drift Engine (`features/02-spec-status-and-drift`)**:
   - Responsibility: Evaluates epic state machines (`Draft → Ready → WIP → Done`), tracks git diffs, and flags spec drift (`Stale`).
   - Owning App: `apps/ade`
3. **Live File Sync (`features/03-live-file-sync`)**:
   - Responsibility: Real-time bidirectional file watching and WebSocket event dispatching with echo suppression.
   - Owning App: `apps/ade`
4. **WYSIWYG Document Editor (`features/04-wysiwyg-document-editor`)**:
   - Responsibility: Renders living Markdown as Document-as-UI with AST Block editing and surgical line patchers.
   - Owning App: `apps/ade`
5. **Agent ACP Handshake (`features/05-acp-agent-handshake`)**:
   - Responsibility: Agent Client Protocol (ACP) JSON-RPC 2.0 communication channel and LiteLLM model routing.
   - Owning App: `apps/ade`
6. **Container Sandboxing (`features/06-podman-sandbox-execution`)**:
   - Responsibility: Rootless Podman container lifecycle management and autonomous TDD test execution.
   - Owning App: `apps/ade`

---

## Integrations

- **Filesystem / Git**: Direct local OS filesystem access bounded by workspace root path verification.
- **LiteLLM / AI Model Gateways**: Outbound HTTP/REST to LiteLLM proxy or local inference servers (Ollama, vLLM).
- **Podman CLI**: Local system execution of rootless containers for sandbox task evaluation.

---

## Decisions

- **2026-08-28 — ADR-001: Rust Axum Backend with Embedded Static React SPA**:
  - Context: Liquid ADE needs high-performance, single-binary distribution, and memory efficiency (<50MB RSS).
  - Decision: Build backend in Rust (Axum) and bundle the React 19 frontend assets via `rust-embed`.
  - Rejected: Electron (too heavy, high memory consumption) and Node.js SSR (requires external runtime).
- **2026-08-31 — ADR-002: ACP (Agent Client Protocol) over Proprietary SDKs**:
  - Context: Agent communication must be standardized, streaming-capable, and decoupled from vendor lock-in.
  - Decision: Implement JSON-RPC 2.0 ACP over stdio/WebSocket.
  - Rejected: Vendor-specific SDK bindings.
- **2026-09-23 — ADR-003: Inception Studio with Kimi-Inspired Dual-Pane Architecture**:
  - Context: Product idea validation requires conversational refinement with immediate visual synchronization to living SCPE specifications (`product_vision.md`, `UI_UX_GUIDELINES.md`, `architecture.md`, `roadmap.md`).
  - Decision: Introduce a dual-pane Inception Studio layout (left: conversational co-pilot streaming suggestions; right: interactive document canvas with AST blocks).
  - Rejected: Unstructured chat modal without persistent document canvas projection.
- **2026-09-23 — ADR-004: Strict Contract Validation and Input Integrity**:
  - Context: Architectural boundaries must prevent corrupted state from entering downstream pipelines.
  - Decision: Reject tolerant parsing; enforce strict schemas and fail-fast validation across all API endpoints and spec parsers.
  - Rejected: Tolerant parsing for malformed input.
- **2026-09-23 — ADR-005: Pluggable Multi-LLM Gateway with Google Gemini Initial Provider**:
  - Context: Product builders require immediate, conversational AI co-pilots to define products and generate SCPE features, with user-configured API keys and models.
  - Decision: Implement a pluggable Multi-LLM Gateway starting with native Google Gemini REST streaming API (`streamGenerateContent`), extensible to OpenAI GPT and Copilot.
  - Rejected: Hardcoding a single proprietary model or requiring local Ollama setup for initial testing.
- **2026-09-23 — ADR-006: One-Click Interactive Task Runner and Jira-Style Board**:
  - Context: Non-technical and product stakeholders need visual lifecycle tracking and immediate execution of individual tasks without using a terminal.
  - Decision: Implement a Jira-style Liquid Board (Kanban projection of the SCPE state machine) with a 1-click execution action on every task item, streaming agent progress and updating disk state (`- [x]`) automatically.
  - Rejected: Passive read-only checklists requiring manual CLI commands.

