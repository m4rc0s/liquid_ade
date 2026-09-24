# 📄 Global PRD — FEAT-03: Live External File Synchronization

- **goal:** Provide sub-50ms bidirectional file synchronization between filesystem and UI with echo suppression
- **users:** Product Engineers, Developers, and System Watchers
- **bounded_context:** Live File Sync
- **business_value:** Enables instant reactivity to IDE and Git changes without manual browser refreshes


**Feature ID:** `FEAT-03`  
**Name:** Live External File Synchronization  
**Project:** Liquid ADE  
**Methodology:** SCPE v0.3.0  
**Global Status:** `Draft` 📝  

---

## 1. Executive Vision & User Journey

### 1.1. End-to-End Vertical Journey
> "As a developer, when I modify a Markdown file in Neovim or VS Code, the Liquid ADE interface updates instantly (<50ms) without reloading the page and without losing my scroll position."

### 1.2. Bounded Context Scope
This feature represents a complete **Vertical Slice**. It delivers direct value to the user by connecting the Interface (React + Astryx), Engine (Rust Axum), and Filesystem / Execution.

---

## 2. Invariants & Non-Negotiable Rules

1. **Strict Vertical Slice:** The feature is only considered `Done` when the user journey functions end-to-end.
2. **Filesystem SSOT:** All state is reflected on disk; zero hidden persistence.
3. **Adherence to UI/UX Pillars:** Components and visual feedback follow `UI_UX_GUIDELINES.md`.

---

## 3. Epic Architecture

| Epic | Name | Description | Status |
| :--- | :--- | :--- | :---: |
| **`EPIC-01`** | **Async File Watcher Engine** | Rust module with `notify` monitoring filesystem modifications in the `features/` directory. | `Draft` 📝 |
| **`EPIC-02`** | **WebSocket Delta Streamer** | WebSocket channel streaming updated content to the Zustand store in the UI. | `Draft` 📝 |

## Epics

- [epic_01_rust_file_watcher](epics/epic_01_rust_file_watcher/index.md) — Draft
- [epic_02_websocket_live_reload](epics/epic_02_websocket_live_reload/index.md) — Draft
