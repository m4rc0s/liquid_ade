# 📐 Technical Plan — Methodology Config Resolver & Adapter

**Epic:** `EPIC-03` of Feature `FEAT-01`  
**Status:** `Draft` 📝

---

## 1. Architecture & Technical Decisions
- Strict implementation using Rust, React 19, Astryx, and Zustand.
- Respects the Bounded Context boundaries of feature `FEAT-01`.
- New `methodology.source` field in workspace configuration, supporting:
  - Local path (folder or single file within the repository itself).
  - External Git repository URL (cloned/updated in a local read-only cache).
- The methodology is always treated as **plain text interpreted by an AI agent**, never as a rigid binary schema — covering everything from a robust multi-file standard (such as SCPE v0.3.0) to a single informal rulebook file.
- The resolution result is a **Methodology Profile** (expected folder/file structure, naming conventions, and state machine when present), cached and exposed for downstream features to consume.
- **Primary configuration channel is the UI, not the file:** the screen persists `methodology.source` to disk under the hood; manual configuration file editing remains possible (technical persona), but is never a prerequisite for a non-technical user workflow.

## 2. Contracts & Interfaces
- Contracts governed by safe types and communication via JSON / ACP.
- Endpoint `/api/workspace/methodology` (GET) exposes the configured source, resolution status (`resolved`, `unresolved`, `fallback-scpe`), and the interpreted Methodology Profile consumed by the UI and `FEAT-02 / EPIC-02`.
- Endpoint `/api/workspace/methodology` (PUT) writes the new source from the configuration screen and triggers synchronous resolution, returning results (success, readable error, or fallback) for immediate UI feedback.
- Methodology Configuration Screen (Astryx modal, consistent with the AI settings modal in `FEAT-05`): single field for path/URL, "Detect" button, resolved Methodology Profile preview, and an option to revert to native SCPE.
