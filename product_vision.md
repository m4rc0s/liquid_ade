# 🌊 Liquid: Manifesto & Product Vision

**Methodology Version:** SCPE v0.3.0 (Spec-Compiled Product Engineering)  
**Document Status:** Active / Single Source of Truth (SSOT)  
**Last Updated:** 2026-09-23  

---

## 1. Executive Vision & Value Proposition

**Liquid** is an agent-oriented autonomous product engineering platform and environment (ADE — *Agentic Development Environment*), designed to unify business conception, technical architecture, and software generation into a single continuous, deterministic experience.

Bridging the historical divide between business requirements and source code, Liquid embraces the **Spec-Driven Development (SDD)** paradigm under the **SCPE v0.3.0** (*Spec-Compiled Product Engineering*) methodology, where:

> **"The Specification is the Sovereign Contract. The Document is the Interface (Document-as-UI). The Agents Execute with Mathematical Precision. The Code is Just the Consequence."**

### 1.1. The Problem: The Illusion of "Vibe Coding" and the Disconnection Chasm
With the advent of Large Language Models (LLMs), the software industry has plunged into *vibe coding* — unguided, unstructured code generation that introduces immediate technical debt, architectural vacuum, and context hallucinations. Today's fundamental issues are:
- **Requirements Disconnect:** Requirements live in fragmented silos (Jira, Notion, Confluence, Slack), while code evolves detached from the original product intent.
- **Context Loss and Agent Hallucination:** AI agents operate without rigid domain boundaries (*Bounded Contexts*), attempting to infer architecture across massive, noisy codebases.
- **Lack of Governance and Traceability:** No guarantees exist that generated code aligns with formal, measurable acceptance criteria.
- **Proprietary Lock-in and Privacy Loss:** Heavy dependency on closed cloud ecosystems and indiscriminate transmission of intellectual property and proprietary code to third-party servers.

### 1.2. The Liquid Solution: Spec-First, Local-First, and Code as Consequence
Liquid restructures the software development lifecycle through living specifications compiled in structured Markdown, featuring bidirectional sub-50ms live sync, open protocol agent orchestration, and strictly sandboxed execution.

### 1.3. Pluggable SDD (Configurable Methodology)
SCPE v0.3.0 is Liquid's native methodology and default fallback, but it is not the only option. Each workspace can declare its own **methodology source** — a repository, folder, or plain text document describing how that team works, ranging from a robust multi-document standard (such as SCPE itself) to a simple set of guidelines authored by a single person. Liquid resolves this source into a **Methodology Profile** used to validate structure, scaffold new spec files, and guide agents — without locking any team into Liquid's native format. **Configuration is handled via a dedicated UI screen, never requiring manual file edits**, since the person defining a team's methodology is often a non-technical stakeholder (PO/PM/Designer) without terminal or code editor access. See `FEAT-01 / EPIC-03` and `FEAT-02 / EPIC-02`.

### 1.4. The Concept-to-Production Continuum: Bridging Vibe Coding and Production Engineering
Liquid transforms the traditional disconnect between unstructured ideation and production code into an unbroken, auditable continuum:
- **From Idea to Formal Contract:** The user converses with an architect co-pilot (powered by pluggable LLMs such as Google Gemini, GPT, or local models) to explore product vision and design principles. The engine compiles these dialogues into structured, living Markdown specifications (`product_vision.md`, `architecture.md`, `roadmap.md`).
- **Context-Preserving Spec Hierarchy:** High-level product vision cascades deterministically down to Features, Epics, and granular Tasks with strict domain invariants (`R#`) and Given/When/Then acceptance scenarios (`S#`).
- **Closing the Implementation Gap:** Instead of blind generation, downstream implementation tasks are executed against explicit architectural contracts. Verification gates ensure that no code lands without passing concrete test assertions linked to specification scenarios.

---

## 2. Personas, Workflows & The Code Frontier

Liquid was built to unite developers and product practitioners over the **same source of truth (Markdown files in Git)**, maintaining a clear separation of concerns regarding source code:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          SINGLE SOURCE OF TRUTH                             │
│                 Markdown Files in Git (.md / SCPE v0.3.0)                   │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │
         ┌─────────────────────────────┴─────────────────────────────┐
         ▼                                                           ▼
