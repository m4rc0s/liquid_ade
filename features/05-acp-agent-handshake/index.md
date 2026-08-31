# 📄 Global PRD — FEAT-05: ACP Protocol & LiteLLM Model Gateway

**Feature ID:** `FEAT-05`  
**Name:** ACP Protocol & LiteLLM Model Gateway  
**Project:** Liquid ADE  
**Methodology:** SCPE v0.3.0  
**Global Status:** `Draft` 📝  

---

## 1. Executive Vision & User Journey

### 1.1. End-to-End Vertical Journey
> "As a developer, I configure my preferred AI model (local Ollama or cloud Claude/OpenAI) in settings, and test connectivity by sending standardized commands via Agent Client Protocol (ACP)."

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
| **`EPIC-01`** | **ACP JSON-RPC Server** | Implementation of ACP protocol based on JSON-RPC 2.0 over WebSocket and stdio. | `Draft` 📝 |
| **`EPIC-02`** | **LiteLLM Flexible Router** | Universal model gateway connecting the ACP server to Ollama or cloud APIs. | `Draft` 📝 |
