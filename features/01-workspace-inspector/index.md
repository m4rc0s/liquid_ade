# 📄 Global PRD — FEAT-01: Workspace Inspector & Spec Viewer

**Feature ID:** `FEAT-01`  
**Name:** Workspace Inspector & Spec Viewer  
**Project:** Liquid ADE  
**Methodology:** SCPE v0.3.0  
**Global Status:** `Draft` 📝  

---

## 1. Executive Vision & User Journey

### 1.1. End-to-End Vertical Journey
> "As a developer or PO, I open Liquid ADE, see the governance and feature hierarchy in the sidebar, and can click to read any specification rendered on the central canvas."

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
| **`EPIC-01`** | **Runtime Shell & Astryx Layout** | Rust server (Axum) serving embedded static SPA with 3-column Astryx layout. | `Draft` 📝 |
| **`EPIC-02`** | **Workspace FS Scanner & Markdown Viewer** | Rust scanner reading workspace directory tree and `/api/workspace/tree` endpoint consumed by the UI. | `Draft` 📝 |
| **`EPIC-03`** | **Methodology Config Resolver & Adapter** | Configuration screen (no terminal needed) to point workspace SDD methodology to an external text source, resolved into a Methodology Profile used for scaffolding and validation. | `Draft` 📝 |
