# 📄 Global PRD — FEAT-04: WYSIWYG Document Editor (Document-as-UI)

- **goal:** Provide Document-as-UI editing with semantic AST blocks and surgical line patchers
- **users:** Product Designers, Technical Writers, and Engineers
- **bounded_context:** WYSIWYG Document Editor
- **business_value:** Transforms static markdown files into rich interactive UI components with surgical disk writes


**Feature ID:** `FEAT-04`  
**Name:** WYSIWYG Document Editor (Linear / Notion Style)  
**Project:** Liquid ADE  
**Methodology:** SCPE v0.3.0  
**Global Status:** `Draft` 📝  

---

## 1. Executive Vision & User Journey

### 1.1. End-to-End Vertical Journey
> *"As a Product Owner, I open the Notion/Linear style specification editor. I can type naturally, use the slash menu (`/`) to insert sections and acceptance criteria checklists, click status chips to change epic states, and Liquid writes surgical patches directly to the `.md` file on disk without breaking original formatting."*

### 1.2. Bounded Context Scope
This feature transforms Markdown files into clean, elegant interactive blocks:
1. **Pills & Metadata (Header):** Tactile chips to toggle statuses, IDs, and priorities with a single click.
2. **Acceptance Criteria Checklists:** Interactive checkboxes with a visual completion progress bar.
3. **Slash Menu (`/`):** Fluid insertion of headings, lists, and callouts in Notion style.
4. **Surgical Line Patcher:** Direct line-by-line disk patching on the `.md` file with echo suppression.

---

## 2. Invariants & Non-Negotiable Rules

1. **Zero Data Loss:** 100% of original Markdown content is preserved. Unrecognized sections use the `RawBlock` fallback.
2. **Surgical Patching:** Targeted edits (e.g., ticking a checkbox) modify strictly the corresponding line on disk.
3. **Minimalist Styling:** Clean interface in Linear/Notion style, fast and keyboard-centric.

---

## 3. Epic Architecture

| Epic | Name | Description | Status |
| :--- | :--- | :--- | :---: |
| **`EPIC-01`** | **Notion-Style Semantic Blocks** | Semantic parser and visual block components (chips, checklists, callouts) built with Astryx. | `Draft` 📝 |
| **`EPIC-02`** | **Surgical Line Patcher & Echo Suppression** | Surgical disk-writing engine with update loop protection and keyboard shortcuts. | `Draft` 📝 |

## Epics

- [epic_01_semantic_ast_blocks](epics/epic_01_semantic_ast_blocks/index.md) — Draft
- [epic_02_surgical_line_patcher](epics/epic_02_surgical_line_patcher/index.md) — Draft
