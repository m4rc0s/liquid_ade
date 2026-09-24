# 📌 Epic 02: Spec Drift Validator

**Epic:** `EPIC-02` of Feature `FEAT-02`  
**Status:** `Draft` 📝  

---

## 1. Objective & Scope
Auditing algorithm that detects modifications to `plan.md` in completed epics and marks them as `Stale`.

## 2. Acceptance Criteria
- [ ] When an external methodology is configured (`FEAT-01 / EPIC-03`), structure and drift validation uses the resolved Methodology Profile instead of the fixed SCPE state machine.
- [ ] In the absence of configuration, validation follows native SCPE v0.3.0 rules (fallback).
- [ ] End-to-end implementation validated by tests.
- [ ] Zero performance impact on the Liquid runtime.
