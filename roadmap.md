# 🗺️ Liquid ADE: Macro Roadmap & Incremental Evolution

**Methodology:** SCPE v0.3.0 (Spec-Compiled Product Engineering)  
**Status:** Canonical Temporal Evolution SSOT  
**Last Updated:** 2026-09-23  
**Governance:** Architecture Council & Product Engineering  

---

## 1. Incremental Feature Discovery & Implementation Strategy

Liquid ADE adheres strictly to the SCPE core principle:
> **"Incremental Feature Discovery: open only the most critical feature now; do not map the whole system upfront in filesystem folders."**

Features are not pre-created as empty directory skeletons in Git. Instead, the workspace maintains laser focus on the single active feature under implementation. The next feature is opened via `scpe.py feature` only when the preceding milestone is completely implemented, verified, and approved.

```
Now (Active Implementation)                Next (Unlocks from Phase 1)              Later (Temporal Evolution)
┌─────────────────────────────────┐        ┌────────────────────────────────┐        ┌─────────────────────────┐
│ Phase 1: v0.1.0 The Inspector   │───────▶│ Phase 2: v0.2.0 Inception      │───────▶│ Phase 3: Live File Sync │
│ • FEAT-01: Workspace Inspector  │        │ • FEAT-02: Inception Studio    │        │ Phase 4: Status & Drift │
│   - epic_01_runtime_shell       │        │   - 7 small epics (Kimi UI,    │        │ Phase 5: WYSIWYG Editor │
│   - epic_02_workspace_fs_scanner│        │     Vision, Guidelines, ADRs)  │        │ Phase 6: ACP Gateway    │
└─────────────────────────────────┘        └────────────────────────────────┘        │ Phase 7: Podman Sandbox │
                                                                                     └─────────────────────────┘
```

---

## 2. Phase Horizons

### 🔹 Phase 1: v0.1.0 — The Inspector (Workspace Inspector & Runtime Shell)
* **Horizon:** `Now` (Active implementation in `features/01-workspace-inspector/`)
* **Goal:** A standalone Day-1 ADE binary that boots locally, embeds the static React 19 SPA, and safely inspects any SCPE workspace on disk.
* **Feature:** `01-workspace-inspector`
* **Epics:**
  1. `epic_01_runtime_shell`: Axum web server embedding static SPA via `rust-embed`, serving Astryx 3-column shell, and `/api/health`.
  2. `epic_02_workspace_fs_scanner`: Path traversal guard, recursive directory tree API (`/api/workspace/tree`), and safe file reader (`/api/workspace/file`).
* **Handoff Gate:** When `epic_01` and `epic_02` pass all tests and `features/01-workspace-inspector` is marked `Done`, Phase 2 is unlocked.

---

### 🔹 Phase 2: v0.2.0 — The Inception Studio (Product Inception & Idea Validation)
* **Horizon:** `Next` (Starts immediately from the completed Phase 1 foundation)
* **Goal:** Enable product creators, senior architects, and autonomous co-pilots to brainstorm, validate product ideas, formulate design/architectural guidelines, adjust roadmaps, and scaffold canonical SCPE projects inside Liquid ADE.
* **Feature Target:** `02-inception-studio` (Specification blueprint in `docs/specs/02-inception-studio/`)
* **Epics (Pre-specified & Ready for Handoff):**
  1. `epic_01_project_scaffold_generator`: Modal UI & `/api/workspace/new` to scaffold canonical SCPE workspaces on disk.
  2. `epic_02_conversational_copilot_panel`: Kimi-inspired dual-pane UI with streaming tokens and active document canvas binding.
  3. `epic_03_product_vision_refiner`: Structured elicitation prompts and live disk patching for `product_vision.md`.
  4. `epic_04_design_guidelines_builder`: Guided formulator of `UI_UX_GUIDELINES.md` using Astryx design tokens.
  5. `epic_05_architecture_and_adr_engine`: Architectural interview and ADR generator updating `architecture.md` and `technical_deal.md`.
  6. `epic_06_interactive_roadmap_adjuster`: Now/Next/Later visual Kanban grid and conversational reordering for `roadmap.md`.
  7. `epic_07_feature_planning_handoff`: Inception completeness validator unlocking downstream Feature Studio mode.

