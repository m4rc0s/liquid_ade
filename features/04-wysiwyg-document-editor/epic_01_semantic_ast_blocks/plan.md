# 📐 Technical Plan — Epic 01: Notion-Style Semantic Blocks

**Epic:** `EPIC-01` of Feature `FEAT-04`  
**Status:** `Draft` 📝  

---

## 1. UI Block Schema (Linear / Notion Style)

```typescript
export type BlockType = 
  | 'header'          // Title and metadata chips
  | 'section'         // H1, H2, H3 with anchor
  | 'criteria_list'   // Acceptance criteria checklists and tasks
  | 'callout'         // Warning and visual highlight blocks
  | 'raw';            // Fallback for generic paragraphs and code

export interface BaseBlock {
  id: string;
  type: BlockType;
  startLine: number;
  endLine: number;
  rawText: string;
}

export interface HeaderBlock extends BaseBlock {
  type: 'header';
  title: string;
  metaChips: Array<{ label: string; value: string; kind?: 'status' | 'id' | 'generic' }>;
}

export interface CriteriaItem {
  id: string;
  checked: boolean;
  text: string;
  lineNumber: number;
}

export interface CriteriaListBlock extends BaseBlock {
  type: 'criteria_list';
  items: CriteriaItem[];
}

export interface CalloutBlock extends BaseBlock {
  type: 'callout';
  text: string;
  icon?: string;
}
```

---

## 2. Editor Interactions
- **Slash Menu (`/`):** Typing `/` on an empty line opens a dropdown menu for quick insertion (Section, Criteria Checklist, Callout).
- **Checkbox Toggle:** Clicking directly on a checkbox toggles `[ ]` ⇄ `[x]` with debouncing and surgical disk patching.
- **Metadata Chips:** Clicking a status chip opens a quick-selection menu for SCPE states.
