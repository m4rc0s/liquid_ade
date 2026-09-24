# 📄 Global PRD — FEAT-06: Podman Sandbox & Autonomous TDD Runner

- **goal:** Execute agent-generated code and TDD pipelines inside isolated rootless Podman containers
- **users:** Tech Leads and Downstream Autonomous Agents
- **bounded_context:** Container Sandboxing
- **business_value:** Guarantees safe and isolated execution of untrusted agent code without host corruption


**Feature ID:** `FEAT-06`  
**Name:** Podman Sandbox & Autonomous TDD Runner  
**Project:** Liquid ADE  
**Methodology:** SCPE v0.3.0  
**Global Status:** `Draft` 📝  

---

## 1. Executive Vision & User Journey

### 1.1. End-to-End Vertical Journey
> "As a Tech Lead, upon approving a feature to 'Ready', I watch an autonomous AI agent spin up an isolated Podman container, write tests via TDD, generate code under `apps/`, and advance the status to 'Done' with 100% test pass rate."

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
| **`EPIC-01`** | **Podman Rootless Container Runner** | Ephemeral and isolated container orchestrator managed via Podman. | `Draft` 📝 |
| **`EPIC-02`** | **Downstream TDD Execution Pipeline** | Autonomous pipeline executing the `tasks.md` checklist sequentially until Reviewer validation. | `Draft` 📝 |

## Epics

- [epic_01_podman_container_runner](epics/epic_01_podman_container_runner/index.md) — Draft
- [epic_02_autonomous_tdd_pipeline](epics/epic_02_autonomous_tdd_pipeline/index.md) — Draft