┌──────────────────────────────────────┐            ┌──────────────────────────────────────┐
│      TECHNICAL PERSONA (CLI-FIRST)   │            │       PRODUCT PERSONA (UI-FIRST)     │
│        (Engineer / Tech Lead)        │            │   (Product Owner / PM / Designer)    │
│                                      │            │                                      │
│ • Creates and edits specs via CLI,   │            │ • Writes and refines specs in the    │
│   terminal, or editor (Neovim/VSCode)│            │   visual WYSIWYG editor (Notion /    │
│ • Uses the UI as a Control Studio:   │            │   Linear style).                     │
│   monitors status, roadmap, and the  │            │ • Follows the Liquid Board (Kanban)  │
│   Liquid Board.                      │            │   and approves the Readiness Gate.   │
│ • Uses the UI for quick spot edits   │            │ • Tracks delivery progress without   │
│   when convenient.                   │            │   needing to open a terminal.        │
└──────────────────┬───────────────────┘            └──────────────────┬───────────────────┘
                   │                                                   │
                   └───────────────────────────┬───────────────────────┘
                                               ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                      THE CODE FRONTIER (`apps/` folder)                     │
│                                                                             │
│ • Liquid ADE DOES NOT attempt to be a generic code editor (like VS Code).   │
│ • Agent-generated code resides physically under `apps/<app_name>/`.         │
│ • Anyone (Dev or PM) wishing to inspect or edit source code manually opens  │
│   the `apps/` folder in their favorite editor (Neovim, VS Code, Cursor).    │
└─────────────────────────────────────────────────────────────────────────────┘
```

* **Focus on What Matters:** Liquid focuses on being the world's best tool for **Specification, Governance, Traceability, and Agent Orchestration**.
* **Zero Editor Lock-in:** We never force anyone into a browser-based code editor. The `apps/` folder contains clean, standard code on disk.

### 2.3. The Living Spec Canvas & 1-Click Interactive Task Runner
Liquid elevates Markdown specifications from passive documentation into an executable, interactive canvas:
- **Actionable Task Blocks:** When viewing `tasks.md`, each individual task is projected as an actionable execution block equipped with a 1-click trigger (`▶ Start task`).
- **Real-Time Visual Execution:** Upon activation, the task block transitions to an active execution state (`🔄 Task in progress`), while the companion session panel streams agent thoughts, steering documents (`product_vision.md`, `architecture.md`, `technical_deal.md`), file inspections, diffs, and diagnostic checks.
- **Instant Completion & Git Sync:** Once the agent satisfies the task criteria and automated tests, the block resolves to `✅ Task completed [View changes] [View execution]`, and the underlying `tasks.md` on disk is atomically updated to `- [x]`.

---

## 3. Unified Architecture: Single Artifact & SPA Model (Figma/Linear Style)

Liquid adopts the **Single Artifact (12-Factor App)** and **Pure Static SPA (Zero SSR)** model. The application operates with maximum fluidity both as a **Cloud SaaS** and as a **Native Desktop App (Tauri 2.0)** without opening a browser:

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                             LIQUID ADE INTERFACE                            │
│         React 19 + TypeScript + Astryx Design System + WYSIWYG Editor       │
│               (Pure / Static Single Page Application — Zero SSR)            │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │ (Same SPA / Same Components)
         ┌─────────────────────────────┴─────────────────────────────┐
         ▼                                                           ▼
┌──────────────────────────────────────┐            ┌──────────────────────────────────────┐
│        DESKTOP DISTRIBUTION          │            │           WEB DISTRIBUTION           │
│         (Tauri 2.0 Wrapper)          │            │       (Axum Server in Rust)          │
│                                      │            │                                      │
│ • Native window without browser UI   │            │ • Served locally or in the cloud     │
│ • 100% Offline and Secure execution  │            │ • Multi-tenant Cloud SaaS via ENV    │
│ • High-performance native WebView    │            │ • Runs in any modern web browser     │
└──────────────────┬───────────────────┘            └──────────────────┬───────────────────┘
                   │                                                   │
                   └───────────────────────────┬───────────────────────┘
                                               ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                              CORE ENGINE (RUST)                             │
│         Axum Server + ACP Protocol (JSON-RPC 2.0) + File Watcher            │
├─────────────────────────────────────────────────────────────────────────────┤
│                       MULTI-LLM GATEWAY & ROUTING                           │
│  (Native Google Gemini REST API + LiteLLM for Claude, GPT-4o, Local Models)  │
├─────────────────────────────────────────────────────────────────────────────┤
│                EMBEDDED SQLITE ENGINE (rusqlite bundled)                    │
│    (Project Registry + LLM Settings + Execution Checkpoints & Telemetry)    │
├─────────────────────────────────────────────────────────────────────────────┤
│                ISOLATED EXECUTION & SANDBOXING WITH PODMAN                  │
│        (Rootless downstream task isolation — without docker-compose)        │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 4. The 9 Non-Negotiable Architectural Pillars

1. **Rust Core Engine (Axum / Tokio):**
   - High-performance core with minimal memory footprint (<50MB RSS at idle) and ultra-fast startup (<100ms).
   - Serves as the web server, ACP dispatcher, filesystem orchestrator, and embedded SQLite host.

2. **Reactive Static SPA Frontend (Zero SSR):**
   - Built with React 19, TypeScript, Vite, Astryx Design System (`@astryxdesign/core`), and Zustand.
   - **Zero SSR (No Node.js in production):** Adopts the pattern of modern engineering tools (Linear/Figma/VS Code Web), generating lightweight static assets that run natively in the browser and within Tauri without intermediate server overhead.

3. **Open Protocol via Agent Client Protocol (ACP):**
   - All communication between the interface (Client) and Agents/Engine is governed by JSON-RPC 2.0 over `stdio` or `WebSocket`.

4. **Pluggable Multi-LLM Gateway:**
   - Seamless, configurable switching between native cloud providers (Google Gemini REST API with streaming, OpenAI, Anthropic) and local private inference (Ollama, Llama.cpp) via LiteLLM abstraction.
   - API keys and model configurations managed securely through dedicated UI settings.

5. **Embedded SQLite for Project Registry & Checkpoints:**
   - Zero-setup embedded database (`rusqlite` bundled) stored at `~/.liquid/liquid.db`.
   - Persists known workspaces, last opened project, user credentials, and durable agent execution checkpoints (`last_task_id`, restore points).
   - Strict separation of concerns: Git Markdown remains the sovereign specification SSOT; SQLite handles operational and recovery metadata.

6. **Secure Rootless Sandboxing with Podman:**
   - AI agents compile code and run tests inside ephemeral containers managed via **Podman**, eliminating root daemons and discarding `docker-compose`.

7. **Task Automation with Just (`Justfile`):**
   - Centralized, deterministic development and build workflows (`just dev`, `just build`, `just test`) for both humans and agents.

8. **WYSIWYG Document Editor (Document-as-UI):**
   - Markdown specs treated as living visual components in Notion/Linear style, featuring real-time bidirectional sync, AST block editing, and echo suppression.

9. **Interactive Task Runner & Liquid Board:**
   - Visual Jira-style Kanban board projecting SCPE epic and task lifecycle states.
   - 1-click execution directly from the specification canvas with real-time agent output streaming and atomic disk synchronization.

---

## 5. AI Agent Hooks & Execution Checkpoints

Liquid bridges conversational product discovery and deterministic engineering through dedicated agent hooks and persistent checkpoints:

### 5.1. Specialized Agent Hooks
- **Spec Compilation Hook:** Transmutes high-level user dialogue into canonical SCPE documents (`product_vision.md`, `architecture.md`, `technical_deal.md`, `roadmap.md`), enforcing domain invariants (`R#`) and test scenarios (`S#`).
- **Interactive Task Execution Hook:** Activated by the 1-click task trigger (`▶ Start task`). Reads relevant steering documents, generates or modifies code under `apps/`, runs unit/integration tests, and reports diffs back to the UI.
- **Continuous Verification Hook:** Evaluates readiness gates, tracks git diffs against specifications, flags specification drift (`Stale`), and generates regression tests.

### 5.2. Durable Execution Checkpoints
- Every agent execution records its checkpoint (`project_id`, `epic_id`, `last_task_id`, git commit SHA, agent status) into embedded SQLite.
- If a session is paused, interrupted, or reloaded, the LLM resumes execution with exact state awareness, preventing duplicate work, context loss, and hallucination.
