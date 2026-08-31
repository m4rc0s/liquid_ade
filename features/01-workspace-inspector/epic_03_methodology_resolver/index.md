# 📌 Epic 03: Methodology Config Resolver & Adapter

**Epic:** `EPIC-03` of Feature `FEAT-01`  
**Status:** `Draft` 📝

---

## 1. Objective & Scope
Allow the workspace to declare, via configuration, which Spec-Driven Development (SDD) standard it should follow — pointing to an external text-based source (Git repository, local directory, or single document) — and enable Liquid to resolve that source into a **Methodology Profile** consumed by subsequent epics (scaffolding in `FEAT-01` and structure/drift validation in `FEAT-02`).

## 2. Acceptance Criteria
- [ ] User can declare a methodology source (local path or Git repository URL) in workspace/project configuration.
- [ ] **Configuration is performed via a dedicated UI screen (Astryx), without requiring manual file editing** — target audience includes non-technical stakeholders (PO/PM/Designer) who do not use the terminal or code editor.
- [ ] The screen displays clear, non-technical feedback on resolution results (valid source, detected methodology, read/connection error, fallback applied).
- [ ] Liquid resolves the source (clone/read) and extracts an agent-interpretable Methodology Profile, even when the source is an informal, unstructured set of rules.
- [ ] In the absence of configuration, Liquid's native SCPE v0.3.0 is used as the default methodology (fallback).
- [ ] End-to-end implementation validated by tests.
- [ ] Zero performance impact on the Liquid runtime.