---

### 🔹 Phase 3: v0.3.0 — The Synchronizer (Live File Sync)
* **Horizon:** `Later`
* **Goal:** Seamless real-time reactivity with external editors (VS Code, Neovim) and Git.
* **Feature Target:** `03-live-file-sync`
* **Deliverables:**
  - Async file watcher in Rust (`notify`) monitoring workspace files.
  - WebSocket event stream (`/ws/workspace`) updating client state in <50ms.
  - Echo suppression preventing UI edit bounce-backs.

---

### 🔹 Phase 4: v0.4.0 — The Auditor (Spec Status, Drift & Liquid Board)
* **Horizon:** `Later`
* **Goal:** Visual traceability and automated drift detection across the SCPE state machine.
* **Feature Target:** `04-spec-status-and-drift`
* **Deliverables:**
  - Workspace status matrix reading all `quick_status.md` files.
  - Tactile Astryx state badges and interactive Kanban board.
  - Spec drift inspector flagging `Stale` epics when `plan.md` changes post-delivery.

---

### 🔹 Phase 5: v0.5.0 — The Studio (WYSIWYG Document Editor)
* **Horizon:** `Later`
* **Goal:** Transform Markdown specifications into rich interactive Document-as-UI components.
* **Feature Target:** `05-wysiwyg-document-editor`
* **Deliverables:**
  - Semantic AST block model (`HeaderNode`, `RequirementTableNode`, `BddScenarioNode`).
  - Surgical line patcher updating disk lines without reformatting adjacent markdown.

---

### 🔹 Phase 6: v0.6.0 — The Protocol (ACP Agent Handshake & Model Gateway)
* **Horizon:** `Later`
* **Goal:** Standardized agent communication and universal model routing.
* **Feature Target:** `06-acp-agent-handshake`
* **Deliverables:**
  - JSON-RPC 2.0 ACP server over stdio and WebSocket.
  - LiteLLM gateway switching transparently between local inference (Ollama) and cloud APIs.

---

### 🔹 Phase 7: v0.7.0 — The Engine (Autonomous Podman Sandbox Factory)
* **Horizon:** `Later`
* **Goal:** Autonomous downstream execution of tasks inside ephemeral rootless containers.
* **Feature Target:** `07-podman-sandbox-execution`
* **Deliverables:**
  - Ephemeral rootless Podman container runner (zero daemon, no docker-compose).
  - TDD execution pipeline compiling code in `apps/` and verifying tests against BDD scenarios (`<epic>#S#`).

---

## 3. Horizon Summary Matrix

| Horizon | Phase | Feature Slug | Title | State |
| :---: | :---: | :--- | :--- | :---: |
| **Now** | **Phase 1** | `01-workspace-inspector` | Workspace Inspector & Runtime Shell | `Draft` 📝 |
| **Next** | **Phase 2** | `02-inception-studio` | Inception Studio & Idea Validation | *Planned* 📋 |
| **Later** | **Phase 3** | `03-live-file-sync` | Live File Sync & Echo Suppression | *Backlog* ⏳ |
| **Later** | **Phase 4** | `04-spec-status-and-drift` | Spec Status, Drift & Liquid Board | *Backlog* ⏳ |
| **Later** | **Phase 5** | `05-wysiwyg-document-editor`| WYSIWYG Document-as-UI Editor | *Backlog* ⏳ |
| **Later** | **Phase 6** | `06-acp-agent-handshake` | ACP Protocol Server & LiteLLM Gateway | *Backlog* ⏳ |
| **Later** | **Phase 7** | `07-podman-sandbox-execution` | Podman Rootless Sandbox Runner | *Backlog* ⏳ |
