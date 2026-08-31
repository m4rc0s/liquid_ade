# 📋 TASKS — Methodology Config Resolver & Adapter

**Epic:** `EPIC-03` of Feature `FEAT-01`  
**Status:** `Draft` 📝

---

## 📝 Atomic Tasks Checklist

- [ ] **TASK-01.3.1**: Define `methodology.source` field in workspace/project configuration (local path or Git repository URL).
- [ ] **TASK-01.3.2**: Implement resolver in Rust that locates/clones the specified source and reads its content as text.
- [ ] **TASK-01.3.3**: Implement AI agent interpretation extracting a Methodology Profile (expected folder/file structure, conventions, state machine when present) from both formal and informal sources.
- [ ] **TASK-01.3.4**: Apply automatic fallback to native SCPE v0.3.0 when no source is configured or resolution fails.
- [ ] **TASK-01.3.5**: Expose `/api/workspace/methodology` (GET/PUT) endpoint with source, status, and resolved Methodology Profile.
- [ ] **TASK-01.3.6**: Build the **Methodology Configuration Screen in Astryx** (path/URL input, detect button, resolved profile preview, and revert to native SCPE option) accessible without a terminal or code editor, for use by non-technical personas (PO/PM/Designer).
- [ ] **TASK-01.3.7**: Use the resolved Methodology Profile to generate (*scaffold*) new spec files matching the configured source structure and templates.

---

## ✅ Definition of Done
- [ ] All tasks completed and tested.
- [ ] Strict compliance with SCPE v0.3.0.
