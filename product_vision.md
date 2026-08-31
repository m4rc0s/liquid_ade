# 🌊 Liquid: Manifesto & Product Vision

**Methodology Version:** SCPE v0.3.0 (Spec-Compiled Product Engineering)  
**Document Status:** Active / Single Source of Truth (SSOT)  
**Last Updated:** 2026-08-31  

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
│                       LITELLM UNIVERSAL MODEL ROUTING                       │
│     (Transparent support for Ollama, Llama.cpp, Claude, GPT-4o, Gemini)     │
├─────────────────────────────────────────────────────────────────────────────┤
│                ISOLATED EXECUTION & SANDBOXING WITH PODMAN                  │
│        (Rootless downstream task isolation — without docker-compose)        │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 4. The 7 Non-Negotiable Architectural Pillars

1. **Rust Core Engine (Axum / Tokio):**
   - High-performance core with minimal memory footprint (<50MB RSS at idle) and ultra-fast startup (<100ms).
   - Serves as the web server, ACP dispatcher, and filesystem orchestrator.

2. **Reactive Static SPA Frontend (Zero SSR):**
   - Built with React 19, TypeScript, Vite, Astryx Design System (`@astryxdesign/core`), and Zustand.
   - **Zero SSR (No Node.js in production):** Adopts the pattern of modern engineering tools (Linear/Figma/VS Code Web), generating lightweight static assets that run natively in the browser and within Tauri without intermediate server overhead.

3. **Open Protocol via Agent Client Protocol (ACP):**
   - All communication between the interface (Client) and Agents/Engine is governed by JSON-RPC 2.0 over `stdio` or `WebSocket`.

4. **Model-Agnostic Routing via LiteLLM:**
   - Seamless, configurable switching between local private inference (Ollama, Llama.cpp) and cloud providers (Claude 3.5 Sonnet, GPT-4o, Gemini Pro) across all execution modes.

5. **Secure Rootless Sandboxing with Podman:**
   - AI agents compile code and run tests inside ephemeral containers managed via **Podman**, eliminating root daemons and discarding `docker-compose`.

6. **Task Automation with Just (`Justfile`):**
   - Centralized, deterministic development and build workflows (`just dev`, `just build`, `just test`) for both humans and agents.

7. **WYSIWYG Document Editor (Document-as-UI):**
   - Markdown specs treated as living visual components in Notion/Linear style, featuring real-time bidirectional sync and echo suppression.
