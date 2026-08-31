# 📄 Global PRD — FEAT-02: Spec Status, Drift & Liquid Board (Kanban)

**Feature ID:** `FEAT-02`  
**Name:** Spec Status, Drift & Liquid Board (Kanban View)  
**Project:** Liquid ADE  
**Methodology:** SCPE v0.3.0  
**Global Status:** `Draft` 📝  

---

## 1. Executive Vision & User Journey

### 1.1. End-to-End Vertical Journeys
> *"As a Product Owner and Tech Lead, I switch between Document mode and **Liquid Board (Kanban)** mode. In Kanban, I see columns representing SCPE states (`Draft`, `Ready`, `WIP`, `Done`, `Blocked`, `Stale`) with cards for each epic/task. When I drag a card from `Draft` to `Ready`, Liquid updates the corresponding `quick_status.md` on disk in real time."*

### 1.2. Bounded Context Scope
This feature delivers a holistic project overview through:
1. **State Badges:** Tactile indicators in the sidebar tree.
2. **Liquid Board (Interactive Kanban):** Visualization and state transitions via SCPE v0.3.0 cards and columns.
3. **Spec Drift Inspector:** Visual alert and inconsistency banner for `Stale` epics.

---

## 2. Invariants & Non-Negotiable Rules

1. **File-Driven Kanban (Zero Hidden Database):** Each Kanban column reflects the SCPE state machine (`Draft`, `Ready`, `WIP`, `Done`, `Blocked`, `Stale`). Moving a card directly alters the corresponding `quick_status.md` file on disk.
2. **Astryx Componentization:** Task and epic cards use card and badge components from the Astryx Design System.
3. **Strict Vertical Slice:** The feature is only considered `Done` when visualization and drag-and-drop / click transitions work end-to-end.

---

## 3. Epic Architecture

| Epic | Name | Description | Status |
| :--- | :--- | :--- | :---: |
| **`EPIC-01`** | **Status Matrix & Liquid Board (Kanban)** | State reading and visual dashboard in Kanban columns with interactive Astryx Cards. | `Draft` 📝 |
| **`EPIC-02`** | **Spec Drift Validator & State Transitions** | State transition engine on disk and post-completion inconsistency detection (`Stale`). | `Draft` 📝 |
