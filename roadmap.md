# 🗺️ Liquid: Macro Roadmap & Release Strategy

**Methodology:** SCPE v0.3.0 (Spec-Compiled Product Engineering)  
**Global Status:** Active / Canonical Source of Temporal Evolution  
**Last Updated:** 2026-08-31  
**Maintained by:** Product Architecture & Engineering Council  

---

## 1. Timeline Vision & Release Strategy across 6 Granular Phases

Development of **Liquid** is structured into **6 Progressive and Incremental Phases**. Each phase delivers a functional, testable product milestone with a concise, hyper-focused scope:

```
2026 Q3                                  2026 Q4                                  2027 Q1
  │                                        │                                        │
  ├─ Phase 1: v0.1.0 (The Reader) ─────────┼─ Phase 3: v0.3.0 (The Synchronizer) ───┼─ Phase 5: v0.5.0 (The Protocol) ────▶
  │  FEAT-01: Workspace Inspector          │  FEAT-03: Live File Sync               │  FEAT-05: ACP & LiteLLM Gateway
  │                                        │                                        │
  ├─ Phase 2: v0.2.0 (The Auditor) ────────┼─ Phase 4: v0.4.0 (The Studio) ─────────┼─ Phase 6: v0.6.0 (The Engine) ──────▶
  │  FEAT-02: Spec Status & Drift          │  FEAT-04: WYSIWYG Document Editor      │  FEAT-06: Podman Sandbox Runner
```

---

## 2. Detailed Breakdown of the 6 Evolution Phases

### 🔹 Phase 1: v0.1.0 — The Reader (Governance & Spec Viewer)
* **Goal:** A functional Day-1 tool that scans and displays any SCPE workspace with elegance.
* **Feature:** **`FEAT-01: Workspace Inspector`**
* **User Journey:** The user opens the app in the browser (`localhost:3000`), views the governance tree in the sidebar, and reads Markdown specs rendered on the canvas with Astryx typography.
* **Deliverables:**
  - Axum web server in Rust serving the embedded static SPA.
  - Filesystem scanner reading `product_vision.md`, `roadmap.md`, and `features/` directories.

---

### 🔹 Phase 2: v0.2.0 — The Auditor (State Management & Spec Drift)
* **Goal:** Add visual traceability and product lifecycle auditing.
* **Feature:** **`FEAT-02: Spec Status & Drift`**
* **User Journey:** The user views tactile badges (`Draft`, `Ready`, `WIP`, `Done`) next to each epic and receives automatic visual *Spec Drift* alerts if `plan.md` is modified post-completion.
* **Deliverables:**
  - `quick_status.md` reader across the entire project tree.
  - Semantic status badge components with Astryx (Green, Amber, Red, Neutral).
  - Drift validation algorithm and inconsistency banner.

---

### 🔹 Phase 3: v0.3.0 — The Synchronizer (Real-Time File Sync)
* **Goal:** Connect the Liquid UI seamlessly to traditional VS Code/Neovim workflows.
* **Feature:** **`FEAT-03: Live File Sync`**
* **User Journey:** The user edits a Markdown file in their favorite editor and Liquid ADE updates instantly (<50ms) without reloading or losing scroll position.
* **Deliverables:**
  - Async File Watcher in Rust (`notify`) monitoring `.md` files.
  - WebSocket channel (`/ws/workspace`) streaming deltas to the Zustand store.
  - Real-time synchronization micro-indicator.

---

### 🔹 Phase 4: v0.4.0 — The Studio (WYSIWYG Document-as-UI Editor)
* **Goal:** Transform Markdown into a rich, editable visual interface with surgical disk patching.
* **Feature:** **`FEAT-04: WYSIWYG Document Editor`**
* **User Journey:** The user clicks acceptance criteria checklists, metadata chips, or domain tables on screen, edits visually, and Liquid writes only modified lines back to disk (with echo suppression).
* **Deliverables:**
  - Semantic AST parser decomposing Markdown into rich UI blocks.
  - Surgical line-by-line disk patching engine (Surgical Line Patcher).
  - Echo suppression algorithm and keyboard shortcuts (`Cmd+Z`, `/`).

---

### 🔹 Phase 5: v0.5.0 — The Protocol (Agent Handshake & Model Gateway)
* **Goal:** Enable standardized communication with AI agents and multiple language models.
* **Feature:** **`FEAT-05: ACP & LiteLLM Gateway`**
* **User Journey:** The user configures their AI provider (local Ollama or cloud Claude/OpenAI) and verifies agent connectivity via Agent Client Protocol (ACP).
* **Deliverables:**
  - JSON-RPC 2.0 ACP server over WebSocket and stdio.
  - Universal model gateway via LiteLLM with ping and latency diagnostics.
  - AI settings modal built with Astryx.

---

### 🔹 Phase 6: v0.6.0 — The Engine (Autonomous Podman Sandbox Factory)
* **Goal:** Close the SCPE loop by compiling approved specifications into automatically tested code.
* **Feature:** **`FEAT-06: Podman Sandbox Runner`**
* **User Journey:** Upon promoting a feature to `Ready`, Liquid triggers an autonomous agent that spins up an isolated Podman container, executes tasks in `tasks.md` via TDD, generates code in `apps/`, and advances the state to `Done`.
* **Deliverables:**
  - Ephemeral, rootless Podman container runner (without docker-compose).
  - TDD execution pipeline with automated Quality Reviewer agent.
  - Live log and terminal streaming to the UI.

---

## 3. Traceability Matrix

| Version | Phase | Name | Target Feature | Status |
| :---: | :---: | :--- | :--- | :---: |
| **`v0.1.0`** | **Phase 1** | **The Reader** | `01-workspace-inspector` | `Draft` 📝 |
| **`v0.2.0`** | **Phase 2** | **The Auditor** | `02-spec-status-and-drift` | `Draft` 📝 |
| **`v0.3.0`** | **Phase 3** | **The Synchronizer** | `03-live-file-sync` | `Draft` 📝 |
| **`v0.4.0`** | **Phase 4** | **The Studio** | `04-wysiwyg-document-editor` | `Draft` 📝 |
| **`v0.5.0`** | **Phase 5** | **The Protocol** | `05-acp-agent-handshake` | `Draft` 📝 |
| **`v0.6.0`** | **Phase 6** | **The Engine** | `06-podman-sandbox-execution` | `Draft` 📝 |
