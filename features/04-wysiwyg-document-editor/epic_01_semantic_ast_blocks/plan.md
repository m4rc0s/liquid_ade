# 📐 Plano Técnico — Epic 01: Notion-Style Semantic Blocks

**Épico:** `EPIC-01` da Feature `FEAT-04`  
**Status:** `Draft` 📝  

---

## 1. Esquema de Blocos de Interface (Estilo Linear / Notion)

```typescript
export type BlockType = 
  | 'header'          // Título e chips de metadados
  | 'section'         // H1, H2, H3 com âncora
  | 'criteria_list'   // Checklists de critérios de aceite e tarefas
  | 'callout'         // Avisos e destaques visuais
  | 'raw';            // Fallback para parágrafos e código

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

## 2. Interações do Editor
- **Menu Slash (`/`):** Digitar `/` em uma linha vazia abre menu suspenso de inserção rápida (Seção, Checklist de Critérios, Callout).
- **Toggle de Checkbox:** Clique direto no checkbox atualiza `[ ]` ⇄ `[x]` com debounce e patch cirúrgico.
- **Chips de Metadados:** Clique no chip de status abre menu de seleção rápida de estados do SCPE.
